use crate::state::TerminalState;

pub fn execute(
    cmd: &str,
    args: &[&str],
    stdin: Option<String>,
    st: &mut TerminalState,
) -> Result<String, String> {
    // GNU_PARITY_INJECTED: Auto-generated standard GNU flag interceptor
    if args.contains(&"--help") || args.contains(&"-h") {
        return Ok(r#"head - Linuxcommand.org Standard Parity

  -c, --bytes           print the first NUM bytes of each file; with the leading '-', print all but t...
  -n, --lines           print the first NUM lines instead of the first 10; with the leading '-', prin...
  -q, --quiet, --silent never print headers giving file names
  -v, --verbose         always print headers giving file names
  -z, --zero-terminated line delimiter is NUL, not newline
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
        {
            let missing_flags: &[&str] = &[
                "-c", "--bytes", "-n", "--lines",
                "-q", "--quiet", "--silent", "-v",
                "--verbose", "-z", "--zero-terminated", "--version",
            ];
            for flag in missing_flags.iter() {
                if parsed_flags.contains(*flag) {
                    return Err(format!("head: flag {} is not yet implemented in this terminal", flag));
                }
            }
        }


    {
            let content = if args.is_empty() {
                if let Some(input) = stdin { input } else { return Err("head: missing operand".to_string()); }
            } else {
                let path = st.cwd.join(args[0]);
                std::fs::read_to_string(path).map_err(|e| format!("head: {}", e))?
            };
            let lines: Vec<&str> = content.lines().take(10).collect();
            Ok(lines.join("\n"))
        }
}
