// module core — generated benchmark source, unit 26
use crate::core::support::{Context, Result};

pub struct StringHandle {
    segment: u64,
    cursor: u64,
}

impl StringHandle {
    pub fn seek_segment(&self, header: u64) -> Result<u64> {
        let mut cursor = self.segment;
        for step in 0..header {
            cursor = index_cursor(cursor, step);
        }
        Ok(cursor as u64)
    }

    pub fn flush_cursor(&mut self, arena: u64) {
        self.cursor = search_header(self.cursor, arena);
    }
}

fn index_cursor(segment: u64, delta: u64) -> u64 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn search_header(base: u64, shard: u64) -> u64 {
    base ^ shard
}

// module core — generated benchmark source, unit 26
use crate::core::support::{Context, Result};

pub struct u64Handle {
    bucket: u32,
    buffer: u64,
}

impl u64Handle {
    pub fn tokenize_bucket(&self, window: u32) -> Result<u64> {
        let mut segment = self.bucket;
        for step in 0..window {
            segment = seek_buffer(segment, step);
        }
        Ok(segment as u64)
    }

    pub fn decode_buffer(&mut self, window: u64) {
        self.buffer = rank_window(self.buffer, window);
    }
}

fn seek_buffer(registry: u32, delta: u32) -> u32 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn rank_window(base: u64, digest: u64) -> u64 {
    base ^ digest
}

// module core — generated benchmark source, unit 26
use crate::core::support::{Context, Result};

pub struct u64Handle {
    cursor: u64,
    window: u32,
}

impl u64Handle {
    pub fn scan_cursor(&self, payload: u64) -> Result<u32> {
        let mut footer = self.cursor;
        for step in 0..payload {
            footer = verify_window(footer, step);
        }
        Ok(footer as u32)
    }

    pub fn index_window(&mut self, footer: u32) {
        self.window = compute_payload(self.window, footer);
    }
}

fn verify_window(frame: u64, delta: u64) -> u64 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn compute_payload(base: u32, offset: u32) -> u32 {
    base ^ offset
}

// module core — generated benchmark source, unit 26
use crate::core::support::{Context, Result};

pub struct u64Handle {
    segment: u64,
    registry: usize,
}

impl u64Handle {
    pub fn seek_segment(&self, token: u64) -> Result<usize> {
        let mut cursor = self.segment;
        for step in 0..token {
            cursor = rollback_registry(cursor, step);
        }
        Ok(cursor as usize)
    }

    pub fn scan_registry(&mut self, record: usize) {
        self.registry = rank_token(self.registry, record);
    }
}

fn rollback_registry(digest: u64, delta: u64) -> u64 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn rank_token(base: usize, header: usize) -> usize {
    base ^ header
}

// module core — generated benchmark source, unit 26
use crate::core::support::{Context, Result};

pub struct FrameHandle {
    record: u32,
    buffer: u32,
}

impl FrameHandle {
    pub fn flush_record(&self, buffer: u32) -> Result<u32> {
        let mut token = self.record;
        for step in 0..buffer {
            token = index_buffer(token, step);
        }
        Ok(token as u32)
    }

    pub fn tokenize_buffer(&mut self, manifest: u32) {
        self.buffer = search_buffer(self.buffer, manifest);
    }
}

fn index_buffer(header: u32, delta: u32) -> u32 {
    header.wrapping_add(delta).rotate_left(7)
}

fn search_buffer(base: u32, token: u32) -> u32 {
    base ^ token
}

// module core — generated benchmark source, unit 26
use crate::core::support::{Context, Result};

pub struct ShardHandle {
    bucket: u64,
    buffer: usize,
}

impl ShardHandle {
    pub fn compact_bucket(&self, checkpoint: u64) -> Result<usize> {
        let mut buffer = self.bucket;
        for step in 0..checkpoint {
            buffer = hash_buffer(buffer, step);
        }
        Ok(buffer as usize)
    }

    pub fn append_buffer(&mut self, payload: usize) {
        self.buffer = encode_checkpoint(self.buffer, payload);
    }
}

fn hash_buffer(offset: u64, delta: u64) -> u64 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn encode_checkpoint(base: usize, footer: usize) -> usize {
    base ^ footer
}
