# `uniq` — Command Reference

**Source**: [linuxcommand.org](https://linuxcommand.org/lc3_man_pages/uniq1.php)
**Rust module**: `src-tauri/src/commands/uniq.rs`

**Status**: ✅ Fully Implemented (13/13 flags)

---

## ✅ Implemented Flags

- `-c`, `--count`
  - prefix lines by the number of occurrences

- `-d`, `--repeated`
  - only print duplicate lines, one for each group

- `-D`
  - print all duplicate lines

- `--all-repeated`
  - like -D, but allow separating groups with an empty line; METHOD={none(default),prepend,separate}

- `-f`, `--skip-fields=N`
  - avoid comparing the first N fields

- `--group`
  - show all items, separating groups with an empty line; METHOD={separate(default),prepend,append,both}

- `-i`, `--ignore-case`
  - ignore differences in case when comparing

- `-s`, `--skip-chars=N`
  - avoid comparing the first N characters

- `-u`, `--unique`
  - only print unique lines

- `-z`, `--zero-terminated`
  - line delimiter is NUL, not newline

- `-w`, `--check-chars=N`
  - compare no more than N characters in lines

- `--help`
  - display this help and exit

- `--version`
  - output version information and exit

---

## ❌ Missing Flags (To Implement)

_All flags are implemented!_ 🎉
