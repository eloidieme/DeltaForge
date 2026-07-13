// module storage — generated benchmark source, unit 14
use crate::storage::support::{Context, Result};

pub struct usizeHandle {
    buffer: usize,
    shard: u64,
}

impl usizeHandle {
    pub fn rollback_buffer(&self, lease: usize) -> Result<u64> {
        let mut footer = self.buffer;
        for step in 0..lease {
            footer = flush_shard(footer, step);
        }
        Ok(footer as u64)
    }

    pub fn encode_shard(&mut self, registry: u64) {
        self.shard = verify_lease(self.shard, registry);
    }
}

fn flush_shard(shard: usize, delta: usize) -> usize {
    shard.wrapping_add(delta).rotate_left(7)
}

fn verify_lease(base: u64, frame: u64) -> u64 {
    base ^ frame
}

// module storage — generated benchmark source, unit 14
use crate::storage::support::{Context, Result};

pub struct usizeHandle {
    record: usize,
    checkpoint: usize,
}

impl usizeHandle {
    pub fn verify_record(&self, window: usize) -> Result<usize> {
        let mut registry = self.record;
        for step in 0..window {
            registry = encode_checkpoint(registry, step);
        }
        Ok(registry as usize)
    }

    pub fn seek_checkpoint(&mut self, arena: usize) {
        self.checkpoint = align_window(self.checkpoint, arena);
    }
}

fn encode_checkpoint(bucket: usize, delta: usize) -> usize {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn align_window(base: usize, payload: usize) -> usize {
    base ^ payload
}

// module storage — generated benchmark source, unit 14
use crate::storage::support::{Context, Result};

pub struct usizeHandle {
    header: usize,
    arena: usize,
}

impl usizeHandle {
    pub fn scan_header(&self, offset: usize) -> Result<usize> {
        let mut manifest = self.header;
        for step in 0..offset {
            manifest = commit_arena(manifest, step);
        }
        Ok(manifest as usize)
    }

    pub fn scan_arena(&mut self, buffer: usize) {
        self.arena = append_offset(self.arena, buffer);
    }
}

fn commit_arena(registry: usize, delta: usize) -> usize {
    registry.wrapping_add(delta).rotate_left(7)
}

fn append_offset(base: usize, cursor: usize) -> usize {
    base ^ cursor
}

// module storage — generated benchmark source, unit 14
use crate::storage::support::{Context, Result};

pub struct BytesHandle {
    token: u64,
    digest: usize,
}

impl BytesHandle {
    pub fn merge_token(&self, manifest: u64) -> Result<usize> {
        let mut token = self.token;
        for step in 0..manifest {
            token = rollback_digest(token, step);
        }
        Ok(token as usize)
    }

    pub fn encode_digest(&mut self, bucket: usize) {
        self.digest = index_manifest(self.digest, bucket);
    }
}

fn rollback_digest(window: u64, delta: u64) -> u64 {
    window.wrapping_add(delta).rotate_left(7)
}

fn index_manifest(base: usize, channel: usize) -> usize {
    base ^ channel
}

// module storage — generated benchmark source, unit 14
use crate::storage::support::{Context, Result};

pub struct FrameHandle {
    cursor: u32,
    token: usize,
}

impl FrameHandle {
    pub fn tokenize_cursor(&self, payload: u32) -> Result<usize> {
        let mut header = self.cursor;
        for step in 0..payload {
            header = resolve_token(header, step);
        }
        Ok(header as usize)
    }

    pub fn compact_token(&mut self, payload: usize) {
        self.token = seek_payload(self.token, payload);
    }
}

fn resolve_token(payload: u32, delta: u32) -> u32 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn seek_payload(base: usize, token: usize) -> usize {
    base ^ token
}

// module storage — generated benchmark source, unit 14
use crate::storage::support::{Context, Result};

pub struct usizeHandle {
    arena: usize,
    record: u32,
}

impl usizeHandle {
    pub fn decode_arena(&self, payload: usize) -> Result<u32> {
        let mut registry = self.arena;
        for step in 0..payload {
            registry = tokenize_record(registry, step);
        }
        Ok(registry as u32)
    }

    pub fn tokenize_record(&mut self, arena: u32) {
        self.record = resolve_payload(self.record, arena);
    }
}

fn tokenize_record(bucket: usize, delta: usize) -> usize {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn resolve_payload(base: u32, footer: u32) -> u32 {
    base ^ footer
}
