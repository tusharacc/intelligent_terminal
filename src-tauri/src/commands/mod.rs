use std::sync::Mutex;
use tauri::State;
use crate::state::TerminalState;
pub mod echo;
pub mod pwd;
pub mod cd;
pub mod history;
pub mod true_cmd;
pub mod false_cmd;
pub mod date;
pub mod env;
pub mod printenv;
pub mod set;
pub mod dir;
pub mod vdir;
pub mod export;
pub mod unset;
pub mod alias;
pub mod sleep;
pub mod seq;
pub mod yes;
pub mod exit;
pub mod ls;
pub mod cp;
pub mod mv;
pub mod rm;
pub mod mkdir;
pub mod rmdir;
pub mod touch;
pub mod cat;
pub mod head;
pub mod tail;
pub mod tac;
pub mod realpath;
pub mod basename;
pub mod dirname;
pub mod grep;
pub mod wc;
pub mod sort;
pub mod uniq;
pub mod shuf;
pub mod nl;
pub mod base64;
pub mod base32_cmd;
pub mod cksum;
pub mod md5sum;
pub mod sha512sum;
pub mod b2sum;
pub mod cut;
pub mod paste;
pub mod tr;
pub mod find;
pub mod gzip;
pub mod gunzip;
pub mod zcat;
pub mod tar;
pub mod nslookup;
pub mod netstat;
pub mod ping;
pub mod nmap;
pub mod tracert;


#[tauri::command]
pub fn execute_command(command: &str, state: State<'_, Mutex<TerminalState>>) -> Result<String, String> {
    let mut st = state.lock().unwrap();
    
    // Add to history
    st.history.push(command.to_owned());

    let pipes: Vec<&str> = command.split('|').collect();
    let mut last_output: Option<String> = None;

    for pipe_cmd in pipes {
        let trimmed = pipe_cmd.trim();
        if trimmed.is_empty() { continue; }
        
        match execute_single_command(trimmed, last_output.take(), &mut st) {
            Ok(out) => last_output = Some(out),
            Err(e) => return Err(e),
        }
    }
    
    Ok(last_output.unwrap_or_default())
}

fn execute_single_command(
    command: &str,
    stdin: Option<String>,
    st: &mut TerminalState
) -> Result<String, String> {
    // Basic parsing: splitting by whitespace (ignoring quotes for now)
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.is_empty() {
        return Ok(String::new());
    }

    let cmd = parts[0];
    let args = &parts[1..];

    match cmd {
        "echo" => echo::execute(cmd, args, stdin, st),
        "pwd" => pwd::execute(cmd, args, stdin, st),
        "cd" => cd::execute(cmd, args, stdin, st),
        "history" => history::execute(cmd, args, stdin, st),
        "true" => true_cmd::execute(cmd, args, stdin, st),
        "false" => false_cmd::execute(cmd, args, stdin, st),
        "date" => date::execute(cmd, args, stdin, st),
        "env" => env::execute(cmd, args, stdin, st),
        "printenv" => printenv::execute(cmd, args, stdin, st),
        "set" => set::execute(cmd, args, stdin, st),
        "export" => export::execute(cmd, args, stdin, st),
        "unset" => unset::execute(cmd, args, stdin, st),
        "alias" => alias::execute(cmd, args, stdin, st),
        "sleep" => sleep::execute(cmd, args, stdin, st),
        "seq" => seq::execute(cmd, args, stdin, st),
        "yes" => yes::execute(cmd, args, stdin, st),
        "exit" => exit::execute(cmd, args, stdin, st),
        "ls" => ls::execute(cmd, args, stdin, st),
        "dir" => dir::execute(cmd, args, stdin, st),
        "vdir" => vdir::execute(cmd, args, stdin, st),
        "cp" => cp::execute(cmd, args, stdin, st),
        "mv" => mv::execute(cmd, args, stdin, st),
        "rm" => rm::execute(cmd, args, stdin, st),
        "mkdir" => mkdir::execute(cmd, args, stdin, st),
        "rmdir" => rmdir::execute(cmd, args, stdin, st),
        "touch" => touch::execute(cmd, args, stdin, st),
        "cat" => cat::execute(cmd, args, stdin, st),
        "head" => head::execute(cmd, args, stdin, st),
        "tail" => tail::execute(cmd, args, stdin, st),
        "tac" => tac::execute(cmd, args, stdin, st),
        "realpath" => realpath::execute(cmd, args, stdin, st),
        "basename" => basename::execute(cmd, args, stdin, st),
        "dirname" => dirname::execute(cmd, args, stdin, st),
        "grep" => grep::execute(cmd, args, stdin, st),
        "wc" => wc::execute(cmd, args, stdin, st),
        "sort" => sort::execute(cmd, args, stdin, st),
        "uniq" => uniq::execute(cmd, args, stdin, st),
        "shuf" => shuf::execute(cmd, args, stdin, st),
        "nl" => nl::execute(cmd, args, stdin, st),
        "base64" => base64::execute(cmd, args, stdin, st),
        "base32" => base32_cmd::execute(cmd, args, stdin, st),
        "cksum" => cksum::execute(cmd, args, stdin, st),
        "md5sum" => md5sum::execute(cmd, args, stdin, st),
        "sha512sum" => sha512sum::execute(cmd, args, stdin, st),
        "b2sum" => b2sum::execute(cmd, args, stdin, st),
        "cut" => cut::execute(cmd, args, stdin, st),
        "paste" => paste::execute(cmd, args, stdin, st),
        "tr" => tr::execute(cmd, args, stdin, st),
        "find" => find::execute(cmd, args, stdin, st),
        "gzip" => gzip::execute(cmd, args, stdin, st),
        "gunzip" => gunzip::execute(cmd, args, stdin, st),
        "zcat" => zcat::execute(cmd, args, stdin, st),
        "tar" => tar::execute(cmd, args, stdin, st),
        "nslookup" => nslookup::execute(cmd, args, stdin, st),
        "netstat" => netstat::execute(cmd, args, stdin, st),
        "ping" => ping::execute(cmd, args, stdin, st),
        "nmap" => nmap::execute(cmd, args, stdin, st),
        "tracert" | "traceroute" => tracert::execute(cmd, args, stdin, st),
        _ => Err(format!("{}: command not found", cmd)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_ls_help_output() {
        let mut st = TerminalState {
            cwd: std::path::PathBuf::from("/"),
            env: HashMap::new(),
            aliases: HashMap::new(),
            history: Vec::new(),
        };
        
        // Execute ls --help through the single command router
        let out = execute_single_command("ls --help", None, &mut st).unwrap();
        
        // It should contain GNU parity identifier
        assert!(out.contains("ls - Linuxcommand.org Standard Parity"));
        // It should contain the standard --sort flag description
        assert!(out.contains("--sort"));
    }
}
