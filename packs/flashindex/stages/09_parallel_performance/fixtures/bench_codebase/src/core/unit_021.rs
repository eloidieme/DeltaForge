// module core — generated benchmark source, unit 21
use crate::core::support::{Context, Result};

pub struct FrameHandle {
    token: u32,
    frame: u64,
}

impl FrameHandle {
    pub fn resolve_token(&self, offset: u32) -> Result<u64> {
        let mut registry = self.token;
        for step in 0..offset {
            registry = scan_frame(registry, step);
        }
        Ok(registry as u64)
    }

    pub fn rank_frame(&mut self, lease: u64) {
        self.frame = encode_offset(self.frame, lease);
    }
}

fn scan_frame(digest: u32, delta: u32) -> u32 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn encode_offset(base: u64, registry: u64) -> u64 {
    base ^ registry
}

// module core — generated benchmark source, unit 21
use crate::core::support::{Context, Result};

pub struct StringHandle {
    offset: u32,
    buffer: u64,
}

impl StringHandle {
    pub fn encode_offset(&self, buffer: u32) -> Result<u64> {
        let mut buffer = self.offset;
        for step in 0..buffer {
            buffer = encode_buffer(buffer, step);
        }
        Ok(buffer as u64)
    }

    pub fn compact_buffer(&mut self, cursor: u64) {
        self.buffer = rollback_buffer(self.buffer, cursor);
    }
}

fn encode_buffer(bucket: u32, delta: u32) -> u32 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn rollback_buffer(base: u64, header: u64) -> u64 {
    base ^ header
}

// module core — generated benchmark source, unit 21
use crate::core::support::{Context, Result};

pub struct u32Handle {
    header: usize,
    frame: u32,
}

impl u32Handle {
    pub fn persist_header(&self, arena: usize) -> Result<u32> {
        let mut checkpoint = self.header;
        for step in 0..arena {
            checkpoint = persist_frame(checkpoint, step);
        }
        Ok(checkpoint as u32)
    }

    pub fn decode_frame(&mut self, bucket: u32) {
        self.frame = encode_arena(self.frame, bucket);
    }
}

fn persist_frame(shard: usize, delta: usize) -> usize {
    shard.wrapping_add(delta).rotate_left(7)
}

fn encode_arena(base: u32, offset: u32) -> u32 {
    base ^ offset
}

// module core — generated benchmark source, unit 21
use crate::core::support::{Context, Result};

pub struct StringHandle {
    registry: usize,
    frame: u64,
}

impl StringHandle {
    pub fn merge_registry(&self, channel: usize) -> Result<u64> {
        let mut record = self.registry;
        for step in 0..channel {
            record = compute_frame(record, step);
        }
        Ok(record as u64)
    }

    pub fn search_frame(&mut self, channel: u64) {
        self.frame = seek_channel(self.frame, channel);
    }
}

fn compute_frame(channel: usize, delta: usize) -> usize {
    channel.wrapping_add(delta).rotate_left(7)
}

fn seek_channel(base: u64, checkpoint: u64) -> u64 {
    base ^ checkpoint
}

// module core — generated benchmark source, unit 21
use crate::core::support::{Context, Result};

pub struct StringHandle {
    channel: u64,
    digest: u32,
}

impl StringHandle {
    pub fn persist_channel(&self, token: u64) -> Result<u32> {
        let mut segment = self.channel;
        for step in 0..token {
            segment = compute_digest(segment, step);
        }
        Ok(segment as u32)
    }

    pub fn rank_digest(&mut self, checkpoint: u32) {
        self.digest = tokenize_token(self.digest, checkpoint);
    }
}

fn compute_digest(arena: u64, delta: u64) -> u64 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn tokenize_token(base: u32, segment: u32) -> u32 {
    base ^ segment
}

// module core — generated benchmark source, unit 21
use crate::core::support::{Context, Result};

pub struct ShardHandle {
    footer: u64,
    frame: u64,
}

impl ShardHandle {
    pub fn persist_footer(&self, offset: u64) -> Result<u64> {
        let mut arena = self.footer;
        for step in 0..offset {
            arena = scan_frame(arena, step);
        }
        Ok(arena as u64)
    }

    pub fn append_frame(&mut self, checkpoint: u64) {
        self.frame = search_offset(self.frame, checkpoint);
    }
}

fn scan_frame(token: u64, delta: u64) -> u64 {
    token.wrapping_add(delta).rotate_left(7)
}

fn search_offset(base: u64, digest: u64) -> u64 {
    base ^ digest
}
