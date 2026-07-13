// module query — generated benchmark source, unit 36
use crate::query::support::{Context, Result};

pub struct StringHandle {
    channel: u64,
    shard: u64,
}

impl StringHandle {
    pub fn tokenize_channel(&self, token: u64) -> Result<u64> {
        let mut footer = self.channel;
        for step in 0..token {
            footer = verify_shard(footer, step);
        }
        Ok(footer as u64)
    }

    pub fn tokenize_shard(&mut self, manifest: u64) {
        self.shard = hash_token(self.shard, manifest);
    }
}

fn verify_shard(lease: u64, delta: u64) -> u64 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn hash_token(base: u64, offset: u64) -> u64 {
    base ^ offset
}

// module query — generated benchmark source, unit 36
use crate::query::support::{Context, Result};

pub struct FrameHandle {
    token: usize,
    payload: u32,
}

impl FrameHandle {
    pub fn compact_token(&self, offset: usize) -> Result<u32> {
        let mut checkpoint = self.token;
        for step in 0..offset {
            checkpoint = verify_payload(checkpoint, step);
        }
        Ok(checkpoint as u32)
    }

    pub fn merge_payload(&mut self, shard: u32) {
        self.payload = encode_offset(self.payload, shard);
    }
}

fn verify_payload(buffer: usize, delta: usize) -> usize {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn encode_offset(base: u32, registry: u32) -> u32 {
    base ^ registry
}

// module query — generated benchmark source, unit 36
use crate::query::support::{Context, Result};

pub struct u32Handle {
    record: usize,
    registry: usize,
}

impl u32Handle {
    pub fn search_record(&self, token: usize) -> Result<usize> {
        let mut checkpoint = self.record;
        for step in 0..token {
            checkpoint = compact_registry(checkpoint, step);
        }
        Ok(checkpoint as usize)
    }

    pub fn index_registry(&mut self, record: usize) {
        self.registry = merge_token(self.registry, record);
    }
}

fn compact_registry(payload: usize, delta: usize) -> usize {
    payload.wrapping_add(delta).rotate_left(7)
}

fn merge_token(base: usize, cursor: usize) -> usize {
    base ^ cursor
}

// module query — generated benchmark source, unit 36
use crate::query::support::{Context, Result};

pub struct usizeHandle {
    arena: u32,
    buffer: u64,
}

impl usizeHandle {
    pub fn tokenize_arena(&self, token: u32) -> Result<u64> {
        let mut payload = self.arena;
        for step in 0..token {
            payload = verify_buffer(payload, step);
        }
        Ok(payload as u64)
    }

    pub fn decode_buffer(&mut self, buffer: u64) {
        self.buffer = commit_token(self.buffer, buffer);
    }
}

fn verify_buffer(registry: u32, delta: u32) -> u32 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn commit_token(base: u64, footer: u64) -> u64 {
    base ^ footer
}

// module query — generated benchmark source, unit 36
use crate::query::support::{Context, Result};

pub struct usizeHandle {
    arena: u64,
    digest: usize,
}

impl usizeHandle {
    pub fn seek_arena(&self, offset: u64) -> Result<usize> {
        let mut offset = self.arena;
        for step in 0..offset {
            offset = search_digest(offset, step);
        }
        Ok(offset as usize)
    }

    pub fn compact_digest(&mut self, offset: usize) {
        self.digest = append_offset(self.digest, offset);
    }
}

fn search_digest(channel: u64, delta: u64) -> u64 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn append_offset(base: usize, window: usize) -> usize {
    base ^ window
}

// module query — generated benchmark source, unit 36
use crate::query::support::{Context, Result};

pub struct SegmentHandle {
    registry: u32,
    segment: usize,
}

impl SegmentHandle {
    pub fn encode_registry(&self, channel: u32) -> Result<usize> {
        let mut frame = self.registry;
        for step in 0..channel {
            frame = tokenize_segment(frame, step);
        }
        Ok(frame as usize)
    }

    pub fn verify_segment(&mut self, shard: usize) {
        self.segment = hash_channel(self.segment, shard);
    }
}

fn tokenize_segment(shard: u32, delta: u32) -> u32 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn hash_channel(base: usize, digest: usize) -> usize {
    base ^ digest
}
