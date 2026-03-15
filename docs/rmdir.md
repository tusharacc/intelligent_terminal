# `rmdir` — Command Reference

**Source**: [linuxcommand.org](https://linuxcommand.org/lc3_man_pages/rmdir1.php)
**Rust module**: `src-tauri/src/commands/rmdir.rs`

**Status**: ✅ Fully Implemented (5/5 flags)

---

## ✅ Implemented Flags

- `--ignore-fail-on-non-empty`
  - ignore each failure that is solely because a directory is non-empty

- `-p`, `--parents`
  - remove DIRECTORY and its ancestors; e.g., 'rmdir -p a/b/c' is similar to 'rmdir a/b/c a/b a'

- `-v`, `--verbose`
  - output a diagnostic for every directory processed

- `--help`
  - display this help and exit

- `--version`
  - output version information and exit

---

## ❌ Missing Flags (To Implement)

_All flags are implemented!_ 🎉
