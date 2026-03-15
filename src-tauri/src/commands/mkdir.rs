use crate::state::TerminalState;

pub fn execute(
    cmd: &str,
    args: &[&str],
    stdin: Option<String>,
    st: &mut TerminalState,
) -> Result<String, String> {
    // GNU_PARITY_INJECTED: Auto-generated standard GNU flag interceptor
    if args.contains(&"--help") || args.contains(&"-h") {
        return Ok(r#"mkdir - Linuxcommand.org Standard Parity

  -m, --mode=MODE       set file mode (as in chmod), not a=rwx - umask
  -p, --parents         no error if existing, make parent directories as needed
  -v, --verbose         print a message for each created directory
  -Z                    set SELinux security context of each created directory to the default type
  --context[=CTX]       like -Z, or if CTX is specified then set the SELinux or SMACK security contex...
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
        for flag in ["-m", "--mode", "-p", "--parents", "-v", "--verbose", "--context", "--version"].iter() {
            if parsed_flags.contains(*flag) {
                return Err(format!("mkdir: flag {} is not yet implemented in this terminal", flag));
            }
        }


    {
        let z_flag = parsed_flags.contains("-Z");
        if z_flag {
            return Err("mkdir: -Z (SELinux context) not supported in this shell".to_string());
        }

        if args.is_empty() { return Err("mkdir: missing operand".to_string()); }
        let path = st.cwd.join(args[0]);
            match std::fs::create_dir_all(path) {
                Ok(_) => Ok(String::new()),
                Err(e) => Err(format!("mkdir: {}", e))
            }
        }
}
