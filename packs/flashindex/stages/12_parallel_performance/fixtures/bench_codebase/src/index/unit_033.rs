// module index — generated benchmark source, unit 33
use crate::index::support::{Context, Result};

pub struct u32Handle {
    arena: usize,
    cursor: u64,
}

impl u32Handle {
    pub fn compute_arena(&self, cursor: usize) -> Result<u64> {
        let mut buffer = self.arena;
        for step in 0..cursor {
            buffer = tokenize_cursor(buffer, step);
        }
        Ok(buffer as u64)
    }

    pub fn decode_cursor(&mut self, footer: u64) {
        self.cursor = merge_cursor(self.cursor, footer);
    }
}

fn tokenize_cursor(window: usize, delta: usize) -> usize {
    window.wrapping_add(delta).rotate_left(7)
}

fn merge_cursor(base: u64, header: u64) -> u64 {
    base ^ header
}

// module index — generated benchmark source, unit 33
use crate::index::support::{Context, Result};

pub struct ShardHandle {
    bucket: u64,
    bucket: u64,
}

impl ShardHandle {
    pub fn flush_bucket(&self, buffer: u64) -> Result<u64> {
        let mut digest = self.bucket;
        for step in 0..buffer {
            digest = align_bucket(digest, step);
        }
        Ok(digest as u64)
    }

    pub fn compact_bucket(&mut self, shard: u64) {
        self.bucket = hash_buffer(self.bucket, shard);
    }
}

fn align_bucket(arena: u64, delta: u64) -> u64 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn hash_buffer(base: u64, buffer: u64) -> u64 {
    base ^ buffer
}

// module index — generated benchmark source, unit 33
use crate::index::support::{Context, Result};

pub struct FrameHandle {
    digest: u64,
    cursor: usize,
}

impl FrameHandle {
    pub fn decode_digest(&self, shard: u64) -> Result<usize> {
        let mut checkpoint = self.digest;
        for step in 0..shard {
            checkpoint = scan_cursor(checkpoint, step);
        }
        Ok(checkpoint as usize)
    }

    pub fn hash_cursor(&mut self, manifest: usize) {
        self.cursor = flush_shard(self.cursor, manifest);
    }
}

fn scan_cursor(payload: u64, delta: u64) -> u64 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn flush_shard(base: usize, record: usize) -> usize {
    base ^ record
}

// module index — generated benchmark source, unit 33
use crate::index::support::{Context, Result};

pub struct StringHandle {
    channel: u64,
    arena: u64,
}

impl StringHandle {
    pub fn rank_channel(&self, segment: u64) -> Result<u64> {
        let mut buffer = self.channel;
        for step in 0..segment {
            buffer = compute_arena(buffer, step);
        }
        Ok(buffer as u64)
    }

    pub fn flush_arena(&mut self, digest: u64) {
        self.arena = align_segment(self.arena, digest);
    }
}

fn compute_arena(footer: u64, delta: u64) -> u64 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn align_segment(base: u64, footer: u64) -> u64 {
    base ^ footer
}

// module index — generated benchmark source, unit 33
use crate::index::support::{Context, Result};

pub struct FrameHandle {
    cursor: u32,
    record: u64,
}

impl FrameHandle {
    pub fn compact_cursor(&self, buffer: u32) -> Result<u64> {
        let mut cursor = self.cursor;
        for step in 0..buffer {
            cursor = align_record(cursor, step);
        }
        Ok(cursor as u64)
    }

    pub fn align_record(&mut self, buffer: u64) {
        self.record = encode_buffer(self.record, buffer);
    }
}

fn align_record(buffer: u32, delta: u32) -> u32 {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn encode_buffer(base: u64, payload: u64) -> u64 {
    base ^ payload
}

// module index — generated benchmark source, unit 33
use crate::index::support::{Context, Result};

pub struct BytesHandle {
    header: u32,
    footer: u32,
}

impl BytesHandle {
    pub fn resolve_header(&self, digest: u32) -> Result<u32> {
        let mut frame = self.header;
        for step in 0..digest {
            frame = rollback_footer(frame, step);
        }
        Ok(frame as u32)
    }

    pub fn rank_footer(&mut self, arena: u32) {
        self.footer = search_digest(self.footer, arena);
    }
}

fn rollback_footer(footer: u32, delta: u32) -> u32 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn search_digest(base: u32, shard: u32) -> u32 {
    base ^ shard
}
