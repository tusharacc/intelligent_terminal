# `head` — Command Reference

**Source**: [linuxcommand.org](https://linuxcommand.org/lc3_man_pages/head1.php)
**Rust module**: `src-tauri/src/commands/head.rs`

**Status**: ✅ Fully Implemented (7/7 flags)

---

## ✅ Implemented Flags

- `-c`, `--bytes`
  - print the first NUM bytes of each file; with the leading '-', print all but the last NUM bytes of each file

- `-n`, `--lines`
  - print the first NUM lines instead of the first 10; with the leading '-', print all but the last NUM lines of each file

- `-q`, `--quiet`, `--silent`
  - never print headers giving file names

- `-v`, `--verbose`
  - always print headers giving file names

- `-z`, `--zero-terminated`
  - line delimiter is NUL, not newline

- `--help`
  - display this help and exit

- `--version`
  - output version information and exit

---

## ❌ Missing Flags (To Implement)

_All flags are implemented!_ 🎉
