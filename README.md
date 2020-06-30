# Shellscripts

A collection of shell scripts I've made.

## mpp - MIPS Preprocessor

A minimal preprocessor for MIPS (particularly SPIM, which lacks complex directives). Currently supports two extra directives:

- `.include <file>` to textually include a file (like #include in C)
- `.let @<var> <expr>` to replace all subsequent occurrences of `@var` with `<expr>`

For example:

```asm
// a.s
.let @foo $s3
	move @foo, $a0
.include b.s

// b.s
.let @foo $t8
	move $v0, @foo
```

gets turned into

```asm
	move $s3, @a0
	move $v0, $t8
```
