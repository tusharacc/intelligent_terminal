# `mv` — Command Reference

**Source**: [linuxcommand.org](https://linuxcommand.org/lc3_man_pages/mv1.php)
**Rust module**: `src-tauri/src/commands/mv.rs`

**Status**: ✅ Fully Implemented (14/14 flags)

---

## ✅ Implemented Flags

- `--backup`, `--backup=CONTROL`
  - make a backup of each existing destination file

- `-b`
  - like --backup but does not accept an argument

- `-f`, `--force`
  - do not prompt before overwriting

- `-i`, `--interactive`
  - prompt before overwrite

- `-n`, `--no-clobber`
  - do not overwrite an existing file

- `--strip-trailing-slashes`
  - remove any trailing slashes from each SOURCE argument

- `-S`, `--suffix=SUFFIX`
  - override the usual backup suffix

- `-t`, `--target-directory=DIRECTORY`
  - move all SOURCE arguments into DIRECTORY

- `-T`, `--no-target-directory`
  - treat DEST as a normal file

- `-u`, `--update`
  - move only when the SOURCE file is newer than the destination file or when the destination file is missing

- `-v`, `--verbose`
  - explain what is being done

- `-Z`, `--context`
  - set SELinux security context of destination file to default type

- `--help`
  - display this help and exit

- `--version`
  - output version information and exit

---

## ❌ Missing Flags (To Implement)

_All flags are implemented!_ 🎉
