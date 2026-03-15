use std::collections::HashMap;
use std::io::{self, BufRead};
use std::path::PathBuf;

use tauri_app_lib::commands;
use tauri_app_lib::state::TerminalState;

fn main() {
    // Initialize terminal state (mirrors what the Tauri app does)
    let mut env_vars: HashMap<String, String> = std::env::vars().collect();
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));

    let mut st = TerminalState {
        cwd,
        env: env_vars,
        aliases: HashMap::new(),
        history: Vec::new(),
    };

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l.trim().to_owned(),
            Err(_) => break,
        };

        if line.is_empty() {
            continue;
        }
        if line == "exit" || line == "quit" {
            break;
        }

        let result = commands::execute_single_command_pub(&line, None, &mut st);

        let json_out = match result {
            Ok(output) => serde_json::json!({
                "command": line,
                "status": "ok",
                "output": output
            }),
            Err(err) => serde_json::json!({
                "command": line,
                "status": "error",
                "output": err
            }),
        };

        println!("{}", json_out);
    }
}
