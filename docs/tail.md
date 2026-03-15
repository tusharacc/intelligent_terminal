# `tail` — Command Reference

**Source**: [linuxcommand.org](https://linuxcommand.org/lc3_man_pages/tail1.php)
**Rust module**: `src-tauri/src/commands/tail.rs`

**Status**: ✅ Fully Implemented (13/13 flags)

---

## ✅ Implemented Flags

- `-c`, `--bytes`
  - output the last NUM bytes; or use -c +NUM to output starting with byte NUM of each file

- `-f`, `--follow`
  - output appended data as the file grows;

- `-F`
  - same as --follow=name --retry

- `-n`, `--lines`
  - output the last NUM lines, instead of the last 10; or use -n +NUM to output starting with line NUM

- `--max-unchanged-stats`
  - with --follow=name, reopen a FILE which has not changed size after N (default 5) iterations to see if it has been unlinked or renamed (this is the usual case of rotated log files); with inotify, this option is rarely useful

- `--pid`
  - with -f, terminate after process ID, PID dies

- `-q`, `--quiet`, `--silent`
  - never output headers giving file names

- `--retry`
  - keep trying to open a file if it is inaccessible

- `-s`, `--sleep-interval`
  - with -f, sleep for approximately N seconds (default 1.0) between iterations; with inotify and --pid=P, check process P at least once every N seconds

- `-v`, `--verbose`
  - always output headers giving file names

- `-z`, `--zero-terminated`
  - line delimiter is NUL, not newline

- `--help`
  - display this help and exit

- `--version`
  - output version information and exit

---

## ❌ Missing Flags (To Implement)

_All flags are implemented!_ 🎉
