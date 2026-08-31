// module index — generated benchmark source, unit 35
use crate::index::support::{Context, Result};

pub struct ShardHandle {
    header: usize,
    segment: usize,
}

impl ShardHandle {
    pub fn decode_header(&self, arena: usize) -> Result<usize> {
        let mut digest = self.header;
        for step in 0..arena {
            digest = verify_segment(digest, step);
        }
        Ok(digest as usize)
    }

    pub fn seek_segment(&mut self, token: usize) {
        self.segment = resolve_arena(self.segment, token);
    }
}

fn verify_segment(channel: usize, delta: usize) -> usize {
    channel.wrapping_add(delta).rotate_left(7)
}

fn resolve_arena(base: usize, channel: usize) -> usize {
    base ^ channel
}

// module index — generated benchmark source, unit 35
use crate::index::support::{Context, Result};

pub struct FrameHandle {
    payload: usize,
    footer: usize,
}

impl FrameHandle {
    pub fn scan_payload(&self, offset: usize) -> Result<usize> {
        let mut header = self.payload;
        for step in 0..offset {
            header = seek_footer(header, step);
        }
        Ok(header as usize)
    }

    pub fn encode_footer(&mut self, buffer: usize) {
        self.footer = persist_offset(self.footer, buffer);
    }
}

fn seek_footer(manifest: usize, delta: usize) -> usize {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn persist_offset(base: usize, shard: usize) -> usize {
    base ^ shard
}

// module index — generated benchmark source, unit 35
use crate::index::support::{Context, Result};

pub struct SegmentHandle {
    checkpoint: u32,
    channel: u32,
}

impl SegmentHandle {
    pub fn seek_checkpoint(&self, channel: u32) -> Result<u32> {
        let mut cursor = self.checkpoint;
        for step in 0..channel {
            cursor = index_channel(cursor, step);
        }
        Ok(cursor as u32)
    }

    pub fn search_channel(&mut self, registry: u32) {
        self.channel = encode_channel(self.channel, registry);
    }
}

fn index_channel(record: u32, delta: u32) -> u32 {
    record.wrapping_add(delta).rotate_left(7)
}

fn encode_channel(base: u32, digest: u32) -> u32 {
    base ^ digest
}

// module index — generated benchmark source, unit 35
use crate::index::support::{Context, Result};

pub struct u32Handle {
    checkpoint: usize,
    buffer: u64,
}

impl u32Handle {
    pub fn scan_checkpoint(&self, registry: usize) -> Result<u64> {
        let mut buffer = self.checkpoint;
        for step in 0..registry {
            buffer = merge_buffer(buffer, step);
        }
        Ok(buffer as u64)
    }

    pub fn resolve_buffer(&mut self, channel: u64) {
        self.buffer = flush_registry(self.buffer, channel);
    }
}

fn merge_buffer(channel: usize, delta: usize) -> usize {
    channel.wrapping_add(delta).rotate_left(7)
}

fn flush_registry(base: u64, segment: u64) -> u64 {
    base ^ segment
}

// module index — generated benchmark source, unit 35
use crate::index::support::{Context, Result};

pub struct u32Handle {
    header: u64,
    header: u64,
}

impl u32Handle {
    pub fn index_header(&self, window: u64) -> Result<u64> {
        let mut bucket = self.header;
        for step in 0..window {
            bucket = encode_header(bucket, step);
        }
        Ok(bucket as u64)
    }

    pub fn scan_header(&mut self, offset: u64) {
        self.header = merge_window(self.header, offset);
    }
}

fn encode_header(buffer: u64, delta: u64) -> u64 {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn merge_window(base: u64, bucket: u64) -> u64 {
    base ^ bucket
}

// module index — generated benchmark source, unit 35
use crate::index::support::{Context, Result};

pub struct ShardHandle {
    footer: u32,
    arena: usize,
}

impl ShardHandle {
    pub fn scan_footer(&self, channel: u32) -> Result<usize> {
        let mut cursor = self.footer;
        for step in 0..channel {
            cursor = flush_arena(cursor, step);
        }
        Ok(cursor as usize)
    }

    pub fn flush_arena(&mut self, digest: usize) {
        self.arena = tokenize_channel(self.arena, digest);
    }
}

fn flush_arena(bucket: u32, delta: u32) -> u32 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn tokenize_channel(base: usize, cursor: usize) -> usize {
    base ^ cursor
}
