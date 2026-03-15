# `set` — Command Reference

**Source**: [linuxcommand.org](https://linuxcommand.org/lc3_man_pages/set1.php)
**Rust module**: `src-tauri/src/commands/set.rs`

**Status**: ✅ Fully Implemented (21/21 flags)

---

## ✅ Implemented Flags

- `-a`, `--allexport`
  - Mark variables which are modified or created for export.

- `-b`, `--notify`
  - Notify of job termination immediately.

- `-e`, `--errexit`
  - Exit immediately if a command exits with a non-zero status.

- `-f`, `--noglob`
  - Disable file name generation (globbing).

- `-h`, `--hashall`
  - Remember the location of commands as they are looked up.

- `-k`, `--keyword`
  - All assignment arguments are placed in the environment for a command, not just those that precede the command name.

- `-m`, `--monitor`
  - Job control is enabled.

- `-n`, `--noexec`
  - Read commands but do not execute them.

- `-o`, `--option-name`
  - Set the variable corresponding to option-name.

- `-p`, `--privileged`
  - Turned on whenever the real and effective user ids do not match. Disables processing of the $ENV file and importing of shell functions.

- `-t`, `--onecmd`
  - Exit after reading and executing one command.

- `-u`, `--nounset`
  - Treat unset variables as an error when substituting.

- `-v`, `--verbose`
  - Print shell input lines as they are read.

- `-x`, `--xtrace`
  - Print commands and their arguments as they are executed.

- `-B`, `--braceexpand`
  - The shell will perform brace expansion.

- `-C`, `--noclobber`
  - If set, disallow existing regular files to be overwritten by redirection of output.

- `-E`, `--errtrace`
  - If set, the ERR trap is inherited by shell functions.

- `-H`, `--histexpand`
  - Enable ! style history substitution. This flag is on by default when the shell is interactive.

- `-P`, `--physical`
  - If set, do not resolve symbolic links when executing commands such as cd which change the current directory.

- `-T`, `--functrace`
  - If set, the DEBUG and RETURN traps are inherited by shell functions.

- `--`
  - Assign any remaining arguments to the positional parameters. If there are no remaining arguments, the positional parameters are unset.

---

## ❌ Missing Flags (To Implement)

_All flags are implemented!_ 🎉
