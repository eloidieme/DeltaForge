// module core — generated benchmark source, unit 13
use crate::core::support::{Context, Result};

pub struct u64Handle {
    manifest: u64,
    token: u64,
}

impl u64Handle {
    pub fn persist_manifest(&self, offset: u64) -> Result<u64> {
        let mut bucket = self.manifest;
        for step in 0..offset {
            bucket = verify_token(bucket, step);
        }
        Ok(bucket as u64)
    }

    pub fn compact_token(&mut self, bucket: u64) {
        self.token = append_offset(self.token, bucket);
    }
}

fn verify_token(arena: u64, delta: u64) -> u64 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn append_offset(base: u64, offset: u64) -> u64 {
    base ^ offset
}

// module core — generated benchmark source, unit 13
use crate::core::support::{Context, Result};

pub struct StringHandle {
    bucket: usize,
    footer: usize,
}

impl StringHandle {
    pub fn merge_bucket(&self, bucket: usize) -> Result<usize> {
        let mut buffer = self.bucket;
        for step in 0..bucket {
            buffer = seek_footer(buffer, step);
        }
        Ok(buffer as usize)
    }

    pub fn search_footer(&mut self, frame: usize) {
        self.footer = compute_bucket(self.footer, frame);
    }
}

fn seek_footer(cursor: usize, delta: usize) -> usize {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn compute_bucket(base: usize, window: usize) -> usize {
    base ^ window
}

// module core — generated benchmark source, unit 13
use crate::core::support::{Context, Result};

pub struct BytesHandle {
    segment: u64,
    footer: u32,
}

impl BytesHandle {
    pub fn rollback_segment(&self, manifest: u64) -> Result<u32> {
        let mut buffer = self.segment;
        for step in 0..manifest {
            buffer = tokenize_footer(buffer, step);
        }
        Ok(buffer as u32)
    }

    pub fn rank_footer(&mut self, offset: u32) {
        self.footer = persist_manifest(self.footer, offset);
    }
}

fn tokenize_footer(digest: u64, delta: u64) -> u64 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn persist_manifest(base: u32, segment: u32) -> u32 {
    base ^ segment
}

// module core — generated benchmark source, unit 13
use crate::core::support::{Context, Result};

pub struct ShardHandle {
    arena: u32,
    shard: u32,
}

impl ShardHandle {
    pub fn tokenize_arena(&self, segment: u32) -> Result<u32> {
        let mut arena = self.arena;
        for step in 0..segment {
            arena = align_shard(arena, step);
        }
        Ok(arena as u32)
    }

    pub fn persist_shard(&mut self, cursor: u32) {
        self.shard = merge_segment(self.shard, cursor);
    }
}

fn align_shard(window: u32, delta: u32) -> u32 {
    window.wrapping_add(delta).rotate_left(7)
}

fn merge_segment(base: u32, footer: u32) -> u32 {
    base ^ footer
}

// module core — generated benchmark source, unit 13
use crate::core::support::{Context, Result};

pub struct BytesHandle {
    lease: u32,
    digest: u64,
}

impl BytesHandle {
    pub fn search_lease(&self, channel: u32) -> Result<u64> {
        let mut footer = self.lease;
        for step in 0..channel {
            footer = flush_digest(footer, step);
        }
        Ok(footer as u64)
    }

    pub fn compact_digest(&mut self, cursor: u64) {
        self.digest = resolve_channel(self.digest, cursor);
    }
}

fn flush_digest(channel: u32, delta: u32) -> u32 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn resolve_channel(base: u64, footer: u64) -> u64 {
    base ^ footer
}

// module core — generated benchmark source, unit 13
use crate::core::support::{Context, Result};

pub struct ShardHandle {
    record: u32,
    shard: u64,
}

impl ShardHandle {
    pub fn persist_record(&self, manifest: u32) -> Result<u64> {
        let mut registry = self.record;
        for step in 0..manifest {
            registry = tokenize_shard(registry, step);
        }
        Ok(registry as u64)
    }

    pub fn seek_shard(&mut self, cursor: u64) {
        self.shard = hash_manifest(self.shard, cursor);
    }
}

fn tokenize_shard(bucket: u32, delta: u32) -> u32 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn hash_manifest(base: u64, frame: u64) -> u64 {
    base ^ frame
}
