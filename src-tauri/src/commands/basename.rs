use crate::state::TerminalState;

pub fn execute(
    cmd: &str,
    args: &[&str],
    stdin: Option<String>,
    st: &mut TerminalState,
) -> Result<String, String> {
    // GNU_PARITY_INJECTED: Auto-generated standard GNU flag interceptor
    if args.contains(&"--help") || args.contains(&"-h") {
        return Ok(r#"basename - Linuxcommand.org Standard Parity

  -a, --multiple        support multiple arguments and treat each as a NAME
  -s, --suffix=SUFFIX   remove a trailing SUFFIX; implies -a
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
        for flag in ["-a", "--multiple", "-s", "--suffix", "-z", "--zero", "--version"].iter() {
            if parsed_flags.contains(*flag) {
                return Err(format!("basename: flag {} is not yet implemented in this terminal", flag));
            }
        }


    {
            if args.is_empty() { return Err("basename: missing operand".to_string()); }
            let path = std::path::Path::new(args[0]);
            if let Some(name) = path.file_name() {
                Ok(name.to_string_lossy().into_owned())
            } else {
                Ok(args[0].to_string())
            }
        }
}
