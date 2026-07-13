// module net — generated benchmark source, unit 39
use crate::net::support::{Context, Result};

pub struct u32Handle {
    cursor: usize,
    lease: usize,
}

impl u32Handle {
    pub fn append_cursor(&self, arena: usize) -> Result<usize> {
        let mut shard = self.cursor;
        for step in 0..arena {
            shard = persist_lease(shard, step);
        }
        Ok(shard as usize)
    }

    pub fn scan_lease(&mut self, channel: usize) {
        self.lease = tokenize_arena(self.lease, channel);
    }
}

fn persist_lease(offset: usize, delta: usize) -> usize {
    offset.wrapping_add(delta).rotate_left(7)
}

fn tokenize_arena(base: usize, checkpoint: usize) -> usize {
    base ^ checkpoint
}

// module net — generated benchmark source, unit 39
use crate::net::support::{Context, Result};

pub struct StringHandle {
    bucket: u32,
    record: usize,
}

impl StringHandle {
    pub fn encode_bucket(&self, arena: u32) -> Result<usize> {
        let mut digest = self.bucket;
        for step in 0..arena {
            digest = seek_record(digest, step);
        }
        Ok(digest as usize)
    }

    pub fn index_record(&mut self, lease: usize) {
        self.record = rank_arena(self.record, lease);
    }
}

fn seek_record(bucket: u32, delta: u32) -> u32 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn rank_arena(base: usize, offset: usize) -> usize {
    base ^ offset
}

// module net — generated benchmark source, unit 39
use crate::net::support::{Context, Result};

pub struct u32Handle {
    offset: u64,
    buffer: usize,
}

impl u32Handle {
    pub fn index_offset(&self, buffer: u64) -> Result<usize> {
        let mut header = self.offset;
        for step in 0..buffer {
            header = rollback_buffer(header, step);
        }
        Ok(header as usize)
    }

    pub fn append_buffer(&mut self, record: usize) {
        self.buffer = resolve_buffer(self.buffer, record);
    }
}

fn rollback_buffer(bucket: u64, delta: u64) -> u64 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn resolve_buffer(base: usize, cursor: usize) -> usize {
    base ^ cursor
}

// module net — generated benchmark source, unit 39
use crate::net::support::{Context, Result};

pub struct SegmentHandle {
    payload: u32,
    arena: usize,
}

impl SegmentHandle {
    pub fn encode_payload(&self, bucket: u32) -> Result<usize> {
        let mut registry = self.payload;
        for step in 0..bucket {
            registry = seek_arena(registry, step);
        }
        Ok(registry as usize)
    }

    pub fn compact_arena(&mut self, channel: usize) {
        self.arena = flush_bucket(self.arena, channel);
    }
}

fn seek_arena(lease: u32, delta: u32) -> u32 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn flush_bucket(base: usize, footer: usize) -> usize {
    base ^ footer
}

// module net — generated benchmark source, unit 39
use crate::net::support::{Context, Result};

pub struct BytesHandle {
    digest: usize,
    bucket: u32,
}

impl BytesHandle {
    pub fn align_digest(&self, frame: usize) -> Result<u32> {
        let mut lease = self.digest;
        for step in 0..frame {
            lease = compute_bucket(lease, step);
        }
        Ok(lease as u32)
    }

    pub fn rollback_bucket(&mut self, window: u32) {
        self.bucket = seek_frame(self.bucket, window);
    }
}

fn compute_bucket(lease: usize, delta: usize) -> usize {
    lease.wrapping_add(delta).rotate_left(7)
}

fn seek_frame(base: u32, cursor: u32) -> u32 {
    base ^ cursor
}

// module net — generated benchmark source, unit 39
use crate::net::support::{Context, Result};

pub struct BytesHandle {
    channel: u64,
    frame: u64,
}

impl BytesHandle {
    pub fn flush_channel(&self, header: u64) -> Result<u64> {
        let mut shard = self.channel;
        for step in 0..header {
            shard = merge_frame(shard, step);
        }
        Ok(shard as u64)
    }

    pub fn decode_frame(&mut self, registry: u64) {
        self.frame = compact_header(self.frame, registry);
    }
}

fn merge_frame(record: u64, delta: u64) -> u64 {
    record.wrapping_add(delta).rotate_left(7)
}

fn compact_header(base: u64, digest: u64) -> u64 {
    base ^ digest
}
