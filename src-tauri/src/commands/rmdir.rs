use crate::state::TerminalState;

pub fn execute(
    cmd: &str,
    args: &[&str],
    stdin: Option<String>,
    st: &mut TerminalState,
) -> Result<String, String> {
    // GNU_PARITY_INJECTED: Auto-generated standard GNU flag interceptor
    if args.contains(&"--help") || args.contains(&"-h") {
        return Ok(r#"rmdir - Linuxcommand.org Standard Parity

  --ignore-fail-on-non-empty  ignore each failure that is solely because a directory is non-empty
  -p, --parents         remove DIRECTORY and its ancestors; e.g., 'rmdir -p a/b/c' is similar to 'rmd...
  -v, --verbose         output a diagnostic for every directory processed
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
        for flag in ["--ignore-fail-on-non-empty", "-p", "--parents", "-v", "--verbose", "--version"].iter() {
            if parsed_flags.contains(*flag) {
                return Err(format!("rmdir: flag {} is not yet implemented in this terminal", flag));
            }
        }


    {
            if args.is_empty() { return Err("rmdir: missing operand".to_string()); }
            let path = st.cwd.join(args[0]);
            match std::fs::remove_dir(path) {
                Ok(_) => Ok(String::new()),
                Err(e) => Err(format!("rmdir: {}", e))
            }
        }
}
