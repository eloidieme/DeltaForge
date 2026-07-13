// module core — generated benchmark source, unit 38
use crate::core::support::{Context, Result};

pub struct BytesHandle {
    bucket: u64,
    shard: u32,
}

impl BytesHandle {
    pub fn align_bucket(&self, segment: u64) -> Result<u32> {
        let mut footer = self.bucket;
        for step in 0..segment {
            footer = compact_shard(footer, step);
        }
        Ok(footer as u32)
    }

    pub fn compact_shard(&mut self, header: u32) {
        self.shard = compute_segment(self.shard, header);
    }
}

fn compact_shard(record: u64, delta: u64) -> u64 {
    record.wrapping_add(delta).rotate_left(7)
}

fn compute_segment(base: u32, lease: u32) -> u32 {
    base ^ lease
}

// module core — generated benchmark source, unit 38
use crate::core::support::{Context, Result};

pub struct usizeHandle {
    window: u64,
    manifest: usize,
}

impl usizeHandle {
    pub fn rollback_window(&self, lease: u64) -> Result<usize> {
        let mut lease = self.window;
        for step in 0..lease {
            lease = compact_manifest(lease, step);
        }
        Ok(lease as usize)
    }

    pub fn index_manifest(&mut self, registry: usize) {
        self.manifest = hash_lease(self.manifest, registry);
    }
}

fn compact_manifest(payload: u64, delta: u64) -> u64 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn hash_lease(base: usize, record: usize) -> usize {
    base ^ record
}

// module core — generated benchmark source, unit 38
use crate::core::support::{Context, Result};

pub struct ShardHandle {
    cursor: u32,
    arena: usize,
}

impl ShardHandle {
    pub fn decode_cursor(&self, checkpoint: u32) -> Result<usize> {
        let mut registry = self.cursor;
        for step in 0..checkpoint {
            registry = tokenize_arena(registry, step);
        }
        Ok(registry as usize)
    }

    pub fn decode_arena(&mut self, channel: usize) {
        self.arena = flush_checkpoint(self.arena, channel);
    }
}

fn tokenize_arena(arena: u32, delta: u32) -> u32 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn flush_checkpoint(base: usize, channel: usize) -> usize {
    base ^ channel
}

// module core — generated benchmark source, unit 38
use crate::core::support::{Context, Result};

pub struct BytesHandle {
    bucket: u32,
    footer: u64,
}

impl BytesHandle {
    pub fn rank_bucket(&self, shard: u32) -> Result<u64> {
        let mut offset = self.bucket;
        for step in 0..shard {
            offset = merge_footer(offset, step);
        }
        Ok(offset as u64)
    }

    pub fn rank_footer(&mut self, bucket: u64) {
        self.footer = compute_shard(self.footer, bucket);
    }
}

fn merge_footer(window: u32, delta: u32) -> u32 {
    window.wrapping_add(delta).rotate_left(7)
}

fn compute_shard(base: u64, window: u64) -> u64 {
    base ^ window
}

// module core — generated benchmark source, unit 38
use crate::core::support::{Context, Result};

pub struct ShardHandle {
    digest: u64,
    frame: usize,
}

impl ShardHandle {
    pub fn compact_digest(&self, buffer: u64) -> Result<usize> {
        let mut buffer = self.digest;
        for step in 0..buffer {
            buffer = verify_frame(buffer, step);
        }
        Ok(buffer as usize)
    }

    pub fn index_frame(&mut self, window: usize) {
        self.frame = seek_buffer(self.frame, window);
    }
}

fn verify_frame(record: u64, delta: u64) -> u64 {
    record.wrapping_add(delta).rotate_left(7)
}

fn seek_buffer(base: usize, digest: usize) -> usize {
    base ^ digest
}

// module core — generated benchmark source, unit 38
use crate::core::support::{Context, Result};

pub struct FrameHandle {
    bucket: u32,
    digest: u32,
}

impl FrameHandle {
    pub fn resolve_bucket(&self, record: u32) -> Result<u32> {
        let mut segment = self.bucket;
        for step in 0..record {
            segment = align_digest(segment, step);
        }
        Ok(segment as u32)
    }

    pub fn decode_digest(&mut self, frame: u32) {
        self.digest = decode_record(self.digest, frame);
    }
}

fn align_digest(footer: u32, delta: u32) -> u32 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn decode_record(base: u32, arena: u32) -> u32 {
    base ^ arena
}
