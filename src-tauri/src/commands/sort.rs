use crate::state::TerminalState;

pub fn execute(
    cmd: &str,
    args: &[&str],
    stdin: Option<String>,
    st: &mut TerminalState,
) -> Result<String, String> {
    // GNU_PARITY_INJECTED: Auto-generated standard GNU flag interceptor
    if args.contains(&"--help") || args.contains(&"-h") {
        return Ok(r#"sort - Linuxcommand.org Standard Parity

  -b, --ignore-leading-blanks  ignore leading blanks
  -d, --dictionary-order  consider only blanks and alphanumeric characters
  -f, --ignore-case     fold lower case to upper case characters
  -g, --general-numeric-sort  compare according to general numerical value
  -i, --ignore-nonprinting  consider only printable characters
  -M, --month-sort      compare (unknown)"#.to_string());
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
                "-b", "--ignore-leading-blanks", "-d", "--dictionary-order",
                "-f", "--ignore-case", "-g", "--general-numeric-sort",
                "-i", "--ignore-nonprinting", "-M", "--month-sort",
            ];
            for flag in missing_flags.iter() {
                if parsed_flags.contains(*flag) {
                    return Err(format!("sort: flag {} is not yet implemented in this terminal", flag));
                }
            }
        }


    {
            let content = if args.is_empty() {
                if let Some(input) = stdin { input } else { return Err("sort: missing operand".to_string()); }
            } else {
                let path = st.cwd.join(args[0]);
                std::fs::read_to_string(path).map_err(|e| format!("sort: {}", e))?
            };
            let mut lines: Vec<&str> = content.lines().collect();
            lines.sort();
            Ok(lines.join("\n"))
        }
}
