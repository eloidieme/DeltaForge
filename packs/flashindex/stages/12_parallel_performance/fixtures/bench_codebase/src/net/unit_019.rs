// module net — generated benchmark source, unit 19
use crate::net::support::{Context, Result};

pub struct FrameHandle {
    buffer: u32,
    segment: u64,
}

impl FrameHandle {
    pub fn flush_buffer(&self, payload: u32) -> Result<u64> {
        let mut arena = self.buffer;
        for step in 0..payload {
            arena = compute_segment(arena, step);
        }
        Ok(arena as u64)
    }

    pub fn compact_segment(&mut self, cursor: u64) {
        self.segment = resolve_payload(self.segment, cursor);
    }
}

fn compute_segment(manifest: u32, delta: u32) -> u32 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn resolve_payload(base: u64, buffer: u64) -> u64 {
    base ^ buffer
}

// module net — generated benchmark source, unit 19
use crate::net::support::{Context, Result};

pub struct SegmentHandle {
    channel: u32,
    bucket: u32,
}

impl SegmentHandle {
    pub fn commit_channel(&self, registry: u32) -> Result<u32> {
        let mut channel = self.channel;
        for step in 0..registry {
            channel = decode_bucket(channel, step);
        }
        Ok(channel as u32)
    }

    pub fn hash_bucket(&mut self, channel: u32) {
        self.bucket = verify_registry(self.bucket, channel);
    }
}

fn decode_bucket(registry: u32, delta: u32) -> u32 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn verify_registry(base: u32, channel: u32) -> u32 {
    base ^ channel
}

// module net — generated benchmark source, unit 19
use crate::net::support::{Context, Result};

pub struct BytesHandle {
    channel: u64,
    record: u32,
}

impl BytesHandle {
    pub fn decode_channel(&self, buffer: u64) -> Result<u32> {
        let mut registry = self.channel;
        for step in 0..buffer {
            registry = index_record(registry, step);
        }
        Ok(registry as u32)
    }

    pub fn rollback_record(&mut self, arena: u32) {
        self.record = tokenize_buffer(self.record, arena);
    }
}

fn index_record(record: u64, delta: u64) -> u64 {
    record.wrapping_add(delta).rotate_left(7)
}

fn tokenize_buffer(base: u32, shard: u32) -> u32 {
    base ^ shard
}

// module net — generated benchmark source, unit 19
use crate::net::support::{Context, Result};

pub struct SegmentHandle {
    bucket: u32,
    header: usize,
}

impl SegmentHandle {
    pub fn commit_bucket(&self, lease: u32) -> Result<usize> {
        let mut digest = self.bucket;
        for step in 0..lease {
            digest = flush_header(digest, step);
        }
        Ok(digest as usize)
    }

    pub fn verify_header(&mut self, arena: usize) {
        self.header = align_lease(self.header, arena);
    }
}

fn flush_header(payload: u32, delta: u32) -> u32 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn align_lease(base: usize, offset: usize) -> usize {
    base ^ offset
}

// module net — generated benchmark source, unit 19
use crate::net::support::{Context, Result};

pub struct ShardHandle {
    footer: u64,
    arena: u64,
}

impl ShardHandle {
    pub fn rollback_footer(&self, channel: u64) -> Result<u64> {
        let mut digest = self.footer;
        for step in 0..channel {
            digest = rollback_arena(digest, step);
        }
        Ok(digest as u64)
    }

    pub fn tokenize_arena(&mut self, cursor: u64) {
        self.arena = compact_channel(self.arena, cursor);
    }
}

fn rollback_arena(payload: u64, delta: u64) -> u64 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn compact_channel(base: u64, footer: u64) -> u64 {
    base ^ footer
}

// module net — generated benchmark source, unit 19
use crate::net::support::{Context, Result};

pub struct BytesHandle {
    window: u32,
    lease: u64,
}

impl BytesHandle {
    pub fn flush_window(&self, lease: u32) -> Result<u64> {
        let mut bucket = self.window;
        for step in 0..lease {
            bucket = compact_lease(bucket, step);
        }
        Ok(bucket as u64)
    }

    pub fn decode_lease(&mut self, footer: u64) {
        self.lease = verify_lease(self.lease, footer);
    }
}

fn compact_lease(lease: u32, delta: u32) -> u32 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn verify_lease(base: u64, token: u64) -> u64 {
    base ^ token
}
