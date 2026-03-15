# `realpath` — Command Reference

**Source**: [linuxcommand.org](https://linuxcommand.org/lc3_man_pages/realpath1.php)
**Rust module**: `src-tauri/src/commands/realpath.rs`

**Status**: ✅ Fully Implemented (11/11 flags)

---

## ✅ Implemented Flags

- `-e`, `--canonicalize-existing`
  - all components of the path must exist

- `-m`, `--canonicalize-missing`
  - no path components need exist or be a directory

- `-L`, `--logical`
  - resolve '..' components before symlinks

- `-P`, `--physical`
  - resolve symlinks as encountered (default)

- `-q`, `--quiet`
  - suppress most error messages

- `--relative-to=DIR`
  - print the resolved path relative to DIR

- `--relative-base=DIR`
  - print absolute paths unless paths below DIR

- `-s`, `--strip`, `--no-symlinks`
  - don't expand symlinks

- `-z`, `--zero`
  - end each output line with NUL, not newline

- `--help`
  - display this help and exit

- `--version`
  - output version information and exit

---

## ❌ Missing Flags (To Implement)

_All flags are implemented!_ 🎉
