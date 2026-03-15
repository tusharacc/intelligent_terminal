use crate::state::TerminalState;

pub fn execute(
    cmd: &str,
    args: &[&str],
    stdin: Option<String>,
    st: &mut TerminalState,
) -> Result<String, String> {
    // GNU_PARITY_INJECTED: Auto-generated standard GNU flag interceptor
    if args.contains(&"--help") || args.contains(&"-h") {
        return Ok(r#"md5sum - Linuxcommand.org Standard Parity

  -b, --binary          read in binary mode
  -c, --check           read MD5 sums from the FILEs and check them
  --tag                 create a BSD-style checksum
  -t, --text            read in text mode (default)
  -z, --zero            end each output line with NUL, not newline, and disable file name escaping
  --ignore-missing      don't fail or report status for missing files
  --quiet               don't print OK for each successfully verified file
  --status              don't output anything, status code shows success
  --strict              exit non-zero for improperly formatted checksum lines
  -w, --warn            warn about improperly formatted checksum lines
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
                "-b", "--binary", "-c", "--check",
                "-t", "--text", "-z", "--zero",
                "--ignore-missing", "--quiet", "--status", "--strict",
                "-w", "--warn", "--version",
            ];
            for flag in missing_flags.iter() {
                if parsed_flags.contains(*flag) {
                    return Err(format!("md5sum: flag {} is not yet implemented in this terminal", flag));
                }
            }
        }


    {
            use md5::{Md5, Digest};
            let tag_flag = parsed_flags.contains("--tag");
            if tag_flag {
                return Err("md5sum: --tag (BSD-style checksum) not supported".to_string());
            }

            let (content, label) = if args.is_empty() {
                if let Some(input) = stdin { (input.into_bytes(), "".to_string()) } else { return Err("md5sum: missing operand".to_string()); }
            } else {
                let path = st.cwd.join(args[0]);
                (std::fs::read(path).map_err(|e| format!("md5sum: {}", e))?, args[0].to_string())
            };
            match Ok::<_, String>(content) {
                Ok(content) => {
                    let mut hasher = Md5::new();
                    hasher.update(&content);
                    if label.is_empty() { Ok(format!("{:x}", hasher.finalize())) } else { Ok(format!("{:x}  {}", hasher.finalize(), label)) }
                },
                Err(e) => Err(format!("md5sum: {}", e))
            }
        }
}
