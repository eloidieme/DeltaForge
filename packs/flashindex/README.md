# FlashIndex

## What you are building

Consider a project containing these three files:

```text
src/main.rs
src/network.rs
src/storage.rs
```

You want to find every place where the project uses the name `retry`.

One option is to read all three files from beginning to end. If you search for another name afterward, you read them all again. This is perfectly reasonable for three short files. It becomes wasteful when the project contains thousands of files and you perform many searches.

FlashIndex will prepare the project so that those later searches require less repeated work.

Before it can do that, it needs to answer several questions.

First: which files should it read?

A real project directory contains more than source code. It may contain images, compiler output, downloaded packages, and internal version-control files. Reading all of them would waste time, and some of them do not contain meaningful text at all.

FlashIndex will therefore scan the directory and choose a smaller collection of searchable files. That chosen collection is called the **corpus**.

Second: what should count as a searchable word?

Look at this line:

```rust
let retry_count2 = load_retry_count();
```

A person can easily see names such as `let`, `retry_count2`, and `load_retry_count`. A computer needs an exact rule for finding their boundaries. FlashIndex uses a compact rule for recognizing identifier-like tokens across different programming languages.

Third: what should a search result remember?

Printing only `retry_count2` would not tell the user where to find it. FlashIndex will record each occurrence together with its relative file path, line, and column:

```text
src/network.rs:18:9 retry_count2
```

Once this information exists, the program can search it directly. But if every search still examines every recorded occurrence, the work will grow with the size of the project.

To avoid that repeated scan, FlashIndex will eventually organize its data by token:

```text
retry       → src/main.rs, src/network.rs
retry_count → src/network.rs, src/storage.rs
timeout     → src/network.rs
```

This structure is called an **inverted index**. It starts with the word being searched and points toward the files that contain it.

## Why this is useful

The pieces of FlashIndex appear in many larger tools. Directory walking is used by build systems and file browsers. Tokenization is used by compilers, editors, and language servers. Inverted indexes support document and web search. Persistence lets prepared data outlive one process, while deterministic merging makes concurrent work trustworthy.

The project is also an exercise in defining behavior. A search tool cannot be correct until it says which files, tokens, paths, and ordering rules count as correct.

## Choosing what belongs in the corpus

Search tools have to decide what to include. FlashIndex skips `.git`, `target`, `build`, and `node_modules` because those names commonly contain version-control data, generated build output, or downloaded dependencies. Searching them would often duplicate results and waste time.

Those four names form FlashIndex's default boundary; they are not a universal list. A project can use a different build directory, and someone may occasionally want to inspect a dependency. A configurable search tool could expose that decision to its user. FlashIndex keeps one fixed definition so every command agrees about the files it sees.

The extension list is another practical boundary. FlashIndex includes Rust, C, C++, Python, Markdown, plain-text, and CMake files. Other formats are not considered searchable here. This keeps corpus selection based on names instead of adding content sniffing and encoding detection.

## From files to ranked results

The design becomes easier to reason about as a chain of connected capabilities:

1. Walk a directory and print every regular file in stable order.
2. Choose which filenames belong to the searchable corpus.
3. Recognize identifier-like tokens and record their positions.
4. Find exact token occurrences without substring surprises.
5. Group each token with the files containing it.
6. Sort and deduplicate that grouped representation.
7. Write the canonical index to a disk artifact.
8. Query the saved artifact in a later process.
9. Report a scan workload as machine-readable benchmark data.
10. Summarize files, token occurrences, and unique token spellings.
11. Build the same index with one or more workers.
12. Measure whether extra workers actually improve the workload.
13. Score files that match several query tokens.
14. Break score ties and limit the final ranked result.

Each capability prepares information needed by the next. File discovery defines the corpus; tokenization turns that corpus into occurrences; canonical indexing makes those occurrences reusable; persistence, parallel construction, and ranking build on the same definitions.

## From characters to locations

FlashIndex uses a small token rule:

- a token begins with an ASCII letter or `_`;
- later characters may also contain ASCII digits;
- punctuation and whitespace separate tokens;
- spelling and case are preserved.

With that rule, `retry_count2` is one token, while the leading digits in `123alpha` are skipped and `alpha` begins where the letter `a` appears.

Locations use one-based line and column numbers because those are natural to display to a person. FlashIndex defines a column as the byte position where the printed token begins. Since its tokens contain only ASCII characters, every character inside a token occupies one byte. A Unicode-aware search tool would need a separate decision about code points, grapheme clusters, tabs, and editor display columns.

The same token rule must be reused later. If indexing treats `_cache` as one token but searching treats `_` as punctuation, a correctly built index can still appear broken. Shared definitions matter as much as shared data structures.

## Why the index is “inverted”

A source file naturally gives you tokens in document order:

```text
src/main.rs → fn, main, retry, retry
```

A search asks the question in the opposite direction:

```text
retry → which files?
```

The index inverts the relationship. Its per-token list of files is often called a **posting list**. A token may occur twenty times in one file, but a file-level posting list should name that file once. Occurrences and postings answer different questions.

FlashIndex also sorts token records and paths. Filesystems do not promise to discover directory entries in the same order on every machine, and parallel workers do not promise to finish in a convenient order. Sorting turns many possible construction histories into one canonical result. Saved artifacts are then reproducible and easy to compare byte for byte.

## Saving work for later

An in-memory index disappears when the process exits. Persistence lets one command pay the construction cost and another command query the result later.

The saved file is a contract between a writer and a future reader. The writer must replace old contents completely; otherwise rebuilding a short index over a longer one can leave stale bytes at the end. The reader must preserve exact token matching and return the same sorted, deduplicated paths as the in-memory representation.

Long-lived formats also need version markers, escaping rules, corruption detection, and plans for upgrades. FlashIndex keeps its format small, but the underlying question is already real: what must remain true when the producer and consumer do not run at the same time?

## Measuring before optimizing

A benchmark distinguishes an observation from a claim. Reporting elapsed time is an observation. Claiming that eight threads are faster is a comparison that requires the same workload, the same output, and repeated measurements.

Correctness comes first: worker-local results must merge into byte-identical canonical output. A benchmark matrix can then compare thread counts and derive speedup from their medians. A fast wrong result is not an optimization.

Extra workers also have costs: dividing work, starting threads, combining partial maps, and contending for memory or shared structures. Small corpora may become slower. That is not a paradox; it is evidence that parallelism has overhead and should be measured on a representative workload.

## Ranking more than one token

An exact query asks whether one token exists. A multi-token query creates degrees of usefulness. FlashIndex uses three scoring rules:

1. prefer files that cover more distinct query tokens;
2. then prefer files with more total occurrences of those tokens;
3. then use the portable relative path to break an exact tie.

The first rule rewards breadth; the second rewards density; the third does not pretend to measure relevance at all. It exists to make the order complete and repeatable. The command prints only the top ten results after that full order is established.

Search systems have explored much richer ranking for decades. Printed concordances already mapped words to locations. Later information-retrieval systems used statistics such as term frequency and document frequency, and modern engines often use descendants of TF-IDF or BM25. FlashIndex keeps its scores visible enough to calculate by hand.

## Words you will meet

- **Corpus:** the documents a search system has agreed to read.
- **Token:** one searchable unit produced from source text.
- **Occurrence:** one token at one path, line, and column.
- **Posting list:** the files associated with a token.
- **Inverted index:** a mapping from tokens to their postings.
- **Canonical output:** one stable representation chosen from several equivalent construction orders.
- **Speedup:** baseline runtime divided by runtime after adding parallel resources.
- **Determinism:** identical observable results for identical input.

## What a strong solution looks like

A strong solution has one corpus policy and one token definition reused everywhere. It prints portable `/`-separated relative paths, does not depend on filesystem or worker completion order, replaces persisted output rather than leaving stale data, and checks that an optimization preserves exact results. Every behavior can be understood from a small example before it is generalized to a large project.

Natural extensions include Unicode-aware tokens, configurable ignore rules, comments-and-strings awareness, incremental updates, deleted-file detection, phrase search, prefix search, and BM25-style ranking. Each extension changes the product's definition of correctness. Write that changed definition down before choosing a clever data structure.
