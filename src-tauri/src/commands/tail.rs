use crate::state::TerminalState;

pub fn execute(
    cmd: &str,
    args: &[&str],
    stdin: Option<String>,
    st: &mut TerminalState,
) -> Result<String, String> {
    // GNU_PARITY_INJECTED: Auto-generated standard GNU flag interceptor
    if args.contains(&"--help") || args.contains(&"-h") {
        return Ok(r#"tail - Linuxcommand.org Standard Parity

  -c, --bytes           output the last NUM bytes; or use -c +NUM to output starting with byte NUM of...
  -f, --follow          output appended data as the file grows;
  -F                    same as --follow=name --retry
  -n, --lines           output the last NUM lines, instead of the last 10; or use -n +NUM to output s...
  --max-unchanged-stats with --follow=name, reopen a FILE which has not changed size after N (default...
  --pid                 with -f, terminate after process ID, PID dies
  -q, --quiet, --silent never output headers giving file names
  --retry               keep trying to open a file if it is inaccessible
  -s, --sleep-interval  with -f, sleep for approximately N seconds (default 1.0) between iterations; ...
  -v, --verbose         always output headers giving file names
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
                "--max-unchanged-stats", "--pid", "-q", "--quiet",
                "--silent", "--retry", "-s", "--sleep-interval",
                "-v", "--verbose", "-z", "--zero-terminated",
                "--version",
            ];
            for flag in missing_flags.iter() {
                if parsed_flags.contains(*flag) {
                    return Err(format!("tail: flag {} is not yet implemented in this terminal", flag));
                }
            }
        }


    {
        let f_flag = parsed_flags.contains("-F") || parsed_flags.contains("-f");
        if f_flag {
            return Err("tail: -F/-f (follow mode) not supported natively in this basic terminal".to_string());
        }

        let content = if args.is_empty() {
                if let Some(input) = stdin { input } else { return Err("tail: missing operand".to_string()); }
            } else {
                let path = st.cwd.join(args[0]);
                std::fs::read_to_string(path).map_err(|e| format!("tail: {}", e))?
            };
            let lines: Vec<&str> = content.lines().collect();
            let start = if lines.len() > 10 { lines.len() - 10 } else { 0 };
            Ok(lines[start..].join("\n"))
        }
}
