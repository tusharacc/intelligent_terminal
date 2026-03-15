use crate::state::TerminalState;

pub fn execute(
    cmd: &str,
    args: &[&str],
    stdin: Option<String>,
    st: &mut TerminalState,
) -> Result<String, String> {
    // GNU_PARITY_INJECTED: Auto-generated standard GNU flag interceptor
    if args.contains(&"--help") || args.contains(&"-h") {
        return Ok(r#"rm - Linuxcommand.org Standard Parity

  -f, --force           ignore nonexistent files and arguments, never prompt
  -i                    prompt before every removal
  -I                    prompt once before removing more than three files, or when removing recursive...
  --interactive[=WHEN]  prompt according to WHEN: never, once (-I), or always (-i); without WHEN, pro...
  --one-file-system     when removing a hierarchy recursively, skip any directory that is on a file s...
  --no-preserve-root    do not treat '/' specially
  --preserve-root[=all] do not remove '/' (default); with 'all', reject any command line argument on ...
  -r, -R, --recursive   remove directories and their contents recursively
  -d, --dir             remove empty directories
  -v, --verbose         explain what is being done
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
        {
            let missing_flags: &[&str] = &[
                "-f", "--force", "--interactive", "--one-file-system",
                "--no-preserve-root", "--preserve-root", "-r", "-R",
                "--recursive", "-d", "--dir", "-v",
                "--verbose", "--version",
            ];
            for flag in missing_flags.iter() {
                if parsed_flags.contains(*flag) {
                    return Err(format!("rm: flag {} is not yet implemented in this terminal", flag));
                }
            }
        }


    {
        let i_flag = parsed_flags.contains("-i");
        let cap_i_flag = parsed_flags.contains("-I");
        if i_flag || cap_i_flag {
            return Err("rm: interactive mode (-i, -I) not supported in this shell".to_string());
        }

        if args.is_empty() { return Err("rm: missing operand".to_string()); }
        let path = st.cwd.join(args[0]);
            if path.is_dir() {
                return Err(format!("rm: cannot remove '{}': Is a directory (use rmdir)", args[0]));
            }
            match std::fs::remove_file(path) {
                Ok(_) => Ok(String::new()),
                Err(e) => Err(format!("rm: {}", e))
            }
        }
}
