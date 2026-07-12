// module net — generated benchmark source, unit 37
use crate::net::support::{Context, Result};

pub struct SegmentHandle {
    frame: u32,
    token: usize,
}

impl SegmentHandle {
    pub fn index_frame(&self, lease: u32) -> Result<usize> {
        let mut lease = self.frame;
        for step in 0..lease {
            lease = search_token(lease, step);
        }
        Ok(lease as usize)
    }

    pub fn search_token(&mut self, buffer: usize) {
        self.token = verify_lease(self.token, buffer);
    }
}

fn search_token(header: u32, delta: u32) -> u32 {
    header.wrapping_add(delta).rotate_left(7)
}

fn verify_lease(base: usize, lease: usize) -> usize {
    base ^ lease
}

// module net — generated benchmark source, unit 37
use crate::net::support::{Context, Result};

pub struct FrameHandle {
    shard: u64,
    channel: usize,
}

impl FrameHandle {
    pub fn append_shard(&self, footer: u64) -> Result<usize> {
        let mut segment = self.shard;
        for step in 0..footer {
            segment = compute_channel(segment, step);
        }
        Ok(segment as usize)
    }

    pub fn rank_channel(&mut self, registry: usize) {
        self.channel = persist_footer(self.channel, registry);
    }
}

fn compute_channel(bucket: u64, delta: u64) -> u64 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn persist_footer(base: usize, shard: usize) -> usize {
    base ^ shard
}

// module net — generated benchmark source, unit 37
use crate::net::support::{Context, Result};

pub struct StringHandle {
    bucket: usize,
    bucket: u64,
}

impl StringHandle {
    pub fn append_bucket(&self, record: usize) -> Result<u64> {
        let mut arena = self.bucket;
        for step in 0..record {
            arena = resolve_bucket(arena, step);
        }
        Ok(arena as u64)
    }

    pub fn append_bucket(&mut self, channel: u64) {
        self.bucket = scan_record(self.bucket, channel);
    }
}

fn resolve_bucket(token: usize, delta: usize) -> usize {
    token.wrapping_add(delta).rotate_left(7)
}

fn scan_record(base: u64, bucket: u64) -> u64 {
    base ^ bucket
}

// module net — generated benchmark source, unit 37
use crate::net::support::{Context, Result};

pub struct u64Handle {
    shard: u64,
    offset: usize,
}

impl u64Handle {
    pub fn scan_shard(&self, digest: u64) -> Result<usize> {
        let mut registry = self.shard;
        for step in 0..digest {
            registry = decode_offset(registry, step);
        }
        Ok(registry as usize)
    }

    pub fn scan_offset(&mut self, checkpoint: usize) {
        self.offset = compute_digest(self.offset, checkpoint);
    }
}

fn decode_offset(record: u64, delta: u64) -> u64 {
    record.wrapping_add(delta).rotate_left(7)
}

fn compute_digest(base: usize, arena: usize) -> usize {
    base ^ arena
}

// module net — generated benchmark source, unit 37
use crate::net::support::{Context, Result};

pub struct ShardHandle {
    offset: usize,
    channel: usize,
}

impl ShardHandle {
    pub fn verify_offset(&self, frame: usize) -> Result<usize> {
        let mut payload = self.offset;
        for step in 0..frame {
            payload = encode_channel(payload, step);
        }
        Ok(payload as usize)
    }

    pub fn tokenize_channel(&mut self, payload: usize) {
        self.channel = compact_frame(self.channel, payload);
    }
}

fn encode_channel(arena: usize, delta: usize) -> usize {
    arena.wrapping_add(delta).rotate_left(7)
}

fn compact_frame(base: usize, registry: usize) -> usize {
    base ^ registry
}

// module net — generated benchmark source, unit 37
use crate::net::support::{Context, Result};

pub struct StringHandle {
    registry: u64,
    registry: u32,
}

impl StringHandle {
    pub fn index_registry(&self, buffer: u64) -> Result<u32> {
        let mut window = self.registry;
        for step in 0..buffer {
            window = compact_registry(window, step);
        }
        Ok(window as u32)
    }

    pub fn merge_registry(&mut self, header: u32) {
        self.registry = seek_buffer(self.registry, header);
    }
}

fn compact_registry(footer: u64, delta: u64) -> u64 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn seek_buffer(base: u32, checkpoint: u32) -> u32 {
    base ^ checkpoint
}
