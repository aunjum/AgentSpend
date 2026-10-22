use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use rusqlite::{params, Connection};
use serde::Deserialize;
use std::{
    fs,
    io::{self, BufRead},
    path::{Path, PathBuf},
    time::Duration,
};

#[derive(Parser)]
#[command(
    name = "agentspend",
    version,
    about = "Your AI works hard. Your wallet shouldn't."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Init,
    Status,
    Scan,
    Watch,
    Dashboard,
    Report,
    Analyze,
    Optimize,
    Doctor,
}

#[derive(Deserialize)]
struct Event {
    timestamp: Option<String>,
    #[serde(rename = "type")]
    kind: Option<String>,
    payload: Option<Payload>,
}
#[derive(Deserialize)]
struct Payload {
    session_id: Option<String>,
    model: Option<String>,
    usage: Option<Usage>,
    turn_token_usage: Option<Usage>,
}
#[derive(Deserialize, Default)]
struct Usage {
    input_tokens: Option<i64>,
    cached_input_tokens: Option<i64>,
    cache_write_input_tokens: Option<i64>,
    output_tokens: Option<i64>,
}
#[derive(Deserialize)]
struct ClaudeEvent {
    #[serde(rename = "type")]
    kind: Option<String>,
    uuid: Option<String>,
    #[serde(rename = "sessionId", alias = "session_id")]
    session_id: Option<String>,
    timestamp: Option<String>,
    message: Option<ClaudeMessage>,
}
#[derive(Deserialize)]
struct ClaudeMessage {
    role: Option<String>,
    model: Option<String>,
    usage: Option<ClaudeUsage>,
}
#[derive(Deserialize)]
struct ClaudeUsage {
    input_tokens: Option<i64>,
    output_tokens: Option<i64>,
    cache_read_input_tokens: Option<i64>,
    cache_creation_input_tokens: Option<i64>,
}

fn db_path() -> Result<PathBuf> {
    Ok(dirs::data_local_dir()
        .context("No local data directory")?
        .join("agentspend/usage.sqlite3"))
}
fn db() -> Result<Connection> {
    let path = db_path()?;
    fs::create_dir_all(path.parent().unwrap())?;
    let c = Connection::open(path)?;
    schema(&c)?;
    Ok(c)
}
fn schema(c: &Connection) -> Result<()> {
    c.execute_batch("CREATE TABLE IF NOT EXISTS events (id TEXT PRIMARY KEY, agent TEXT NOT NULL, session TEXT, model TEXT, at TEXT, input INTEGER NOT NULL, cached INTEGER NOT NULL, cache_write INTEGER NOT NULL, output INTEGER NOT NULL);")?;
    Ok(())
}
fn locations() -> Vec<(&'static str, PathBuf)> {
    let h = dirs::home_dir().unwrap_or_default();
    vec![
        ("codex", h.join(".codex/sessions")),
        ("claude", h.join(".claude/projects")),
    ]
}
fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for e in entries.flatten() {
            let p = e.path();
            if let Ok(kind) = e.file_type() {
                if kind.is_dir() {
                    walk(&p, out)
                } else if kind.is_file() && p.extension().is_some_and(|x| x == "jsonl") {
                    out.push(p)
                }
            }
        }
    }
}
fn scan() -> Result<usize> {
    let c = db()?;
    let mut count = 0;
    for (agent, root) in locations() {
        if !root.exists() {
            continue;
        }
        let mut files = vec![];
        walk(&root, &mut files);
        for file in files {
            if agent == "codex" || agent == "claude" {
                count += import_file(&c, agent, &file)?;
            }
        }
    }
    Ok(count)
}
fn import_file(c: &Connection, agent: &str, file: &Path) -> Result<usize> {
    let f = fs::File::open(file)?;
    let mut count = 0;
    for (n, line) in io::BufReader::new(f).lines().enumerate() {
        let Ok(line) = line else { continue };
        if agent == "claude" {
            let Ok(e) = serde_json::from_str::<ClaudeEvent>(&line) else {
                continue;
            };
            if e.kind.as_deref() != Some("assistant") {
                continue;
            }
            let Some(message) = e.message else { continue };
            if message.role.as_deref() != Some("assistant") {
                continue;
            }
            let Some(u) = message.usage else { continue };
            let session = e.session_id.unwrap_or_default();
            let id = format!(
                "claude:{}:{}",
                file.display(),
                e.uuid.unwrap_or_else(|| n.to_string())
            );
            count += insert_usage(
                c,
                &id,
                agent,
                &session,
                message.model.as_deref().unwrap_or_default(),
                e.timestamp.as_deref().unwrap_or_default(),
                u.input_tokens.unwrap_or(0),
                u.cache_read_input_tokens.unwrap_or(0),
                u.cache_creation_input_tokens.unwrap_or(0),
                u.output_tokens.unwrap_or(0),
            )?;
            continue;
        }
        let Ok(e) = serde_json::from_str::<Event>(&line) else {
            continue;
        };
        if e.kind.as_deref() != Some("token_usage_record") {
            continue;
        }
        let Some(p) = e.payload else { continue };
        let Some(u) = p.turn_token_usage.or(p.usage) else {
            continue;
        };
        let session = p.session_id.unwrap_or_default();
        let id = format!("{}:{}:{}", file.display(), n, session);
        count += insert_usage(
            c,
            &id,
            agent,
            &session,
            p.model.as_deref().unwrap_or_default(),
            e.timestamp.as_deref().unwrap_or_default(),
            u.input_tokens.unwrap_or(0),
            u.cached_input_tokens.unwrap_or(0),
            u.cache_write_input_tokens.unwrap_or(0),
            u.output_tokens.unwrap_or(0),
        )?;
    }
    Ok(count)
}
#[allow(clippy::too_many_arguments)]
fn insert_usage(
    c: &Connection,
    id: &str,
    agent: &str,
    session: &str,
    model: &str,
    at: &str,
    input: i64,
    cached: i64,
    cache_write: i64,
    output: i64,
) -> Result<usize> {
    Ok(c.execute(
        "INSERT OR IGNORE INTO events VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9)",
        params![
            id,
            agent,
            session,
            model,
            at,
            input,
            cached,
            cache_write,
            output
        ],
    )?)
}
fn totals(c: &Connection) -> Result<(i64, i64, i64, i64, i64)> {
    Ok(c.query_row("SELECT COUNT(*),COALESCE(SUM(input),0),COALESCE(SUM(cached),0),COALESCE(SUM(cache_write),0),COALESCE(SUM(output),0) FROM events",[],|r|Ok((r.get(0)?,r.get(1)?,r.get(2)?,r.get(3)?,r.get(4)?)))?)
}
#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Init => {
            db()?;
            println!("Initialized local database at {}", db_path()?.display());
        }
        Command::Scan => println!("Imported {} new agent usage records.", scan()?),
        Command::Status | Command::Report | Command::Analyze | Command::Optimize => {
            scan()?;
            let c = db()?;
            let (n, i, ca, cw, o) = totals(&c)?;
            println!("AgentSpend overview\nRecords: {n}\nInput: {i} | Cached input: {ca} | Cache write: {cw} | Output: {o}\nAPI-equivalent cost: not calculated (pricing is not implemented)\nDetected: {}",locations().iter().filter(|(_,p)|p.exists()).map(|(a,_)|*a).collect::<Vec<_>>().join(", "));
            if n == 0 {
                println!("No supported usage records found yet.")
            }
        }
        Command::Doctor => {
            println!(
                "Codex: {}",
                if locations()[0].1.exists() {
                    "session directory found"
                } else {
                    "not found"
                }
            );
            println!(
                "Claude Code: {} (imports local assistant usage records when present)",
                if locations()[1].1.exists() {
                    "session directory found"
                } else {
                    "not found"
                }
            );
            println!("Ollama/Gemma analysis: not implemented.");
        }
        Command::Watch => {
            println!("Watching local Codex sessions; press Ctrl-C to stop.");
            loop {
                let n = scan()?;
                if n > 0 {
                    println!("Imported {n} new records.")
                }
                tokio::time::sleep(Duration::from_secs(10)).await
            }
        }
        Command::Dashboard => {
            scan()?;
            let c = db()?;
            let (n, i, ca, cw, o) = totals(&c)?;
            println!("AGENTSPEND OVERVIEW\n\nRecords {n}\nInput {i} · Cached {ca} · Cache write {cw} · Output {o}\n\nLive TUI is not available in this MVP.")
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_explicit_codex_turn_usage() {
        let event: Event = serde_json::from_str(r#"{"type":"token_usage_record","timestamp":"2026-10-22T00:00:00Z","payload":{"session_id":"s1","turn_token_usage":{"input_tokens":100,"cached_input_tokens":40,"output_tokens":20}}}"#).unwrap();
        assert_eq!(event.kind.as_deref(), Some("token_usage_record"));
        let usage = event.payload.unwrap().turn_token_usage.unwrap();
        assert_eq!(usage.input_tokens, Some(100));
        assert_eq!(usage.cached_input_tokens, Some(40));
        assert_eq!(usage.output_tokens, Some(20));
    }

    #[test]
    fn malformed_events_are_skipped_and_unrelated_events_have_no_usage() {
        assert!(serde_json::from_str::<Event>("not-json").is_err());
        let event: Event = serde_json::from_str(r#"{"type":"response_item"}"#).unwrap();
        assert_ne!(event.kind.as_deref(), Some("token_usage_record"));
    }

    #[test]
    fn repeated_imports_are_deduplicated() {
        let root = std::env::temp_dir().join(format!(
            "agentspend-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&root).unwrap();
        let file = root.join("session.jsonl");
        fs::write(&file, r#"{"type":"token_usage_record","payload":{"session_id":"s1","turn_token_usage":{"input_tokens":10,"output_tokens":2}}}"#).unwrap();
        let c = Connection::open_in_memory().unwrap();
        schema(&c).unwrap();
        assert_eq!(import_file(&c, "codex", &file).unwrap(), 1);
        assert_eq!(import_file(&c, "codex", &file).unwrap(), 0);
        assert_eq!(totals(&c).unwrap().0, 1);
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn imports_claude_usage_and_cache_fields() {
        let root = std::env::temp_dir().join(format!(
            "agentspend-claude-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&root).unwrap();
        let file = root.join("session.jsonl");
        fs::write(&file, r#"{"type":"assistant","uuid":"m1","sessionId":"s1","timestamp":"2026-10-22T00:00:00Z","message":{"role":"assistant","model":"claude-test","usage":{"input_tokens":12,"output_tokens":3,"cache_read_input_tokens":8,"cache_creation_input_tokens":5}}}"#).unwrap();
        let c = Connection::open_in_memory().unwrap();
        schema(&c).unwrap();
        assert_eq!(import_file(&c, "claude", &file).unwrap(), 1);
        let totals = totals(&c).unwrap();
        assert_eq!((totals.1, totals.2, totals.3, totals.4), (12, 8, 5, 3));
        assert_eq!(import_file(&c, "claude", &file).unwrap(), 0);
        fs::remove_dir_all(root).unwrap();
    }
}
