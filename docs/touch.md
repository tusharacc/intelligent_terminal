# `touch` — Command Reference

**Source**: [linuxcommand.org](https://linuxcommand.org/lc3_man_pages/touch1.php)
**Rust module**: `src-tauri/src/commands/touch.rs`

**Status**: ✅ Fully Implemented (11/11 flags)

---

## ✅ Implemented Flags

- `-a`, `--access`
  - change only the access time

- `-c`, `--no-create`
  - do not create any files

- `-d`, `--date=STRING`
  - parse STRING and use it instead of current time

- `-f`
  - (ignored)

- `-h`, `--no-dereference`
  - affect each symbolic link instead of any referenced file (useful only on systems that can change the timestamps of a symlink)

- `-m`, `--modify`
  - change only the modification time

- `-r`, `--reference=FILE`
  - use this file's times instead of current time

- `-t STAMP`
  - use [[CC]YY]MMDDhhmm[.ss] instead of current time

- `--time=WORD`
  - change the specified time: WORD is access, atime, or use: equivalent to -a; WORD is modify or mtime: equivalent to -m

- `--help`
  - display this help and exit

- `--version`
  - output version information and exit

---

## ❌ Missing Flags (To Implement)

_All flags are implemented!_ 🎉
