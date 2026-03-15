use crate::state::TerminalState;

pub fn execute(
    cmd: &str,
    args: &[&str],
    stdin: Option<String>,
    st: &mut TerminalState,
) -> Result<String, String> {
    // GNU_PARITY_INJECTED: Auto-generated standard GNU flag interceptor
    if args.contains(&"--help") || args.contains(&"-h") {
        return Ok(r#"tac - Linuxcommand.org Standard Parity

  -b, --before          attach the separator before instead of after
  -r, --regex           interpret the separator as a regular expression
  -s, --separator=STRING  use STRING as the separator instead of newline
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
        for flag in ["-b", "--before", "-r", "--regex", "-s", "--separator", "--version"].iter() {
            if parsed_flags.contains(*flag) {
                return Err(format!("tac: flag {} is not yet implemented in this terminal", flag));
            }
        }


    {
            let content = if args.is_empty() {
                if let Some(input) = stdin { input } else { return Err("tac: missing operand".to_string()); }
            } else {
                let path = st.cwd.join(args[0]);
                std::fs::read_to_string(path).map_err(|e| format!("tac: {}", e))?
            };
            let mut lines: Vec<&str> = content.lines().collect();
            lines.reverse();
            Ok(lines.join("\n"))
        }
}
