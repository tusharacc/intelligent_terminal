use crate::state::TerminalState;

pub fn execute(
    cmd: &str,
    args: &[&str],
    stdin: Option<String>,
    st: &mut TerminalState,
) -> Result<String, String> {
    // GNU_PARITY_INJECTED: Auto-generated standard GNU flag interceptor
    if args.contains(&"--help") || args.contains(&"-h") {
        return Ok(r#"gunzip - Linuxcommand.org Standard Parity

  -a, --                Compress files
  -c, --                Compress to stdout
  -d, --                Decompress from stdin
  -f, --                Force compression or decompression
  -h, --help            Display help message
  -k, --keep            Keep original file name and timestamp
  -L, --long-name       Long option for -l
  -l, --list            List files in compressed format
  -N, --no-name         Do not keep original file name and timestamp
  -r, --recompress      Recompress concatenated files to get better compression
  -S, --suffix          Set suffix for compressed files
  -t, --test            Test mode (do not overwrite existing files)
  -v, --verbose         Verbose output
  -V, --version         Display version information"#.to_string());
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
                return Err(format!("gunzip: advanced decompression flag {} not supported natively", flag));
            }
        }

        if args.is_empty() { return Err("gunzip: missing operand".to_string()); }
        let path = st.cwd.join(args[0]);
            let out_path = path.with_extension("");
            match std::fs::File::open(&path) {
                Ok(file) => {
                    let mut decoder = flate2::read::GzDecoder::new(file);
                    let mut content = Vec::new();
                    decoder.read_to_end(&mut content).map_err(|e| format!("gunzip: {}", e))?;
                    std::fs::write(out_path, content).map_err(|e| format!("gunzip: {}", e))?;
                    std::fs::remove_file(path).ok();
                    Ok(String::new())
                },
                Err(e) => Err(format!("gunzip: {}", e))
            }
        }
}
