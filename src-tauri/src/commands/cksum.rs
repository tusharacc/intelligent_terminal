use crate::state::TerminalState;

pub fn execute(
    cmd: &str,
    args: &[&str],
    stdin: Option<String>,
    st: &mut TerminalState,
) -> Result<String, String> {
    // GNU_PARITY_INJECTED: Auto-generated standard GNU flag interceptor
    if args.contains(&"--help") || args.contains(&"-h") {
        return Ok(r#"cksum - Linuxcommand.org Standard Parity

  --help                display this help and exit
  --version             output version information and exit"#.to_string());
    }
    
    let mut positional_args = Vec::new();
    let mut parsed_flags = std::collections::HashSet::new();
    for arg in args {
        if arg.starts_with('-') && *arg != "-" {
            parsed_flags.insert(*arg);
        } else {
            positional_args.push(*arg);
        }
    }
    // Rebind args to only positional for the legacy naive implementation
    let args = &positional_args[..];

        // AUTO-GENERATED: Missing flag handling from missing_flags.yaml
        for flag in ["--version"].iter() {
            if parsed_flags.contains(*flag) {
                return Err(format!("cksum: flag {} is not yet implemented in this terminal", flag));
            }
        }


    {
            let (content, label) = if args.is_empty() {
                if let Some(input) = stdin { (input.into_bytes(), "".to_string()) } else { return Err("cksum: missing operand".to_string()); }
            } else {
                let path = st.cwd.join(args[0]);
                (std::fs::read(path).map_err(|e| format!("cksum: {}", e))?, args[0].to_string())
            };
            match Ok::<_, String>(content) {
                Ok(content) => {
                    let mut hasher = crc32fast::Hasher::new();
                    hasher.update(&content);
                    if label.is_empty() { Ok(format!("{}", hasher.finalize())) } else { Ok(format!("{} {}", hasher.finalize(), label)) }
                },
                Err(e) => Err(format!("cksum: {}", e))
            }
        }
}
