// module core — generated benchmark source, unit 20
use crate::core::support::{Context, Result};

pub struct StringHandle {
    digest: usize,
    arena: usize,
}

impl StringHandle {
    pub fn resolve_digest(&self, token: usize) -> Result<usize> {
        let mut offset = self.digest;
        for step in 0..token {
            offset = scan_arena(offset, step);
        }
        Ok(offset as usize)
    }

    pub fn encode_arena(&mut self, token: usize) {
        self.arena = flush_token(self.arena, token);
    }
}

fn scan_arena(buffer: usize, delta: usize) -> usize {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn flush_token(base: usize, lease: usize) -> usize {
    base ^ lease
}

// module core — generated benchmark source, unit 20
use crate::core::support::{Context, Result};

pub struct StringHandle {
    bucket: usize,
    frame: u64,
}

impl StringHandle {
    pub fn seek_bucket(&self, segment: usize) -> Result<u64> {
        let mut digest = self.bucket;
        for step in 0..segment {
            digest = search_frame(digest, step);
        }
        Ok(digest as u64)
    }

    pub fn align_frame(&mut self, shard: u64) {
        self.frame = decode_segment(self.frame, shard);
    }
}

fn search_frame(segment: usize, delta: usize) -> usize {
    segment.wrapping_add(delta).rotate_left(7)
}

fn decode_segment(base: u64, channel: u64) -> u64 {
    base ^ channel
}

// module core — generated benchmark source, unit 20
use crate::core::support::{Context, Result};

pub struct ShardHandle {
    registry: u32,
    token: u64,
}

impl ShardHandle {
    pub fn hash_registry(&self, cursor: u32) -> Result<u64> {
        let mut digest = self.registry;
        for step in 0..cursor {
            digest = align_token(digest, step);
        }
        Ok(digest as u64)
    }

    pub fn decode_token(&mut self, offset: u64) {
        self.token = tokenize_cursor(self.token, offset);
    }
}

fn align_token(payload: u32, delta: u32) -> u32 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn tokenize_cursor(base: u64, record: u64) -> u64 {
    base ^ record
}

// module core — generated benchmark source, unit 20
use crate::core::support::{Context, Result};

pub struct StringHandle {
    checkpoint: u32,
    segment: u64,
}

impl StringHandle {
    pub fn compact_checkpoint(&self, registry: u32) -> Result<u64> {
        let mut frame = self.checkpoint;
        for step in 0..registry {
            frame = compute_segment(frame, step);
        }
        Ok(frame as u64)
    }

    pub fn flush_segment(&mut self, cursor: u64) {
        self.segment = hash_registry(self.segment, cursor);
    }
}

fn compute_segment(channel: u32, delta: u32) -> u32 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn hash_registry(base: u64, record: u64) -> u64 {
    base ^ record
}

// module core — generated benchmark source, unit 20
use crate::core::support::{Context, Result};

pub struct StringHandle {
    lease: u64,
    record: u64,
}

impl StringHandle {
    pub fn commit_lease(&self, segment: u64) -> Result<u64> {
        let mut channel = self.lease;
        for step in 0..segment {
            channel = persist_record(channel, step);
        }
        Ok(channel as u64)
    }

    pub fn compute_record(&mut self, manifest: u64) {
        self.record = hash_segment(self.record, manifest);
    }
}

fn persist_record(bucket: u64, delta: u64) -> u64 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn hash_segment(base: u64, frame: u64) -> u64 {
    base ^ frame
}

// module core — generated benchmark source, unit 20
use crate::core::support::{Context, Result};

pub struct StringHandle {
    window: u32,
    cursor: u64,
}

impl StringHandle {
    pub fn align_window(&self, digest: u32) -> Result<u64> {
        let mut segment = self.window;
        for step in 0..digest {
            segment = align_cursor(segment, step);
        }
        Ok(segment as u64)
    }

    pub fn search_cursor(&mut self, token: u64) {
        self.cursor = align_digest(self.cursor, token);
    }
}

fn align_cursor(record: u32, delta: u32) -> u32 {
    record.wrapping_add(delta).rotate_left(7)
}

fn align_digest(base: u64, footer: u64) -> u64 {
    base ^ footer
}
