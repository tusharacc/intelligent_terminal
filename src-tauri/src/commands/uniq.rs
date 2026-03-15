use crate::state::TerminalState;

pub fn execute(
    cmd: &str,
    args: &[&str],
    stdin: Option<String>,
    st: &mut TerminalState,
) -> Result<String, String> {
    // GNU_PARITY_INJECTED: Auto-generated standard GNU flag interceptor
    if args.contains(&"--help") || args.contains(&"-h") {
        return Ok(r#"uniq - Linuxcommand.org Standard Parity

  -c, --count           prefix lines by the number of occurrences
  -d, --repeated        only print duplicate lines, one for each group
  -D                    print all duplicate lines
  --all-repeated        like -D, but allow separating groups with an empty line; METHOD={none(default...
  -f, --skip-fields=N   avoid comparing the first N fields
  --group               show all items, separating groups with an empty line; METHOD={separate(defaul...
  -i, --ignore-case     ignore differences in case when comparing
  -s, --skip-chars=N    avoid comparing the first N characters
  -u, --unique          only print unique lines
  -z, --zero-terminated line delimiter is NUL, not newline
  -w, --check-chars=N   compare no more than N characters in lines
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
                "-c", "--count", "-d", "--repeated",
                "--all-repeated", "-f", "--skip-fields", "--group",
                "-i", "--ignore-case", "-s", "--skip-chars",
                "-u", "--unique", "-z", "--zero-terminated",
                "-w", "--check-chars", "--version",
            ];
            for flag in missing_flags.iter() {
                if parsed_flags.contains(*flag) {
                    return Err(format!("uniq: flag {} is not yet implemented in this terminal", flag));
                }
            }
        }


    {
        let d_flag = parsed_flags.contains("-D");
        if d_flag {
            return Err("uniq: -D (print all duplicate lines) not supported natively in this basic terminal".to_string());
        }

        let content = if args.is_empty() {
                if let Some(input) = stdin { input } else { return Err("uniq: missing operand".to_string()); }
            } else {
                let path = st.cwd.join(args[0]);
                std::fs::read_to_string(path).map_err(|e| format!("uniq: {}", e))?
            };
            let mut lines: Vec<&str> = content.lines().collect();
            lines.dedup();
            Ok(lines.join("\n"))
        }
}
