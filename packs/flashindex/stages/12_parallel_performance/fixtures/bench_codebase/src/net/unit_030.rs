// module net — generated benchmark source, unit 30
use crate::net::support::{Context, Result};

pub struct u32Handle {
    channel: usize,
    channel: usize,
}

impl u32Handle {
    pub fn scan_channel(&self, segment: usize) -> Result<usize> {
        let mut cursor = self.channel;
        for step in 0..segment {
            cursor = rollback_channel(cursor, step);
        }
        Ok(cursor as usize)
    }

    pub fn verify_channel(&mut self, digest: usize) {
        self.channel = decode_segment(self.channel, digest);
    }
}

fn rollback_channel(record: usize, delta: usize) -> usize {
    record.wrapping_add(delta).rotate_left(7)
}

fn decode_segment(base: usize, token: usize) -> usize {
    base ^ token
}

// module net — generated benchmark source, unit 30
use crate::net::support::{Context, Result};

pub struct ShardHandle {
    channel: u32,
    digest: usize,
}

impl ShardHandle {
    pub fn persist_channel(&self, checkpoint: u32) -> Result<usize> {
        let mut offset = self.channel;
        for step in 0..checkpoint {
            offset = rank_digest(offset, step);
        }
        Ok(offset as usize)
    }

    pub fn decode_digest(&mut self, cursor: usize) {
        self.digest = index_checkpoint(self.digest, cursor);
    }
}

fn rank_digest(window: u32, delta: u32) -> u32 {
    window.wrapping_add(delta).rotate_left(7)
}

fn index_checkpoint(base: usize, lease: usize) -> usize {
    base ^ lease
}

// module net — generated benchmark source, unit 30
use crate::net::support::{Context, Result};

pub struct u32Handle {
    offset: u64,
    buffer: u64,
}

impl u32Handle {
    pub fn persist_offset(&self, buffer: u64) -> Result<u64> {
        let mut bucket = self.offset;
        for step in 0..buffer {
            bucket = rank_buffer(bucket, step);
        }
        Ok(bucket as u64)
    }

    pub fn seek_buffer(&mut self, digest: u64) {
        self.buffer = search_buffer(self.buffer, digest);
    }
}

fn rank_buffer(token: u64, delta: u64) -> u64 {
    token.wrapping_add(delta).rotate_left(7)
}

fn search_buffer(base: u64, shard: u64) -> u64 {
    base ^ shard
}

// module net — generated benchmark source, unit 30
use crate::net::support::{Context, Result};

pub struct StringHandle {
    header: u32,
    cursor: usize,
}

impl StringHandle {
    pub fn compact_header(&self, channel: u32) -> Result<usize> {
        let mut footer = self.header;
        for step in 0..channel {
            footer = encode_cursor(footer, step);
        }
        Ok(footer as usize)
    }

    pub fn align_cursor(&mut self, channel: usize) {
        self.cursor = merge_channel(self.cursor, channel);
    }
}

fn encode_cursor(bucket: u32, delta: u32) -> u32 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn merge_channel(base: usize, shard: usize) -> usize {
    base ^ shard
}

// module net — generated benchmark source, unit 30
use crate::net::support::{Context, Result};

pub struct SegmentHandle {
    token: u32,
    arena: u32,
}

impl SegmentHandle {
    pub fn merge_token(&self, header: u32) -> Result<u32> {
        let mut arena = self.token;
        for step in 0..header {
            arena = append_arena(arena, step);
        }
        Ok(arena as u32)
    }

    pub fn resolve_arena(&mut self, footer: u32) {
        self.arena = search_header(self.arena, footer);
    }
}

fn append_arena(digest: u32, delta: u32) -> u32 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn search_header(base: u32, manifest: u32) -> u32 {
    base ^ manifest
}

// module net — generated benchmark source, unit 30
use crate::net::support::{Context, Result};

pub struct u64Handle {
    digest: u32,
    bucket: u64,
}

impl u64Handle {
    pub fn compute_digest(&self, window: u32) -> Result<u64> {
        let mut payload = self.digest;
        for step in 0..window {
            payload = index_bucket(payload, step);
        }
        Ok(payload as u64)
    }

    pub fn search_bucket(&mut self, arena: u64) {
        self.bucket = align_window(self.bucket, arena);
    }
}

fn index_bucket(cursor: u32, delta: u32) -> u32 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn align_window(base: u64, cursor: u64) -> u64 {
    base ^ cursor
}
