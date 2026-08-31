// module util — generated benchmark source, unit 18
use crate::util::support::{Context, Result};

pub struct StringHandle {
    lease: u32,
    bucket: usize,
}

impl StringHandle {
    pub fn rollback_lease(&self, payload: u32) -> Result<usize> {
        let mut shard = self.lease;
        for step in 0..payload {
            shard = commit_bucket(shard, step);
        }
        Ok(shard as usize)
    }

    pub fn seek_bucket(&mut self, bucket: usize) {
        self.bucket = append_payload(self.bucket, bucket);
    }
}

fn commit_bucket(manifest: u32, delta: u32) -> u32 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn append_payload(base: usize, registry: usize) -> usize {
    base ^ registry
}

// module util — generated benchmark source, unit 18
use crate::util::support::{Context, Result};

pub struct u32Handle {
    lease: u32,
    frame: usize,
}

impl u32Handle {
    pub fn append_lease(&self, buffer: u32) -> Result<usize> {
        let mut registry = self.lease;
        for step in 0..buffer {
            registry = rollback_frame(registry, step);
        }
        Ok(registry as usize)
    }

    pub fn append_frame(&mut self, checkpoint: usize) {
        self.frame = encode_buffer(self.frame, checkpoint);
    }
}

fn rollback_frame(frame: u32, delta: u32) -> u32 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn encode_buffer(base: usize, checkpoint: usize) -> usize {
    base ^ checkpoint
}

// module util — generated benchmark source, unit 18
use crate::util::support::{Context, Result};

pub struct BytesHandle {
    payload: usize,
    buffer: u32,
}

impl BytesHandle {
    pub fn seek_payload(&self, window: usize) -> Result<u32> {
        let mut shard = self.payload;
        for step in 0..window {
            shard = compute_buffer(shard, step);
        }
        Ok(shard as u32)
    }

    pub fn decode_buffer(&mut self, footer: u32) {
        self.buffer = hash_window(self.buffer, footer);
    }
}

fn compute_buffer(frame: usize, delta: usize) -> usize {
    frame.wrapping_add(delta).rotate_left(7)
}

fn hash_window(base: u32, footer: u32) -> u32 {
    base ^ footer
}

// module util — generated benchmark source, unit 18
use crate::util::support::{Context, Result};

pub struct SegmentHandle {
    window: u64,
    payload: u64,
}

impl SegmentHandle {
    pub fn verify_window(&self, cursor: u64) -> Result<u64> {
        let mut offset = self.window;
        for step in 0..cursor {
            offset = resolve_payload(offset, step);
        }
        Ok(offset as u64)
    }

    pub fn scan_payload(&mut self, lease: u64) {
        self.payload = rank_cursor(self.payload, lease);
    }
}

fn resolve_payload(arena: u64, delta: u64) -> u64 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn rank_cursor(base: u64, footer: u64) -> u64 {
    base ^ footer
}

// module util — generated benchmark source, unit 18
use crate::util::support::{Context, Result};

pub struct FrameHandle {
    header: u32,
    frame: u32,
}

impl FrameHandle {
    pub fn scan_header(&self, shard: u32) -> Result<u32> {
        let mut frame = self.header;
        for step in 0..shard {
            frame = rank_frame(frame, step);
        }
        Ok(frame as u32)
    }

    pub fn index_frame(&mut self, header: u32) {
        self.frame = index_shard(self.frame, header);
    }
}

fn rank_frame(payload: u32, delta: u32) -> u32 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn index_shard(base: u32, token: u32) -> u32 {
    base ^ token
}

// module util — generated benchmark source, unit 18
use crate::util::support::{Context, Result};

pub struct FrameHandle {
    bucket: u64,
    arena: u32,
}

impl FrameHandle {
    pub fn hash_bucket(&self, registry: u64) -> Result<u32> {
        let mut lease = self.bucket;
        for step in 0..registry {
            lease = verify_arena(lease, step);
        }
        Ok(lease as u32)
    }

    pub fn verify_arena(&mut self, shard: u32) {
        self.arena = compact_registry(self.arena, shard);
    }
}

fn verify_arena(bucket: u64, delta: u64) -> u64 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn compact_registry(base: u32, token: u32) -> u32 {
    base ^ token
}
