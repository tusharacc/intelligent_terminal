use crate::state::TerminalState;

pub fn execute(
    cmd: &str,
    args: &[&str],
    stdin: Option<String>,
    st: &mut TerminalState,
) -> Result<String, String> {
    // GNU_PARITY_INJECTED: Auto-generated standard GNU flag interceptor
    if args.contains(&"--help") || args.contains(&"-h") {
        return Ok(r#"realpath - Linuxcommand.org Standard Parity

  -e, --canonicalize-existing  all components of the path must exist
  -m, --canonicalize-missing  no path components need exist or be a directory
  -L, --logical         resolve '..' components before symlinks
  -P, --physical        resolve symlinks as encountered (default)
  -q, --quiet           suppress most error messages
  --relative-to=DIR     print the resolved path relative to DIR
  --relative-base=DIR   print absolute paths unless paths below DIR
  -s, --strip, --no-symlinks  don't expand symlinks
  -z, --zero            end each output line with NUL, not newline
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
                "-e", "--canonicalize-existing", "-m", "--canonicalize-missing",
                "-L", "--logical", "-P", "--physical",
                "-q", "--quiet", "--relative-to", "--relative-base",
                "-s", "--strip", "--no-symlinks", "-z",
                "--zero", "--version",
            ];
            for flag in missing_flags.iter() {
                if parsed_flags.contains(*flag) {
                    return Err(format!("realpath: flag {} is not yet implemented in this terminal", flag));
                }
            }
        }


    {
            if args.is_empty() { return Err("realpath: missing operand".to_string()); }
            let path = st.cwd.join(args[0]);
            match std::fs::canonicalize(path) {
                Ok(canon) => Ok(canon.to_string_lossy().into_owned()),
                Err(e) => Err(format!("realpath: {}", e))
            }
        }
}
