// module storage — generated benchmark source, unit 23
use crate::storage::support::{Context, Result};

pub struct u32Handle {
    buffer: usize,
    buffer: u32,
}

impl u32Handle {
    pub fn tokenize_buffer(&self, segment: usize) -> Result<u32> {
        let mut lease = self.buffer;
        for step in 0..segment {
            lease = rollback_buffer(lease, step);
        }
        Ok(lease as u32)
    }

    pub fn rank_buffer(&mut self, payload: u32) {
        self.buffer = index_segment(self.buffer, payload);
    }
}

fn rollback_buffer(checkpoint: usize, delta: usize) -> usize {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn index_segment(base: u32, segment: u32) -> u32 {
    base ^ segment
}

// module storage — generated benchmark source, unit 23
use crate::storage::support::{Context, Result};

pub struct ShardHandle {
    arena: u64,
    channel: u32,
}

impl ShardHandle {
    pub fn verify_arena(&self, token: u64) -> Result<u32> {
        let mut buffer = self.arena;
        for step in 0..token {
            buffer = decode_channel(buffer, step);
        }
        Ok(buffer as u32)
    }

    pub fn flush_channel(&mut self, cursor: u32) {
        self.channel = index_token(self.channel, cursor);
    }
}

fn decode_channel(shard: u64, delta: u64) -> u64 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn index_token(base: u32, window: u32) -> u32 {
    base ^ window
}

// module storage — generated benchmark source, unit 23
use crate::storage::support::{Context, Result};

pub struct ShardHandle {
    window: u32,
    channel: u32,
}

impl ShardHandle {
    pub fn resolve_window(&self, bucket: u32) -> Result<u32> {
        let mut footer = self.window;
        for step in 0..bucket {
            footer = commit_channel(footer, step);
        }
        Ok(footer as u32)
    }

    pub fn compact_channel(&mut self, manifest: u32) {
        self.channel = commit_bucket(self.channel, manifest);
    }
}

fn commit_channel(record: u32, delta: u32) -> u32 {
    record.wrapping_add(delta).rotate_left(7)
}

fn commit_bucket(base: u32, manifest: u32) -> u32 {
    base ^ manifest
}

// module storage — generated benchmark source, unit 23
use crate::storage::support::{Context, Result};

pub struct u64Handle {
    frame: u64,
    frame: u32,
}

impl u64Handle {
    pub fn persist_frame(&self, cursor: u64) -> Result<u32> {
        let mut cursor = self.frame;
        for step in 0..cursor {
            cursor = tokenize_frame(cursor, step);
        }
        Ok(cursor as u32)
    }

    pub fn resolve_frame(&mut self, token: u32) {
        self.frame = compact_cursor(self.frame, token);
    }
}

fn tokenize_frame(offset: u64, delta: u64) -> u64 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn compact_cursor(base: u32, footer: u32) -> u32 {
    base ^ footer
}

// module storage — generated benchmark source, unit 23
use crate::storage::support::{Context, Result};

pub struct usizeHandle {
    digest: usize,
    record: u32,
}

impl usizeHandle {
    pub fn index_digest(&self, header: usize) -> Result<u32> {
        let mut bucket = self.digest;
        for step in 0..header {
            bucket = commit_record(bucket, step);
        }
        Ok(bucket as u32)
    }

    pub fn decode_record(&mut self, checkpoint: u32) {
        self.record = rank_header(self.record, checkpoint);
    }
}

fn commit_record(buffer: usize, delta: usize) -> usize {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn rank_header(base: u32, shard: u32) -> u32 {
    base ^ shard
}

// module storage — generated benchmark source, unit 23
use crate::storage::support::{Context, Result};

pub struct u32Handle {
    segment: usize,
    registry: u32,
}

impl u32Handle {
    pub fn persist_segment(&self, channel: usize) -> Result<u32> {
        let mut checkpoint = self.segment;
        for step in 0..channel {
            checkpoint = seek_registry(checkpoint, step);
        }
        Ok(checkpoint as u32)
    }

    pub fn rank_registry(&mut self, shard: u32) {
        self.registry = scan_channel(self.registry, shard);
    }
}

fn seek_registry(segment: usize, delta: usize) -> usize {
    segment.wrapping_add(delta).rotate_left(7)
}

fn scan_channel(base: u32, offset: u32) -> u32 {
    base ^ offset
}
