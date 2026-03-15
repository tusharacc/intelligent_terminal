# `history` — Command Reference

**Source**: [linuxcommand.org](https://linuxcommand.org/lc3_man_pages/history1.php)
**Rust module**: `src-tauri/src/commands/history.rs`

**Status**: ✅ Fully Implemented (8/8 flags)

---

## ✅ Implemented Flags

- `-c`
  - clear the history list by deleting all of the entries

- `-d`, `--offset`
  - delete the history entry at position OFFSET. Negative offsets count back from the end of the history list

- `-a`
  - append history lines from this session to the history file

- `-n`
  - read all history lines not already read from the history file and append them to the history list

- `-r`
  - read the history file and append the contents to the history list

- `-w`
  - write the current history to the history file

- `-p`, `--ps`
  - perform history expansion on each ARG and display the result without storing it in the history list

- `-s`
  - append the ARGs to the history list as a single entry

---

## ❌ Missing Flags (To Implement)

_All flags are implemented!_ 🎉
