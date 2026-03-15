# `env` — Command Reference

**Source**: [linuxcommand.org](https://linuxcommand.org/lc3_man_pages/env1.php)
**Rust module**: `src-tauri/src/commands/env.rs`

**Status**: ✅ Fully Implemented (10/10 flags)

---

## ✅ Implemented Flags

- `-i`, `--ignore-environment`
  - start with an empty environment

- `-0`, `--null`
  - end each output line with NUL, not newline

- `-u`, `--unset=NAME`
  - remove variable from the environment

- `-C`, `--chdir=DIR`
  - change working directory to DIR

- `-S`, `--split-string=S`
  - process and split S into separate arguments; used to pass multiple arguments on shebang lines

- `--block-signal[=SIG]`, `--default-signal[=SIG]`, `--ignore-signal[=SIG]`
  - set handling of SIG signal(s) to do nothing, reset handling of SIG signal(s) to the default, or block delivery of SIG signal(s) to COMMAND

- `--list-signal-handling`
  - list non default signal handling to stderr

- `-v`, `--debug`
  - print verbose information for each processing step

- `--help`
  - display this help and exit

- `--version`
  - output version information and exit

---

## ❌ Missing Flags (To Implement)

_All flags are implemented!_ 🎉
