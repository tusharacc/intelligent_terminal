# `cd` — Command Reference

**Source**: [linuxcommand.org](https://linuxcommand.org/lc3_man_pages/cd1.php)
**Rust module**: `src-tauri/src/commands/cd.rs`

**Status**: ✅ Fully Implemented (4/4 flags)

---

## ✅ Implemented Flags

- `-L`, `--follow-symlinks`
  - Force symbolic links to be followed: resolve symbolic links in DIR after processing instances of `..'

- `-P`, `--physical-directory`
  - Use the physical directory structure without following symbolic links: resolve symbolic links in DIR before processing instances of `..'

- `-e`
  - If the -P option is supplied, and the current working directory cannot be determined successfully, exit with a non-zero status

- `-@`, `--extended-attributes`
  - On systems that support it, present a file with extended attributes as a directory containing the file attributes

---

## ❌ Missing Flags (To Implement)

_All flags are implemented!_ 🎉
