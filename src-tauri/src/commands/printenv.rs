use crate::state::TerminalState;

pub fn execute(
    _cmd: &str,
    args: &[&str],
    _stdin: Option<String>,
    st: &mut TerminalState,
) -> Result<String, String> {
    if args.contains(&"--help") || args.contains(&"-h") {
        return Ok(r#"printenv - Linuxcommand.org Standard Parity

  -0, --null            end each output line with NUL, not newline
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
    let args = &positional_args[..];

    // Handle flags
    for flag in ["-0", "--null", "--version"].iter() {
        if parsed_flags.contains(*flag) {
            return Err(format!("printenv: flag {} is not yet implemented in this terminal", flag));
        }
    }

    if args.is_empty() {
        let mut result = String::new();
        for (k, v) in &st.env {
            result.push_str(&format!("{}={}\n", k, v));
        }
        Ok(result)
    } else {
        match st.env.get(args[0]) {
            Some(val) => Ok(val.clone()),
            None => Err(format!("printenv: '{}': not set", args[0])),
        }
    }
}
