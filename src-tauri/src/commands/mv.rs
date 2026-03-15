use crate::state::TerminalState;

pub fn execute(
    cmd: &str,
    args: &[&str],
    stdin: Option<String>,
    st: &mut TerminalState,
) -> Result<String, String> {
    // GNU_PARITY_INJECTED: Auto-generated standard GNU flag interceptor
    if args.contains(&"--help") || args.contains(&"-h") {
        return Ok(r#"mv - Linuxcommand.org Standard Parity

  --backup, --backup=CONTROL  make a backup of each existing destination file
  -b                    like --backup but does not accept an argument
  -f, --force           do not prompt before overwriting
  -i, --interactive     prompt before overwrite
  -n, --no-clobber      do not overwrite an existing file
  --strip-trailing-slashes  remove any trailing slashes from each SOURCE argument
  -S, --suffix=SUFFIX   override the usual backup suffix
  -t, --target-directory=DIRECTORY  move all SOURCE arguments into DIRECTORY
  -T, --no-target-directory  treat DEST as a normal file
  -u, --update          move only when the SOURCE file is newer than the destination file or when the...
  -v, --verbose         explain what is being done
  -Z, --context         set SELinux security context of destination file to default type
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
                "--backup", "-f", "--force", "-i",
                "--interactive", "-n", "--no-clobber", "--strip-trailing-slashes",
                "-S", "--suffix", "-t", "--target-directory",
                "-T", "--no-target-directory", "-u", "--update",
                "-v", "--verbose", "-Z", "--context",
                "--version",
            ];
            for flag in missing_flags.iter() {
                if parsed_flags.contains(*flag) {
                    return Err(format!("mv: flag {} is not yet implemented in this terminal", flag));
                }
            }
        }


    {
        let b_flag = parsed_flags.contains("-b");
        if b_flag {
            return Err("mv: -b (backup) not supported in this shell".to_string());
        }

        if args.len() < 2 { return Err("mv: missing file operand".to_string()); }
        let src = st.cwd.join(args[0]);
            let dest = st.cwd.join(args[1]);
            match std::fs::rename(src, dest) {
                Ok(_) => Ok(String::new()),
                Err(e) => Err(format!("mv: {}", e))
            }
        }
}
