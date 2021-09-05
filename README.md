# Micro

A collection of tiny projects I've made.
No guarantees on how well these work -- I might not have touched them in a long time!

## tunnel-run - Transparently run on the mounted filesystem

This is a command wrapper that runs the command over ssh if you're in sshfs, and locally if you're on a local filesystem.

You can also symlink other executables to this script, and `tunnel-run` will transparently tunnel them, or call the underlying executable if running locally.
Doing this with svn or git, for example, can make those run reasonably fast.

Note that this does not detect what options were supplied to sshfs.

Tip: you can ssh into the target machine with `tunnel-run sh -c 'exec $SHELL --login'`.

## mpp - MIPS Preprocessor

A minimal preprocessor for MIPS (particularly SPIM, which lacks complex directives).
Currently supports two extra directives:

- `.include <file>` to textually include a file (like #include in C)
- `.let @<var> <expr>` to replace all subsequent occurrences of `@var` with `<expr>`
- `.unlet @<var>` to undo a previous .let

For example:

<table>
<tr> <th>a.s</th> <th>b.s</th> </tr>
<tr> <td><pre lang="asm">
.let @foo $s3
	move @foo, $a0
.include b.s
</pre></td>

<td><pre lang="asm">
.let @foo $t8
	move $v0, @foo
</pre></td>
</table>

gets turned into

```asm
	move $s3, $a0
	move $v0, $t8
```

## git-ff - fast-forward a branch without checking it out

This is similar to `git pull --ff-only` but can operate on branches you haven't checked out.
There is also an `--all` flag to operate on all branches.
Useful for updating your local repo to the state of upstream.

## rofi-unicode - rofi-based unicode character/string picker

This is currently only compatible with wayland.
The list of choices is embedded within the script.

## ifdef-clean - C preprocessor conditional cleaner

Usage: `ifdef-clean file1.c file2.c file3.c`.
This program works in-place, rewriting the input files.
`#elif` is not supported.

The use case for this program is if you have a C project with platform-specific macros, like this:

```c
#ifdef _WIN32
// windows-specific thing
#else
// not windows
#ifdef __unix__
// linux stuff
#else
// some other operating system
#endif
#endif

#if FEATURE
// some feature
#else
// feature not supported
#endif
```

`ifdef-clean` will prompt you for whether to include (`y`es) or exclude (`n`o) a particular branch of the condition (eliminating corresponding `#if`/`#else`/`#endif` lines), or to keep all branches as-is (`i`gnore).
Your choices will be remembered and you will only be prompted on the first occurrence of any particular condition.

e.g. with the above content in `test.c`:
```
$ ifdef-clean test.c
#ifdef _WIN32 [yni]? n
#ifdef __unix__ [yni]? y
#if FEATURE [yni]? i
```

and `test.c` will now only contain

```c
// not windows
// linux stuff

#if FEATURE
// some feature
#else
// feature not supported
#endif
```

## clacks - Shows icon for X-Clacks-Overhead

This is a browser extension.
If the current page has the `X-Clacks-Overhead` header or a corresponding meta tag, a clacks icon will appear in your URL bar.

## cs - CSE server helper

This is like the git command in that its actions are accessible via `cs <command>`.
Most of these require the environment variables `$COURSE` (e.g. `2041`) and `$TASK` (e.g. `lab03`).

- `cs help` shows a summary of the available commands.
- `cs fetch` to fetch the files for the task (like `1521 fetch lab03`) into the current directory.
- `cs task <task>` downloads the resources into a folder, but doesn't require `$TASK` to be set.
- `cs remote <cmd>...` to sync the files to CSE and run a command.
- `cs autotest <test>...` to run an autotest on your files (like `2041 autotest shell_backup`)
- `cs give <exercise> <files>...` to submit files for a given exercise. The task is prepended to exercise, i.e. making `cs give shell_backup backup.sh` run `give cs2041 lab05_shell_backup backup.sh`.

These commands assume that `rsync` is installed on your local system and that `ssh cse` connects to the CSE servers.
