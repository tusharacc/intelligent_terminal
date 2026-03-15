use crate::state::TerminalState;

pub fn execute(
    cmd: &str,
    args: &[&str],
    stdin: Option<String>,
    st: &mut TerminalState,
) -> Result<String, String> {
    // GNU_PARITY_INJECTED: Auto-generated standard GNU flag interceptor
    if args.contains(&"--help") || args.contains(&"-h") {
        return Ok(r#"shuf - Linuxcommand.org Standard Parity

  -e, --echo            treat each ARG as an input line
  -i, --input-range     treat each number LO through HI as an input line
  -n, --head-count      output at most COUNT lines
  -o, --output          write result to FILE instead of standard output
  --random-source       get random bytes from FILE
  -r, --repeat          output lines can be repeated
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
                "-e", "--echo", "-i", "--input-range",
                "-n", "--head-count", "-o", "--output",
                "--random-source", "-r", "--repeat", "-z",
                "--zero-terminated", "--version",
            ];
            for flag in missing_flags.iter() {
                if parsed_flags.contains(*flag) {
                    return Err(format!("shuf: flag {} is not yet implemented in this terminal", flag));
                }
            }
        }


    {
            use rand::seq::SliceRandom;
            let content = if args.is_empty() {
                if let Some(input) = stdin { input } else { return Err("shuf: missing operand".to_string()); }
            } else {
                let path = st.cwd.join(args[0]);
                std::fs::read_to_string(path).map_err(|e| format!("shuf: {}", e))?
            };
            let mut lines: Vec<&str> = content.lines().collect();
            let mut rng = rand::thread_rng();
            lines.shuffle(&mut rng);
            Ok(lines.join("\n"))
        }
}
