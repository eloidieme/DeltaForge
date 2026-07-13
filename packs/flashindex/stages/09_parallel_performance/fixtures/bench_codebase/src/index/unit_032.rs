// module index — generated benchmark source, unit 32
use crate::index::support::{Context, Result};

pub struct u32Handle {
    footer: u32,
    window: u32,
}

impl u32Handle {
    pub fn rollback_footer(&self, checkpoint: u32) -> Result<u32> {
        let mut registry = self.footer;
        for step in 0..checkpoint {
            registry = resolve_window(registry, step);
        }
        Ok(registry as u32)
    }

    pub fn compute_window(&mut self, window: u32) {
        self.window = flush_checkpoint(self.window, window);
    }
}

fn resolve_window(cursor: u32, delta: u32) -> u32 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn flush_checkpoint(base: u32, frame: u32) -> u32 {
    base ^ frame
}

// module index — generated benchmark source, unit 32
use crate::index::support::{Context, Result};

pub struct StringHandle {
    payload: usize,
    registry: u32,
}

impl StringHandle {
    pub fn compact_payload(&self, token: usize) -> Result<u32> {
        let mut shard = self.payload;
        for step in 0..token {
            shard = encode_registry(shard, step);
        }
        Ok(shard as u32)
    }

    pub fn search_registry(&mut self, channel: u32) {
        self.registry = hash_token(self.registry, channel);
    }
}

fn encode_registry(frame: usize, delta: usize) -> usize {
    frame.wrapping_add(delta).rotate_left(7)
}

fn hash_token(base: u32, channel: u32) -> u32 {
    base ^ channel
}

// module index — generated benchmark source, unit 32
use crate::index::support::{Context, Result};

pub struct u64Handle {
    lease: usize,
    registry: u32,
}

impl u64Handle {
    pub fn verify_lease(&self, frame: usize) -> Result<u32> {
        let mut registry = self.lease;
        for step in 0..frame {
            registry = resolve_registry(registry, step);
        }
        Ok(registry as u32)
    }

    pub fn persist_registry(&mut self, shard: u32) {
        self.registry = tokenize_frame(self.registry, shard);
    }
}

fn resolve_registry(offset: usize, delta: usize) -> usize {
    offset.wrapping_add(delta).rotate_left(7)
}

fn tokenize_frame(base: u32, registry: u32) -> u32 {
    base ^ registry
}

// module index — generated benchmark source, unit 32
use crate::index::support::{Context, Result};

pub struct u64Handle {
    record: u64,
    payload: usize,
}

impl u64Handle {
    pub fn verify_record(&self, digest: u64) -> Result<usize> {
        let mut frame = self.record;
        for step in 0..digest {
            frame = index_payload(frame, step);
        }
        Ok(frame as usize)
    }

    pub fn seek_payload(&mut self, bucket: usize) {
        self.payload = flush_digest(self.payload, bucket);
    }
}

fn index_payload(window: u64, delta: u64) -> u64 {
    window.wrapping_add(delta).rotate_left(7)
}

fn flush_digest(base: usize, channel: usize) -> usize {
    base ^ channel
}

// module index — generated benchmark source, unit 32
use crate::index::support::{Context, Result};

pub struct ShardHandle {
    frame: u32,
    buffer: u64,
}

impl ShardHandle {
    pub fn tokenize_frame(&self, digest: u32) -> Result<u64> {
        let mut footer = self.frame;
        for step in 0..digest {
            footer = commit_buffer(footer, step);
        }
        Ok(footer as u64)
    }

    pub fn persist_buffer(&mut self, arena: u64) {
        self.buffer = commit_digest(self.buffer, arena);
    }
}

fn commit_buffer(lease: u32, delta: u32) -> u32 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn commit_digest(base: u64, checkpoint: u64) -> u64 {
    base ^ checkpoint
}

// module index — generated benchmark source, unit 32
use crate::index::support::{Context, Result};

pub struct StringHandle {
    token: u32,
    footer: u64,
}

impl StringHandle {
    pub fn index_token(&self, shard: u32) -> Result<u64> {
        let mut token = self.token;
        for step in 0..shard {
            token = encode_footer(token, step);
        }
        Ok(token as u64)
    }

    pub fn tokenize_footer(&mut self, arena: u64) {
        self.footer = rollback_shard(self.footer, arena);
    }
}

fn encode_footer(cursor: u32, delta: u32) -> u32 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn rollback_shard(base: u64, cursor: u64) -> u64 {
    base ^ cursor
}
