# `md5sum` — Command Reference

**Source**: [linuxcommand.org](https://linuxcommand.org/lc3_man_pages/md5sum1.php)
**Rust module**: `src-tauri/src/commands/md5sum.rs`

**Status**: ✅ Fully Implemented (12/12 flags)

---

## ✅ Implemented Flags

- `-b`, `--binary`
  - read in binary mode

- `-c`, `--check`
  - read MD5 sums from the FILEs and check them

- `--tag`
  - create a BSD-style checksum

- `-t`, `--text`
  - read in text mode (default)

- `-z`, `--zero`
  - end each output line with NUL, not newline, and disable file name escaping

- `--ignore-missing`
  - don't fail or report status for missing files

- `--quiet`
  - don't print OK for each successfully verified file

- `--status`
  - don't output anything, status code shows success

- `--strict`
  - exit non-zero for improperly formatted checksum lines

- `-w`, `--warn`
  - warn about improperly formatted checksum lines

- `--help`
  - display this help and exit

- `--version`
  - output version information and exit

---

## ❌ Missing Flags (To Implement)

_All flags are implemented!_ 🎉
