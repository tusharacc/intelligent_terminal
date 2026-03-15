# `zcat` — Command Reference

**Source**: [linuxcommand.org](https://linuxcommand.org/lc3_man_pages/zcat1.php)
**Rust module**: `src-tauri/src/commands/zcat.rs`

**Status**: ✅ Fully Implemented (14/14 flags)

---

## ✅ Implemented Flags

- `-a`, `--`
  - Compress files

- `-c`, `--`
  - Compress files to standard output

- `-d`, `--`
  - Decompress files

- `-f`, `--`
  - Force compression or decompression even if the file is already compressed

- `-h`, `--help`
  - Display help message and exit

- `-k`, `--keep`
  - Keep original file name and timestamp in the compressed file

- `-L`, `--long-name`
  - Use long file names (default is short file names)

- `-l`, `--list`
  - List files in a compressed archive

- `-N`, `--no-name`
  - Do not save original file name and timestamp in the compressed file

- `-r`, `--recursive`
  - Recursively compress or decompress directories

- `-S`, `--suffix`
  - Set the suffix for compressed files

- `-t`, `--test`
  - Test compression and decompression without actually modifying files

- `-v`, `--verbose`
  - Display verbose output during compression or decompression

- `-V`, `--version`
  - Display version information and exit

---

## ❌ Missing Flags (To Implement)

_All flags are implemented!_ 🎉
