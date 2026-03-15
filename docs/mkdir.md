# `mkdir` — Command Reference

**Source**: [linuxcommand.org](https://linuxcommand.org/lc3_man_pages/mkdir1.php)
**Rust module**: `src-tauri/src/commands/mkdir.rs`

**Status**: ✅ Fully Implemented (7/7 flags)

---

## ✅ Implemented Flags

- `-m`, `--mode=MODE`
  - set file mode (as in chmod), not a=rwx - umask

- `-p`, `--parents`
  - no error if existing, make parent directories as needed

- `-v`, `--verbose`
  - print a message for each created directory

- `-Z`
  - set SELinux security context of each created directory to the default type

- `--context[=CTX]`
  - like -Z, or if CTX is specified then set the SELinux or SMACK security context to CTX

- `--help`
  - display this help and exit

- `--version`
  - output version information and exit

---

## ❌ Missing Flags (To Implement)

_All flags are implemented!_ 🎉
