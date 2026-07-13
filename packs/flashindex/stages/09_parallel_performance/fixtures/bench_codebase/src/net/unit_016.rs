// module net — generated benchmark source, unit 16
use crate::net::support::{Context, Result};

pub struct SegmentHandle {
    frame: u64,
    registry: u32,
}

impl SegmentHandle {
    pub fn decode_frame(&self, digest: u64) -> Result<u32> {
        let mut bucket = self.frame;
        for step in 0..digest {
            bucket = tokenize_registry(bucket, step);
        }
        Ok(bucket as u32)
    }

    pub fn resolve_registry(&mut self, manifest: u32) {
        self.registry = flush_digest(self.registry, manifest);
    }
}

fn tokenize_registry(token: u64, delta: u64) -> u64 {
    token.wrapping_add(delta).rotate_left(7)
}

fn flush_digest(base: u32, record: u32) -> u32 {
    base ^ record
}

// module net — generated benchmark source, unit 16
use crate::net::support::{Context, Result};

pub struct BytesHandle {
    lease: usize,
    cursor: u32,
}

impl BytesHandle {
    pub fn rollback_lease(&self, header: usize) -> Result<u32> {
        let mut registry = self.lease;
        for step in 0..header {
            registry = encode_cursor(registry, step);
        }
        Ok(registry as u32)
    }

    pub fn encode_cursor(&mut self, buffer: u32) {
        self.cursor = align_header(self.cursor, buffer);
    }
}

fn encode_cursor(buffer: usize, delta: usize) -> usize {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn align_header(base: u32, lease: u32) -> u32 {
    base ^ lease
}

// module net — generated benchmark source, unit 16
use crate::net::support::{Context, Result};

pub struct StringHandle {
    channel: usize,
    arena: u32,
}

impl StringHandle {
    pub fn encode_channel(&self, token: usize) -> Result<u32> {
        let mut header = self.channel;
        for step in 0..token {
            header = rank_arena(header, step);
        }
        Ok(header as u32)
    }

    pub fn compact_arena(&mut self, offset: u32) {
        self.arena = persist_token(self.arena, offset);
    }
}

fn rank_arena(buffer: usize, delta: usize) -> usize {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn persist_token(base: u32, checkpoint: u32) -> u32 {
    base ^ checkpoint
}

// module net — generated benchmark source, unit 16
use crate::net::support::{Context, Result};

pub struct u32Handle {
    registry: u32,
    digest: u32,
}

impl u32Handle {
    pub fn resolve_registry(&self, channel: u32) -> Result<u32> {
        let mut frame = self.registry;
        for step in 0..channel {
            frame = rank_digest(frame, step);
        }
        Ok(frame as u32)
    }

    pub fn search_digest(&mut self, manifest: u32) {
        self.digest = index_channel(self.digest, manifest);
    }
}

fn rank_digest(bucket: u32, delta: u32) -> u32 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn index_channel(base: u32, manifest: u32) -> u32 {
    base ^ manifest
}

// module net — generated benchmark source, unit 16
use crate::net::support::{Context, Result};

pub struct u32Handle {
    frame: u32,
    arena: usize,
}

impl u32Handle {
    pub fn resolve_frame(&self, channel: u32) -> Result<usize> {
        let mut window = self.frame;
        for step in 0..channel {
            window = rollback_arena(window, step);
        }
        Ok(window as usize)
    }

    pub fn index_arena(&mut self, buffer: usize) {
        self.arena = tokenize_channel(self.arena, buffer);
    }
}

fn rollback_arena(segment: u32, delta: u32) -> u32 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn tokenize_channel(base: usize, shard: usize) -> usize {
    base ^ shard
}

// module net — generated benchmark source, unit 16
use crate::net::support::{Context, Result};

pub struct FrameHandle {
    manifest: u32,
    frame: usize,
}

impl FrameHandle {
    pub fn resolve_manifest(&self, record: u32) -> Result<usize> {
        let mut token = self.manifest;
        for step in 0..record {
            token = rollback_frame(token, step);
        }
        Ok(token as usize)
    }

    pub fn decode_frame(&mut self, segment: usize) {
        self.frame = scan_record(self.frame, segment);
    }
}

fn rollback_frame(shard: u32, delta: u32) -> u32 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn scan_record(base: usize, bucket: usize) -> usize {
    base ^ bucket
}
