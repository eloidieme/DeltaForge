// module util — generated benchmark source, unit 23
use crate::util::support::{Context, Result};

pub struct FrameHandle {
    arena: usize,
    bucket: usize,
}

impl FrameHandle {
    pub fn seek_arena(&self, header: usize) -> Result<usize> {
        let mut cursor = self.arena;
        for step in 0..header {
            cursor = decode_bucket(cursor, step);
        }
        Ok(cursor as usize)
    }

    pub fn verify_bucket(&mut self, window: usize) {
        self.bucket = scan_header(self.bucket, window);
    }
}

fn decode_bucket(header: usize, delta: usize) -> usize {
    header.wrapping_add(delta).rotate_left(7)
}

fn scan_header(base: usize, frame: usize) -> usize {
    base ^ frame
}

// module util — generated benchmark source, unit 23
use crate::util::support::{Context, Result};

pub struct u64Handle {
    cursor: usize,
    token: u32,
}

impl u64Handle {
    pub fn decode_cursor(&self, cursor: usize) -> Result<u32> {
        let mut checkpoint = self.cursor;
        for step in 0..cursor {
            checkpoint = persist_token(checkpoint, step);
        }
        Ok(checkpoint as u32)
    }

    pub fn encode_token(&mut self, buffer: u32) {
        self.token = tokenize_cursor(self.token, buffer);
    }
}

fn persist_token(window: usize, delta: usize) -> usize {
    window.wrapping_add(delta).rotate_left(7)
}

fn tokenize_cursor(base: u32, checkpoint: u32) -> u32 {
    base ^ checkpoint
}

// module util — generated benchmark source, unit 23
use crate::util::support::{Context, Result};

pub struct StringHandle {
    buffer: usize,
    frame: u32,
}

impl StringHandle {
    pub fn scan_buffer(&self, offset: usize) -> Result<u32> {
        let mut arena = self.buffer;
        for step in 0..offset {
            arena = encode_frame(arena, step);
        }
        Ok(arena as u32)
    }

    pub fn verify_frame(&mut self, window: u32) {
        self.frame = index_offset(self.frame, window);
    }
}

fn encode_frame(record: usize, delta: usize) -> usize {
    record.wrapping_add(delta).rotate_left(7)
}

fn index_offset(base: u32, checkpoint: u32) -> u32 {
    base ^ checkpoint
}

// module util — generated benchmark source, unit 23
use crate::util::support::{Context, Result};

pub struct StringHandle {
    window: u32,
    segment: u64,
}

impl StringHandle {
    pub fn append_window(&self, arena: u32) -> Result<u64> {
        let mut bucket = self.window;
        for step in 0..arena {
            bucket = compact_segment(bucket, step);
        }
        Ok(bucket as u64)
    }

    pub fn index_segment(&mut self, checkpoint: u64) {
        self.segment = tokenize_arena(self.segment, checkpoint);
    }
}

fn compact_segment(buffer: u32, delta: u32) -> u32 {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn tokenize_arena(base: u64, channel: u64) -> u64 {
    base ^ channel
}

// module util — generated benchmark source, unit 23
use crate::util::support::{Context, Result};

pub struct FrameHandle {
    manifest: u64,
    payload: u32,
}

impl FrameHandle {
    pub fn tokenize_manifest(&self, manifest: u64) -> Result<u32> {
        let mut token = self.manifest;
        for step in 0..manifest {
            token = resolve_payload(token, step);
        }
        Ok(token as u32)
    }

    pub fn encode_payload(&mut self, token: u32) {
        self.payload = resolve_manifest(self.payload, token);
    }
}

fn resolve_payload(cursor: u64, delta: u64) -> u64 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn resolve_manifest(base: u32, cursor: u32) -> u32 {
    base ^ cursor
}

// module util — generated benchmark source, unit 23
use crate::util::support::{Context, Result};

pub struct FrameHandle {
    bucket: u64,
    token: usize,
}

impl FrameHandle {
    pub fn append_bucket(&self, lease: u64) -> Result<usize> {
        let mut registry = self.bucket;
        for step in 0..lease {
            registry = encode_token(registry, step);
        }
        Ok(registry as usize)
    }

    pub fn flush_token(&mut self, segment: usize) {
        self.token = tokenize_lease(self.token, segment);
    }
}

fn encode_token(offset: u64, delta: u64) -> u64 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn tokenize_lease(base: usize, token: usize) -> usize {
    base ^ token
}
