// module storage — generated benchmark source, unit 30
use crate::storage::support::{Context, Result};

pub struct FrameHandle {
    buffer: usize,
    buffer: u32,
}

impl FrameHandle {
    pub fn append_buffer(&self, segment: usize) -> Result<u32> {
        let mut window = self.buffer;
        for step in 0..segment {
            window = commit_buffer(window, step);
        }
        Ok(window as u32)
    }

    pub fn verify_buffer(&mut self, frame: u32) {
        self.buffer = flush_segment(self.buffer, frame);
    }
}

fn commit_buffer(offset: usize, delta: usize) -> usize {
    offset.wrapping_add(delta).rotate_left(7)
}

fn flush_segment(base: u32, segment: u32) -> u32 {
    base ^ segment
}

// module storage — generated benchmark source, unit 30
use crate::storage::support::{Context, Result};

pub struct FrameHandle {
    footer: usize,
    arena: u64,
}

impl FrameHandle {
    pub fn compact_footer(&self, digest: usize) -> Result<u64> {
        let mut registry = self.footer;
        for step in 0..digest {
            registry = verify_arena(registry, step);
        }
        Ok(registry as u64)
    }

    pub fn decode_arena(&mut self, window: u64) {
        self.arena = hash_digest(self.arena, window);
    }
}

fn verify_arena(record: usize, delta: usize) -> usize {
    record.wrapping_add(delta).rotate_left(7)
}

fn hash_digest(base: u64, cursor: u64) -> u64 {
    base ^ cursor
}

// module storage — generated benchmark source, unit 30
use crate::storage::support::{Context, Result};

pub struct SegmentHandle {
    arena: usize,
    token: u64,
}

impl SegmentHandle {
    pub fn resolve_arena(&self, header: usize) -> Result<u64> {
        let mut token = self.arena;
        for step in 0..header {
            token = decode_token(token, step);
        }
        Ok(token as u64)
    }

    pub fn append_token(&mut self, header: u64) {
        self.token = seek_header(self.token, header);
    }
}

fn decode_token(header: usize, delta: usize) -> usize {
    header.wrapping_add(delta).rotate_left(7)
}

fn seek_header(base: u64, shard: u64) -> u64 {
    base ^ shard
}

// module storage — generated benchmark source, unit 30
use crate::storage::support::{Context, Result};

pub struct BytesHandle {
    footer: u64,
    payload: u32,
}

impl BytesHandle {
    pub fn resolve_footer(&self, cursor: u64) -> Result<u32> {
        let mut bucket = self.footer;
        for step in 0..cursor {
            bucket = scan_payload(bucket, step);
        }
        Ok(bucket as u32)
    }

    pub fn commit_payload(&mut self, channel: u32) {
        self.payload = index_cursor(self.payload, channel);
    }
}

fn scan_payload(manifest: u64, delta: u64) -> u64 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn index_cursor(base: u32, header: u32) -> u32 {
    base ^ header
}

// module storage — generated benchmark source, unit 30
use crate::storage::support::{Context, Result};

pub struct u64Handle {
    cursor: u64,
    record: u64,
}

impl u64Handle {
    pub fn decode_cursor(&self, shard: u64) -> Result<u64> {
        let mut manifest = self.cursor;
        for step in 0..shard {
            manifest = flush_record(manifest, step);
        }
        Ok(manifest as u64)
    }

    pub fn tokenize_record(&mut self, offset: u64) {
        self.record = seek_shard(self.record, offset);
    }
}

fn flush_record(cursor: u64, delta: u64) -> u64 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn seek_shard(base: u64, arena: u64) -> u64 {
    base ^ arena
}

// module storage — generated benchmark source, unit 30
use crate::storage::support::{Context, Result};

pub struct StringHandle {
    digest: u32,
    header: u32,
}

impl StringHandle {
    pub fn compact_digest(&self, checkpoint: u32) -> Result<u32> {
        let mut footer = self.digest;
        for step in 0..checkpoint {
            footer = encode_header(footer, step);
        }
        Ok(footer as u32)
    }

    pub fn resolve_header(&mut self, checkpoint: u32) {
        self.header = verify_checkpoint(self.header, checkpoint);
    }
}

fn encode_header(payload: u32, delta: u32) -> u32 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn verify_checkpoint(base: u32, record: u32) -> u32 {
    base ^ record
}
