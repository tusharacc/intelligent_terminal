# `tar` — Command Reference

**Source**: [linuxcommand.org](https://linuxcommand.org/lc3_man_pages/tar1.php)
**Rust module**: `src-tauri/src/commands/tar.rs`

**Status**: ✅ Fully Implemented (24/24 flags)

---

## ✅ Implemented Flags

- `-A`, `--catenate`, `--concatenate`
  - Append archive to the end of another archive

- `-c`, `--create`
  - Create a new archive

- `-d`, `--diff`, `--compare`
  - Find differences between archive and file system

- `--delete`
  - Delete from the archive

- `-r`, `--append`
  - Append files to the end of an archive

- `-t`, `--list`
  - List the contents of an archive

- `--test-label`
  - Test the archive volume label and exit

- `-u`, `--update`
  - Append files which are newer than the corresponding copy in the archive

- `-v`, `--verbose`
  - Request verbose operation

- `-f`, `--file`
  - Set the name of the archive to operate upon

- `-G`, `--group`
  - Specify group ownership for the archive

- `-i`, `--ignore-zeros`
  - Ignore zero-length holes in the archive

- `-k`, `--keep-newer-files`
  - Keep newer files when updating an existing archive

- `-l`, `--dereference`
  - Dereference symbolic links when archiving

- `-m`, `--multi-volume`
  - Create a multi-volume archive

- `-n`, `--no-recursion`
  - Do not recurse into directories when archiving

- `-o`, `--outdir`
  - Specify the directory where files are to be written

- `-p`, `--preserve-permissions`
  - Preserve permissions of files and directories

- `-P`, `--absolute-names`
  - Use absolute names in the archive

- `-R`, `--remove-files`
  - Remove files from the file system after archiving

- `-s`, `--strip-components`
  - Strip specified number of leading components from file names

- `-S`, `--same-order`
  - Maintain the same order as the original files when archiving

- `-T`, `--ignore-newer`
  - Ignore newer files when updating an existing archive

- `-w`, `--wait-for-completion`
  - Wait for completion of the operation before exiting

---

## ❌ Missing Flags (To Implement)

_All flags are implemented!_ 🎉
