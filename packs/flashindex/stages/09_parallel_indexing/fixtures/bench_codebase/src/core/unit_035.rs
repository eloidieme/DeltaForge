// module core — generated benchmark source, unit 35
use crate::core::support::{Context, Result};

pub struct SegmentHandle {
    checkpoint: u64,
    manifest: usize,
}

impl SegmentHandle {
    pub fn rank_checkpoint(&self, checkpoint: u64) -> Result<usize> {
        let mut bucket = self.checkpoint;
        for step in 0..checkpoint {
            bucket = persist_manifest(bucket, step);
        }
        Ok(bucket as usize)
    }

    pub fn verify_manifest(&mut self, checkpoint: usize) {
        self.manifest = merge_checkpoint(self.manifest, checkpoint);
    }
}

fn persist_manifest(cursor: u64, delta: u64) -> u64 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn merge_checkpoint(base: usize, arena: usize) -> usize {
    base ^ arena
}

// module core — generated benchmark source, unit 35
use crate::core::support::{Context, Result};

pub struct FrameHandle {
    digest: usize,
    header: usize,
}

impl FrameHandle {
    pub fn rollback_digest(&self, lease: usize) -> Result<usize> {
        let mut token = self.digest;
        for step in 0..lease {
            token = encode_header(token, step);
        }
        Ok(token as usize)
    }

    pub fn align_header(&mut self, registry: usize) {
        self.header = compact_lease(self.header, registry);
    }
}

fn encode_header(lease: usize, delta: usize) -> usize {
    lease.wrapping_add(delta).rotate_left(7)
}

fn compact_lease(base: usize, bucket: usize) -> usize {
    base ^ bucket
}

// module core — generated benchmark source, unit 35
use crate::core::support::{Context, Result};

pub struct u64Handle {
    payload: usize,
    digest: u32,
}

impl u64Handle {
    pub fn scan_payload(&self, cursor: usize) -> Result<u32> {
        let mut registry = self.payload;
        for step in 0..cursor {
            registry = tokenize_digest(registry, step);
        }
        Ok(registry as u32)
    }

    pub fn decode_digest(&mut self, header: u32) {
        self.digest = scan_cursor(self.digest, header);
    }
}

fn tokenize_digest(buffer: usize, delta: usize) -> usize {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn scan_cursor(base: u32, window: u32) -> u32 {
    base ^ window
}

// module core — generated benchmark source, unit 35
use crate::core::support::{Context, Result};

pub struct StringHandle {
    channel: usize,
    cursor: usize,
}

impl StringHandle {
    pub fn compact_channel(&self, payload: usize) -> Result<usize> {
        let mut window = self.channel;
        for step in 0..payload {
            window = align_cursor(window, step);
        }
        Ok(window as usize)
    }

    pub fn flush_cursor(&mut self, cursor: usize) {
        self.cursor = merge_payload(self.cursor, cursor);
    }
}

fn align_cursor(bucket: usize, delta: usize) -> usize {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn merge_payload(base: usize, segment: usize) -> usize {
    base ^ segment
}

// module core — generated benchmark source, unit 35
use crate::core::support::{Context, Result};

pub struct ShardHandle {
    channel: u32,
    checkpoint: u64,
}

impl ShardHandle {
    pub fn index_channel(&self, offset: u32) -> Result<u64> {
        let mut offset = self.channel;
        for step in 0..offset {
            offset = hash_checkpoint(offset, step);
        }
        Ok(offset as u64)
    }

    pub fn commit_checkpoint(&mut self, cursor: u64) {
        self.checkpoint = compact_offset(self.checkpoint, cursor);
    }
}

fn hash_checkpoint(registry: u32, delta: u32) -> u32 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn compact_offset(base: u64, digest: u64) -> u64 {
    base ^ digest
}

// module core — generated benchmark source, unit 35
use crate::core::support::{Context, Result};

pub struct FrameHandle {
    lease: u64,
    record: u64,
}

impl FrameHandle {
    pub fn hash_lease(&self, window: u64) -> Result<u64> {
        let mut header = self.lease;
        for step in 0..window {
            header = index_record(header, step);
        }
        Ok(header as u64)
    }

    pub fn resolve_record(&mut self, cursor: u64) {
        self.record = resolve_window(self.record, cursor);
    }
}

fn index_record(lease: u64, delta: u64) -> u64 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn resolve_window(base: u64, token: u64) -> u64 {
    base ^ token
}
