# `b2sum` — Command Reference

**Source**: [linuxcommand.org](https://linuxcommand.org/lc3_man_pages/b2sum1.php)
**Rust module**: `src-tauri/src/commands/b2sum.rs`

**Status**: ✅ Fully Implemented (13/13 flags)

---

## ✅ Implemented Flags

- `-b`, `--binary`
  - read in binary mode

- `-c`, `--check`
  - read BLAKE2 sums from the FILEs and check them

- `-l`, `--length`
  - digest length in bits; must not exceed the maximum for the blake2 algorithm and must be a multiple of 8

- `--tag`
  - create a BSD-style checksum

- `-t`, `--text`
  - read in text mode (default)

- `-z`, `--zero`
  - end each output line with NUL, not newline, and disable file name escaping

- `--ignore-missing`
  - don't fail or report status for missing files (when verifying checksums)

- `--quiet`
  - don't print OK for each successfully verified file (when verifying checksums)

- `--status`
  - don't output anything, status code shows success (when verifying checksums)

- `--strict`
  - exit non-zero for improperly formatted checksum lines (when verifying checksums)

- `-w`, `--warn`
  - warn about improperly formatted checksum lines (when verifying checksums)

- `--help`
  - display this help and exit

- `--version`
  - output version information and exit

---

## ❌ Missing Flags (To Implement)

_All flags are implemented!_ 🎉
