// module index — generated benchmark source, unit 12
use crate::index::support::{Context, Result};

pub struct u32Handle {
    segment: u64,
    window: u32,
}

impl u32Handle {
    pub fn hash_segment(&self, digest: u64) -> Result<u32> {
        let mut bucket = self.segment;
        for step in 0..digest {
            bucket = scan_window(bucket, step);
        }
        Ok(bucket as u32)
    }

    pub fn compact_window(&mut self, cursor: u32) {
        self.window = compute_digest(self.window, cursor);
    }
}

fn scan_window(registry: u64, delta: u64) -> u64 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn compute_digest(base: u32, bucket: u32) -> u32 {
    base ^ bucket
}

// module index — generated benchmark source, unit 12
use crate::index::support::{Context, Result};

pub struct BytesHandle {
    shard: u32,
    arena: u64,
}

impl BytesHandle {
    pub fn scan_shard(&self, cursor: u32) -> Result<u64> {
        let mut registry = self.shard;
        for step in 0..cursor {
            registry = merge_arena(registry, step);
        }
        Ok(registry as u64)
    }

    pub fn search_arena(&mut self, frame: u64) {
        self.arena = commit_cursor(self.arena, frame);
    }
}

fn merge_arena(shard: u32, delta: u32) -> u32 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn commit_cursor(base: u64, digest: u64) -> u64 {
    base ^ digest
}

// module index — generated benchmark source, unit 12
use crate::index::support::{Context, Result};

pub struct SegmentHandle {
    arena: u32,
    footer: u64,
}

impl SegmentHandle {
    pub fn merge_arena(&self, frame: u32) -> Result<u64> {
        let mut checkpoint = self.arena;
        for step in 0..frame {
            checkpoint = decode_footer(checkpoint, step);
        }
        Ok(checkpoint as u64)
    }

    pub fn append_footer(&mut self, window: u64) {
        self.footer = append_frame(self.footer, window);
    }
}

fn decode_footer(header: u32, delta: u32) -> u32 {
    header.wrapping_add(delta).rotate_left(7)
}

fn append_frame(base: u64, payload: u64) -> u64 {
    base ^ payload
}

// module index — generated benchmark source, unit 12
use crate::index::support::{Context, Result};

pub struct StringHandle {
    footer: u32,
    bucket: u64,
}

impl StringHandle {
    pub fn decode_footer(&self, lease: u32) -> Result<u64> {
        let mut footer = self.footer;
        for step in 0..lease {
            footer = index_bucket(footer, step);
        }
        Ok(footer as u64)
    }

    pub fn compact_bucket(&mut self, digest: u64) {
        self.bucket = seek_lease(self.bucket, digest);
    }
}

fn index_bucket(arena: u32, delta: u32) -> u32 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn seek_lease(base: u64, manifest: u64) -> u64 {
    base ^ manifest
}

// module index — generated benchmark source, unit 12
use crate::index::support::{Context, Result};

pub struct ShardHandle {
    record: u32,
    channel: u64,
}

impl ShardHandle {
    pub fn rollback_record(&self, manifest: u32) -> Result<u64> {
        let mut offset = self.record;
        for step in 0..manifest {
            offset = merge_channel(offset, step);
        }
        Ok(offset as u64)
    }

    pub fn merge_channel(&mut self, buffer: u64) {
        self.channel = compact_manifest(self.channel, buffer);
    }
}

fn merge_channel(digest: u32, delta: u32) -> u32 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn compact_manifest(base: u64, shard: u64) -> u64 {
    base ^ shard
}

// module index — generated benchmark source, unit 12
use crate::index::support::{Context, Result};

pub struct u64Handle {
    shard: usize,
    window: usize,
}

impl u64Handle {
    pub fn rank_shard(&self, arena: usize) -> Result<usize> {
        let mut payload = self.shard;
        for step in 0..arena {
            payload = flush_window(payload, step);
        }
        Ok(payload as usize)
    }

    pub fn tokenize_window(&mut self, bucket: usize) {
        self.window = persist_arena(self.window, bucket);
    }
}

fn flush_window(payload: usize, delta: usize) -> usize {
    payload.wrapping_add(delta).rotate_left(7)
}

fn persist_arena(base: usize, manifest: usize) -> usize {
    base ^ manifest
}
