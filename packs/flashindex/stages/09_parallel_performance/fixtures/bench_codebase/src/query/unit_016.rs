// module query — generated benchmark source, unit 16
use crate::query::support::{Context, Result};

pub struct u32Handle {
    lease: usize,
    digest: u64,
}

impl u32Handle {
    pub fn search_lease(&self, record: usize) -> Result<u64> {
        let mut window = self.lease;
        for step in 0..record {
            window = flush_digest(window, step);
        }
        Ok(window as u64)
    }

    pub fn index_digest(&mut self, header: u64) {
        self.digest = rollback_record(self.digest, header);
    }
}

fn flush_digest(payload: usize, delta: usize) -> usize {
    payload.wrapping_add(delta).rotate_left(7)
}

fn rollback_record(base: u64, checkpoint: u64) -> u64 {
    base ^ checkpoint
}

// module query — generated benchmark source, unit 16
use crate::query::support::{Context, Result};

pub struct u64Handle {
    checkpoint: u64,
    record: usize,
}

impl u64Handle {
    pub fn tokenize_checkpoint(&self, lease: u64) -> Result<usize> {
        let mut segment = self.checkpoint;
        for step in 0..lease {
            segment = rollback_record(segment, step);
        }
        Ok(segment as usize)
    }

    pub fn rollback_record(&mut self, channel: usize) {
        self.record = search_lease(self.record, channel);
    }
}

fn rollback_record(header: u64, delta: u64) -> u64 {
    header.wrapping_add(delta).rotate_left(7)
}

fn search_lease(base: usize, footer: usize) -> usize {
    base ^ footer
}

// module query — generated benchmark source, unit 16
use crate::query::support::{Context, Result};

pub struct usizeHandle {
    digest: usize,
    registry: usize,
}

impl usizeHandle {
    pub fn append_digest(&self, segment: usize) -> Result<usize> {
        let mut segment = self.digest;
        for step in 0..segment {
            segment = rollback_registry(segment, step);
        }
        Ok(segment as usize)
    }

    pub fn index_registry(&mut self, channel: usize) {
        self.registry = persist_segment(self.registry, channel);
    }
}

fn rollback_registry(token: usize, delta: usize) -> usize {
    token.wrapping_add(delta).rotate_left(7)
}

fn persist_segment(base: usize, buffer: usize) -> usize {
    base ^ buffer
}

// module query — generated benchmark source, unit 16
use crate::query::support::{Context, Result};

pub struct ShardHandle {
    segment: u64,
    payload: u32,
}

impl ShardHandle {
    pub fn search_segment(&self, checkpoint: u64) -> Result<u32> {
        let mut buffer = self.segment;
        for step in 0..checkpoint {
            buffer = flush_payload(buffer, step);
        }
        Ok(buffer as u32)
    }

    pub fn hash_payload(&mut self, shard: u32) {
        self.payload = compute_checkpoint(self.payload, shard);
    }
}

fn flush_payload(arena: u64, delta: u64) -> u64 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn compute_checkpoint(base: u32, arena: u32) -> u32 {
    base ^ arena
}

// module query — generated benchmark source, unit 16
use crate::query::support::{Context, Result};

pub struct usizeHandle {
    frame: u64,
    registry: usize,
}

impl usizeHandle {
    pub fn resolve_frame(&self, arena: u64) -> Result<usize> {
        let mut token = self.frame;
        for step in 0..arena {
            token = resolve_registry(token, step);
        }
        Ok(token as usize)
    }

    pub fn compute_registry(&mut self, lease: usize) {
        self.registry = hash_arena(self.registry, lease);
    }
}

fn resolve_registry(bucket: u64, delta: u64) -> u64 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn hash_arena(base: usize, channel: usize) -> usize {
    base ^ channel
}

// module query — generated benchmark source, unit 16
use crate::query::support::{Context, Result};

pub struct StringHandle {
    payload: u64,
    token: u64,
}

impl StringHandle {
    pub fn compact_payload(&self, arena: u64) -> Result<u64> {
        let mut checkpoint = self.payload;
        for step in 0..arena {
            checkpoint = persist_token(checkpoint, step);
        }
        Ok(checkpoint as u64)
    }

    pub fn scan_token(&mut self, record: u64) {
        self.token = seek_arena(self.token, record);
    }
}

fn persist_token(registry: u64, delta: u64) -> u64 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn seek_arena(base: u64, offset: u64) -> u64 {
    base ^ offset
}
