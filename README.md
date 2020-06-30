# Shellscripts

A collection of shell scripts I've made.

## mpp - MIPS Preprocessor

A minimal preprocessor for MIPS (particularly SPIM, which lacks complex directives). Currently supports two extra directives:

- `.include <file>` to textually include a file (like #include in C)
- `.let @<var> <expr>` to replace all subsequent occurrences of `@var` with `<expr>`

For example:

```asm
# a.s
.let @foo $s3
	move @foo, $a0
.include b.s

# b.s
.let @foo $t8
	move $v0, @foo
```

gets turned into

```asm
	move $s3, @a0
	move $v0, $t8
```

## cs - CSE server helper

This is like the git command in that its actions are accessible via `cs <command>`. Most of these require the environment variables `$COURSE` (e.g. `2041`) and `$TASK` (e.g. `lab03`).

- `cs help` shows a summary of the available commands.
- `cs fetch` to fetch the files for the task (like `1521 fetch lab03`) into the current directory.
- `cs task <task>` downloads the resources into a folder, but doesn't require `$TASK` to be set.
- `cs remote <cmd>...` to sync the files to CSE and run a command.
- `cs autotest <test>...` to run an autotest on your files (like `2041 autotest shell_backup`)
- `cs give <exercise> <files>...` to submit files for a given exercise. The task is prepended to exercise, i.e. making `cs give shell_backup backup.sh` run `give cs2041 lab05_shell_backup backup.sh`.

These commands assume that `rsync` is installed on your local system and that `ssh cse` connects to the CSE servers.
