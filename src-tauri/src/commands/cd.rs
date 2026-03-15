use crate::state::TerminalState;

pub fn execute(
    cmd: &str,
    args: &[&str],
    stdin: Option<String>,
    st: &mut TerminalState,
) -> Result<String, String> {
    // GNU_PARITY_INJECTED: Auto-generated standard GNU flag interceptor
    if args.contains(&"--help") || args.contains(&"-h") {
        return Ok(r#"cd - Linuxcommand.org Standard Parity

  -L, --follow-symlinks Force symbolic links to be followed: resolve symbolic links in DIR after proc...
  -P, --physical-directory  Use the physical directory structure without following symbolic links: resolv...
  -e                    If the -P option is supplied, and the current working directory cannot be det...
  -@, --extended-attributes  On systems that support it, present a file with extended attributes as a dire..."#.to_string());
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
            let z_flag = parsed_flags.contains("-z");
            if z_flag {
                return Err("cd: -z flag not supported natively in this basic terminal".to_string());
            }

            let mut force_physical = false;
            let mut dest_opt = None;

            for arg in args {
                if *arg == "-P" {
                    force_physical = true;
                } else if *arg == "-L" || *arg == "-e" || *arg == "-@" {
                    // Supported logically but defaults to logical anyway. 
                    // -@ is extended attributes on mac but acts as silent logical for generic terminals
                    force_physical = false;
                } else if dest_opt.is_none() {
                    dest_opt = Some(*arg);
                }
            }

            let dest = match dest_opt {
                Some(d) => d,
                None => return Ok(String::new()), // Could parse $HOME if needed in future
            };

            let mut new_path = st.cwd.clone();
            if dest.starts_with('/') {
                new_path = std::path::PathBuf::from(dest);
            } else {
                new_path.push(dest);
            }

            if new_path.exists() && new_path.is_dir() {
                if force_physical {
                    if let Ok(canon) = std::fs::canonicalize(&new_path) {
                        st.cwd = canon;
                    } else {
                        st.cwd = new_path;
                    }
                } else {
                    // Logical resolution: Try to clean `..` and `.` mathematically without resolving symlink tree
                    let mut logical_path = std::path::PathBuf::new();
                    for component in new_path.components() {
                        if component.as_os_str() == ".." {
                            logical_path.pop();
                        } else if component.as_os_str() != "." {
                            logical_path.push(component);
                        }
                    }
                    st.cwd = logical_path;
                }
                Ok(String::new())
            } else {
                Err(format!("cd: {}: No such file or directory", dest))
            }
        }
}
