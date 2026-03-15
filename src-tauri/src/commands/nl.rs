use crate::state::TerminalState;

pub fn execute(
    cmd: &str,
    args: &[&str],
    stdin: Option<String>,
    st: &mut TerminalState,
) -> Result<String, String> {
    // GNU_PARITY_INJECTED: Auto-generated standard GNU flag interceptor
    if args.contains(&"--help") || args.contains(&"-h") {
        return Ok(r#"nl - Linuxcommand.org Standard Parity

  -b, --body-numbering  use STYLE for numbering body lines
  -d, --section-delimiter  use CC for logical page delimiters
  -f, --footer-numbering  use STYLE for numbering footer lines
  -h, --header-numbering  use STYLE for numbering header lines
  -i, --line-increment  line number increment at each line
  -l, --join-blank-lines  group of NUMBER empty lines counted as one
  -n, --number-format   insert line numbers according to FORMAT
  -p, --no-renumber     do not reset line numbers for each section
  -s, --number-separator  add STRING after (possible) line number
  -v, --starting-line-number  first line number for each section
  -w, --number-width    use NUMBER columns for line numbers
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
                "-b", "--body-numbering", "-d", "--section-delimiter",
                "-f", "--footer-numbering", "-i", "--line-increment",
                "-l", "--join-blank-lines", "-n", "--number-format",
                "-p", "--no-renumber", "-s", "--number-separator",
                "-v", "--starting-line-number", "-w", "--number-width",
                "--version",
            ];
            for flag in missing_flags.iter() {
                if parsed_flags.contains(*flag) {
                    return Err(format!("nl: flag {} is not yet implemented in this terminal", flag));
                }
            }
        }


    {
            let content = if args.is_empty() {
                if let Some(input) = stdin { input } else { return Err("nl: missing operand".to_string()); }
            } else {
                let path = st.cwd.join(args[0]);
                std::fs::read_to_string(path).map_err(|e| format!("nl: {}", e))?
            };
            let mut result = String::new();
            for (i, line) in content.lines().enumerate() {
                result.push_str(&format!("{:6}  {}\n", i + 1, line));
            }
            if !result.is_empty() { result.pop(); }
            Ok(result)
        }
}
