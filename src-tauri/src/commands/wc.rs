use crate::state::TerminalState;

pub fn execute(
    cmd: &str,
    args: &[&str],
    stdin: Option<String>,
    st: &mut TerminalState,
) -> Result<String, String> {
    // GNU_PARITY_INJECTED: Auto-generated standard GNU flag interceptor
    if args.contains(&"--help") || args.contains(&"-h") {
        return Ok(r#"wc - Linuxcommand.org Standard Parity

  -c, --bytes           print the byte counts
  -m, --chars           print the character counts
  -l, --lines           print the newline counts
  --files0-from=F       read input from the files specified by NUL-terminated names in file F; If F i...
  -L, --max-line-length print the maximum display width
  -w, --words           print the word counts
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
                "-c", "--bytes", "-m", "--chars",
                "-l", "--lines", "--files0-from", "-L",
                "--max-line-length", "-w", "--words", "--version",
            ];
            for flag in missing_flags.iter() {
                if parsed_flags.contains(*flag) {
                    return Err(format!("wc: flag {} is not yet implemented in this terminal", flag));
                }
            }
        }


    {
            let (content, label) = if args.is_empty() {
                if let Some(input) = stdin {
                    (input, "".to_string())
                } else {
                    return Err("wc: missing operand".to_string());
                }
            } else {
                let path = st.cwd.join(args[0]);
                (std::fs::read_to_string(path).map_err(|e| format!("wc: {}", e))?, args[0].to_string())
            };
            let lines = content.lines().count();
            let words = content.split_whitespace().count();
            let chars = content.chars().count();
            if label.is_empty() {
                Ok(format!("{} {} {}", lines, words, chars))
            } else {
                Ok(format!("{} {} {} {}", lines, words, chars, label))
            }
        }
}
