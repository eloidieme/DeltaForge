# Stage 03 — Find words in source files

## Goal

Read the files chosen in Stage 02 and turn their text into searchable word-like pieces, each with a path, line, and column that can lead a person back to the source.

## Background

A computer sees a source file as a sequence of bytes. A search tool needs boundaries: where does one searchable piece end and the next begin? The act of splitting text into those pieces is called **tokenization**, and each piece is a **token**.

Full compilers have a different tokenizer for each language. They know about Rust lifetimes, Python strings, C++ comments, and much more. FlashIndex is not a compiler, so this course uses one intentionally small rule across every language: a token is an identifier-like run made from ASCII letters, digits, and `_`.

There is one wrinkle. Programming identifiers commonly contain digits but do not begin with them. Under our rule, `value_32` is one token, while the leading digits in `123alpha` are skipped and `alpha` becomes the token. This is another project policy, not a universal definition of a word.

Positions make search results useful. Line and column numbers are one-based because that is how editors and compiler messages usually speak to people: the first character is at line 1, column 1. Columns count bytes in this stage. Because tokens are ASCII-only, each character inside a token is exactly one byte.

## Requirements

Add `flashindex tokenize <path>`. It must read only the files accepted by Stage 02.

Recognize the longest possible runs of ASCII letters, digits, and `_` that begin with an ASCII letter or `_`. Skip leading digits until a valid token start appears. Preserve the token's original uppercase or lowercase spelling.

Print every occurrence in this form:

```text
relative/path:line:column token
```

Use one-based line and byte-column positions. Sort files by their portable relative paths, then print tokens in the order they appear within each file. For now, words inside comments and string literals count like any other text.

## Example

For a file `src/main.rs` containing:

```rust
fn main() {
    let value_32 = 123alpha;
}
```

the relevant output is:

```console
$ flashindex tokenize project
src/main.rs:1:1 fn
src/main.rs:1:4 main
src/main.rs:2:5 let
src/main.rs:2:9 value_32
src/main.rs:2:23 alpha
```

Notice that punctuation and the leading `123` are not printed.

## Edge cases

- `_` may begin a token and remains inside names such as `_cache` and `fetch_or`.
- Digits remain inside a token after a valid start, as in `value_32`.
- In `123abc`, only `abc` is emitted, at the column where `a` actually appears.
- Punctuation separates tokens and is never emitted.
- Empty source files produce no occurrences.
- Multiple occurrences of the same token are all printed; deduplication comes later.

## Success criteria

All `deltaforge test` cases pass, every reported position points to the first byte of its printed token, and the tokenizer benchmark completes.

### Reading the benchmark

After measuring tokenization, write down the fixture size, occurrence count, median, and p95. Then consider:

1. How much time belongs to scanning bytes, and how much to formatting and printing results?
2. Would a few enormous tokens behave like many short tokens with the same total byte count?
3. What pair of fixtures could reveal repeated memory allocation?
4. Which position check must stay true after any optimization?

### Try it on paper

Choose a punctuation-heavy line and circle each token by hand. Mark its first byte-column. Comparing that sketch with the command output is often the fastest way to find an off-by-one error.

## Non-goals

- Parsing a particular programming language or classifying keywords.
- Removing comments or string literals.
- Unicode identifiers or human-language word breaking.
- Lowercasing, stemming, or removing duplicate tokens.
