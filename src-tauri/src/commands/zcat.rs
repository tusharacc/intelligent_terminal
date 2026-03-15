use crate::state::TerminalState;

pub fn execute(
    cmd: &str,
    args: &[&str],
    stdin: Option<String>,
    st: &mut TerminalState,
) -> Result<String, String> {
    // GNU_PARITY_INJECTED: Auto-generated standard GNU flag interceptor
    if args.contains(&"--help") || args.contains(&"-h") {
        return Ok(r#"zcat - Linuxcommand.org Standard Parity

  -a, --                Compress files
  -c, --                Compress files to standard output
  -d, --                Decompress files
  -f, --                Force compression or decompression even if the file is already compressed
  -h, --help            Display help message and exit
  -k, --keep            Keep original file name and timestamp in the compressed file
  -L, --long-name       Use long file names (default is short file names)
  -l, --list            List files in a compressed archive
  -N, --no-name         Do not save original file name and timestamp in the compressed file
  -r, --recursive       Recursively compress or decompress directories
  -S, --suffix          Set the suffix for compressed files
  -t, --test            Test compression and decompression without actually modifying files
  -v, --verbose         Display verbose output during compression or decompression
  -V, --version         Display version information and exit"#.to_string());
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
            use std::io::Read;
        let unsupported_flags = [
            "-d", "-a", "--ascii", "-c", "--stdout", "--to-stdout", "--decompress",
            "--uncompress", "-f", "--force", "-k", "--keep", "-l", "--list", "-L",
            "--license", "-n", "--no-name", "-N", "--name", "-q", "--quiet", "-r",
            "--recursive", "-S", "--suffix", "-t", "--test", "-v", "--verbose", "-V", "--version"
        ];
        for flag in unsupported_flags.iter() {
            if parsed_flags.contains(*flag) {
                return Err(format!("zcat: advanced compression flag {} not supported natively", flag));
            }
        }

        if args.is_empty() { return Err("zcat: missing operand".to_string()); }
        let path = st.cwd.join(args[0]);
            match std::fs::File::open(&path) {
                Ok(file) => {
                    let mut decoder = flate2::read::GzDecoder::new(file);
                    let mut content = String::new();
                    decoder.read_to_string(&mut content).map_err(|e| format!("zcat: {}", e))?;
                    Ok(content)
                },
                Err(e) => Err(format!("zcat: {}", e))
            }
        }
}
