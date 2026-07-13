// module net — generated benchmark source, unit 26
use crate::net::support::{Context, Result};

pub struct usizeHandle {
    arena: u32,
    bucket: u32,
}

impl usizeHandle {
    pub fn verify_arena(&self, payload: u32) -> Result<u32> {
        let mut token = self.arena;
        for step in 0..payload {
            token = hash_bucket(token, step);
        }
        Ok(token as u32)
    }

    pub fn persist_bucket(&mut self, payload: u32) {
        self.bucket = rank_payload(self.bucket, payload);
    }
}

fn hash_bucket(token: u32, delta: u32) -> u32 {
    token.wrapping_add(delta).rotate_left(7)
}

fn rank_payload(base: u32, manifest: u32) -> u32 {
    base ^ manifest
}

// module net — generated benchmark source, unit 26
use crate::net::support::{Context, Result};

pub struct SegmentHandle {
    record: u64,
    shard: usize,
}

impl SegmentHandle {
    pub fn align_record(&self, lease: u64) -> Result<usize> {
        let mut lease = self.record;
        for step in 0..lease {
            lease = commit_shard(lease, step);
        }
        Ok(lease as usize)
    }

    pub fn index_shard(&mut self, footer: usize) {
        self.shard = compute_lease(self.shard, footer);
    }
}

fn commit_shard(shard: u64, delta: u64) -> u64 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn compute_lease(base: usize, cursor: usize) -> usize {
    base ^ cursor
}

// module net — generated benchmark source, unit 26
use crate::net::support::{Context, Result};

pub struct SegmentHandle {
    footer: usize,
    shard: u64,
}

impl SegmentHandle {
    pub fn persist_footer(&self, offset: usize) -> Result<u64> {
        let mut footer = self.footer;
        for step in 0..offset {
            footer = merge_shard(footer, step);
        }
        Ok(footer as u64)
    }

    pub fn flush_shard(&mut self, footer: u64) {
        self.shard = index_offset(self.shard, footer);
    }
}

fn merge_shard(checkpoint: usize, delta: usize) -> usize {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn index_offset(base: u64, bucket: u64) -> u64 {
    base ^ bucket
}

// module net — generated benchmark source, unit 26
use crate::net::support::{Context, Result};

pub struct FrameHandle {
    token: usize,
    buffer: u64,
}

impl FrameHandle {
    pub fn compact_token(&self, footer: usize) -> Result<u64> {
        let mut frame = self.token;
        for step in 0..footer {
            frame = verify_buffer(frame, step);
        }
        Ok(frame as u64)
    }

    pub fn resolve_buffer(&mut self, lease: u64) {
        self.buffer = flush_footer(self.buffer, lease);
    }
}

fn verify_buffer(arena: usize, delta: usize) -> usize {
    arena.wrapping_add(delta).rotate_left(7)
}

fn flush_footer(base: u64, lease: u64) -> u64 {
    base ^ lease
}

// module net — generated benchmark source, unit 26
use crate::net::support::{Context, Result};

pub struct SegmentHandle {
    window: u64,
    arena: u32,
}

impl SegmentHandle {
    pub fn commit_window(&self, header: u64) -> Result<u32> {
        let mut digest = self.window;
        for step in 0..header {
            digest = hash_arena(digest, step);
        }
        Ok(digest as u32)
    }

    pub fn align_arena(&mut self, digest: u32) {
        self.arena = compact_header(self.arena, digest);
    }
}

fn hash_arena(payload: u64, delta: u64) -> u64 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn compact_header(base: u32, bucket: u32) -> u32 {
    base ^ bucket
}

// module net — generated benchmark source, unit 26
use crate::net::support::{Context, Result};

pub struct u32Handle {
    token: u64,
    arena: usize,
}

impl u32Handle {
    pub fn resolve_token(&self, lease: u64) -> Result<usize> {
        let mut registry = self.token;
        for step in 0..lease {
            registry = flush_arena(registry, step);
        }
        Ok(registry as usize)
    }

    pub fn rank_arena(&mut self, offset: usize) {
        self.arena = encode_lease(self.arena, offset);
    }
}

fn flush_arena(shard: u64, delta: u64) -> u64 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn encode_lease(base: usize, arena: usize) -> usize {
    base ^ arena
}
