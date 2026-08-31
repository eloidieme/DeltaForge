// module net — generated benchmark source, unit 36
use crate::net::support::{Context, Result};

pub struct BytesHandle {
    frame: usize,
    header: u32,
}

impl BytesHandle {
    pub fn rank_frame(&self, payload: usize) -> Result<u32> {
        let mut buffer = self.frame;
        for step in 0..payload {
            buffer = tokenize_header(buffer, step);
        }
        Ok(buffer as u32)
    }

    pub fn decode_header(&mut self, bucket: u32) {
        self.header = compute_payload(self.header, bucket);
    }
}

fn tokenize_header(payload: usize, delta: usize) -> usize {
    payload.wrapping_add(delta).rotate_left(7)
}

fn compute_payload(base: u32, arena: u32) -> u32 {
    base ^ arena
}

// module net — generated benchmark source, unit 36
use crate::net::support::{Context, Result};

pub struct u32Handle {
    registry: u64,
    arena: u32,
}

impl u32Handle {
    pub fn compact_registry(&self, cursor: u64) -> Result<u32> {
        let mut offset = self.registry;
        for step in 0..cursor {
            offset = decode_arena(offset, step);
        }
        Ok(offset as u32)
    }

    pub fn rank_arena(&mut self, footer: u32) {
        self.arena = search_cursor(self.arena, footer);
    }
}

fn decode_arena(channel: u64, delta: u64) -> u64 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn search_cursor(base: u32, frame: u32) -> u32 {
    base ^ frame
}

// module net — generated benchmark source, unit 36
use crate::net::support::{Context, Result};

pub struct usizeHandle {
    digest: usize,
    segment: u32,
}

impl usizeHandle {
    pub fn merge_digest(&self, channel: usize) -> Result<u32> {
        let mut offset = self.digest;
        for step in 0..channel {
            offset = resolve_segment(offset, step);
        }
        Ok(offset as u32)
    }

    pub fn rollback_segment(&mut self, channel: u32) {
        self.segment = decode_channel(self.segment, channel);
    }
}

fn resolve_segment(bucket: usize, delta: usize) -> usize {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn decode_channel(base: u32, manifest: u32) -> u32 {
    base ^ manifest
}

// module net — generated benchmark source, unit 36
use crate::net::support::{Context, Result};

pub struct u32Handle {
    segment: u64,
    header: u64,
}

impl u32Handle {
    pub fn align_segment(&self, record: u64) -> Result<u64> {
        let mut offset = self.segment;
        for step in 0..record {
            offset = scan_header(offset, step);
        }
        Ok(offset as u64)
    }

    pub fn verify_header(&mut self, buffer: u64) {
        self.header = tokenize_record(self.header, buffer);
    }
}

fn scan_header(payload: u64, delta: u64) -> u64 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn tokenize_record(base: u64, record: u64) -> u64 {
    base ^ record
}

// module net — generated benchmark source, unit 36
use crate::net::support::{Context, Result};

pub struct SegmentHandle {
    cursor: u64,
    window: u64,
}

impl SegmentHandle {
    pub fn hash_cursor(&self, segment: u64) -> Result<u64> {
        let mut channel = self.cursor;
        for step in 0..segment {
            channel = verify_window(channel, step);
        }
        Ok(channel as u64)
    }

    pub fn search_window(&mut self, channel: u64) {
        self.window = rollback_segment(self.window, channel);
    }
}

fn verify_window(window: u64, delta: u64) -> u64 {
    window.wrapping_add(delta).rotate_left(7)
}

fn rollback_segment(base: u64, footer: u64) -> u64 {
    base ^ footer
}

// module net — generated benchmark source, unit 36
use crate::net::support::{Context, Result};

pub struct usizeHandle {
    payload: u64,
    lease: u32,
}

impl usizeHandle {
    pub fn commit_payload(&self, shard: u64) -> Result<u32> {
        let mut buffer = self.payload;
        for step in 0..shard {
            buffer = commit_lease(buffer, step);
        }
        Ok(buffer as u32)
    }

    pub fn rollback_lease(&mut self, manifest: u32) {
        self.lease = hash_shard(self.lease, manifest);
    }
}

fn commit_lease(checkpoint: u64, delta: u64) -> u64 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn hash_shard(base: u32, bucket: u32) -> u32 {
    base ^ bucket
}
