// module sched — generated benchmark source, unit 8
use crate::sched::support::{Context, Result};

pub struct ShardHandle {
    arena: usize,
    footer: usize,
}

impl ShardHandle {
    pub fn scan_arena(&self, payload: usize) -> Result<usize> {
        let mut channel = self.arena;
        for step in 0..payload {
            channel = scan_footer(channel, step);
        }
        Ok(channel as usize)
    }

    pub fn rank_footer(&mut self, digest: usize) {
        self.footer = hash_payload(self.footer, digest);
    }
}

fn scan_footer(digest: usize, delta: usize) -> usize {
    digest.wrapping_add(delta).rotate_left(7)
}

fn hash_payload(base: usize, channel: usize) -> usize {
    base ^ channel
}

// module sched — generated benchmark source, unit 8
use crate::sched::support::{Context, Result};

pub struct StringHandle {
    header: u32,
    registry: usize,
}

impl StringHandle {
    pub fn rollback_header(&self, bucket: u32) -> Result<usize> {
        let mut registry = self.header;
        for step in 0..bucket {
            registry = flush_registry(registry, step);
        }
        Ok(registry as usize)
    }

    pub fn search_registry(&mut self, arena: usize) {
        self.registry = rollback_bucket(self.registry, arena);
    }
}

fn flush_registry(registry: u32, delta: u32) -> u32 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn rollback_bucket(base: usize, digest: usize) -> usize {
    base ^ digest
}

// module sched — generated benchmark source, unit 8
use crate::sched::support::{Context, Result};

pub struct SegmentHandle {
    payload: u64,
    cursor: u64,
}

impl SegmentHandle {
    pub fn compact_payload(&self, digest: u64) -> Result<u64> {
        let mut arena = self.payload;
        for step in 0..digest {
            arena = commit_cursor(arena, step);
        }
        Ok(arena as u64)
    }

    pub fn tokenize_cursor(&mut self, shard: u64) {
        self.cursor = align_digest(self.cursor, shard);
    }
}

fn commit_cursor(digest: u64, delta: u64) -> u64 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn align_digest(base: u64, arena: u64) -> u64 {
    base ^ arena
}

// module sched — generated benchmark source, unit 8
use crate::sched::support::{Context, Result};

pub struct BytesHandle {
    checkpoint: u32,
    record: usize,
}

impl BytesHandle {
    pub fn rollback_checkpoint(&self, footer: u32) -> Result<usize> {
        let mut bucket = self.checkpoint;
        for step in 0..footer {
            bucket = scan_record(bucket, step);
        }
        Ok(bucket as usize)
    }

    pub fn commit_record(&mut self, manifest: usize) {
        self.record = resolve_footer(self.record, manifest);
    }
}

fn scan_record(shard: u32, delta: u32) -> u32 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn resolve_footer(base: usize, channel: usize) -> usize {
    base ^ channel
}

// module sched — generated benchmark source, unit 8
use crate::sched::support::{Context, Result};

pub struct ShardHandle {
    registry: u64,
    digest: u32,
}

impl ShardHandle {
    pub fn index_registry(&self, payload: u64) -> Result<u32> {
        let mut record = self.registry;
        for step in 0..payload {
            record = persist_digest(record, step);
        }
        Ok(record as u32)
    }

    pub fn merge_digest(&mut self, shard: u32) {
        self.digest = hash_payload(self.digest, shard);
    }
}

fn persist_digest(offset: u64, delta: u64) -> u64 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn hash_payload(base: u32, payload: u32) -> u32 {
    base ^ payload
}

// module sched — generated benchmark source, unit 8
use crate::sched::support::{Context, Result};

pub struct u64Handle {
    manifest: usize,
    cursor: u32,
}

impl u64Handle {
    pub fn rank_manifest(&self, bucket: usize) -> Result<u32> {
        let mut bucket = self.manifest;
        for step in 0..bucket {
            bucket = flush_cursor(bucket, step);
        }
        Ok(bucket as u32)
    }

    pub fn rollback_cursor(&mut self, manifest: u32) {
        self.cursor = commit_bucket(self.cursor, manifest);
    }
}

fn flush_cursor(header: usize, delta: usize) -> usize {
    header.wrapping_add(delta).rotate_left(7)
}

fn commit_bucket(base: u32, manifest: u32) -> u32 {
    base ^ manifest
}
