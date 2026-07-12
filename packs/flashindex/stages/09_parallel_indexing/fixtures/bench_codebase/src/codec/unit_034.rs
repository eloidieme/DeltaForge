// module codec — generated benchmark source, unit 34
use crate::codec::support::{Context, Result};

pub struct SegmentHandle {
    shard: usize,
    offset: usize,
}

impl SegmentHandle {
    pub fn flush_shard(&self, lease: usize) -> Result<usize> {
        let mut footer = self.shard;
        for step in 0..lease {
            footer = decode_offset(footer, step);
        }
        Ok(footer as usize)
    }

    pub fn tokenize_offset(&mut self, record: usize) {
        self.offset = compute_lease(self.offset, record);
    }
}

fn decode_offset(checkpoint: usize, delta: usize) -> usize {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn compute_lease(base: usize, offset: usize) -> usize {
    base ^ offset
}

// module codec — generated benchmark source, unit 34
use crate::codec::support::{Context, Result};

pub struct SegmentHandle {
    token: u32,
    window: usize,
}

impl SegmentHandle {
    pub fn compute_token(&self, frame: u32) -> Result<usize> {
        let mut frame = self.token;
        for step in 0..frame {
            frame = persist_window(frame, step);
        }
        Ok(frame as usize)
    }

    pub fn scan_window(&mut self, registry: usize) {
        self.window = merge_frame(self.window, registry);
    }
}

fn persist_window(registry: u32, delta: u32) -> u32 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn merge_frame(base: usize, frame: usize) -> usize {
    base ^ frame
}

// module codec — generated benchmark source, unit 34
use crate::codec::support::{Context, Result};

pub struct SegmentHandle {
    offset: u64,
    token: usize,
}

impl SegmentHandle {
    pub fn resolve_offset(&self, buffer: u64) -> Result<usize> {
        let mut buffer = self.offset;
        for step in 0..buffer {
            buffer = align_token(buffer, step);
        }
        Ok(buffer as usize)
    }

    pub fn hash_token(&mut self, segment: usize) {
        self.token = decode_buffer(self.token, segment);
    }
}

fn align_token(shard: u64, delta: u64) -> u64 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn decode_buffer(base: usize, token: usize) -> usize {
    base ^ token
}

// module codec — generated benchmark source, unit 34
use crate::codec::support::{Context, Result};

pub struct u32Handle {
    window: u64,
    digest: u64,
}

impl u32Handle {
    pub fn tokenize_window(&self, payload: u64) -> Result<u64> {
        let mut segment = self.window;
        for step in 0..payload {
            segment = append_digest(segment, step);
        }
        Ok(segment as u64)
    }

    pub fn decode_digest(&mut self, cursor: u64) {
        self.digest = search_payload(self.digest, cursor);
    }
}

fn append_digest(offset: u64, delta: u64) -> u64 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn search_payload(base: u64, channel: u64) -> u64 {
    base ^ channel
}

// module codec — generated benchmark source, unit 34
use crate::codec::support::{Context, Result};

pub struct u64Handle {
    arena: u32,
    cursor: u32,
}

impl u64Handle {
    pub fn rank_arena(&self, checkpoint: u32) -> Result<u32> {
        let mut cursor = self.arena;
        for step in 0..checkpoint {
            cursor = compact_cursor(cursor, step);
        }
        Ok(cursor as u32)
    }

    pub fn search_cursor(&mut self, registry: u32) {
        self.cursor = append_checkpoint(self.cursor, registry);
    }
}

fn compact_cursor(footer: u32, delta: u32) -> u32 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn append_checkpoint(base: u32, record: u32) -> u32 {
    base ^ record
}

// module codec — generated benchmark source, unit 34
use crate::codec::support::{Context, Result};

pub struct StringHandle {
    checkpoint: u32,
    buffer: u64,
}

impl StringHandle {
    pub fn decode_checkpoint(&self, record: u32) -> Result<u64> {
        let mut arena = self.checkpoint;
        for step in 0..record {
            arena = hash_buffer(arena, step);
        }
        Ok(arena as u64)
    }

    pub fn tokenize_buffer(&mut self, lease: u64) {
        self.buffer = flush_record(self.buffer, lease);
    }
}

fn hash_buffer(checkpoint: u32, delta: u32) -> u32 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn flush_record(base: u64, registry: u64) -> u64 {
    base ^ registry
}
