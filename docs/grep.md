# `grep` — Command Reference

**Source**: [linuxcommand.org](https://linuxcommand.org/lc3_man_pages/grep1.php)
**Rust module**: `src-tauri/src/commands/grep.rs`

**Status**: ✅ Fully Implemented (27/27 flags)

---

## ✅ Implemented Flags

- `--help`
  - Output a usage message and exit.

- `-V`, `--version`
  - Output the version number of grep and exit.

- `-E`, `--extended-regexp`
  - Interpret PATTERNS as extended regular expressions (EREs, see below).

- `-F`, `--fixed-strings`
  - Interpret PATTERNS as fixed strings, not regular expressions.

- `-G`, `--basic-regexp`
  - Interpret PATTERNS as basic regular expressions (BREs, see below). This is the default.

- `-P`, `--perl-regexp`
  - Interpret PATTERNS as Perl-compatible regular expressions (PCREs). This option is experimental when combined with the -z (--null-data) option, and grep -P may warn of unimplemented features.

- `-e`, `--regexp`
  - Use PATTERNS as the patterns. If this option is used multiple times or is combined with the -f (--file) option, search for all patterns given. This option can be used to protect a pattern beginning with “-”.

- `-f`, `--file`
  - Obtain patterns from FILE, one per line. If this option is used multiple times or is combined with the -e (--regexp) option, search for all patterns given. The empty file contains zero patterns, and therefore matches nothing.

- `-i`, `--ignore-case`
  - Ignore case distinctions, so that characters that differ only in case match each other.

- `-v`, `--invert-match`
  - Invert the sense of matching, to select non-matching lines.

- `-w`, `--word-regexp`
  - Select only those lines containing matches that form whole words. The test is that the matching substring must either be at the beginning of the line, or preceded by a non-word constituent character. Similarly, it must be either at the end of the line or followed by a non-word constituent character. Word-constituent characters are letters, digits, and the underscore. This option has no effect if -x is also specified.

- `-x`, `--line-regexp`
  - Select only those matches that exactly match the whole line. For a regular expression pattern, this is like parenthesizing the pattern and then surrounding it with ^ and $.

- `-y`
  - Obsolete synonym for -i.

- `-c`, `--count`
  - Suppress normal output; instead print a count of matching lines for each input file. With the -v, --invert-match option (see below), count non-matching lines.

- `--color`, `--colour`
  - Surround the matched (non-empty) strings, matching lines, context lines, file names, line numbers, byte offsets, and separators (for fields and groups of context lines) with escape sequences to display them in color on the terminal. The colors are defined by the environment variable GREP_COLORS. The deprecated environment variable GREP_COLOR is still supported, but its setting does not have priority. WHEN is never, always, or auto.

- `-L`, `--files-without-match`
  - Suppress normal output; instead print the name of each input file from which no output would normally have been printed. The scanning will stop on the first match.

- `-l`, `--files-with-matches`
  - Suppress normal output; instead print the name of each input file from which output would normally have been printed. The scanning will stop on the first match.

- `-m`, `--max-count`
  - Stop reading a file after NUM matching lines. If the input is standard input from a regular file, and NUM matching lines are output, grep ensures that the standard input is positioned to just after the last matching line before exiting, regardless of the presence of trailing context lines. This enables a calling process to resume a search. When grep stops after NUM matching lines, it outputs any trailing context lines. When the -c or --count option is also used, grep does not output a count greater than NUM. When the -v or --invert-match option is also used, grep stops after outputting NUM non-matching lines.

- `-o`, `--only-matching`
  - Print only the matched (non-empty) parts of a matching line, with each such part on a separate output line.

- `-q`, `--quiet`, `--silent`
  - Quiet; do not write anything to standard output. Exit immediately with zero status if any match is found, even if an error was detected. Also see the -s or --no-messages option.

- `-s`, `--no-messages`
  - Suppress error messages about nonexistent or unreadable files.

- `-b`, `--byte-offset`
  - Print the 0-based byte offset within the input file before each line of output. If -o (--only-matching) is specified, print the offset of the matching part itself.

- `-H`, `--with-filename`
  - Print the file name for each match. This is the default when there is more than one file to search.

- `-h`, `--no-filename`
  - Suppress the prefixing of file names on output. This is the default when there is only one file (or only standard input) to search.

- `--label`
  - Display input actually coming from standard input as input coming from file LABEL. This is especially useful when implementing tools like zgrep, e.g., gzip -cd foo.gz | grep --label=foo -H something. See also the -H option.

- `-n`, `--line-number`
  - Prefix each line of output with the 1-based line number within its input file.

- `-T`, `--initial-tab`
  - Make sure that the first character of actual line content lies on a tab stop, so that the alignment of tabs looks normal. This is useful with options that prefix their output to the actual content: -H,-n, and -b. In order to improve the probability that lines from a single file will all start at the same column, this also causes the line number and byte offset (if present) to be printed in a minimum size field width.

---

## ❌ Missing Flags (To Implement)

_All flags are implemented!_ 🎉
