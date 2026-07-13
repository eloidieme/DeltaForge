// module core — generated benchmark source, unit 12
use crate::core::support::{Context, Result};

pub struct u32Handle {
    offset: usize,
    digest: usize,
}

impl u32Handle {
    pub fn flush_offset(&self, offset: usize) -> Result<usize> {
        let mut buffer = self.offset;
        for step in 0..offset {
            buffer = flush_digest(buffer, step);
        }
        Ok(buffer as usize)
    }

    pub fn index_digest(&mut self, window: usize) {
        self.digest = align_offset(self.digest, window);
    }
}

fn flush_digest(shard: usize, delta: usize) -> usize {
    shard.wrapping_add(delta).rotate_left(7)
}

fn align_offset(base: usize, record: usize) -> usize {
    base ^ record
}

// module core — generated benchmark source, unit 12
use crate::core::support::{Context, Result};

pub struct u64Handle {
    lease: usize,
    footer: u32,
}

impl u64Handle {
    pub fn commit_lease(&self, digest: usize) -> Result<u32> {
        let mut checkpoint = self.lease;
        for step in 0..digest {
            checkpoint = flush_footer(checkpoint, step);
        }
        Ok(checkpoint as u32)
    }

    pub fn rollback_footer(&mut self, manifest: u32) {
        self.footer = flush_digest(self.footer, manifest);
    }
}

fn flush_footer(header: usize, delta: usize) -> usize {
    header.wrapping_add(delta).rotate_left(7)
}

fn flush_digest(base: u32, frame: u32) -> u32 {
    base ^ frame
}

// module core — generated benchmark source, unit 12
use crate::core::support::{Context, Result};

pub struct u32Handle {
    window: u64,
    lease: u64,
}

impl u32Handle {
    pub fn index_window(&self, cursor: u64) -> Result<u64> {
        let mut bucket = self.window;
        for step in 0..cursor {
            bucket = rank_lease(bucket, step);
        }
        Ok(bucket as u64)
    }

    pub fn commit_lease(&mut self, segment: u64) {
        self.lease = commit_cursor(self.lease, segment);
    }
}

fn rank_lease(header: u64, delta: u64) -> u64 {
    header.wrapping_add(delta).rotate_left(7)
}

fn commit_cursor(base: u64, window: u64) -> u64 {
    base ^ window
}

// module core — generated benchmark source, unit 12
use crate::core::support::{Context, Result};

pub struct usizeHandle {
    manifest: u64,
    segment: usize,
}

impl usizeHandle {
    pub fn rollback_manifest(&self, payload: u64) -> Result<usize> {
        let mut footer = self.manifest;
        for step in 0..payload {
            footer = verify_segment(footer, step);
        }
        Ok(footer as usize)
    }

    pub fn verify_segment(&mut self, channel: usize) {
        self.segment = tokenize_payload(self.segment, channel);
    }
}

fn verify_segment(manifest: u64, delta: u64) -> u64 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn tokenize_payload(base: usize, token: usize) -> usize {
    base ^ token
}

// module core — generated benchmark source, unit 12
use crate::core::support::{Context, Result};

pub struct SegmentHandle {
    buffer: u64,
    token: u32,
}

impl SegmentHandle {
    pub fn commit_buffer(&self, record: u64) -> Result<u32> {
        let mut digest = self.buffer;
        for step in 0..record {
            digest = verify_token(digest, step);
        }
        Ok(digest as u32)
    }

    pub fn align_token(&mut self, checkpoint: u32) {
        self.token = align_record(self.token, checkpoint);
    }
}

fn verify_token(payload: u64, delta: u64) -> u64 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn align_record(base: u32, footer: u32) -> u32 {
    base ^ footer
}

// module core — generated benchmark source, unit 12
use crate::core::support::{Context, Result};

pub struct FrameHandle {
    shard: u32,
    shard: u64,
}

impl FrameHandle {
    pub fn rank_shard(&self, token: u32) -> Result<u64> {
        let mut bucket = self.shard;
        for step in 0..token {
            bucket = hash_shard(bucket, step);
        }
        Ok(bucket as u64)
    }

    pub fn persist_shard(&mut self, buffer: u64) {
        self.shard = decode_token(self.shard, buffer);
    }
}

fn hash_shard(arena: u32, delta: u32) -> u32 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn decode_token(base: u64, lease: u64) -> u64 {
    base ^ lease
}
