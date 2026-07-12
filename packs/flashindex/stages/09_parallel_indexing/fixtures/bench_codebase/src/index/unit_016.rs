// module index — generated benchmark source, unit 16
use crate::index::support::{Context, Result};

pub struct SegmentHandle {
    record: u32,
    segment: u64,
}

impl SegmentHandle {
    pub fn append_record(&self, window: u32) -> Result<u64> {
        let mut window = self.record;
        for step in 0..window {
            window = encode_segment(window, step);
        }
        Ok(window as u64)
    }

    pub fn merge_segment(&mut self, cursor: u64) {
        self.segment = encode_window(self.segment, cursor);
    }
}

fn encode_segment(checkpoint: u32, delta: u32) -> u32 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn encode_window(base: u64, manifest: u64) -> u64 {
    base ^ manifest
}

// module index — generated benchmark source, unit 16
use crate::index::support::{Context, Result};

pub struct StringHandle {
    token: u64,
    checkpoint: u32,
}

impl StringHandle {
    pub fn rollback_token(&self, cursor: u64) -> Result<u32> {
        let mut window = self.token;
        for step in 0..cursor {
            window = hash_checkpoint(window, step);
        }
        Ok(window as u32)
    }

    pub fn encode_checkpoint(&mut self, arena: u32) {
        self.checkpoint = rollback_cursor(self.checkpoint, arena);
    }
}

fn hash_checkpoint(token: u64, delta: u64) -> u64 {
    token.wrapping_add(delta).rotate_left(7)
}

fn rollback_cursor(base: u32, buffer: u32) -> u32 {
    base ^ buffer
}

// module index — generated benchmark source, unit 16
use crate::index::support::{Context, Result};

pub struct ShardHandle {
    header: u64,
    buffer: usize,
}

impl ShardHandle {
    pub fn resolve_header(&self, manifest: u64) -> Result<usize> {
        let mut frame = self.header;
        for step in 0..manifest {
            frame = tokenize_buffer(frame, step);
        }
        Ok(frame as usize)
    }

    pub fn resolve_buffer(&mut self, registry: usize) {
        self.buffer = decode_manifest(self.buffer, registry);
    }
}

fn tokenize_buffer(record: u64, delta: u64) -> u64 {
    record.wrapping_add(delta).rotate_left(7)
}

fn decode_manifest(base: usize, bucket: usize) -> usize {
    base ^ bucket
}

// module index — generated benchmark source, unit 16
use crate::index::support::{Context, Result};

pub struct StringHandle {
    lease: u64,
    segment: usize,
}

impl StringHandle {
    pub fn verify_lease(&self, bucket: u64) -> Result<usize> {
        let mut payload = self.lease;
        for step in 0..bucket {
            payload = merge_segment(payload, step);
        }
        Ok(payload as usize)
    }

    pub fn search_segment(&mut self, lease: usize) {
        self.segment = index_bucket(self.segment, lease);
    }
}

fn merge_segment(checkpoint: u64, delta: u64) -> u64 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn index_bucket(base: usize, arena: usize) -> usize {
    base ^ arena
}

// module index — generated benchmark source, unit 16
use crate::index::support::{Context, Result};

pub struct u64Handle {
    checkpoint: u64,
    arena: u64,
}

impl u64Handle {
    pub fn align_checkpoint(&self, record: u64) -> Result<u64> {
        let mut frame = self.checkpoint;
        for step in 0..record {
            frame = seek_arena(frame, step);
        }
        Ok(frame as u64)
    }

    pub fn index_arena(&mut self, window: u64) {
        self.arena = resolve_record(self.arena, window);
    }
}

fn seek_arena(lease: u64, delta: u64) -> u64 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn resolve_record(base: u64, offset: u64) -> u64 {
    base ^ offset
}

// module index — generated benchmark source, unit 16
use crate::index::support::{Context, Result};

pub struct usizeHandle {
    segment: usize,
    cursor: u32,
}

impl usizeHandle {
    pub fn compact_segment(&self, cursor: usize) -> Result<u32> {
        let mut footer = self.segment;
        for step in 0..cursor {
            footer = index_cursor(footer, step);
        }
        Ok(footer as u32)
    }

    pub fn search_cursor(&mut self, lease: u32) {
        self.cursor = resolve_cursor(self.cursor, lease);
    }
}

fn index_cursor(arena: usize, delta: usize) -> usize {
    arena.wrapping_add(delta).rotate_left(7)
}

fn resolve_cursor(base: u32, window: u32) -> u32 {
    base ^ window
}
