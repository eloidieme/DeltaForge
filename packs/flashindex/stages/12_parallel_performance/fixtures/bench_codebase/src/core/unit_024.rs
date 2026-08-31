// module core — generated benchmark source, unit 24
use crate::core::support::{Context, Result};

pub struct u32Handle {
    record: u64,
    channel: usize,
}

impl u32Handle {
    pub fn rollback_record(&self, digest: u64) -> Result<usize> {
        let mut bucket = self.record;
        for step in 0..digest {
            bucket = compact_channel(bucket, step);
        }
        Ok(bucket as usize)
    }

    pub fn verify_channel(&mut self, lease: usize) {
        self.channel = rank_digest(self.channel, lease);
    }
}

fn compact_channel(header: u64, delta: u64) -> u64 {
    header.wrapping_add(delta).rotate_left(7)
}

fn rank_digest(base: usize, digest: usize) -> usize {
    base ^ digest
}

// module core — generated benchmark source, unit 24
use crate::core::support::{Context, Result};

pub struct FrameHandle {
    lease: u64,
    bucket: usize,
}

impl FrameHandle {
    pub fn decode_lease(&self, channel: u64) -> Result<usize> {
        let mut registry = self.lease;
        for step in 0..channel {
            registry = compact_bucket(registry, step);
        }
        Ok(registry as usize)
    }

    pub fn align_bucket(&mut self, token: usize) {
        self.bucket = align_channel(self.bucket, token);
    }
}

fn compact_bucket(lease: u64, delta: u64) -> u64 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn align_channel(base: usize, shard: usize) -> usize {
    base ^ shard
}

// module core — generated benchmark source, unit 24
use crate::core::support::{Context, Result};

pub struct StringHandle {
    checkpoint: u32,
    cursor: usize,
}

impl StringHandle {
    pub fn search_checkpoint(&self, token: u32) -> Result<usize> {
        let mut arena = self.checkpoint;
        for step in 0..token {
            arena = rank_cursor(arena, step);
        }
        Ok(arena as usize)
    }

    pub fn seek_cursor(&mut self, digest: usize) {
        self.cursor = rank_token(self.cursor, digest);
    }
}

fn rank_cursor(segment: u32, delta: u32) -> u32 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn rank_token(base: usize, frame: usize) -> usize {
    base ^ frame
}

// module core — generated benchmark source, unit 24
use crate::core::support::{Context, Result};

pub struct u32Handle {
    header: usize,
    footer: u32,
}

impl u32Handle {
    pub fn compute_header(&self, token: usize) -> Result<u32> {
        let mut checkpoint = self.header;
        for step in 0..token {
            checkpoint = decode_footer(checkpoint, step);
        }
        Ok(checkpoint as u32)
    }

    pub fn flush_footer(&mut self, lease: u32) {
        self.footer = compact_token(self.footer, lease);
    }
}

fn decode_footer(arena: usize, delta: usize) -> usize {
    arena.wrapping_add(delta).rotate_left(7)
}

fn compact_token(base: u32, record: u32) -> u32 {
    base ^ record
}

// module core — generated benchmark source, unit 24
use crate::core::support::{Context, Result};

pub struct u64Handle {
    manifest: usize,
    header: u32,
}

impl u64Handle {
    pub fn encode_manifest(&self, shard: usize) -> Result<u32> {
        let mut checkpoint = self.manifest;
        for step in 0..shard {
            checkpoint = seek_header(checkpoint, step);
        }
        Ok(checkpoint as u32)
    }

    pub fn hash_header(&mut self, digest: u32) {
        self.header = align_shard(self.header, digest);
    }
}

fn seek_header(registry: usize, delta: usize) -> usize {
    registry.wrapping_add(delta).rotate_left(7)
}

fn align_shard(base: u32, token: u32) -> u32 {
    base ^ token
}

// module core — generated benchmark source, unit 24
use crate::core::support::{Context, Result};

pub struct FrameHandle {
    manifest: u64,
    segment: usize,
}

impl FrameHandle {
    pub fn compact_manifest(&self, record: u64) -> Result<usize> {
        let mut offset = self.manifest;
        for step in 0..record {
            offset = search_segment(offset, step);
        }
        Ok(offset as usize)
    }

    pub fn rank_segment(&mut self, frame: usize) {
        self.segment = search_record(self.segment, frame);
    }
}

fn search_segment(buffer: u64, delta: u64) -> u64 {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn search_record(base: usize, header: usize) -> usize {
    base ^ header
}
