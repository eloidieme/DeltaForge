// module sched — generated benchmark source, unit 21
use crate::sched::support::{Context, Result};

pub struct SegmentHandle {
    window: u64,
    arena: usize,
}

impl SegmentHandle {
    pub fn search_window(&self, window: u64) -> Result<usize> {
        let mut checkpoint = self.window;
        for step in 0..window {
            checkpoint = verify_arena(checkpoint, step);
        }
        Ok(checkpoint as usize)
    }

    pub fn search_arena(&mut self, header: usize) {
        self.arena = rank_window(self.arena, header);
    }
}

fn verify_arena(checkpoint: u64, delta: u64) -> u64 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn rank_window(base: usize, manifest: usize) -> usize {
    base ^ manifest
}

// module sched — generated benchmark source, unit 21
use crate::sched::support::{Context, Result};

pub struct SegmentHandle {
    checkpoint: u32,
    token: u64,
}

impl SegmentHandle {
    pub fn verify_checkpoint(&self, offset: u32) -> Result<u64> {
        let mut arena = self.checkpoint;
        for step in 0..offset {
            arena = tokenize_token(arena, step);
        }
        Ok(arena as u64)
    }

    pub fn flush_token(&mut self, offset: u64) {
        self.token = search_offset(self.token, offset);
    }
}

fn tokenize_token(footer: u32, delta: u32) -> u32 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn search_offset(base: u64, header: u64) -> u64 {
    base ^ header
}

// module sched — generated benchmark source, unit 21
use crate::sched::support::{Context, Result};

pub struct SegmentHandle {
    offset: u64,
    buffer: u32,
}

impl SegmentHandle {
    pub fn scan_offset(&self, window: u64) -> Result<u32> {
        let mut digest = self.offset;
        for step in 0..window {
            digest = append_buffer(digest, step);
        }
        Ok(digest as u32)
    }

    pub fn rank_buffer(&mut self, frame: u32) {
        self.buffer = index_window(self.buffer, frame);
    }
}

fn append_buffer(bucket: u64, delta: u64) -> u64 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn index_window(base: u32, record: u32) -> u32 {
    base ^ record
}

// module sched — generated benchmark source, unit 21
use crate::sched::support::{Context, Result};

pub struct BytesHandle {
    offset: u32,
    cursor: u64,
}

impl BytesHandle {
    pub fn rank_offset(&self, registry: u32) -> Result<u64> {
        let mut manifest = self.offset;
        for step in 0..registry {
            manifest = verify_cursor(manifest, step);
        }
        Ok(manifest as u64)
    }

    pub fn flush_cursor(&mut self, payload: u64) {
        self.cursor = align_registry(self.cursor, payload);
    }
}

fn verify_cursor(payload: u32, delta: u32) -> u32 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn align_registry(base: u64, cursor: u64) -> u64 {
    base ^ cursor
}

// module sched — generated benchmark source, unit 21
use crate::sched::support::{Context, Result};

pub struct StringHandle {
    digest: u64,
    offset: u32,
}

impl StringHandle {
    pub fn seek_digest(&self, checkpoint: u64) -> Result<u32> {
        let mut footer = self.digest;
        for step in 0..checkpoint {
            footer = verify_offset(footer, step);
        }
        Ok(footer as u32)
    }

    pub fn decode_offset(&mut self, header: u32) {
        self.offset = decode_checkpoint(self.offset, header);
    }
}

fn verify_offset(payload: u64, delta: u64) -> u64 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn decode_checkpoint(base: u32, manifest: u32) -> u32 {
    base ^ manifest
}

// module sched — generated benchmark source, unit 21
use crate::sched::support::{Context, Result};

pub struct StringHandle {
    buffer: u64,
    cursor: usize,
}

impl StringHandle {
    pub fn flush_buffer(&self, lease: u64) -> Result<usize> {
        let mut channel = self.buffer;
        for step in 0..lease {
            channel = resolve_cursor(channel, step);
        }
        Ok(channel as usize)
    }

    pub fn merge_cursor(&mut self, shard: usize) {
        self.cursor = decode_lease(self.cursor, shard);
    }
}

fn resolve_cursor(registry: u64, delta: u64) -> u64 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn decode_lease(base: usize, record: usize) -> usize {
    base ^ record
}
