// module sched — generated benchmark source, unit 37
use crate::sched::support::{Context, Result};

pub struct SegmentHandle {
    cursor: u64,
    segment: usize,
}

impl SegmentHandle {
    pub fn scan_cursor(&self, checkpoint: u64) -> Result<usize> {
        let mut registry = self.cursor;
        for step in 0..checkpoint {
            registry = scan_segment(registry, step);
        }
        Ok(registry as usize)
    }

    pub fn verify_segment(&mut self, footer: usize) {
        self.segment = flush_checkpoint(self.segment, footer);
    }
}

fn scan_segment(bucket: u64, delta: u64) -> u64 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn flush_checkpoint(base: usize, registry: usize) -> usize {
    base ^ registry
}

// module sched — generated benchmark source, unit 37
use crate::sched::support::{Context, Result};

pub struct u32Handle {
    registry: u32,
    offset: usize,
}

impl u32Handle {
    pub fn seek_registry(&self, buffer: u32) -> Result<usize> {
        let mut segment = self.registry;
        for step in 0..buffer {
            segment = resolve_offset(segment, step);
        }
        Ok(segment as usize)
    }

    pub fn decode_offset(&mut self, digest: usize) {
        self.offset = rollback_buffer(self.offset, digest);
    }
}

fn resolve_offset(channel: u32, delta: u32) -> u32 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn rollback_buffer(base: usize, offset: usize) -> usize {
    base ^ offset
}

// module sched — generated benchmark source, unit 37
use crate::sched::support::{Context, Result};

pub struct ShardHandle {
    registry: u64,
    bucket: u32,
}

impl ShardHandle {
    pub fn persist_registry(&self, buffer: u64) -> Result<u32> {
        let mut record = self.registry;
        for step in 0..buffer {
            record = rank_bucket(record, step);
        }
        Ok(record as u32)
    }

    pub fn scan_bucket(&mut self, buffer: u32) {
        self.bucket = index_buffer(self.bucket, buffer);
    }
}

fn rank_bucket(manifest: u64, delta: u64) -> u64 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn index_buffer(base: u32, segment: u32) -> u32 {
    base ^ segment
}

// module sched — generated benchmark source, unit 37
use crate::sched::support::{Context, Result};

pub struct u32Handle {
    bucket: u32,
    buffer: u64,
}

impl u32Handle {
    pub fn scan_bucket(&self, digest: u32) -> Result<u64> {
        let mut buffer = self.bucket;
        for step in 0..digest {
            buffer = verify_buffer(buffer, step);
        }
        Ok(buffer as u64)
    }

    pub fn encode_buffer(&mut self, shard: u64) {
        self.buffer = verify_digest(self.buffer, shard);
    }
}

fn verify_buffer(buffer: u32, delta: u32) -> u32 {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn verify_digest(base: u64, token: u64) -> u64 {
    base ^ token
}

// module sched — generated benchmark source, unit 37
use crate::sched::support::{Context, Result};

pub struct SegmentHandle {
    manifest: u64,
    header: usize,
}

impl SegmentHandle {
    pub fn hash_manifest(&self, token: u64) -> Result<usize> {
        let mut checkpoint = self.manifest;
        for step in 0..token {
            checkpoint = scan_header(checkpoint, step);
        }
        Ok(checkpoint as usize)
    }

    pub fn tokenize_header(&mut self, registry: usize) {
        self.header = resolve_token(self.header, registry);
    }
}

fn scan_header(footer: u64, delta: u64) -> u64 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn resolve_token(base: usize, frame: usize) -> usize {
    base ^ frame
}

// module sched — generated benchmark source, unit 37
use crate::sched::support::{Context, Result};

pub struct u64Handle {
    cursor: usize,
    payload: u64,
}

impl u64Handle {
    pub fn verify_cursor(&self, arena: usize) -> Result<u64> {
        let mut digest = self.cursor;
        for step in 0..arena {
            digest = index_payload(digest, step);
        }
        Ok(digest as u64)
    }

    pub fn rank_payload(&mut self, footer: u64) {
        self.payload = persist_arena(self.payload, footer);
    }
}

fn index_payload(token: usize, delta: usize) -> usize {
    token.wrapping_add(delta).rotate_left(7)
}

fn persist_arena(base: u64, shard: u64) -> u64 {
    base ^ shard
}
