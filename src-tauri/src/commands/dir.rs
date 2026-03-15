use crate::state::TerminalState;

/// `dir` is equivalent to `ls -C -b` per GNU coreutils.
/// Delegates to ls::execute with full flag support.
pub fn execute(
    _cmd: &str,
    args: &[&str],
    stdin: Option<String>,
    st: &mut TerminalState,
) -> Result<String, String> {
    if args.contains(&"--help") || args.contains(&"-h") {
        return Ok(r#"dir - Linuxcommand.org Standard Parity

  -a, --all             do not ignore entries starting with .
  -A, --almost-all      do not list implied . and ..
  --author              with -l, print the author of each file
  -b, --escape          print C-style escapes for nongraphic characters
  --block-size=SIZE     with -l, scale sizes by SIZE when printing them; e.g., '--block-size=M'; see ...
  -B, --ignore-backups  do not list implied entries ending with ~
  -c                    with -lt: sort by, and show, ctime (time of last modification of file status ...
  -C                    list entries by columns
  --color[=WHEN]        colorize the output; WHEN can be 'always' (default if omitted), 'auto', or 'n...
  -d, --directory       list directories themselves, not their contents
  -D, --dired           generate output designed for Emacs' dired mode
  -f                    do not sort, enable -aU, disable -ls --color
  -F, --classify        append indicator (one of */=>@|) to entries
  --file-type           likewise, except do not append '*'
  --format=WORD         across -x, commas -m, horizontal -x, long -l, single-column -1, verbose -l, v...
  --full-time           like -l --time-style=full-iso
  -g, --group-directories-first  group directories before files; can be augmented with a --sort option, but an...
  -G, --no-group        in a long listing, don't print group names
  -h, --human-readable  with -l and -s, print sizes like 1K 234M 2G etc.
  --si                  likewise, but use powers of 1000 not 1024
  -H, --dereference-command-line  follow symbolic links listed on the command line
  --dereference-command-line-symlink-to-dir  follow each command line symbolic link that points to a directory
  --hide=PATTERN        do not list implied entries matching shell PATTERN (overridden by -a or -A)
  --hyperlink[=WHEN]    hyperlink file names; WHEN can be 'always' (default if omitted), 'auto', or '...
  --indicator-style=WORD  append indicator with style WORD to entry names: none (default), slash (-p), ...
  -i, --inode           print the index number of each file
  -I, --ignore=PATTERN  do not list implied entries matching shell PATTERN
  -k, --kibibytes       default to 1024-byte blocks for disk usage; used only with -s and per directo...
  -l                    use a long listing format
  -L, --dereference     when showing file information for a symbolic link, show information for the f...
  -m                    fill width with a comma separated list of entries
  -n, --numeric-uid-gid like -l, but list numeric user and group IDs
  -N, --literal         print entry names without quoting
  -o                    like -l, but do not list group information
  -p, --indicator-style=slash  append / indicator to directories
  -q, --hide-control-chars  print ? instead of nongraphic characters
  --show-control-chars  show nongraphic characters as-is (the default, unless program is 'ls' and out...
  -Q, --quote-name      enclose entry names in double quotes
  --quoting-style=WORD  use quoting style WORD for entry names: literal, locale, shell, shell-always,...
  -r, --reverse         reverse order while sorting
  -R, --recursive       list subdirectories recursively
  -s, --size            print the allocated size of each file, in blocks
  -S                    sort by file size, largest first
  --sort=WORD           sort by WORD instead of name: none (-U), size (-S), time (-t), version (-v), ...
  --time=WORD           change the default of using modification times; access time (-u): atime, acce...
  -t                    sort by time, newest first; see --time
  -T, --tabsize=COLS    assume tab stops at each COLS instead of 8
  -u                    with -lt: sort by, and show, access time; with -l: show access time and sort ...
  -U                    do not sort; list entries in directory order
  -v                    natural sort of (version) numbers within text
  -w, --width=COLS      set output width to COLS. 0 means no limit
  -x                    list entries by lines instead of by columns
  -X                    sort alphabetically by entry extension
  --help                display this help and exit
  --version             output version information and exit"#.to_string());
    }

    // Delegate to ls with all flag handling
    super::ls::execute("dir", args, stdin, st)
}
