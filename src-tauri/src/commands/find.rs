use crate::state::TerminalState;

pub fn execute(
    cmd: &str,
    args: &[&str],
    stdin: Option<String>,
    st: &mut TerminalState,
) -> Result<String, String> {
    // GNU_PARITY_INJECTED: Auto-generated standard GNU flag interceptor
    if args.contains(&"--help") || args.contains(&"-h") {
        return Ok(r#"find - Linuxcommand.org Standard Parity

  -H, --no-follow       Do not follow symbolic links, except while processing command line arguments
  -L, --follow          Follow symbolic links
  -P, --no-dereference  Never follow symbolic links
  -D, --debug           Print diagnostic information; this can be helpful to diagnose problems with w...
  --help                For a complete list of valid debug options, see the output of find -D help"#.to_string());
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


    {
        let unsupported_flags = ["-L", "-P", "-H", "-D"];
        for flag in unsupported_flags.iter() {
            if parsed_flags.contains(*flag) {
                return Err(format!("find: advanced symlink/debug flag {} not supported natively", flag));
            }
        }

        let target = if args.is_empty() { st.cwd.clone() } else { st.cwd.join(args[0]) };
            let mut result = String::new();
            for entry in walkdir::WalkDir::new(target).into_iter().filter_map(|e| e.ok()) {
                result.push_str(&format!("{}\n", entry.path().display()));
            }
            if !result.is_empty() { result.pop(); }
            Ok(result)
        }
}
