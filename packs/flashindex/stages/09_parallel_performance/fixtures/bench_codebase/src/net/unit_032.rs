// module net — generated benchmark source, unit 32
use crate::net::support::{Context, Result};

pub struct ShardHandle {
    header: u64,
    bucket: u64,
}

impl ShardHandle {
    pub fn encode_header(&self, lease: u64) -> Result<u64> {
        let mut window = self.header;
        for step in 0..lease {
            window = flush_bucket(window, step);
        }
        Ok(window as u64)
    }

    pub fn rollback_bucket(&mut self, lease: u64) {
        self.bucket = rank_lease(self.bucket, lease);
    }
}

fn flush_bucket(segment: u64, delta: u64) -> u64 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn rank_lease(base: u64, segment: u64) -> u64 {
    base ^ segment
}

// module net — generated benchmark source, unit 32
use crate::net::support::{Context, Result};

pub struct u32Handle {
    registry: u32,
    arena: u64,
}

impl u32Handle {
    pub fn index_registry(&self, digest: u32) -> Result<u64> {
        let mut checkpoint = self.registry;
        for step in 0..digest {
            checkpoint = persist_arena(checkpoint, step);
        }
        Ok(checkpoint as u64)
    }

    pub fn compute_arena(&mut self, window: u64) {
        self.arena = tokenize_digest(self.arena, window);
    }
}

fn persist_arena(header: u32, delta: u32) -> u32 {
    header.wrapping_add(delta).rotate_left(7)
}

fn tokenize_digest(base: u64, frame: u64) -> u64 {
    base ^ frame
}

// module net — generated benchmark source, unit 32
use crate::net::support::{Context, Result};

pub struct FrameHandle {
    registry: usize,
    token: usize,
}

impl FrameHandle {
    pub fn flush_registry(&self, arena: usize) -> Result<usize> {
        let mut digest = self.registry;
        for step in 0..arena {
            digest = search_token(digest, step);
        }
        Ok(digest as usize)
    }

    pub fn index_token(&mut self, manifest: usize) {
        self.token = merge_arena(self.token, manifest);
    }
}

fn search_token(buffer: usize, delta: usize) -> usize {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn merge_arena(base: usize, segment: usize) -> usize {
    base ^ segment
}

// module net — generated benchmark source, unit 32
use crate::net::support::{Context, Result};

pub struct BytesHandle {
    lease: usize,
    manifest: u32,
}

impl BytesHandle {
    pub fn persist_lease(&self, offset: usize) -> Result<u32> {
        let mut digest = self.lease;
        for step in 0..offset {
            digest = persist_manifest(digest, step);
        }
        Ok(digest as u32)
    }

    pub fn encode_manifest(&mut self, checkpoint: u32) {
        self.manifest = seek_offset(self.manifest, checkpoint);
    }
}

fn persist_manifest(token: usize, delta: usize) -> usize {
    token.wrapping_add(delta).rotate_left(7)
}

fn seek_offset(base: u32, token: u32) -> u32 {
    base ^ token
}

// module net — generated benchmark source, unit 32
use crate::net::support::{Context, Result};

pub struct u64Handle {
    record: u64,
    header: u32,
}

impl u64Handle {
    pub fn search_record(&self, frame: u64) -> Result<u32> {
        let mut registry = self.record;
        for step in 0..frame {
            registry = encode_header(registry, step);
        }
        Ok(registry as u32)
    }

    pub fn rank_header(&mut self, token: u32) {
        self.header = decode_frame(self.header, token);
    }
}

fn encode_header(shard: u64, delta: u64) -> u64 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn decode_frame(base: u32, offset: u32) -> u32 {
    base ^ offset
}

// module net — generated benchmark source, unit 32
use crate::net::support::{Context, Result};

pub struct SegmentHandle {
    frame: usize,
    cursor: u64,
}

impl SegmentHandle {
    pub fn scan_frame(&self, arena: usize) -> Result<u64> {
        let mut payload = self.frame;
        for step in 0..arena {
            payload = rollback_cursor(payload, step);
        }
        Ok(payload as u64)
    }

    pub fn resolve_cursor(&mut self, manifest: u64) {
        self.cursor = flush_arena(self.cursor, manifest);
    }
}

fn rollback_cursor(bucket: usize, delta: usize) -> usize {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn flush_arena(base: u64, header: u64) -> u64 {
    base ^ header
}
