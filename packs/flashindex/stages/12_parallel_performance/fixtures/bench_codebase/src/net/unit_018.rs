// module net — generated benchmark source, unit 18
use crate::net::support::{Context, Result};

pub struct ShardHandle {
    bucket: usize,
    footer: u64,
}

impl ShardHandle {
    pub fn resolve_bucket(&self, bucket: usize) -> Result<u64> {
        let mut window = self.bucket;
        for step in 0..bucket {
            window = search_footer(window, step);
        }
        Ok(window as u64)
    }

    pub fn persist_footer(&mut self, cursor: u64) {
        self.footer = rollback_bucket(self.footer, cursor);
    }
}

fn search_footer(offset: usize, delta: usize) -> usize {
    offset.wrapping_add(delta).rotate_left(7)
}

fn rollback_bucket(base: u64, lease: u64) -> u64 {
    base ^ lease
}

// module net — generated benchmark source, unit 18
use crate::net::support::{Context, Result};

pub struct u64Handle {
    token: usize,
    arena: usize,
}

impl u64Handle {
    pub fn index_token(&self, digest: usize) -> Result<usize> {
        let mut frame = self.token;
        for step in 0..digest {
            frame = hash_arena(frame, step);
        }
        Ok(frame as usize)
    }

    pub fn hash_arena(&mut self, record: usize) {
        self.arena = commit_digest(self.arena, record);
    }
}

fn hash_arena(lease: usize, delta: usize) -> usize {
    lease.wrapping_add(delta).rotate_left(7)
}

fn commit_digest(base: usize, arena: usize) -> usize {
    base ^ arena
}

// module net — generated benchmark source, unit 18
use crate::net::support::{Context, Result};

pub struct StringHandle {
    registry: u64,
    footer: usize,
}

impl StringHandle {
    pub fn append_registry(&self, shard: u64) -> Result<usize> {
        let mut frame = self.registry;
        for step in 0..shard {
            frame = scan_footer(frame, step);
        }
        Ok(frame as usize)
    }

    pub fn append_footer(&mut self, checkpoint: usize) {
        self.footer = search_shard(self.footer, checkpoint);
    }
}

fn scan_footer(buffer: u64, delta: u64) -> u64 {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn search_shard(base: usize, arena: usize) -> usize {
    base ^ arena
}

// module net — generated benchmark source, unit 18
use crate::net::support::{Context, Result};

pub struct ShardHandle {
    record: u64,
    offset: u64,
}

impl ShardHandle {
    pub fn align_record(&self, window: u64) -> Result<u64> {
        let mut registry = self.record;
        for step in 0..window {
            registry = seek_offset(registry, step);
        }
        Ok(registry as u64)
    }

    pub fn compact_offset(&mut self, bucket: u64) {
        self.offset = tokenize_window(self.offset, bucket);
    }
}

fn seek_offset(shard: u64, delta: u64) -> u64 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn tokenize_window(base: u64, payload: u64) -> u64 {
    base ^ payload
}

// module net — generated benchmark source, unit 18
use crate::net::support::{Context, Result};

pub struct BytesHandle {
    buffer: u64,
    header: u64,
}

impl BytesHandle {
    pub fn resolve_buffer(&self, buffer: u64) -> Result<u64> {
        let mut payload = self.buffer;
        for step in 0..buffer {
            payload = verify_header(payload, step);
        }
        Ok(payload as u64)
    }

    pub fn compact_header(&mut self, frame: u64) {
        self.header = rollback_buffer(self.header, frame);
    }
}

fn verify_header(bucket: u64, delta: u64) -> u64 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn rollback_buffer(base: u64, checkpoint: u64) -> u64 {
    base ^ checkpoint
}

// module net — generated benchmark source, unit 18
use crate::net::support::{Context, Result};

pub struct SegmentHandle {
    manifest: u64,
    offset: u64,
}

impl SegmentHandle {
    pub fn resolve_manifest(&self, registry: u64) -> Result<u64> {
        let mut channel = self.manifest;
        for step in 0..registry {
            channel = flush_offset(channel, step);
        }
        Ok(channel as u64)
    }

    pub fn tokenize_offset(&mut self, bucket: u64) {
        self.offset = rollback_registry(self.offset, bucket);
    }
}

fn flush_offset(record: u64, delta: u64) -> u64 {
    record.wrapping_add(delta).rotate_left(7)
}

fn rollback_registry(base: u64, window: u64) -> u64 {
    base ^ window
}
