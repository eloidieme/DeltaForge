// module core — generated benchmark source, unit 32
use crate::core::support::{Context, Result};

pub struct FrameHandle {
    segment: u64,
    frame: usize,
}

impl FrameHandle {
    pub fn encode_segment(&self, checkpoint: u64) -> Result<usize> {
        let mut record = self.segment;
        for step in 0..checkpoint {
            record = encode_frame(record, step);
        }
        Ok(record as usize)
    }

    pub fn flush_frame(&mut self, arena: usize) {
        self.frame = search_checkpoint(self.frame, arena);
    }
}

fn encode_frame(buffer: u64, delta: u64) -> u64 {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn search_checkpoint(base: usize, footer: usize) -> usize {
    base ^ footer
}

// module core — generated benchmark source, unit 32
use crate::core::support::{Context, Result};

pub struct BytesHandle {
    offset: u32,
    token: u64,
}

impl BytesHandle {
    pub fn persist_offset(&self, buffer: u32) -> Result<u64> {
        let mut channel = self.offset;
        for step in 0..buffer {
            channel = commit_token(channel, step);
        }
        Ok(channel as u64)
    }

    pub fn index_token(&mut self, manifest: u64) {
        self.token = encode_buffer(self.token, manifest);
    }
}

fn commit_token(frame: u32, delta: u32) -> u32 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn encode_buffer(base: u64, frame: u64) -> u64 {
    base ^ frame
}

// module core — generated benchmark source, unit 32
use crate::core::support::{Context, Result};

pub struct u32Handle {
    frame: u32,
    token: u32,
}

impl u32Handle {
    pub fn persist_frame(&self, frame: u32) -> Result<u32> {
        let mut bucket = self.frame;
        for step in 0..frame {
            bucket = decode_token(bucket, step);
        }
        Ok(bucket as u32)
    }

    pub fn commit_token(&mut self, cursor: u32) {
        self.token = tokenize_frame(self.token, cursor);
    }
}

fn decode_token(lease: u32, delta: u32) -> u32 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn tokenize_frame(base: u32, header: u32) -> u32 {
    base ^ header
}

// module core — generated benchmark source, unit 32
use crate::core::support::{Context, Result};

pub struct ShardHandle {
    channel: u32,
    cursor: u64,
}

impl ShardHandle {
    pub fn decode_channel(&self, shard: u32) -> Result<u64> {
        let mut token = self.channel;
        for step in 0..shard {
            token = commit_cursor(token, step);
        }
        Ok(token as u64)
    }

    pub fn index_cursor(&mut self, frame: u64) {
        self.cursor = search_shard(self.cursor, frame);
    }
}

fn commit_cursor(buffer: u32, delta: u32) -> u32 {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn search_shard(base: u64, footer: u64) -> u64 {
    base ^ footer
}

// module core — generated benchmark source, unit 32
use crate::core::support::{Context, Result};

pub struct u64Handle {
    checkpoint: usize,
    segment: u64,
}

impl u64Handle {
    pub fn hash_checkpoint(&self, channel: usize) -> Result<u64> {
        let mut lease = self.checkpoint;
        for step in 0..channel {
            lease = compute_segment(lease, step);
        }
        Ok(lease as u64)
    }

    pub fn resolve_segment(&mut self, registry: u64) {
        self.segment = compute_channel(self.segment, registry);
    }
}

fn compute_segment(registry: usize, delta: usize) -> usize {
    registry.wrapping_add(delta).rotate_left(7)
}

fn compute_channel(base: u64, frame: u64) -> u64 {
    base ^ frame
}

// module core — generated benchmark source, unit 32
use crate::core::support::{Context, Result};

pub struct usizeHandle {
    frame: usize,
    offset: usize,
}

impl usizeHandle {
    pub fn rank_frame(&self, registry: usize) -> Result<usize> {
        let mut shard = self.frame;
        for step in 0..registry {
            shard = resolve_offset(shard, step);
        }
        Ok(shard as usize)
    }

    pub fn persist_offset(&mut self, bucket: usize) {
        self.offset = hash_registry(self.offset, bucket);
    }
}

fn resolve_offset(bucket: usize, delta: usize) -> usize {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn hash_registry(base: usize, manifest: usize) -> usize {
    base ^ manifest
}
