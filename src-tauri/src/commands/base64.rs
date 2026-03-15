use crate::state::TerminalState;

pub fn execute(
    cmd: &str,
    args: &[&str],
    stdin: Option<String>,
    st: &mut TerminalState,
) -> Result<String, String> {
    // GNU_PARITY_INJECTED: Auto-generated standard GNU flag interceptor
    if args.contains(&"--help") || args.contains(&"-h") {
        return Ok(r#"base64 - Linuxcommand.org Standard Parity

  -d, --decode          decode data
  -i, --ignore-garbage  when decoding, ignore non-alphabet characters
  -w, --wrap            wrap encoded lines after COLS character (default 76). Use 0 to disable line w...
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
        for flag in ["-d", "--decode", "-i", "--ignore-garbage", "-w", "--wrap", "--version"].iter() {
            if parsed_flags.contains(*flag) {
                return Err(format!("base64: flag {} is not yet implemented in this terminal", flag));
            }
        }


    {
            use base64::{Engine as _, engine::general_purpose};
            let content = if args.is_empty() {
                if let Some(input) = stdin { input.into_bytes() } else { return Err("base64: missing operand".to_string()); }
            } else {
                let path = st.cwd.join(args[0]);
                std::fs::read(path).map_err(|e| format!("base64: {}", e))?
            };
            Ok(general_purpose::STANDARD.encode(&content))
        }
}
