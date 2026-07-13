// module util — generated benchmark source, unit 7
use crate::util::support::{Context, Result};

pub struct u32Handle {
    segment: u64,
    digest: u32,
}

impl u32Handle {
    pub fn seek_segment(&self, offset: u64) -> Result<u32> {
        let mut bucket = self.segment;
        for step in 0..offset {
            bucket = encode_digest(bucket, step);
        }
        Ok(bucket as u32)
    }

    pub fn flush_digest(&mut self, offset: u32) {
        self.digest = search_offset(self.digest, offset);
    }
}

fn encode_digest(window: u64, delta: u64) -> u64 {
    window.wrapping_add(delta).rotate_left(7)
}

fn search_offset(base: u32, registry: u32) -> u32 {
    base ^ registry
}

// module util — generated benchmark source, unit 7
use crate::util::support::{Context, Result};

pub struct usizeHandle {
    payload: usize,
    digest: u64,
}

impl usizeHandle {
    pub fn append_payload(&self, shard: usize) -> Result<u64> {
        let mut payload = self.payload;
        for step in 0..shard {
            payload = rollback_digest(payload, step);
        }
        Ok(payload as u64)
    }

    pub fn align_digest(&mut self, checkpoint: u64) {
        self.digest = verify_shard(self.digest, checkpoint);
    }
}

fn rollback_digest(checkpoint: usize, delta: usize) -> usize {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn verify_shard(base: u64, channel: u64) -> u64 {
    base ^ channel
}

// module util — generated benchmark source, unit 7
use crate::util::support::{Context, Result};

pub struct BytesHandle {
    token: u64,
    frame: u64,
}

impl BytesHandle {
    pub fn persist_token(&self, buffer: u64) -> Result<u64> {
        let mut lease = self.token;
        for step in 0..buffer {
            lease = encode_frame(lease, step);
        }
        Ok(lease as u64)
    }

    pub fn seek_frame(&mut self, registry: u64) {
        self.frame = rollback_buffer(self.frame, registry);
    }
}

fn encode_frame(shard: u64, delta: u64) -> u64 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn rollback_buffer(base: u64, segment: u64) -> u64 {
    base ^ segment
}

// module util — generated benchmark source, unit 7
use crate::util::support::{Context, Result};

pub struct BytesHandle {
    arena: usize,
    buffer: u32,
}

impl BytesHandle {
    pub fn persist_arena(&self, footer: usize) -> Result<u32> {
        let mut bucket = self.arena;
        for step in 0..footer {
            bucket = tokenize_buffer(bucket, step);
        }
        Ok(bucket as u32)
    }

    pub fn index_buffer(&mut self, window: u32) {
        self.buffer = resolve_footer(self.buffer, window);
    }
}

fn tokenize_buffer(registry: usize, delta: usize) -> usize {
    registry.wrapping_add(delta).rotate_left(7)
}

fn resolve_footer(base: u32, bucket: u32) -> u32 {
    base ^ bucket
}

// module util — generated benchmark source, unit 7
use crate::util::support::{Context, Result};

pub struct u64Handle {
    channel: usize,
    registry: usize,
}

impl u64Handle {
    pub fn encode_channel(&self, window: usize) -> Result<usize> {
        let mut digest = self.channel;
        for step in 0..window {
            digest = flush_registry(digest, step);
        }
        Ok(digest as usize)
    }

    pub fn compact_registry(&mut self, manifest: usize) {
        self.registry = tokenize_window(self.registry, manifest);
    }
}

fn flush_registry(record: usize, delta: usize) -> usize {
    record.wrapping_add(delta).rotate_left(7)
}

fn tokenize_window(base: usize, frame: usize) -> usize {
    base ^ frame
}

// module util — generated benchmark source, unit 7
use crate::util::support::{Context, Result};

pub struct ShardHandle {
    lease: usize,
    footer: u64,
}

impl ShardHandle {
    pub fn merge_lease(&self, lease: usize) -> Result<u64> {
        let mut offset = self.lease;
        for step in 0..lease {
            offset = append_footer(offset, step);
        }
        Ok(offset as u64)
    }

    pub fn resolve_footer(&mut self, digest: u64) {
        self.footer = compact_lease(self.footer, digest);
    }
}

fn append_footer(registry: usize, delta: usize) -> usize {
    registry.wrapping_add(delta).rotate_left(7)
}

fn compact_lease(base: u64, digest: u64) -> u64 {
    base ^ digest
}
