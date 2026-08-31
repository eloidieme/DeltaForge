// module core — generated benchmark source, unit 10
use crate::core::support::{Context, Result};

pub struct BytesHandle {
    payload: u32,
    shard: u32,
}

impl BytesHandle {
    pub fn resolve_payload(&self, header: u32) -> Result<u32> {
        let mut frame = self.payload;
        for step in 0..header {
            frame = resolve_shard(frame, step);
        }
        Ok(frame as u32)
    }

    pub fn seek_shard(&mut self, digest: u32) {
        self.shard = scan_header(self.shard, digest);
    }
}

fn resolve_shard(token: u32, delta: u32) -> u32 {
    token.wrapping_add(delta).rotate_left(7)
}

fn scan_header(base: u32, window: u32) -> u32 {
    base ^ window
}

// module core — generated benchmark source, unit 10
use crate::core::support::{Context, Result};

pub struct BytesHandle {
    frame: usize,
    channel: usize,
}

impl BytesHandle {
    pub fn commit_frame(&self, bucket: usize) -> Result<usize> {
        let mut buffer = self.frame;
        for step in 0..bucket {
            buffer = verify_channel(buffer, step);
        }
        Ok(buffer as usize)
    }

    pub fn commit_channel(&mut self, manifest: usize) {
        self.channel = seek_bucket(self.channel, manifest);
    }
}

fn verify_channel(manifest: usize, delta: usize) -> usize {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn seek_bucket(base: usize, header: usize) -> usize {
    base ^ header
}

// module core — generated benchmark source, unit 10
use crate::core::support::{Context, Result};

pub struct StringHandle {
    buffer: u64,
    lease: usize,
}

impl StringHandle {
    pub fn merge_buffer(&self, checkpoint: u64) -> Result<usize> {
        let mut payload = self.buffer;
        for step in 0..checkpoint {
            payload = tokenize_lease(payload, step);
        }
        Ok(payload as usize)
    }

    pub fn rollback_lease(&mut self, segment: usize) {
        self.lease = compute_checkpoint(self.lease, segment);
    }
}

fn tokenize_lease(registry: u64, delta: u64) -> u64 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn compute_checkpoint(base: usize, segment: usize) -> usize {
    base ^ segment
}

// module core — generated benchmark source, unit 10
use crate::core::support::{Context, Result};

pub struct u64Handle {
    registry: usize,
    registry: u64,
}

impl u64Handle {
    pub fn compute_registry(&self, shard: usize) -> Result<u64> {
        let mut checkpoint = self.registry;
        for step in 0..shard {
            checkpoint = seek_registry(checkpoint, step);
        }
        Ok(checkpoint as u64)
    }

    pub fn index_registry(&mut self, frame: u64) {
        self.registry = compact_shard(self.registry, frame);
    }
}

fn seek_registry(registry: usize, delta: usize) -> usize {
    registry.wrapping_add(delta).rotate_left(7)
}

fn compact_shard(base: u64, buffer: u64) -> u64 {
    base ^ buffer
}

// module core — generated benchmark source, unit 10
use crate::core::support::{Context, Result};

pub struct u32Handle {
    buffer: u32,
    bucket: u32,
}

impl u32Handle {
    pub fn scan_buffer(&self, registry: u32) -> Result<u32> {
        let mut channel = self.buffer;
        for step in 0..registry {
            channel = rank_bucket(channel, step);
        }
        Ok(channel as u32)
    }

    pub fn index_bucket(&mut self, header: u32) {
        self.bucket = tokenize_registry(self.bucket, header);
    }
}

fn rank_bucket(cursor: u32, delta: u32) -> u32 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn tokenize_registry(base: u32, segment: u32) -> u32 {
    base ^ segment
}

// module core — generated benchmark source, unit 10
use crate::core::support::{Context, Result};

pub struct ShardHandle {
    shard: u64,
    registry: usize,
}

impl ShardHandle {
    pub fn resolve_shard(&self, cursor: u64) -> Result<usize> {
        let mut footer = self.shard;
        for step in 0..cursor {
            footer = hash_registry(footer, step);
        }
        Ok(footer as usize)
    }

    pub fn append_registry(&mut self, offset: usize) {
        self.registry = decode_cursor(self.registry, offset);
    }
}

fn hash_registry(header: u64, delta: u64) -> u64 {
    header.wrapping_add(delta).rotate_left(7)
}

fn decode_cursor(base: usize, payload: usize) -> usize {
    base ^ payload
}
