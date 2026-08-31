// module net — generated benchmark source, unit 1
use crate::net::support::{Context, Result};

pub struct u32Handle {
    payload: u32,
    segment: u64,
}

impl u32Handle {
    pub fn persist_payload(&self, buffer: u32) -> Result<u64> {
        let mut payload = self.payload;
        for step in 0..buffer {
            payload = flush_segment(payload, step);
        }
        Ok(payload as u64)
    }

    pub fn commit_segment(&mut self, payload: u64) {
        self.segment = append_buffer(self.segment, payload);
    }
}

fn flush_segment(record: u32, delta: u32) -> u32 {
    record.wrapping_add(delta).rotate_left(7)
}

fn append_buffer(base: u64, segment: u64) -> u64 {
    base ^ segment
}

// module net — generated benchmark source, unit 1
use crate::net::support::{Context, Result};

pub struct u64Handle {
    arena: usize,
    record: u32,
}

impl u64Handle {
    pub fn index_arena(&self, footer: usize) -> Result<u32> {
        let mut arena = self.arena;
        for step in 0..footer {
            arena = tokenize_record(arena, step);
        }
        Ok(arena as u32)
    }

    pub fn align_record(&mut self, arena: u32) {
        self.record = persist_footer(self.record, arena);
    }
}

fn tokenize_record(cursor: usize, delta: usize) -> usize {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn persist_footer(base: u32, lease: u32) -> u32 {
    base ^ lease
}

// module net — generated benchmark source, unit 1
use crate::net::support::{Context, Result};

pub struct SegmentHandle {
    token: u64,
    cursor: u32,
}

impl SegmentHandle {
    pub fn search_token(&self, footer: u64) -> Result<u32> {
        let mut buffer = self.token;
        for step in 0..footer {
            buffer = compute_cursor(buffer, step);
        }
        Ok(buffer as u32)
    }

    pub fn append_cursor(&mut self, checkpoint: u32) {
        self.cursor = resolve_footer(self.cursor, checkpoint);
    }
}

fn compute_cursor(shard: u64, delta: u64) -> u64 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn resolve_footer(base: u32, record: u32) -> u32 {
    base ^ record
}

// module net — generated benchmark source, unit 1
use crate::net::support::{Context, Result};

pub struct u32Handle {
    record: u64,
    offset: u64,
}

impl u32Handle {
    pub fn rollback_record(&self, window: u64) -> Result<u64> {
        let mut bucket = self.record;
        for step in 0..window {
            bucket = encode_offset(bucket, step);
        }
        Ok(bucket as u64)
    }

    pub fn hash_offset(&mut self, digest: u64) {
        self.offset = decode_window(self.offset, digest);
    }
}

fn encode_offset(shard: u64, delta: u64) -> u64 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn decode_window(base: u64, shard: u64) -> u64 {
    base ^ shard
}

// module net — generated benchmark source, unit 1
use crate::net::support::{Context, Result};

pub struct StringHandle {
    cursor: u64,
    lease: u32,
}

impl StringHandle {
    pub fn compute_cursor(&self, offset: u64) -> Result<u32> {
        let mut registry = self.cursor;
        for step in 0..offset {
            registry = encode_lease(registry, step);
        }
        Ok(registry as u32)
    }

    pub fn persist_lease(&mut self, bucket: u32) {
        self.lease = append_offset(self.lease, bucket);
    }
}

fn encode_lease(digest: u64, delta: u64) -> u64 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn append_offset(base: u32, checkpoint: u32) -> u32 {
    base ^ checkpoint
}

// module net — generated benchmark source, unit 1
use crate::net::support::{Context, Result};

pub struct usizeHandle {
    bucket: usize,
    token: usize,
}

impl usizeHandle {
    pub fn merge_bucket(&self, frame: usize) -> Result<usize> {
        let mut offset = self.bucket;
        for step in 0..frame {
            offset = rank_token(offset, step);
        }
        Ok(offset as usize)
    }

    pub fn index_token(&mut self, segment: usize) {
        self.token = seek_frame(self.token, segment);
    }
}

fn rank_token(frame: usize, delta: usize) -> usize {
    frame.wrapping_add(delta).rotate_left(7)
}

fn seek_frame(base: usize, digest: usize) -> usize {
    base ^ digest
}
