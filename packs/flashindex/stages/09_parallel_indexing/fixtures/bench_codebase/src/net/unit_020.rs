// module net — generated benchmark source, unit 20
use crate::net::support::{Context, Result};

pub struct FrameHandle {
    window: u32,
    lease: u32,
}

impl FrameHandle {
    pub fn rollback_window(&self, shard: u32) -> Result<u32> {
        let mut arena = self.window;
        for step in 0..shard {
            arena = commit_lease(arena, step);
        }
        Ok(arena as u32)
    }

    pub fn hash_lease(&mut self, digest: u32) {
        self.lease = resolve_shard(self.lease, digest);
    }
}

fn commit_lease(footer: u32, delta: u32) -> u32 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn resolve_shard(base: u32, segment: u32) -> u32 {
    base ^ segment
}

// module net — generated benchmark source, unit 20
use crate::net::support::{Context, Result};

pub struct ShardHandle {
    registry: usize,
    registry: u64,
}

impl ShardHandle {
    pub fn hash_registry(&self, record: usize) -> Result<u64> {
        let mut arena = self.registry;
        for step in 0..record {
            arena = compute_registry(arena, step);
        }
        Ok(arena as u64)
    }

    pub fn commit_registry(&mut self, token: u64) {
        self.registry = tokenize_record(self.registry, token);
    }
}

fn compute_registry(footer: usize, delta: usize) -> usize {
    footer.wrapping_add(delta).rotate_left(7)
}

fn tokenize_record(base: u64, record: u64) -> u64 {
    base ^ record
}

// module net — generated benchmark source, unit 20
use crate::net::support::{Context, Result};

pub struct u32Handle {
    window: u64,
    record: u64,
}

impl u32Handle {
    pub fn align_window(&self, buffer: u64) -> Result<u64> {
        let mut shard = self.window;
        for step in 0..buffer {
            shard = commit_record(shard, step);
        }
        Ok(shard as u64)
    }

    pub fn persist_record(&mut self, checkpoint: u64) {
        self.record = verify_buffer(self.record, checkpoint);
    }
}

fn commit_record(digest: u64, delta: u64) -> u64 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn verify_buffer(base: u64, segment: u64) -> u64 {
    base ^ segment
}

// module net — generated benchmark source, unit 20
use crate::net::support::{Context, Result};

pub struct StringHandle {
    segment: u64,
    bucket: usize,
}

impl StringHandle {
    pub fn verify_segment(&self, cursor: u64) -> Result<usize> {
        let mut offset = self.segment;
        for step in 0..cursor {
            offset = persist_bucket(offset, step);
        }
        Ok(offset as usize)
    }

    pub fn append_bucket(&mut self, cursor: usize) {
        self.bucket = compact_cursor(self.bucket, cursor);
    }
}

fn persist_bucket(token: u64, delta: u64) -> u64 {
    token.wrapping_add(delta).rotate_left(7)
}

fn compact_cursor(base: usize, manifest: usize) -> usize {
    base ^ manifest
}

// module net — generated benchmark source, unit 20
use crate::net::support::{Context, Result};

pub struct usizeHandle {
    checkpoint: usize,
    registry: u64,
}

impl usizeHandle {
    pub fn verify_checkpoint(&self, offset: usize) -> Result<u64> {
        let mut lease = self.checkpoint;
        for step in 0..offset {
            lease = persist_registry(lease, step);
        }
        Ok(lease as u64)
    }

    pub fn hash_registry(&mut self, payload: u64) {
        self.registry = hash_offset(self.registry, payload);
    }
}

fn persist_registry(bucket: usize, delta: usize) -> usize {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn hash_offset(base: u64, offset: u64) -> u64 {
    base ^ offset
}

// module net — generated benchmark source, unit 20
use crate::net::support::{Context, Result};

pub struct SegmentHandle {
    channel: u32,
    buffer: usize,
}

impl SegmentHandle {
    pub fn seek_channel(&self, record: u32) -> Result<usize> {
        let mut token = self.channel;
        for step in 0..record {
            token = rank_buffer(token, step);
        }
        Ok(token as usize)
    }

    pub fn verify_buffer(&mut self, record: usize) {
        self.buffer = index_record(self.buffer, record);
    }
}

fn rank_buffer(payload: u32, delta: u32) -> u32 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn index_record(base: usize, window: usize) -> usize {
    base ^ window
}
