use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use std::sync::Mutex;

pub struct TerminalState {
    pub cwd: PathBuf,
    pub env: HashMap<String, String>,
    pub aliases: HashMap<String, String>,
    pub history: Vec<String>,
}

impl TerminalState {
    pub fn new() -> Self {
        let current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));
        
        let mut env_map = HashMap::new();
        for (key, val) in env::vars() {
            env_map.insert(key, val);
        }

        Self {
            cwd: current_dir,
            env: env_map,
            aliases: HashMap::new(),
            history: Vec::new(),
        }
    }
}
