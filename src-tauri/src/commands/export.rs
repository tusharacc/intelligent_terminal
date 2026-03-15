use crate::state::TerminalState;

pub fn execute(
    cmd: &str,
    args: &[&str],
    stdin: Option<String>,
    st: &mut TerminalState,
) -> Result<String, String> {
    // GNU_PARITY_INJECTED: Auto-generated standard GNU flag interceptor
    if args.contains(&"--help") || args.contains(&"-h") {
        return Ok(r#"export - Linuxcommand.org Standard Parity

  -f, --                refer to shell functions
  -n                    remove the export property from each NAME
  -p                    display a list of all exported variables and functions"#.to_string());
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


    let f_flag = parsed_flags.contains("-f");
    let n_flag = parsed_flags.contains("-n");
    let p_flag = parsed_flags.contains("-p");
    
    if f_flag {
        return Err("export: -f (functions) not supported in this shell".to_string());
    }

    if args.is_empty() || p_flag {
        let mut result = String::new();
        for (k, v) in &st.env {
            result.push_str(&format!("declare -x {}=\"{}\"\n", k, v));
        }
        Ok(result)
    } else {
        for arg in args {
            if n_flag {
                // -n un-exports a variable (we emulate by removing it)
                st.env.remove(*arg);
            } else if let Some((k, v)) = arg.split_once('=') {
                st.env.insert(k.to_string(), v.to_string());
            } else {
                // simple export without assignment usually does nothing unless the var exists
            }
        }
        Ok(String::new())
    }
}
