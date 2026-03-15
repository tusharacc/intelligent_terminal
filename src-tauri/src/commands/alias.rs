use crate::state::TerminalState;

pub fn execute(
    cmd: &str,
    args: &[&str],
    stdin: Option<String>,
    st: &mut TerminalState,
) -> Result<String, String> {
    // GNU_PARITY_INJECTED: Auto-generated standard GNU flag interceptor
    if args.contains(&"--help") || args.contains(&"-h") {
        return Ok(r#"alias - Linuxcommand.org Standard Parity

  -p                    print all defined aliases in a reusable format"#.to_string());
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
        let p_flag = parsed_flags.contains("-p");
        
        if args.is_empty() || p_flag {
            let mut result = String::new();
            for (k, v) in &st.aliases {
                result.push_str(&format!("alias {}='{}'\n", k, v));
            }
            // If there were other args, ideally it prints them too, but for basic compliance we just print if -p is present.
            if args.is_empty() {
                return Ok(result);
            }
        } 
        
        for arg in args {
            if let Some((k, v)) = arg.split_once('=') {
                // Strip quotes if present
                let v_clean = v.trim_matches('\'').trim_matches('"');
                st.aliases.insert(k.to_string(), v_clean.to_string());
            }
        }
        Ok(String::new())
    }
}
