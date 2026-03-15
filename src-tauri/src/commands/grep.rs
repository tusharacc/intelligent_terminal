use crate::state::TerminalState;

pub fn execute(
    cmd: &str,
    args: &[&str],
    stdin: Option<String>,
    st: &mut TerminalState,
) -> Result<String, String> {
    // GNU_PARITY_INJECTED: Auto-generated standard GNU flag interceptor
    if args.contains(&"--help") || args.contains(&"-h") {
        return Ok(r#"grep - Linuxcommand.org Standard Parity

  --help                Output a usage message and exit.
  -V, --version         Output the version number of grep and exit.
  -E, --extended-regexp Interpret PATTERNS as extended regular expressions (EREs, see below).
  -F, --fixed-strings   Interpret PATTERNS as fixed strings, not regular expressions.
  -G, --basic-regexp    Interpret PATTERNS as basic regular expressions (BREs, see below). This is th...
  -P, --perl-regexp     Interpret PATTERNS as Perl-compatible regular expressions (PCREs). This optio...
  -e, --regexp          Use PATTERNS as the patterns. If this option is used multiple times or is com...
  -f, --file            Obtain patterns from FILE, one per line. If this option is used multiple time...
  -i, --ignore-case     Ignore case distinctions, so that characters that differ only in case match e...
  -v, --invert-match    Invert the sense of matching, to select non-matching lines.
  -w, --word-regexp     Select only those lines containing matches that form whole words. The test is...
  -x, --line-regexp     Select only those matches that exactly match the whole line. For a regular ex...
  -y                    Obsolete synonym for -i.
  -c, --count           Suppress normal output; instead print a count of matching lines for each inpu...
  --color, --colour     Surround the matched (non-empty) strings, matching lines, context lines, file...
  -L, --files-without-match  Suppress normal output; instead print the name of each input file from which ...
  -l, --files-with-matches  Suppress normal output; instead print the name of each input file from which ...
  -m, --max-count       Stop reading a file after NUM matching lines. If the input is standard input ...
  -o, --only-matching   Print only the matched (non-empty) parts of a matching line, with each such p...
  -q, --quiet, --silent Quiet; do not write anything to standard output. Exit immediately with zero s...
  -s, --no-messages     Suppress error messages about nonexistent or unreadable files.
  -b, --byte-offset     Print the 0-based byte offset within the input file before each line of outpu...
  -H, --with-filename   Print the file name for each match. This is the default when there is more th...
  -h, --no-filename     Suppress the prefixing of file names on output. This is the default when ther...
  --label               Display input actually coming from standard input as input coming from file L...
  -n, --line-number     Prefix each line of output with the 1-based line number within its input file.
  -T, --initial-tab     Make sure that the first character of actual line content lies on a tab stop,..."#.to_string());
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
                "-V", "--version", "-E", "--extended-regexp",
                "-F", "--fixed-strings", "-G", "--basic-regexp",
                "-P", "--perl-regexp", "-i", "--ignore-case",
                "-v", "--invert-match", "-w", "--word-regexp",
                "-x", "--line-regexp", "-c", "--count",
                "--color", "--colour", "-L", "--files-without-match",
                "-l", "--files-with-matches", "-o", "--only-matching",
                "-s", "--no-messages", "-b", "--byte-offset",
                "-H", "--with-filename", "--label", "-n",
                "--line-number", "-T", "--initial-tab",
            ];
            for flag in missing_flags.iter() {
                if parsed_flags.contains(*flag) {
                    return Err(format!("grep: flag {} is not yet implemented in this terminal", flag));
                }
            }
        }


    {
        let unsupported_flags = [
            "-e", "--regexp", "-f", "--file", "-y", "-m", "--max-count",
            "-A", "--after-context", "-B", "--before-context", "-C", "-N", "--context",
            "-D", "--devices", "-d", "--directories", "-I", "--quiet"
        ];
        for flag in unsupported_flags.iter() {
            if parsed_flags.contains(*flag) {
                return Err(format!("grep: advanced search/formatting flag {} not supported natively", flag));
            }
        }

        if args.is_empty() { return Err("grep: missing operand".to_string()); }
        let pattern = args[0];
            let content = if args.len() >= 2 {
                let path = st.cwd.join(args[1]);
                std::fs::read_to_string(path).map_err(|e| format!("grep: {}", e))?
            } else if let Some(input) = stdin {
                input
            } else {
                return Err("grep: missing file operand".to_string());
            };
            let re = match regex::Regex::new(pattern) {
                Ok(r) => r,
                Err(e) => return Err(format!("grep: invalid regex: {}", e))
            };
            let mut result = String::new();
            for line in content.lines() {
                if re.is_match(line) {
                    result.push_str(&format!("{}\n", line));
                }
            }
            if !result.is_empty() { result.pop(); }
            Ok(result)
        }
}
