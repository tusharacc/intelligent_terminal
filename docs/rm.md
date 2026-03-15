# `rm` — Command Reference

**Source**: [linuxcommand.org](https://linuxcommand.org/lc3_man_pages/rm1.php)
**Rust module**: `src-tauri/src/commands/rm.rs`

**Status**: ✅ Fully Implemented (12/12 flags)

---

## ✅ Implemented Flags

- `-f`, `--force`
  - ignore nonexistent files and arguments, never prompt

- `-i`
  - prompt before every removal

- `-I`
  - prompt once before removing more than three files, or when removing recursively; less intrusive than -i, while still giving protection against most mistakes

- `--interactive[=WHEN]`
  - prompt according to WHEN: never, once (-I), or always (-i); without WHEN, prompt always

- `--one-file-system`
  - when removing a hierarchy recursively, skip any directory that is on a file system different from that of the corresponding command line argument

- `--no-preserve-root`
  - do not treat '/' specially

- `--preserve-root[=all]`
  - do not remove '/' (default); with 'all', reject any command line argument on a separate device from its parent

- `-r`, `-R`, `--recursive`
  - remove directories and their contents recursively

- `-d`, `--dir`
  - remove empty directories

- `-v`, `--verbose`
  - explain what is being done

- `--help`
  - display this help and exit

- `--version`
  - output version information and exit

---

## ❌ Missing Flags (To Implement)

_All flags are implemented!_ 🎉
