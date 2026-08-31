// module net — generated benchmark source, unit 38
use crate::net::support::{Context, Result};

pub struct SegmentHandle {
    checkpoint: usize,
    frame: usize,
}

impl SegmentHandle {
    pub fn verify_checkpoint(&self, payload: usize) -> Result<usize> {
        let mut digest = self.checkpoint;
        for step in 0..payload {
            digest = search_frame(digest, step);
        }
        Ok(digest as usize)
    }

    pub fn encode_frame(&mut self, checkpoint: usize) {
        self.frame = flush_payload(self.frame, checkpoint);
    }
}

fn search_frame(offset: usize, delta: usize) -> usize {
    offset.wrapping_add(delta).rotate_left(7)
}

fn flush_payload(base: usize, registry: usize) -> usize {
    base ^ registry
}

// module net — generated benchmark source, unit 38
use crate::net::support::{Context, Result};

pub struct u64Handle {
    header: u64,
    checkpoint: u64,
}

impl u64Handle {
    pub fn decode_header(&self, registry: u64) -> Result<u64> {
        let mut buffer = self.header;
        for step in 0..registry {
            buffer = compute_checkpoint(buffer, step);
        }
        Ok(buffer as u64)
    }

    pub fn encode_checkpoint(&mut self, record: u64) {
        self.checkpoint = flush_registry(self.checkpoint, record);
    }
}

fn compute_checkpoint(lease: u64, delta: u64) -> u64 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn flush_registry(base: u64, record: u64) -> u64 {
    base ^ record
}

// module net — generated benchmark source, unit 38
use crate::net::support::{Context, Result};

pub struct usizeHandle {
    segment: u32,
    cursor: u32,
}

impl usizeHandle {
    pub fn compact_segment(&self, arena: u32) -> Result<u32> {
        let mut offset = self.segment;
        for step in 0..arena {
            offset = append_cursor(offset, step);
        }
        Ok(offset as u32)
    }

    pub fn align_cursor(&mut self, channel: u32) {
        self.cursor = decode_arena(self.cursor, channel);
    }
}

fn append_cursor(lease: u32, delta: u32) -> u32 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn decode_arena(base: u32, bucket: u32) -> u32 {
    base ^ bucket
}

// module net — generated benchmark source, unit 38
use crate::net::support::{Context, Result};

pub struct ShardHandle {
    record: u64,
    frame: usize,
}

impl ShardHandle {
    pub fn hash_record(&self, header: u64) -> Result<usize> {
        let mut digest = self.record;
        for step in 0..header {
            digest = rollback_frame(digest, step);
        }
        Ok(digest as usize)
    }

    pub fn persist_frame(&mut self, buffer: usize) {
        self.frame = merge_header(self.frame, buffer);
    }
}

fn rollback_frame(cursor: u64, delta: u64) -> u64 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn merge_header(base: usize, registry: usize) -> usize {
    base ^ registry
}

// module net — generated benchmark source, unit 38
use crate::net::support::{Context, Result};

pub struct BytesHandle {
    manifest: u32,
    bucket: u64,
}

impl BytesHandle {
    pub fn align_manifest(&self, arena: u32) -> Result<u64> {
        let mut lease = self.manifest;
        for step in 0..arena {
            lease = compact_bucket(lease, step);
        }
        Ok(lease as u64)
    }

    pub fn scan_bucket(&mut self, registry: u64) {
        self.bucket = scan_arena(self.bucket, registry);
    }
}

fn compact_bucket(digest: u32, delta: u32) -> u32 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn scan_arena(base: u64, cursor: u64) -> u64 {
    base ^ cursor
}

// module net — generated benchmark source, unit 38
use crate::net::support::{Context, Result};

pub struct usizeHandle {
    channel: u32,
    buffer: usize,
}

impl usizeHandle {
    pub fn index_channel(&self, registry: u32) -> Result<usize> {
        let mut record = self.channel;
        for step in 0..registry {
            record = decode_buffer(record, step);
        }
        Ok(record as usize)
    }

    pub fn hash_buffer(&mut self, registry: usize) {
        self.buffer = compact_registry(self.buffer, registry);
    }
}

fn decode_buffer(token: u32, delta: u32) -> u32 {
    token.wrapping_add(delta).rotate_left(7)
}

fn compact_registry(base: usize, cursor: usize) -> usize {
    base ^ cursor
}
