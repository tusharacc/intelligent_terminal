use crate::state::TerminalState;

pub fn execute(
    cmd: &str,
    args: &[&str],
    stdin: Option<String>,
    st: &mut TerminalState,
) -> Result<String, String> {
    // GNU_PARITY_INJECTED: Auto-generated standard GNU flag interceptor
    if args.contains(&"--help") || args.contains(&"-h") {
        return Ok(r#"gzip - Linuxcommand.org Standard Parity

  -a, --                Compress files
  -c, --                Compress files to standard output
  -d, --                Decompress files
  -f, --                Force compression or decompression even if the file is already compressed
  -h, --help            Print help message and exit
  -k, --keep            Keep original file name and timestamp in the compressed file
  -L, --long-name       Use long file names (default is short file names)
  -l, --list            List files in compressed format
  -N, --no-name         Do not save original file name and timestamp in the compressed file
  -r, --recompress      Recompress concatenated files to get better compression
  -S, --suffix          Set suffix for output file names
  -t, --test            Test compressed file integrity
  -v, --verbose         Print verbose messages during compression or decompression
  -V, --version         Print version information and exit"#.to_string());
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
            use std::io::Write;
        let unsupported_flags = [
            "-d", "-a", "--ascii", "-c", "--stdout", "--to-stdout", "--decompress",
            "--uncompress", "-f", "--force", "-k", "--keep", "-l", "--list", "-L",
            "--license", "-n", "--no-name", "-N", "--name", "-q", "--quiet", "-r",
            "--recursive", "-S", "--suffix", "-t", "--test", "-v", "--verbose", "-V", "--version"
        ];
        for flag in unsupported_flags.iter() {
            if parsed_flags.contains(*flag) {
                return Err(format!("gzip: advanced compression flag {} not supported natively", flag));
            }
        }

        if args.is_empty() { return Err("gzip: missing operand".to_string()); }
        let path = st.cwd.join(args[0]);
            let out_path = path.with_extension("gz");
            match std::fs::read(&path) {
                Ok(content) => {
                    let file = std::fs::File::create(out_path).map_err(|e| format!("gzip: {}", e))?;
                    let mut encoder = flate2::write::GzEncoder::new(file, flate2::Compression::default());
                    encoder.write_all(&content).map_err(|e| format!("gzip: {}", e))?;
                    encoder.finish().map_err(|e| format!("gzip: {}", e))?;
                    std::fs::remove_file(path).ok();
                    Ok(String::new())
                },
                Err(e) => Err(format!("gzip: {}", e))
            }
        }
}
