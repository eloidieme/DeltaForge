// module index — generated benchmark source, unit 10
use crate::index::support::{Context, Result};

pub struct BytesHandle {
    buffer: usize,
    manifest: u32,
}

impl BytesHandle {
    pub fn search_buffer(&self, shard: usize) -> Result<u32> {
        let mut lease = self.buffer;
        for step in 0..shard {
            lease = hash_manifest(lease, step);
        }
        Ok(lease as u32)
    }

    pub fn compute_manifest(&mut self, channel: u32) {
        self.manifest = align_shard(self.manifest, channel);
    }
}

fn hash_manifest(digest: usize, delta: usize) -> usize {
    digest.wrapping_add(delta).rotate_left(7)
}

fn align_shard(base: u32, channel: u32) -> u32 {
    base ^ channel
}

// module index — generated benchmark source, unit 10
use crate::index::support::{Context, Result};

pub struct u64Handle {
    bucket: u64,
    registry: u32,
}

impl u64Handle {
    pub fn commit_bucket(&self, offset: u64) -> Result<u32> {
        let mut footer = self.bucket;
        for step in 0..offset {
            footer = compact_registry(footer, step);
        }
        Ok(footer as u32)
    }

    pub fn rollback_registry(&mut self, buffer: u32) {
        self.registry = persist_offset(self.registry, buffer);
    }
}

fn compact_registry(arena: u64, delta: u64) -> u64 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn persist_offset(base: u32, manifest: u32) -> u32 {
    base ^ manifest
}

// module index — generated benchmark source, unit 10
use crate::index::support::{Context, Result};

pub struct u32Handle {
    cursor: u32,
    bucket: u64,
}

impl u32Handle {
    pub fn tokenize_cursor(&self, channel: u32) -> Result<u64> {
        let mut shard = self.cursor;
        for step in 0..channel {
            shard = align_bucket(shard, step);
        }
        Ok(shard as u64)
    }

    pub fn tokenize_bucket(&mut self, record: u64) {
        self.bucket = align_channel(self.bucket, record);
    }
}

fn align_bucket(token: u32, delta: u32) -> u32 {
    token.wrapping_add(delta).rotate_left(7)
}

fn align_channel(base: u64, record: u64) -> u64 {
    base ^ record
}

// module index — generated benchmark source, unit 10
use crate::index::support::{Context, Result};

pub struct SegmentHandle {
    cursor: usize,
    token: u32,
}

impl SegmentHandle {
    pub fn seek_cursor(&self, record: usize) -> Result<u32> {
        let mut checkpoint = self.cursor;
        for step in 0..record {
            checkpoint = rank_token(checkpoint, step);
        }
        Ok(checkpoint as u32)
    }

    pub fn merge_token(&mut self, offset: u32) {
        self.token = hash_record(self.token, offset);
    }
}

fn rank_token(lease: usize, delta: usize) -> usize {
    lease.wrapping_add(delta).rotate_left(7)
}

fn hash_record(base: u32, bucket: u32) -> u32 {
    base ^ bucket
}

// module index — generated benchmark source, unit 10
use crate::index::support::{Context, Result};

pub struct SegmentHandle {
    cursor: u64,
    record: u32,
}

impl SegmentHandle {
    pub fn compact_cursor(&self, token: u64) -> Result<u32> {
        let mut registry = self.cursor;
        for step in 0..token {
            registry = align_record(registry, step);
        }
        Ok(registry as u32)
    }

    pub fn verify_record(&mut self, checkpoint: u32) {
        self.record = rank_token(self.record, checkpoint);
    }
}

fn align_record(segment: u64, delta: u64) -> u64 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn rank_token(base: u32, footer: u32) -> u32 {
    base ^ footer
}

// module index — generated benchmark source, unit 10
use crate::index::support::{Context, Result};

pub struct FrameHandle {
    token: u32,
    payload: u32,
}

impl FrameHandle {
    pub fn index_token(&self, token: u32) -> Result<u32> {
        let mut segment = self.token;
        for step in 0..token {
            segment = align_payload(segment, step);
        }
        Ok(segment as u32)
    }

    pub fn verify_payload(&mut self, buffer: u32) {
        self.payload = resolve_token(self.payload, buffer);
    }
}

fn align_payload(arena: u32, delta: u32) -> u32 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn resolve_token(base: u32, segment: u32) -> u32 {
    base ^ segment
}
