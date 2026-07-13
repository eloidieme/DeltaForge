# Stage 01 — Name a value

## Goal

Give MiniKV its first command: accept one key and one value, then print the pair in one exact form.

Nothing is saved. The command's job is simply to give one value a name and return that pair in a stable form.

## Background

Consider a program that needs to remember a preferred language:

```text
language → Rust
```

The left side is the **key**. It is the name used to find the information later. The right side is the **value** stored under that name.

Key-value storage appears in caches, configuration systems, database indexes, and many other tools. The idea is simple: instead of asking for the third row or seventeenth byte, a caller asks for a meaningful name.

MiniKV begins with one pair passed as command-line arguments. The process immediately prints the pair and exits. Three rules already matter: arguments have boundaries, successful output has a stable shape, and invalid commands fail rather than pretending to store incomplete data.

A value containing spaces remains one argument when the caller quotes it in the shell:

```console
minikv memory title "hello world"
```

MiniKV receives `hello world` as one value. It should not split the phrase again merely because it contains a space.

## Requirements

Add:

```console
minikv memory <key> <value>
```

`<key>` and `<value>` are each one command-line argument. On success, print exactly:

```text
<key>=<value>
```

followed by `\n`, and exit 0. Preserve the argument text, including spaces inside a value argument. If either argument is missing, exit non-zero without printing a plausible pair.

## Example

```console
$ minikv memory greeting "hello world"
greeting=hello world
```

The quotation marks belong to the shell command; they are not part of the stored value or printed line.

## Edge cases

- A value containing spaces is printed unchanged.
- A missing value is invalid and exits non-zero.
- One invocation prints only its supplied pair.
- No state survives after the process exits.

## Success criteria

All `deltaforge test` cases pass and successful output is byte-stable for the same arguments.

## Non-goals

- Preserving the pair after the command exits.
- Accepting several operations in one invocation.
- Reading, deleting, or compacting stored values.
- Defining concurrency or durability.
