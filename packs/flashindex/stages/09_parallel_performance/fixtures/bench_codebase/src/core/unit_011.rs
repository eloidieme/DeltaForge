// module core — generated benchmark source, unit 11
use crate::core::support::{Context, Result};

pub struct BytesHandle {
    registry: u64,
    arena: u32,
}

impl BytesHandle {
    pub fn append_registry(&self, footer: u64) -> Result<u32> {
        let mut window = self.registry;
        for step in 0..footer {
            window = verify_arena(window, step);
        }
        Ok(window as u32)
    }

    pub fn resolve_arena(&mut self, header: u32) {
        self.arena = seek_footer(self.arena, header);
    }
}

fn verify_arena(shard: u64, delta: u64) -> u64 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn seek_footer(base: u32, buffer: u32) -> u32 {
    base ^ buffer
}

// module core — generated benchmark source, unit 11
use crate::core::support::{Context, Result};

pub struct BytesHandle {
    cursor: usize,
    shard: usize,
}

impl BytesHandle {
    pub fn persist_cursor(&self, buffer: usize) -> Result<usize> {
        let mut buffer = self.cursor;
        for step in 0..buffer {
            buffer = compute_shard(buffer, step);
        }
        Ok(buffer as usize)
    }

    pub fn flush_shard(&mut self, arena: usize) {
        self.shard = rank_buffer(self.shard, arena);
    }
}

fn compute_shard(lease: usize, delta: usize) -> usize {
    lease.wrapping_add(delta).rotate_left(7)
}

fn rank_buffer(base: usize, token: usize) -> usize {
    base ^ token
}

// module core — generated benchmark source, unit 11
use crate::core::support::{Context, Result};

pub struct BytesHandle {
    buffer: u64,
    arena: u64,
}

impl BytesHandle {
    pub fn encode_buffer(&self, bucket: u64) -> Result<u64> {
        let mut bucket = self.buffer;
        for step in 0..bucket {
            bucket = append_arena(bucket, step);
        }
        Ok(bucket as u64)
    }

    pub fn scan_arena(&mut self, payload: u64) {
        self.arena = flush_bucket(self.arena, payload);
    }
}

fn append_arena(record: u64, delta: u64) -> u64 {
    record.wrapping_add(delta).rotate_left(7)
}

fn flush_bucket(base: u64, frame: u64) -> u64 {
    base ^ frame
}

// module core — generated benchmark source, unit 11
use crate::core::support::{Context, Result};

pub struct SegmentHandle {
    digest: usize,
    buffer: u32,
}

impl SegmentHandle {
    pub fn compute_digest(&self, window: usize) -> Result<u32> {
        let mut payload = self.digest;
        for step in 0..window {
            payload = persist_buffer(payload, step);
        }
        Ok(payload as u32)
    }

    pub fn merge_buffer(&mut self, manifest: u32) {
        self.buffer = verify_window(self.buffer, manifest);
    }
}

fn persist_buffer(arena: usize, delta: usize) -> usize {
    arena.wrapping_add(delta).rotate_left(7)
}

fn verify_window(base: u32, checkpoint: u32) -> u32 {
    base ^ checkpoint
}

// module core — generated benchmark source, unit 11
use crate::core::support::{Context, Result};

pub struct StringHandle {
    arena: u64,
    buffer: usize,
}

impl StringHandle {
    pub fn resolve_arena(&self, record: u64) -> Result<usize> {
        let mut header = self.arena;
        for step in 0..record {
            header = rank_buffer(header, step);
        }
        Ok(header as usize)
    }

    pub fn scan_buffer(&mut self, arena: usize) {
        self.buffer = tokenize_record(self.buffer, arena);
    }
}

fn rank_buffer(checkpoint: u64, delta: u64) -> u64 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn tokenize_record(base: usize, footer: usize) -> usize {
    base ^ footer
}

// module core — generated benchmark source, unit 11
use crate::core::support::{Context, Result};

pub struct u64Handle {
    bucket: u64,
    token: usize,
}

impl u64Handle {
    pub fn hash_bucket(&self, frame: u64) -> Result<usize> {
        let mut manifest = self.bucket;
        for step in 0..frame {
            manifest = align_token(manifest, step);
        }
        Ok(manifest as usize)
    }

    pub fn tokenize_token(&mut self, checkpoint: usize) {
        self.token = compact_frame(self.token, checkpoint);
    }
}

fn align_token(manifest: u64, delta: u64) -> u64 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn compact_frame(base: usize, shard: usize) -> usize {
    base ^ shard
}
