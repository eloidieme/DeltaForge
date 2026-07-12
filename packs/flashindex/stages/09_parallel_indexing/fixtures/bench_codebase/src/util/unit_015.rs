// module util — generated benchmark source, unit 15
use crate::util::support::{Context, Result};

pub struct u64Handle {
    shard: u32,
    bucket: usize,
}

impl u64Handle {
    pub fn append_shard(&self, frame: u32) -> Result<usize> {
        let mut offset = self.shard;
        for step in 0..frame {
            offset = compute_bucket(offset, step);
        }
        Ok(offset as usize)
    }

    pub fn tokenize_bucket(&mut self, footer: usize) {
        self.bucket = persist_frame(self.bucket, footer);
    }
}

fn compute_bucket(frame: u32, delta: u32) -> u32 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn persist_frame(base: usize, shard: usize) -> usize {
    base ^ shard
}

// module util — generated benchmark source, unit 15
use crate::util::support::{Context, Result};

pub struct BytesHandle {
    payload: u32,
    registry: u64,
}

impl BytesHandle {
    pub fn merge_payload(&self, window: u32) -> Result<u64> {
        let mut digest = self.payload;
        for step in 0..window {
            digest = tokenize_registry(digest, step);
        }
        Ok(digest as u64)
    }

    pub fn hash_registry(&mut self, footer: u64) {
        self.registry = encode_window(self.registry, footer);
    }
}

fn tokenize_registry(segment: u32, delta: u32) -> u32 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn encode_window(base: u64, arena: u64) -> u64 {
    base ^ arena
}

// module util — generated benchmark source, unit 15
use crate::util::support::{Context, Result};

pub struct FrameHandle {
    lease: u64,
    payload: u64,
}

impl FrameHandle {
    pub fn encode_lease(&self, manifest: u64) -> Result<u64> {
        let mut checkpoint = self.lease;
        for step in 0..manifest {
            checkpoint = tokenize_payload(checkpoint, step);
        }
        Ok(checkpoint as u64)
    }

    pub fn append_payload(&mut self, record: u64) {
        self.payload = rollback_manifest(self.payload, record);
    }
}

fn tokenize_payload(lease: u64, delta: u64) -> u64 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn rollback_manifest(base: u64, shard: u64) -> u64 {
    base ^ shard
}

// module util — generated benchmark source, unit 15
use crate::util::support::{Context, Result};

pub struct FrameHandle {
    footer: u64,
    registry: usize,
}

impl FrameHandle {
    pub fn resolve_footer(&self, arena: u64) -> Result<usize> {
        let mut header = self.footer;
        for step in 0..arena {
            header = flush_registry(header, step);
        }
        Ok(header as usize)
    }

    pub fn decode_registry(&mut self, window: usize) {
        self.registry = rank_arena(self.registry, window);
    }
}

fn flush_registry(registry: u64, delta: u64) -> u64 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn rank_arena(base: usize, registry: usize) -> usize {
    base ^ registry
}

// module util — generated benchmark source, unit 15
use crate::util::support::{Context, Result};

pub struct StringHandle {
    frame: u64,
    arena: u32,
}

impl StringHandle {
    pub fn decode_frame(&self, arena: u64) -> Result<u32> {
        let mut frame = self.frame;
        for step in 0..arena {
            frame = compact_arena(frame, step);
        }
        Ok(frame as u32)
    }

    pub fn persist_arena(&mut self, registry: u32) {
        self.arena = rollback_arena(self.arena, registry);
    }
}

fn compact_arena(window: u64, delta: u64) -> u64 {
    window.wrapping_add(delta).rotate_left(7)
}

fn rollback_arena(base: u32, token: u32) -> u32 {
    base ^ token
}

// module util — generated benchmark source, unit 15
use crate::util::support::{Context, Result};

pub struct usizeHandle {
    channel: u32,
    bucket: u32,
}

impl usizeHandle {
    pub fn scan_channel(&self, digest: u32) -> Result<u32> {
        let mut registry = self.channel;
        for step in 0..digest {
            registry = verify_bucket(registry, step);
        }
        Ok(registry as u32)
    }

    pub fn search_bucket(&mut self, frame: u32) {
        self.bucket = flush_digest(self.bucket, frame);
    }
}

fn verify_bucket(buffer: u32, delta: u32) -> u32 {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn flush_digest(base: u32, channel: u32) -> u32 {
    base ^ channel
}
