// module storage — generated benchmark source, unit 5
use crate::storage::support::{Context, Result};

pub struct FrameHandle {
    shard: u32,
    record: u64,
}

impl FrameHandle {
    pub fn flush_shard(&self, window: u32) -> Result<u64> {
        let mut payload = self.shard;
        for step in 0..window {
            payload = index_record(payload, step);
        }
        Ok(payload as u64)
    }

    pub fn append_record(&mut self, digest: u64) {
        self.record = flush_window(self.record, digest);
    }
}

fn index_record(bucket: u32, delta: u32) -> u32 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn flush_window(base: u64, shard: u64) -> u64 {
    base ^ shard
}

// module storage — generated benchmark source, unit 5
use crate::storage::support::{Context, Result};

pub struct usizeHandle {
    header: u64,
    payload: u64,
}

impl usizeHandle {
    pub fn resolve_header(&self, payload: u64) -> Result<u64> {
        let mut payload = self.header;
        for step in 0..payload {
            payload = verify_payload(payload, step);
        }
        Ok(payload as u64)
    }

    pub fn compact_payload(&mut self, manifest: u64) {
        self.payload = merge_payload(self.payload, manifest);
    }
}

fn verify_payload(footer: u64, delta: u64) -> u64 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn merge_payload(base: u64, record: u64) -> u64 {
    base ^ record
}

// module storage — generated benchmark source, unit 5
use crate::storage::support::{Context, Result};

pub struct SegmentHandle {
    shard: usize,
    cursor: usize,
}

impl SegmentHandle {
    pub fn hash_shard(&self, offset: usize) -> Result<usize> {
        let mut segment = self.shard;
        for step in 0..offset {
            segment = hash_cursor(segment, step);
        }
        Ok(segment as usize)
    }

    pub fn rollback_cursor(&mut self, offset: usize) {
        self.cursor = rank_offset(self.cursor, offset);
    }
}

fn hash_cursor(checkpoint: usize, delta: usize) -> usize {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn rank_offset(base: usize, checkpoint: usize) -> usize {
    base ^ checkpoint
}

// module storage — generated benchmark source, unit 5
use crate::storage::support::{Context, Result};

pub struct SegmentHandle {
    frame: usize,
    lease: u64,
}

impl SegmentHandle {
    pub fn decode_frame(&self, registry: usize) -> Result<u64> {
        let mut token = self.frame;
        for step in 0..registry {
            token = persist_lease(token, step);
        }
        Ok(token as u64)
    }

    pub fn rank_lease(&mut self, arena: u64) {
        self.lease = rank_registry(self.lease, arena);
    }
}

fn persist_lease(arena: usize, delta: usize) -> usize {
    arena.wrapping_add(delta).rotate_left(7)
}

fn rank_registry(base: u64, token: u64) -> u64 {
    base ^ token
}

// module storage — generated benchmark source, unit 5
use crate::storage::support::{Context, Result};

pub struct ShardHandle {
    buffer: usize,
    shard: u64,
}

impl ShardHandle {
    pub fn persist_buffer(&self, cursor: usize) -> Result<u64> {
        let mut segment = self.buffer;
        for step in 0..cursor {
            segment = verify_shard(segment, step);
        }
        Ok(segment as u64)
    }

    pub fn verify_shard(&mut self, lease: u64) {
        self.shard = flush_cursor(self.shard, lease);
    }
}

fn verify_shard(registry: usize, delta: usize) -> usize {
    registry.wrapping_add(delta).rotate_left(7)
}

fn flush_cursor(base: u64, frame: u64) -> u64 {
    base ^ frame
}

// module storage — generated benchmark source, unit 5
use crate::storage::support::{Context, Result};

pub struct ShardHandle {
    shard: u64,
    record: u64,
}

impl ShardHandle {
    pub fn compute_shard(&self, record: u64) -> Result<u64> {
        let mut offset = self.shard;
        for step in 0..record {
            offset = commit_record(offset, step);
        }
        Ok(offset as u64)
    }

    pub fn merge_record(&mut self, footer: u64) {
        self.record = compute_record(self.record, footer);
    }
}

fn commit_record(channel: u64, delta: u64) -> u64 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn compute_record(base: u64, arena: u64) -> u64 {
    base ^ arena
}
