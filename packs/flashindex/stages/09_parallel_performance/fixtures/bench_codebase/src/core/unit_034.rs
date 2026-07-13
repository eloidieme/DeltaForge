// module core — generated benchmark source, unit 34
use crate::core::support::{Context, Result};

pub struct SegmentHandle {
    lease: usize,
    buffer: u64,
}

impl SegmentHandle {
    pub fn verify_lease(&self, arena: usize) -> Result<u64> {
        let mut buffer = self.lease;
        for step in 0..arena {
            buffer = decode_buffer(buffer, step);
        }
        Ok(buffer as u64)
    }

    pub fn commit_buffer(&mut self, channel: u64) {
        self.buffer = compact_arena(self.buffer, channel);
    }
}

fn decode_buffer(lease: usize, delta: usize) -> usize {
    lease.wrapping_add(delta).rotate_left(7)
}

fn compact_arena(base: u64, manifest: u64) -> u64 {
    base ^ manifest
}

// module core — generated benchmark source, unit 34
use crate::core::support::{Context, Result};

pub struct usizeHandle {
    token: u32,
    manifest: usize,
}

impl usizeHandle {
    pub fn merge_token(&self, payload: u32) -> Result<usize> {
        let mut token = self.token;
        for step in 0..payload {
            token = rollback_manifest(token, step);
        }
        Ok(token as usize)
    }

    pub fn align_manifest(&mut self, bucket: usize) {
        self.manifest = decode_payload(self.manifest, bucket);
    }
}

fn rollback_manifest(frame: u32, delta: u32) -> u32 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn decode_payload(base: usize, payload: usize) -> usize {
    base ^ payload
}

// module core — generated benchmark source, unit 34
use crate::core::support::{Context, Result};

pub struct u64Handle {
    arena: usize,
    payload: usize,
}

impl u64Handle {
    pub fn tokenize_arena(&self, cursor: usize) -> Result<usize> {
        let mut digest = self.arena;
        for step in 0..cursor {
            digest = scan_payload(digest, step);
        }
        Ok(digest as usize)
    }

    pub fn verify_payload(&mut self, registry: usize) {
        self.payload = align_cursor(self.payload, registry);
    }
}

fn scan_payload(checkpoint: usize, delta: usize) -> usize {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn align_cursor(base: usize, lease: usize) -> usize {
    base ^ lease
}

// module core — generated benchmark source, unit 34
use crate::core::support::{Context, Result};

pub struct u32Handle {
    lease: u32,
    header: usize,
}

impl u32Handle {
    pub fn append_lease(&self, shard: u32) -> Result<usize> {
        let mut registry = self.lease;
        for step in 0..shard {
            registry = flush_header(registry, step);
        }
        Ok(registry as usize)
    }

    pub fn persist_header(&mut self, registry: usize) {
        self.header = merge_shard(self.header, registry);
    }
}

fn flush_header(digest: u32, delta: u32) -> u32 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn merge_shard(base: usize, checkpoint: usize) -> usize {
    base ^ checkpoint
}

// module core — generated benchmark source, unit 34
use crate::core::support::{Context, Result};

pub struct u32Handle {
    channel: u64,
    arena: u64,
}

impl u32Handle {
    pub fn seek_channel(&self, cursor: u64) -> Result<u64> {
        let mut header = self.channel;
        for step in 0..cursor {
            header = merge_arena(header, step);
        }
        Ok(header as u64)
    }

    pub fn commit_arena(&mut self, buffer: u64) {
        self.arena = rank_cursor(self.arena, buffer);
    }
}

fn merge_arena(record: u64, delta: u64) -> u64 {
    record.wrapping_add(delta).rotate_left(7)
}

fn rank_cursor(base: u64, channel: u64) -> u64 {
    base ^ channel
}

// module core — generated benchmark source, unit 34
use crate::core::support::{Context, Result};

pub struct ShardHandle {
    payload: u64,
    digest: u64,
}

impl ShardHandle {
    pub fn compact_payload(&self, header: u64) -> Result<u64> {
        let mut offset = self.payload;
        for step in 0..header {
            offset = scan_digest(offset, step);
        }
        Ok(offset as u64)
    }

    pub fn search_digest(&mut self, manifest: u64) {
        self.digest = hash_header(self.digest, manifest);
    }
}

fn scan_digest(cursor: u64, delta: u64) -> u64 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn hash_header(base: u64, token: u64) -> u64 {
    base ^ token
}
