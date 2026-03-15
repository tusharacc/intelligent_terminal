use crate::state::TerminalState;

pub fn execute(
    cmd: &str,
    args: &[&str],
    stdin: Option<String>,
    st: &mut TerminalState,
) -> Result<String, String> {
    // GNU_PARITY_INJECTED: Auto-generated standard GNU flag interceptor
    if args.contains(&"--help") || args.contains(&"-h") {
        return Ok(r#"cp - Linuxcommand.org Standard Parity

  -a, --archive         same as -dR --preserve=all
  --attributes-only     don't copy the file data, just the attributes
  --backup[=CONTROL]    make a backup of each existing destination file
  -b                    like --backup but does not accept an argument
  --copy-contents       copy contents of special files when recursive
  -d, --no-dereference, --preserve=links  same as --no-dereference --preserve=links
  -f, --force           if an existing destination file cannot be opened, remove it and try again (th...
  -i, --interactive     prompt before overwrite (overrides a previous -n option)
  -H                    follow command-line symbolic links in SOURCE
  -l, --link            hard link files instead of copying
  -L, --dereference     always follow symbolic links in SOURCE
  -n, --no-clobber      do not overwrite an existing file (overrides a previous -i option)
  -P, --no-dereference  never follow symbolic links in SOURCE
  -p                    same as --preserve=mode,ownership,timestamps
  --preserve[=ATTR_LIST]  preserve the specified attributes (default: mode, ownership, timestamps), if ...
  -c                    deprecated, same as --preserve=context
  --no-preserve=ATTR_LIST  don't preserve the specified attributes
  --parents             use full source file name under DIRECTORY
  -R, -r, --recursive   copy directories recursively
  --reflink[=WHEN]      control clone/CoW copies. See below
  --remove-destination  remove each existing destination file before attempting to open it (contrast ...
  --sparse=WHEN         control creation of sparse files. See below
  --strip-trailing-slashes  remove any trailing slashes from each SOURCE argument
  -s, --symbolic-link   make symbolic links instead of copying
  -S, --suffix=SUFFIX   override the usual backup suffix
  -t, --target-directory=DIRECTORY  copy all SOURCE arguments into DIRECTORY
  -T, --no-target-directory  treat DEST as a normal file
  -u, --update          copy only when the SOURCE file is newer than the destination file or when the...
  -v, --verbose         explain what is being done
  -x, --one-file-system stay on this file system
  -Z                    set SELinux security context of destination file to default type
  --context[=CTX]       like -Z, or if CTX is specified then set the SELinux or SMACK security contex...
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
                "-a", "--archive", "--attributes-only", "--backup",
                "--copy-contents", "-f", "--force", "-i",
                "--interactive", "-l", "--link", "-L",
                "--dereference", "-n", "--no-clobber", "-P",
                "--no-dereference", "--preserve", "--no-preserve", "--parents",
                "-R", "-r", "--recursive", "--reflink",
                "--remove-destination", "--sparse", "--strip-trailing-slashes", "-s",
                "--symbolic-link", "-S", "--suffix", "-t",
                "--target-directory", "-T", "--no-target-directory", "-u",
                "--update", "-v", "--verbose", "-x",
                "--one-file-system", "--context", "--version",
            ];
            for flag in missing_flags.iter() {
                if parsed_flags.contains(*flag) {
                    return Err(format!("cp: flag {} is not yet implemented in this terminal", flag));
                }
            }
        }


    {
        let unsupported_flags = ["-b", "-d", "-H", "-p", "-c", "-Z"];
        for flag in unsupported_flags.iter() {
            if parsed_flags.contains(*flag) {
                return Err(format!("cp: advanced flag {} not supported natively in this basic terminal", flag));
            }
        }

        if args.len() < 2 { return Err("cp: missing file operand".to_string()); }
        let src = st.cwd.join(args[0]);
            let dest = st.cwd.join(args[1]);
            match std::fs::copy(src, dest) {
                Ok(_) => Ok(String::new()),
                Err(e) => Err(format!("cp: {}", e))
            }
        }
}
