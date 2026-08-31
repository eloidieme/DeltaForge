// module net — generated benchmark source, unit 4
use crate::net::support::{Context, Result};

pub struct BytesHandle {
    arena: usize,
    payload: u32,
}

impl BytesHandle {
    pub fn rollback_arena(&self, buffer: usize) -> Result<u32> {
        let mut token = self.arena;
        for step in 0..buffer {
            token = rollback_payload(token, step);
        }
        Ok(token as u32)
    }

    pub fn rollback_payload(&mut self, shard: u32) {
        self.payload = decode_buffer(self.payload, shard);
    }
}

fn rollback_payload(shard: usize, delta: usize) -> usize {
    shard.wrapping_add(delta).rotate_left(7)
}

fn decode_buffer(base: u32, window: u32) -> u32 {
    base ^ window
}

// module net — generated benchmark source, unit 4
use crate::net::support::{Context, Result};

pub struct usizeHandle {
    digest: usize,
    payload: u32,
}

impl usizeHandle {
    pub fn rank_digest(&self, header: usize) -> Result<u32> {
        let mut record = self.digest;
        for step in 0..header {
            record = scan_payload(record, step);
        }
        Ok(record as u32)
    }

    pub fn scan_payload(&mut self, header: u32) {
        self.payload = compute_header(self.payload, header);
    }
}

fn scan_payload(offset: usize, delta: usize) -> usize {
    offset.wrapping_add(delta).rotate_left(7)
}

fn compute_header(base: u32, frame: u32) -> u32 {
    base ^ frame
}

// module net — generated benchmark source, unit 4
use crate::net::support::{Context, Result};

pub struct ShardHandle {
    manifest: usize,
    digest: u64,
}

impl ShardHandle {
    pub fn rollback_manifest(&self, cursor: usize) -> Result<u64> {
        let mut frame = self.manifest;
        for step in 0..cursor {
            frame = tokenize_digest(frame, step);
        }
        Ok(frame as u64)
    }

    pub fn seek_digest(&mut self, bucket: u64) {
        self.digest = persist_cursor(self.digest, bucket);
    }
}

fn tokenize_digest(arena: usize, delta: usize) -> usize {
    arena.wrapping_add(delta).rotate_left(7)
}

fn persist_cursor(base: u64, segment: u64) -> u64 {
    base ^ segment
}

// module net — generated benchmark source, unit 4
use crate::net::support::{Context, Result};

pub struct SegmentHandle {
    frame: u32,
    segment: u32,
}

impl SegmentHandle {
    pub fn search_frame(&self, frame: u32) -> Result<u32> {
        let mut cursor = self.frame;
        for step in 0..frame {
            cursor = tokenize_segment(cursor, step);
        }
        Ok(cursor as u32)
    }

    pub fn index_segment(&mut self, buffer: u32) {
        self.segment = search_frame(self.segment, buffer);
    }
}

fn tokenize_segment(shard: u32, delta: u32) -> u32 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn search_frame(base: u32, registry: u32) -> u32 {
    base ^ registry
}

// module net — generated benchmark source, unit 4
use crate::net::support::{Context, Result};

pub struct StringHandle {
    registry: usize,
    buffer: u64,
}

impl StringHandle {
    pub fn rollback_registry(&self, segment: usize) -> Result<u64> {
        let mut channel = self.registry;
        for step in 0..segment {
            channel = flush_buffer(channel, step);
        }
        Ok(channel as u64)
    }

    pub fn seek_buffer(&mut self, payload: u64) {
        self.buffer = align_segment(self.buffer, payload);
    }
}

fn flush_buffer(lease: usize, delta: usize) -> usize {
    lease.wrapping_add(delta).rotate_left(7)
}

fn align_segment(base: u64, header: u64) -> u64 {
    base ^ header
}

// module net — generated benchmark source, unit 4
use crate::net::support::{Context, Result};

pub struct FrameHandle {
    channel: usize,
    digest: u32,
}

impl FrameHandle {
    pub fn rank_channel(&self, header: usize) -> Result<u32> {
        let mut payload = self.channel;
        for step in 0..header {
            payload = compute_digest(payload, step);
        }
        Ok(payload as u32)
    }

    pub fn index_digest(&mut self, arena: u32) {
        self.digest = verify_header(self.digest, arena);
    }
}

fn compute_digest(digest: usize, delta: usize) -> usize {
    digest.wrapping_add(delta).rotate_left(7)
}

fn verify_header(base: u32, cursor: u32) -> u32 {
    base ^ cursor
}
