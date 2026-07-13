# Stage 04 — Find an exact token

## Goal

Add a search command that prints every occurrence of one complete token.

The tokenizer can now describe everything it finds. This stage asks it to answer one focused question: where does this exact name appear?

## Background

Suppose a project contains these names:

```text
main
main_loop
domain
```

A character search for `main` can find all three strings because those five letters appear inside each one. That may be useful when searching prose, but it is often too broad when looking for a program identifier.

FlashIndex already has a boundary rule from Stage 03. According to that rule, `main`, `main_loop`, and `domain` are three different tokens. An exact-token search should therefore return `main` and nothing else.

This stage does not introduce a second interpretation of source text. `tokenize` and `search` must agree. If one command thinks `_` belongs inside a token while the other treats it as punctuation, the same project would have two incompatible meanings.

Case is also part of the token text. `Request`, `request`, and `REQUEST` remain different names. That is a deliberate project rule; some search tools provide an optional case-insensitive mode, but FlashIndex does not add one here.

No result is an ordinary answer. If the requested token does not appear, the command succeeds and prints nothing. A missing match is different from a missing directory or malformed command.

## Requirements

Add:

```console
flashindex search <path> <token>
```

Reuse Stage 02 file selection and every Stage 03 token rule: boundaries, case, positions, portable paths, and ordering.

Print only occurrences whose complete token is byte-for-byte equal to `<token>`. Use the same line format as `tokenize`:

```text
path:line:column token
```

When nothing matches, exit 0 with empty stdout. Missing arguments or an unreadable root must exit non-zero.

## Example

Given:

```rust
fn main() {
    let main_loop = domain();
}
```

the query is exact:

```console
$ flashindex search project main
src/main.rs:1:4 main
```

`main_loop` and `domain` are not partial matches.

## Edge cases

- A query never matches a substring inside a larger token.
- Matching is case-sensitive.
- The same token may produce several occurrences in one file or across several files.
- Multiple matches retain sorted path order and source order.
- A token absent from the corpus produces empty stdout and status 0.

## Success criteria

All `deltaforge test` cases pass and every search occurrence is identical to an occurrence that `tokenize` would print for the same corpus.

## Non-goals

- Substring, prefix, regular-expression, or fuzzy search.
- Case-insensitive matching.
- Ranking one exact occurrence above another.
- Reading a persisted index; that comes later.
