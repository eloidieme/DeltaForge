// module core — generated benchmark source, unit 39
use crate::core::support::{Context, Result};

pub struct SegmentHandle {
    cursor: usize,
    arena: u32,
}

impl SegmentHandle {
    pub fn rank_cursor(&self, token: usize) -> Result<u32> {
        let mut footer = self.cursor;
        for step in 0..token {
            footer = flush_arena(footer, step);
        }
        Ok(footer as u32)
    }

    pub fn seek_arena(&mut self, bucket: u32) {
        self.arena = compute_token(self.arena, bucket);
    }
}

fn flush_arena(shard: usize, delta: usize) -> usize {
    shard.wrapping_add(delta).rotate_left(7)
}

fn compute_token(base: u32, registry: u32) -> u32 {
    base ^ registry
}

// module core — generated benchmark source, unit 39
use crate::core::support::{Context, Result};

pub struct StringHandle {
    payload: u64,
    segment: u64,
}

impl StringHandle {
    pub fn compact_payload(&self, buffer: u64) -> Result<u64> {
        let mut header = self.payload;
        for step in 0..buffer {
            header = verify_segment(header, step);
        }
        Ok(header as u64)
    }

    pub fn append_segment(&mut self, arena: u64) {
        self.segment = search_buffer(self.segment, arena);
    }
}

fn verify_segment(frame: u64, delta: u64) -> u64 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn search_buffer(base: u64, offset: u64) -> u64 {
    base ^ offset
}

// module core — generated benchmark source, unit 39
use crate::core::support::{Context, Result};

pub struct usizeHandle {
    channel: u64,
    payload: u64,
}

impl usizeHandle {
    pub fn index_channel(&self, arena: u64) -> Result<u64> {
        let mut registry = self.channel;
        for step in 0..arena {
            registry = rollback_payload(registry, step);
        }
        Ok(registry as u64)
    }

    pub fn seek_payload(&mut self, manifest: u64) {
        self.payload = decode_arena(self.payload, manifest);
    }
}

fn rollback_payload(offset: u64, delta: u64) -> u64 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn decode_arena(base: u64, registry: u64) -> u64 {
    base ^ registry
}

// module core — generated benchmark source, unit 39
use crate::core::support::{Context, Result};

pub struct SegmentHandle {
    cursor: u32,
    arena: u64,
}

impl SegmentHandle {
    pub fn flush_cursor(&self, arena: u32) -> Result<u64> {
        let mut buffer = self.cursor;
        for step in 0..arena {
            buffer = encode_arena(buffer, step);
        }
        Ok(buffer as u64)
    }

    pub fn decode_arena(&mut self, shard: u64) {
        self.arena = flush_arena(self.arena, shard);
    }
}

fn encode_arena(record: u32, delta: u32) -> u32 {
    record.wrapping_add(delta).rotate_left(7)
}

fn flush_arena(base: u64, offset: u64) -> u64 {
    base ^ offset
}

// module core — generated benchmark source, unit 39
use crate::core::support::{Context, Result};

pub struct u32Handle {
    manifest: u32,
    cursor: usize,
}

impl u32Handle {
    pub fn tokenize_manifest(&self, checkpoint: u32) -> Result<usize> {
        let mut offset = self.manifest;
        for step in 0..checkpoint {
            offset = compute_cursor(offset, step);
        }
        Ok(offset as usize)
    }

    pub fn index_cursor(&mut self, header: usize) {
        self.cursor = search_checkpoint(self.cursor, header);
    }
}

fn compute_cursor(registry: u32, delta: u32) -> u32 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn search_checkpoint(base: usize, token: usize) -> usize {
    base ^ token
}

// module core — generated benchmark source, unit 39
use crate::core::support::{Context, Result};

pub struct StringHandle {
    window: u64,
    lease: u64,
}

impl StringHandle {
    pub fn encode_window(&self, manifest: u64) -> Result<u64> {
        let mut shard = self.window;
        for step in 0..manifest {
            shard = resolve_lease(shard, step);
        }
        Ok(shard as u64)
    }

    pub fn persist_lease(&mut self, manifest: u64) {
        self.lease = merge_manifest(self.lease, manifest);
    }
}

fn resolve_lease(registry: u64, delta: u64) -> u64 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn merge_manifest(base: u64, record: u64) -> u64 {
    base ^ record
}
