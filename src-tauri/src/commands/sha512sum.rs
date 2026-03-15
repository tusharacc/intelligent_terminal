use crate::state::TerminalState;

pub fn execute(
    cmd: &str,
    args: &[&str],
    stdin: Option<String>,
    st: &mut TerminalState,
) -> Result<String, String> {
    // GNU_PARITY_INJECTED: Auto-generated standard GNU flag interceptor
    if args.contains(&"--help") || args.contains(&"-h") {
        return Ok(r#"sha512sum - Linuxcommand.org Standard Parity

  -b, --binary          read in binary mode
  -c, --check           read SHA512 sums from the FILEs and check them
  --tag                 create a BSD-style checksum
  -t, --text            read in text mode (default)
  -z, --zero            end each output line with NUL, not newline, and disable file name escaping
  --ignore-missing      don't fail or report status for missing files (when verifying checksums)
  --quiet               don't print OK for each successfully verified file (when verifying checksums)
  --status              don't output anything, status code shows success (when verifying checksums)
  --strict              exit non-zero for improperly formatted checksum lines (when verifying checksums)
  -w, --warn            warn about improperly formatted checksum lines (when verifying checksums)
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
                    return Err(format!("sha512sum: flag {} is not yet implemented in this terminal", flag));
                }
            }
        }


    {
            use sha2::{Sha512, Digest};
            let tag_flag = parsed_flags.contains("--tag");
            if tag_flag {
                return Err("sha512sum: --tag (BSD-style checksum) not supported".to_string());
            }

            let (content, label) = if args.is_empty() {
                if let Some(input) = stdin { (input.into_bytes(), "".to_string()) } else { return Err("sha512sum: missing operand".to_string()); }
            } else {
                let path = st.cwd.join(args[0]);
                (std::fs::read(path).map_err(|e| format!("sha512sum: {}", e))?, args[0].to_string())
            };
            match Ok::<_, String>(content) {
                Ok(content) => {
                    let mut hasher = Sha512::new();
                    hasher.update(&content);
                    if label.is_empty() { Ok(format!("{:x}", hasher.finalize())) } else { Ok(format!("{:x}  {}", hasher.finalize(), label)) }
                },
                Err(e) => Err(format!("sha512sum: {}", e))
            }
        }
}
