use crate::state::TerminalState;

pub fn execute(
    cmd: &str,
    args: &[&str],
    stdin: Option<String>,
    st: &mut TerminalState,
) -> Result<String, String> {
    // GNU_PARITY_INJECTED: Auto-generated standard GNU flag interceptor
    if args.contains(&"--help") || args.contains(&"-h") {
        return Ok(r#"tar - Linuxcommand.org Standard Parity

  -A, --catenate, --concatenate  Append archive to the end of another archive
  -c, --create          Create a new archive
  -d, --diff, --compare Find differences between archive and file system
  --delete              Delete from the archive
  -r, --append          Append files to the end of an archive
  -t, --list            List the contents of an archive
  --test-label          Test the archive volume label and exit
  -u, --update          Append files which are newer than the corresponding copy in the archive
  -v, --verbose         Request verbose operation
  -f, --file            Set the name of the archive to operate upon
  -G, --group           Specify group ownership for the archive
  -i, --ignore-zeros    Ignore zero-length holes in the archive
  -k, --keep-newer-files  Keep newer files when updating an existing archive
  -l, --dereference     Dereference symbolic links when archiving
  -m, --multi-volume    Create a multi-volume archive
  -n, --no-recursion    Do not recurse into directories when archiving
  -o, --outdir          Specify the directory where files are to be written
  -p, --preserve-permissions  Preserve permissions of files and directories
  -P, --absolute-names  Use absolute names in the archive
  -R, --remove-files    Remove files from the file system after archiving
  -s, --strip-components  Strip specified number of leading components from file names
  -S, --same-order      Maintain the same order as the original files when archiving
  -T, --ignore-newer    Ignore newer files when updating an existing archive
  -w, --wait-for-completion  Wait for completion of the operation before exiting"#.to_string());
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
                "-A", "--catenate", "--concatenate", "-d",
                "--diff", "--compare", "--delete", "-r",
                "--append", "-t", "--list", "--test-label",
                "-u", "--update", "-v", "--verbose",
                "-f", "--file", "-G", "--group",
                "-i", "--ignore-zeros", "-k", "--keep-newer-files",
                "-l", "--dereference", "-m", "--multi-volume",
                "-n", "--no-recursion", "-p", "--preserve-permissions",
                "-P", "--absolute-names", "-R", "--remove-files",
                "-s", "--strip-components", "-S", "--same-order",
                "-T", "--ignore-newer", "-w", "--wait-for-completion",
            ];
            for flag in missing_flags.iter() {
                if parsed_flags.contains(*flag) {
                    return Err(format!("tar: flag {} is not yet implemented in this terminal", flag));
                }
            }
        }


    {
        let unsupported_flags = [
            "--acls", "--lzip", "--lzma", "--lzop", "--zstd", "--null", "--utc", "-o"
        ];
        for flag in unsupported_flags.iter() {
            if parsed_flags.contains(*flag) {
                return Err(format!("tar: advanced flag {} not supported natively", flag));
            }
        }

        if args.len() < 2 { return Err("tar: missing operand (Usage: tar cf dest.tar dir OR tar xf src.tar)".to_string()); }
        let mode = args[0];
            let archive = st.cwd.join(args[1]);
            
            if mode == "cf" || mode == "-cf" {
                if args.len() < 3 { return Err("tar cf: missing directory to archive".to_string()); }
                let target = st.cwd.join(args[2]);
                let file = std::fs::File::create(&archive).map_err(|e| format!("tar: {}", e))?;
                let mut builder = tar::Builder::new(file);
                builder.append_dir_all(".", target).map_err(|e| format!("tar: {}", e))?;
                builder.finish().map_err(|e| format!("tar: {}", e))?;
                Ok(String::new())
            } else if mode == "xf" || mode == "-xf" {
                let file = std::fs::File::open(&archive).map_err(|e| format!("tar: {}", e))?;
                let mut archive_reader = tar::Archive::new(file);
                archive_reader.unpack(&st.cwd).map_err(|e| format!("tar: {}", e))?;
                Ok(String::new())
            } else {
                Err(format!("tar: unsupported mode {}", mode))
            }
        }
}
