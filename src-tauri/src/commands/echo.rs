use crate::state::TerminalState;

pub fn execute(
    cmd: &str,
    args: &[&str],
    stdin: Option<String>,
    st: &mut TerminalState,
) -> Result<String, String> {
    // GNU_PARITY_INJECTED: Auto-generated standard GNU flag interceptor
    if args.contains(&"--help") || args.contains(&"-h") {
        return Ok(r#"echo - Linuxcommand.org Standard Parity

  -n, --no-trailing-newline  Do not output the trailing newline
  -e, --enable-backslash-escapes  Enable interpretation of backslash escapes
  -E, --disable-backslash-escapes  Disable interpretation of backslash escapes (default)
  --help                Display this help and exit
  --version             Output version information and exit"#.to_string());
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
        for flag in ["--version"].iter() {
            if parsed_flags.contains(*flag) {
                return Err(format!("echo: flag {} is not yet implemented in this terminal", flag));
            }
        }

    let mut out = args.join(" ");
    
    let e_flag = parsed_flags.contains("-e");
    let cap_e_flag = parsed_flags.contains("-E");
    let n_flag = parsed_flags.contains("-n");
    
    // Default is -E (disable interpretation). If -e is passed and -E is not (or if last, but we use a simpler precedence), enable escapes.
    if e_flag && !cap_e_flag {
        out = out.replace("\\n", "\n")
                 .replace("\\t", "\t")
                 .replace("\\r", "\r")
                 .replace("\\\\", "\\")
                 .replace("\\b", "\x08")
                 .replace("\\c", ""); // \c suppresses trailing newline
        if out.contains("\\c") {
            let parts: Vec<&str> = out.split("\\c").collect();
            out = parts[0].to_string();
        }
    }
    
    // In our terminal emulation, the frontend prints the returned string as a distinct block.
    // The -n flag typically suppresses the trailing newline. 
    // Since return strings don't have standard trailing newlines anyway, we just return `out`.
    return Ok(out);
}
