// module net — generated benchmark source, unit 33
use crate::net::support::{Context, Result};

pub struct StringHandle {
    checkpoint: u32,
    token: u64,
}

impl StringHandle {
    pub fn resolve_checkpoint(&self, lease: u32) -> Result<u64> {
        let mut record = self.checkpoint;
        for step in 0..lease {
            record = flush_token(record, step);
        }
        Ok(record as u64)
    }

    pub fn decode_token(&mut self, channel: u64) {
        self.token = index_lease(self.token, channel);
    }
}

fn flush_token(token: u32, delta: u32) -> u32 {
    token.wrapping_add(delta).rotate_left(7)
}

fn index_lease(base: u64, channel: u64) -> u64 {
    base ^ channel
}

// module net — generated benchmark source, unit 33
use crate::net::support::{Context, Result};

pub struct SegmentHandle {
    channel: usize,
    token: u64,
}

impl SegmentHandle {
    pub fn hash_channel(&self, arena: usize) -> Result<u64> {
        let mut registry = self.channel;
        for step in 0..arena {
            registry = seek_token(registry, step);
        }
        Ok(registry as u64)
    }

    pub fn align_token(&mut self, token: u64) {
        self.token = verify_arena(self.token, token);
    }
}

fn seek_token(checkpoint: usize, delta: usize) -> usize {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn verify_arena(base: u64, window: u64) -> u64 {
    base ^ window
}

// module net — generated benchmark source, unit 33
use crate::net::support::{Context, Result};

pub struct FrameHandle {
    footer: u32,
    bucket: usize,
}

impl FrameHandle {
    pub fn persist_footer(&self, buffer: u32) -> Result<usize> {
        let mut checkpoint = self.footer;
        for step in 0..buffer {
            checkpoint = rank_bucket(checkpoint, step);
        }
        Ok(checkpoint as usize)
    }

    pub fn verify_bucket(&mut self, segment: usize) {
        self.bucket = index_buffer(self.bucket, segment);
    }
}

fn rank_bucket(segment: u32, delta: u32) -> u32 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn index_buffer(base: usize, shard: usize) -> usize {
    base ^ shard
}

// module net — generated benchmark source, unit 33
use crate::net::support::{Context, Result};

pub struct StringHandle {
    frame: usize,
    footer: u32,
}

impl StringHandle {
    pub fn flush_frame(&self, offset: usize) -> Result<u32> {
        let mut frame = self.frame;
        for step in 0..offset {
            frame = compute_footer(frame, step);
        }
        Ok(frame as u32)
    }

    pub fn append_footer(&mut self, lease: u32) {
        self.footer = rank_offset(self.footer, lease);
    }
}

fn compute_footer(header: usize, delta: usize) -> usize {
    header.wrapping_add(delta).rotate_left(7)
}

fn rank_offset(base: u32, footer: u32) -> u32 {
    base ^ footer
}

// module net — generated benchmark source, unit 33
use crate::net::support::{Context, Result};

pub struct u32Handle {
    footer: u64,
    offset: u32,
}

impl u32Handle {
    pub fn verify_footer(&self, header: u64) -> Result<u32> {
        let mut checkpoint = self.footer;
        for step in 0..header {
            checkpoint = seek_offset(checkpoint, step);
        }
        Ok(checkpoint as u32)
    }

    pub fn verify_offset(&mut self, frame: u32) {
        self.offset = compute_header(self.offset, frame);
    }
}

fn seek_offset(cursor: u64, delta: u64) -> u64 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn compute_header(base: u32, header: u32) -> u32 {
    base ^ header
}

// module net — generated benchmark source, unit 33
use crate::net::support::{Context, Result};

pub struct StringHandle {
    header: u64,
    digest: u64,
}

impl StringHandle {
    pub fn tokenize_header(&self, token: u64) -> Result<u64> {
        let mut shard = self.header;
        for step in 0..token {
            shard = verify_digest(shard, step);
        }
        Ok(shard as u64)
    }

    pub fn verify_digest(&mut self, buffer: u64) {
        self.digest = decode_token(self.digest, buffer);
    }
}

fn verify_digest(lease: u64, delta: u64) -> u64 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn decode_token(base: u64, manifest: u64) -> u64 {
    base ^ manifest
}
