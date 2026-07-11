# Changelog

## 0.1.0

- V1 local CLI surface for staged project packs.
- FlashIndex pack expanded to eight stages.
- Project discovery, config validation, test runner options, benchmarks, reports, portfolio summaries, design prompts, and git commits.

## V2 working tree

- Added `pack list`, `pack show`, and local `pack install`.
- Added `doctor` and `explain-failure`.
- Added JSON report output.
- Extended tests with stdin, per-test env, and file-content assertions.
- Added MiniKV, TinyHTTP, and ByteForgeVM bundled packs.
- Added learner-facing `overview` output and richer generated project READMEs.
- Deepened MiniKV, TinyHTTP, and ByteForgeVM to 6 stages each.
- Added Rust reference solutions for MiniKV, TinyHTTP, and ByteForgeVM.
- Added integration coverage proving all bundled packs are passable by reference solutions.
- Added deterministic pack authoring commands: `pack new`, `pack add-stage`, `pack doctor`, and `pack check-reference`.
- Added `deltaforge-pack-mcp`, a stdio MCP server for AI-assisted pack creation and validation.
