# Stage 03 — Recognize tokens

## Goal

Read the corpus chosen in Stage 02 and print every identifier-like token together with the path, line, and column where it begins.

This is the first stage that looks inside a source file. It turns file contents into searchable occurrences without trying to understand an entire programming language.

## Background

Consider this line from `src/main.rs`:

```rust
let retry_count2 = load_retry_count();
```

A programmer can point out three names immediately: `let`, `retry_count2`, and `load_retry_count`. A computer initially sees a sequence of bytes. It needs an exact rule for deciding where a name begins and ends.

The act of dividing text into searchable pieces is called **tokenization**. Each piece is a **token**.

A Rust compiler has rules for comments, strings, lifetimes, numbers, and punctuation. A Python or C++ compiler has a different set of rules. FlashIndex is not compiling these files, so using a complete language grammar would be unnecessary and would make a multi-language corpus difficult to define.

Instead, FlashIndex uses one deliberately small rule. A token begins with an ASCII letter or `_`. After that first character, ASCII letters, digits, and `_` remain part of the same token.

Under this rule:

```text
retry_count2  → one token
123alpha      → alpha
file-name     → file, name
```

The `123` in `123alpha` is skipped because a digit cannot begin a token. The `alpha` portion still begins a valid token at its real position.

A token by itself is not yet a useful search result. If FlashIndex prints `retry_count2`, the learner still needs to know where it came from. Each occurrence therefore carries a root-relative path, a one-based line number, and a one-based byte column.

The first character in a file is at line 1, column 1. Columns count bytes rather than displayed glyphs. Tokens are ASCII-only in this project, so every character within a token occupies one byte.

## Requirements

Add `flashindex tokenize <path>`. Read only the files admitted by Stage 02.

Recognize the longest runs that begin with an ASCII letter or `_` and continue through ASCII letters, digits, or `_`. Skip leading digits until a valid token start appears. Preserve uppercase and lowercase spelling.

Print every occurrence as:

```text
relative/path:line:column token
```

Use one-based line and byte-column positions. Process files in sorted portable-path order, then print occurrences in their source order within each file. Comments and string literals are ordinary searchable text at this stage.

## Example

For `src/main.rs`:

```rust
fn main() {
    let retry_count2 = 123alpha;
}
```

the relevant output is:

```console
$ flashindex tokenize project
src/main.rs:1:1 fn
src/main.rs:1:4 main
src/main.rs:2:5 let
src/main.rs:2:9 retry_count2
src/main.rs:2:27 alpha
```

Punctuation is absent. The leading digits are absent too, but `alpha` keeps the column where its `a` actually appears.

## Edge cases

- `_` may begin a token and remains part of names such as `_cache` and `fetch_or`.
- Digits remain inside a token after a valid first character, as in `retry_count2`.
- In `123abc`, only `abc` is emitted, at the column where `a` appears.
- Punctuation separates tokens and is never emitted.
- An empty source file produces no occurrences.
- Nested files and occurrences retain stable path and source order.

## Success criteria

All `deltaforge test` cases pass, every printed position identifies the first byte of its token, and the tokenizer benchmark completes.

### Reading the benchmark

After `deltaforge bench`, record the fixture size, occurrence count, median, and p95. Then consider:

1. How much work belongs to reading bytes, and how much belongs to formatting output?
2. Would a few very long tokens behave like many short tokens with the same total byte count?
3. What pair of fixtures could expose repeated memory allocation?
4. Which position check must remain true after an optimization?

## Non-goals

- Parsing any one programming language completely.
- Removing comments or string literals.
- Unicode identifiers or human-language word breaking.
- Lowercasing, stemming, or removing duplicate occurrences.
