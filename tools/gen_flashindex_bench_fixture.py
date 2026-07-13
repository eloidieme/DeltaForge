#!/usr/bin/env python3
"""Generate the deterministic benchmark fixture for flashindex stage 09.

FlashIndex's stage-09 benchmark measures how indexing throughput scales with
worker threads, so it needs a codebase large enough that the work is real yet
small enough to embed in the DeltaForge binary (`include_dir!`). This script
emits a synthetic-but-realistic Rust-ish source tree with a bounded, repeated
vocabulary so the inverted index has meaningful token overlap across files.

It is deterministic: a fixed seed drives a small linear-congruential generator,
so re-running reproduces byte-identical output. It is NOT run at build or test
time — run it by hand, then commit the generated tree:

    python3 tools/gen_flashindex_bench_fixture.py

Output goes to packs/flashindex/stages/09_parallel_performance/fixtures/bench_codebase
and stays under ~2 MiB total.
"""

from __future__ import annotations

import pathlib
import shutil

# Deterministic LCG (glibc constants). No dependency on Python's `random`, whose
# internals could change between versions and break byte-for-byte reproduction.
_STATE = 0x2545F4914F6CDD1D


def rnd() -> int:
    global _STATE
    _STATE = (_STATE * 6364136223846793005 + 1442695040888963407) & ((1 << 64) - 1)
    return _STATE >> 16


def pick(items):
    return items[rnd() % len(items)]


VERBS = [
    "compute", "resolve", "merge", "scan", "encode", "decode", "flush",
    "commit", "rollback", "index", "search", "rank", "tokenize", "persist",
    "compact", "append", "seek", "align", "hash", "verify",
]
NOUNS = [
    "buffer", "segment", "record", "cursor", "manifest", "shard", "window",
    "digest", "offset", "payload", "header", "footer", "checkpoint", "arena",
    "registry", "channel", "frame", "bucket", "lease", "token",
]
TYPES = ["u32", "u64", "usize", "String", "Bytes", "Frame", "Segment", "Shard"]

MODULES = ["core", "storage", "net", "index", "query", "util", "codec", "sched"]

FILE_TEMPLATE = """\
// module {module} — generated benchmark source, unit {unit}
use crate::{module}::support::{{Context, Result}};

pub struct {ty1} {{
    {noun1}: {prim1},
    {noun2}: {prim2},
}}

impl {ty1} {{
    pub fn {verb1}_{noun1}(&self, {noun3}: {prim1}) -> Result<{prim2}> {{
        let mut {noun4} = self.{noun1};
        for step in 0..{noun3} {{
            {noun4} = {verb2}_{noun2}({noun4}, step);
        }}
        Ok({noun4} as {prim2})
    }}

    pub fn {verb3}_{noun2}(&mut self, {noun5}: {prim2}) {{
        self.{noun2} = {verb4}_{noun3}(self.{noun2}, {noun5});
    }}
}}

fn {verb2}_{noun2}({noun6}: {prim1}, delta: {prim1}) -> {prim1} {{
    {noun6}.wrapping_add(delta).rotate_left(7)
}}

fn {verb4}_{noun3}(base: {prim2}, {noun7}: {prim2}) -> {prim2} {{
    base ^ {noun7}
}}
"""


def render(module: str, unit: int) -> str:
    return FILE_TEMPLATE.format(
        module=module,
        unit=unit,
        ty1=pick(TYPES) + "Handle",
        noun1=pick(NOUNS),
        noun2=pick(NOUNS),
        noun3=pick(NOUNS),
        noun4=pick(NOUNS),
        noun5=pick(NOUNS),
        noun6=pick(NOUNS),
        noun7=pick(NOUNS),
        prim1=pick(["u32", "u64", "usize"]),
        prim2=pick(["u32", "u64", "usize"]),
        verb1=pick(VERBS),
        verb2=pick(VERBS),
        verb3=pick(VERBS),
        verb4=pick(VERBS),
    )


def main() -> None:
    root = pathlib.Path(__file__).resolve().parents[1]
    out = (
        root
        / "packs/flashindex/stages/09_parallel_performance/fixtures/bench_codebase"
    )
    if out.exists():
        shutil.rmtree(out)

    files_per_module = 40  # 8 modules * 40 = 320 files
    repeats = 6  # each file repeats its body block N times to reach a few KB
    total_bytes = 0
    for module in MODULES:
        module_dir = out / "src" / module
        module_dir.mkdir(parents=True, exist_ok=True)
        for unit in range(files_per_module):
            body = "\n".join(render(module, unit) for _ in range(repeats))
            path = module_dir / f"unit_{unit:03d}.rs"
            data = body.encode("utf-8")
            path.write_bytes(data)
            total_bytes += len(data)

    print(f"wrote {out}")
    print(f"total size: {total_bytes} bytes ({total_bytes / 1024 / 1024:.2f} MiB)")


if __name__ == "__main__":
    main()
