# Bundled curriculum map

The bundled packs preserve their original stage IDs and insert follow-up stages after concepts that were previously bundled together. This keeps existing project state loadable while giving new projects a gentler sequence.

## FlashIndex

1. `01_scan_files` — discover a deterministic, portable file list.
2. `02_filter_files` — choose the searchable corpus.
3. `03_tokenize` — recognize identifier-like tokens and positions.
4. `04_exact_search` — retrieve complete token matches.
5. `05_inverted_index` — group tokens by containing file.
6. `05_canonical_index` — deduplicate and canonically order postings.
7. `06_persist_index` — write a reusable index artifact.
8. `06_query_index` — read and query the saved artifact.
9. `07_benchmark_mode` — expose a machine-readable workload measurement.
10. `08_report_summary` — report corpus and vocabulary counts.
11. `09_parallel_indexing` — preserve index bytes across thread counts.
12. `09_parallel_performance` — measure scaling and meet the speedup gate.
13. `10_ranked_search` — score multi-token candidates.
14. `10_stable_ranking` — define ties, repeated terms, and the result limit.

## MiniKV

1. `01_memory_commands` — establish the key/value command boundary.
2. `02_append_log` — create a durable `SET` record.
3. `02_preserve_history` — append without destroying existing history.
4. `03_recovery` — replay valid history and recover the latest value.
5. `03_reject_malformed_log` — fail ambiguous or damaged history explicitly.
6. `04_compaction` — produce one equivalent live record per key.
7. `04_safe_compaction` — replace destinations safely without changing the input.
8. `05_delete_tombstones` — append and replay durable deletion markers.
9. `05_compact_tombstones` — compact deletion history without resurrection.
10. `06_log_statistics` — distinguish physical records from logical state.

## TinyHTTP

1. `01_parse_request` — parse a valid HTTP request line.
2. `01_strict_request_line` — reject empty and ambiguous request lines.
3. `02_static_response` — frame successful and missing-file responses.
4. `02_safe_paths` — keep client paths inside the document root.
5. `03_headers` — normalize valid HTTP header fields.
6. `03_header_boundaries` — stop at the body boundary and reject malformed fields.
7. `04_keep_alive` — apply HTTP/1.0 and HTTP/1.1 persistence rules.
8. `05_mime_types` — describe response media types.
9. `06_range_requests` — return one valid inclusive byte range.
10. `06_invalid_ranges` — reject malformed, impossible, and unsafe ranges.

## ByteForgeVM

1. `01_disassemble` — load and address a valid instruction listing.
2. `01_loader_errors` — reject malformed program text.
3. `02_stack_arithmetic` — execute stack constants, output, and addition.
4. `02_more_arithmetic` — add multiplication, subtraction, and signed values.
5. `03_control_flow` — perform unconditional jumps.
6. `03_conditional_jumps` — consume conditions and choose a path.
7. `04_errors` — turn unknown operations and stack underflow into guest errors.
8. `04_operand_errors` — validate operands and control-flow targets.
9. `05_call_return` — call one routine and return to its caller.
10. `05_nested_calls` — support nested calls and call-stack failures.
11. `06_trace_mode` — explain execution with a deterministic pre-instruction trace.

## Existing projects

All original IDs remain present, so pinned state continues to refer to a real stage after `deltaforge sync-pack`. The inserted stages are primarily for new learners following the new order. Because tests and fixtures are redistributed, completed stages whose behavioral digest changes require revalidation. A new project is recommended for experiencing the revised progression from the beginning.
