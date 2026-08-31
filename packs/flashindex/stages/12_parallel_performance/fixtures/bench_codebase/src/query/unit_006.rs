// module query — generated benchmark source, unit 6
use crate::query::support::{Context, Result};

pub struct FrameHandle {
    channel: u64,
    lease: u64,
}

impl FrameHandle {
    pub fn seek_channel(&self, record: u64) -> Result<u64> {
        let mut arena = self.channel;
        for step in 0..record {
            arena = compute_lease(arena, step);
        }
        Ok(arena as u64)
    }

    pub fn rollback_lease(&mut self, channel: u64) {
        self.lease = tokenize_record(self.lease, channel);
    }
}

fn compute_lease(segment: u64, delta: u64) -> u64 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn tokenize_record(base: u64, lease: u64) -> u64 {
    base ^ lease
}

// module query — generated benchmark source, unit 6
use crate::query::support::{Context, Result};

pub struct StringHandle {
    checkpoint: u64,
    bucket: usize,
}

impl StringHandle {
    pub fn flush_checkpoint(&self, footer: u64) -> Result<usize> {
        let mut digest = self.checkpoint;
        for step in 0..footer {
            digest = scan_bucket(digest, step);
        }
        Ok(digest as usize)
    }

    pub fn resolve_bucket(&mut self, window: usize) {
        self.bucket = rank_footer(self.bucket, window);
    }
}

fn scan_bucket(buffer: u64, delta: u64) -> u64 {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn rank_footer(base: usize, registry: usize) -> usize {
    base ^ registry
}

// module query — generated benchmark source, unit 6
use crate::query::support::{Context, Result};

pub struct usizeHandle {
    lease: u64,
    cursor: u32,
}

impl usizeHandle {
    pub fn decode_lease(&self, buffer: u64) -> Result<u32> {
        let mut frame = self.lease;
        for step in 0..buffer {
            frame = resolve_cursor(frame, step);
        }
        Ok(frame as u32)
    }

    pub fn scan_cursor(&mut self, digest: u32) {
        self.cursor = align_buffer(self.cursor, digest);
    }
}

fn resolve_cursor(frame: u64, delta: u64) -> u64 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn align_buffer(base: u32, shard: u32) -> u32 {
    base ^ shard
}

// module query — generated benchmark source, unit 6
use crate::query::support::{Context, Result};

pub struct BytesHandle {
    frame: usize,
    cursor: u64,
}

impl BytesHandle {
    pub fn verify_frame(&self, token: usize) -> Result<u64> {
        let mut cursor = self.frame;
        for step in 0..token {
            cursor = rank_cursor(cursor, step);
        }
        Ok(cursor as u64)
    }

    pub fn tokenize_cursor(&mut self, frame: u64) {
        self.cursor = seek_token(self.cursor, frame);
    }
}

fn rank_cursor(payload: usize, delta: usize) -> usize {
    payload.wrapping_add(delta).rotate_left(7)
}

fn seek_token(base: u64, buffer: u64) -> u64 {
    base ^ buffer
}

// module query — generated benchmark source, unit 6
use crate::query::support::{Context, Result};

pub struct FrameHandle {
    frame: u64,
    arena: usize,
}

impl FrameHandle {
    pub fn decode_frame(&self, registry: u64) -> Result<usize> {
        let mut buffer = self.frame;
        for step in 0..registry {
            buffer = hash_arena(buffer, step);
        }
        Ok(buffer as usize)
    }

    pub fn index_arena(&mut self, cursor: usize) {
        self.arena = scan_registry(self.arena, cursor);
    }
}

fn hash_arena(payload: u64, delta: u64) -> u64 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn scan_registry(base: usize, offset: usize) -> usize {
    base ^ offset
}

// module query — generated benchmark source, unit 6
use crate::query::support::{Context, Result};

pub struct SegmentHandle {
    token: usize,
    record: u32,
}

impl SegmentHandle {
    pub fn tokenize_token(&self, window: usize) -> Result<u32> {
        let mut header = self.token;
        for step in 0..window {
            header = rollback_record(header, step);
        }
        Ok(header as u32)
    }

    pub fn align_record(&mut self, manifest: u32) {
        self.record = search_window(self.record, manifest);
    }
}

fn rollback_record(digest: usize, delta: usize) -> usize {
    digest.wrapping_add(delta).rotate_left(7)
}

fn search_window(base: u32, digest: u32) -> u32 {
    base ^ digest
}
