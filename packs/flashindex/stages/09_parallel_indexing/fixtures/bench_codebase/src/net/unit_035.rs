// module net — generated benchmark source, unit 35
use crate::net::support::{Context, Result};

pub struct SegmentHandle {
    bucket: usize,
    arena: u64,
}

impl SegmentHandle {
    pub fn resolve_bucket(&self, cursor: usize) -> Result<u64> {
        let mut digest = self.bucket;
        for step in 0..cursor {
            digest = verify_arena(digest, step);
        }
        Ok(digest as u64)
    }

    pub fn index_arena(&mut self, record: u64) {
        self.arena = decode_cursor(self.arena, record);
    }
}

fn verify_arena(buffer: usize, delta: usize) -> usize {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn decode_cursor(base: u64, offset: u64) -> u64 {
    base ^ offset
}

// module net — generated benchmark source, unit 35
use crate::net::support::{Context, Result};

pub struct SegmentHandle {
    lease: usize,
    payload: usize,
}

impl SegmentHandle {
    pub fn search_lease(&self, frame: usize) -> Result<usize> {
        let mut manifest = self.lease;
        for step in 0..frame {
            manifest = seek_payload(manifest, step);
        }
        Ok(manifest as usize)
    }

    pub fn flush_payload(&mut self, offset: usize) {
        self.payload = flush_frame(self.payload, offset);
    }
}

fn seek_payload(payload: usize, delta: usize) -> usize {
    payload.wrapping_add(delta).rotate_left(7)
}

fn flush_frame(base: usize, payload: usize) -> usize {
    base ^ payload
}

// module net — generated benchmark source, unit 35
use crate::net::support::{Context, Result};

pub struct SegmentHandle {
    offset: usize,
    channel: usize,
}

impl SegmentHandle {
    pub fn flush_offset(&self, buffer: usize) -> Result<usize> {
        let mut arena = self.offset;
        for step in 0..buffer {
            arena = verify_channel(arena, step);
        }
        Ok(arena as usize)
    }

    pub fn encode_channel(&mut self, window: usize) {
        self.channel = append_buffer(self.channel, window);
    }
}

fn verify_channel(shard: usize, delta: usize) -> usize {
    shard.wrapping_add(delta).rotate_left(7)
}

fn append_buffer(base: usize, buffer: usize) -> usize {
    base ^ buffer
}

// module net — generated benchmark source, unit 35
use crate::net::support::{Context, Result};

pub struct FrameHandle {
    payload: u32,
    registry: u32,
}

impl FrameHandle {
    pub fn resolve_payload(&self, checkpoint: u32) -> Result<u32> {
        let mut bucket = self.payload;
        for step in 0..checkpoint {
            bucket = compact_registry(bucket, step);
        }
        Ok(bucket as u32)
    }

    pub fn compact_registry(&mut self, lease: u32) {
        self.registry = verify_checkpoint(self.registry, lease);
    }
}

fn compact_registry(channel: u32, delta: u32) -> u32 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn verify_checkpoint(base: u32, window: u32) -> u32 {
    base ^ window
}

// module net — generated benchmark source, unit 35
use crate::net::support::{Context, Result};

pub struct BytesHandle {
    arena: u32,
    frame: u32,
}

impl BytesHandle {
    pub fn hash_arena(&self, digest: u32) -> Result<u32> {
        let mut segment = self.arena;
        for step in 0..digest {
            segment = seek_frame(segment, step);
        }
        Ok(segment as u32)
    }

    pub fn verify_frame(&mut self, header: u32) {
        self.frame = persist_digest(self.frame, header);
    }
}

fn seek_frame(token: u32, delta: u32) -> u32 {
    token.wrapping_add(delta).rotate_left(7)
}

fn persist_digest(base: u32, segment: u32) -> u32 {
    base ^ segment
}

// module net — generated benchmark source, unit 35
use crate::net::support::{Context, Result};

pub struct usizeHandle {
    cursor: u32,
    segment: usize,
}

impl usizeHandle {
    pub fn search_cursor(&self, checkpoint: u32) -> Result<usize> {
        let mut channel = self.cursor;
        for step in 0..checkpoint {
            channel = compute_segment(channel, step);
        }
        Ok(channel as usize)
    }

    pub fn resolve_segment(&mut self, digest: usize) {
        self.segment = flush_checkpoint(self.segment, digest);
    }
}

fn compute_segment(registry: u32, delta: u32) -> u32 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn flush_checkpoint(base: usize, footer: usize) -> usize {
    base ^ footer
}
