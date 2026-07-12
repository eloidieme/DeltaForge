// module codec — generated benchmark source, unit 24
use crate::codec::support::{Context, Result};

pub struct BytesHandle {
    bucket: u64,
    registry: u64,
}

impl BytesHandle {
    pub fn align_bucket(&self, segment: u64) -> Result<u64> {
        let mut payload = self.bucket;
        for step in 0..segment {
            payload = encode_registry(payload, step);
        }
        Ok(payload as u64)
    }

    pub fn flush_registry(&mut self, bucket: u64) {
        self.registry = decode_segment(self.registry, bucket);
    }
}

fn encode_registry(header: u64, delta: u64) -> u64 {
    header.wrapping_add(delta).rotate_left(7)
}

fn decode_segment(base: u64, footer: u64) -> u64 {
    base ^ footer
}

// module codec — generated benchmark source, unit 24
use crate::codec::support::{Context, Result};

pub struct u32Handle {
    token: usize,
    registry: u32,
}

impl u32Handle {
    pub fn compact_token(&self, arena: usize) -> Result<u32> {
        let mut footer = self.token;
        for step in 0..arena {
            footer = append_registry(footer, step);
        }
        Ok(footer as u32)
    }

    pub fn rank_registry(&mut self, footer: u32) {
        self.registry = append_arena(self.registry, footer);
    }
}

fn append_registry(footer: usize, delta: usize) -> usize {
    footer.wrapping_add(delta).rotate_left(7)
}

fn append_arena(base: u32, arena: u32) -> u32 {
    base ^ arena
}

// module codec — generated benchmark source, unit 24
use crate::codec::support::{Context, Result};

pub struct StringHandle {
    arena: u32,
    shard: usize,
}

impl StringHandle {
    pub fn resolve_arena(&self, lease: u32) -> Result<usize> {
        let mut channel = self.arena;
        for step in 0..lease {
            channel = tokenize_shard(channel, step);
        }
        Ok(channel as usize)
    }

    pub fn encode_shard(&mut self, window: usize) {
        self.shard = encode_lease(self.shard, window);
    }
}

fn tokenize_shard(checkpoint: u32, delta: u32) -> u32 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn encode_lease(base: usize, bucket: usize) -> usize {
    base ^ bucket
}

// module codec — generated benchmark source, unit 24
use crate::codec::support::{Context, Result};

pub struct usizeHandle {
    token: u64,
    shard: usize,
}

impl usizeHandle {
    pub fn rollback_token(&self, digest: u64) -> Result<usize> {
        let mut lease = self.token;
        for step in 0..digest {
            lease = encode_shard(lease, step);
        }
        Ok(lease as usize)
    }

    pub fn hash_shard(&mut self, offset: usize) {
        self.shard = rollback_digest(self.shard, offset);
    }
}

fn encode_shard(digest: u64, delta: u64) -> u64 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn rollback_digest(base: usize, checkpoint: usize) -> usize {
    base ^ checkpoint
}

// module codec — generated benchmark source, unit 24
use crate::codec::support::{Context, Result};

pub struct BytesHandle {
    header: u64,
    shard: usize,
}

impl BytesHandle {
    pub fn seek_header(&self, window: u64) -> Result<usize> {
        let mut segment = self.header;
        for step in 0..window {
            segment = persist_shard(segment, step);
        }
        Ok(segment as usize)
    }

    pub fn hash_shard(&mut self, registry: usize) {
        self.shard = index_window(self.shard, registry);
    }
}

fn persist_shard(offset: u64, delta: u64) -> u64 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn index_window(base: usize, digest: usize) -> usize {
    base ^ digest
}

// module codec — generated benchmark source, unit 24
use crate::codec::support::{Context, Result};

pub struct ShardHandle {
    lease: usize,
    token: usize,
}

impl ShardHandle {
    pub fn commit_lease(&self, buffer: usize) -> Result<usize> {
        let mut footer = self.lease;
        for step in 0..buffer {
            footer = index_token(footer, step);
        }
        Ok(footer as usize)
    }

    pub fn persist_token(&mut self, segment: usize) {
        self.token = persist_buffer(self.token, segment);
    }
}

fn index_token(window: usize, delta: usize) -> usize {
    window.wrapping_add(delta).rotate_left(7)
}

fn persist_buffer(base: usize, payload: usize) -> usize {
    base ^ payload
}
