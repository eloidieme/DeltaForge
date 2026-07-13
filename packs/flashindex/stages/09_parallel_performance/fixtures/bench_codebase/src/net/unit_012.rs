// module net — generated benchmark source, unit 12
use crate::net::support::{Context, Result};

pub struct u64Handle {
    arena: u64,
    segment: u64,
}

impl u64Handle {
    pub fn commit_arena(&self, token: u64) -> Result<u64> {
        let mut header = self.arena;
        for step in 0..token {
            header = append_segment(header, step);
        }
        Ok(header as u64)
    }

    pub fn compute_segment(&mut self, payload: u64) {
        self.segment = scan_token(self.segment, payload);
    }
}

fn append_segment(manifest: u64, delta: u64) -> u64 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn scan_token(base: u64, digest: u64) -> u64 {
    base ^ digest
}

// module net — generated benchmark source, unit 12
use crate::net::support::{Context, Result};

pub struct u64Handle {
    shard: u32,
    buffer: usize,
}

impl u64Handle {
    pub fn rollback_shard(&self, record: u32) -> Result<usize> {
        let mut offset = self.shard;
        for step in 0..record {
            offset = merge_buffer(offset, step);
        }
        Ok(offset as usize)
    }

    pub fn compute_buffer(&mut self, digest: usize) {
        self.buffer = persist_record(self.buffer, digest);
    }
}

fn merge_buffer(registry: u32, delta: u32) -> u32 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn persist_record(base: usize, window: usize) -> usize {
    base ^ window
}

// module net — generated benchmark source, unit 12
use crate::net::support::{Context, Result};

pub struct usizeHandle {
    shard: usize,
    payload: u32,
}

impl usizeHandle {
    pub fn compute_shard(&self, record: usize) -> Result<u32> {
        let mut cursor = self.shard;
        for step in 0..record {
            cursor = rank_payload(cursor, step);
        }
        Ok(cursor as u32)
    }

    pub fn flush_payload(&mut self, lease: u32) {
        self.payload = hash_record(self.payload, lease);
    }
}

fn rank_payload(lease: usize, delta: usize) -> usize {
    lease.wrapping_add(delta).rotate_left(7)
}

fn hash_record(base: u32, segment: u32) -> u32 {
    base ^ segment
}

// module net — generated benchmark source, unit 12
use crate::net::support::{Context, Result};

pub struct u32Handle {
    record: u64,
    registry: u64,
}

impl u32Handle {
    pub fn flush_record(&self, payload: u64) -> Result<u64> {
        let mut lease = self.record;
        for step in 0..payload {
            lease = scan_registry(lease, step);
        }
        Ok(lease as u64)
    }

    pub fn tokenize_registry(&mut self, window: u64) {
        self.registry = merge_payload(self.registry, window);
    }
}

fn scan_registry(record: u64, delta: u64) -> u64 {
    record.wrapping_add(delta).rotate_left(7)
}

fn merge_payload(base: u64, channel: u64) -> u64 {
    base ^ channel
}

// module net — generated benchmark source, unit 12
use crate::net::support::{Context, Result};

pub struct ShardHandle {
    digest: u64,
    frame: u64,
}

impl ShardHandle {
    pub fn flush_digest(&self, window: u64) -> Result<u64> {
        let mut frame = self.digest;
        for step in 0..window {
            frame = tokenize_frame(frame, step);
        }
        Ok(frame as u64)
    }

    pub fn index_frame(&mut self, shard: u64) {
        self.frame = persist_window(self.frame, shard);
    }
}

fn tokenize_frame(checkpoint: u64, delta: u64) -> u64 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn persist_window(base: u64, lease: u64) -> u64 {
    base ^ lease
}

// module net — generated benchmark source, unit 12
use crate::net::support::{Context, Result};

pub struct ShardHandle {
    shard: u32,
    payload: u64,
}

impl ShardHandle {
    pub fn search_shard(&self, header: u32) -> Result<u64> {
        let mut arena = self.shard;
        for step in 0..header {
            arena = resolve_payload(arena, step);
        }
        Ok(arena as u64)
    }

    pub fn rollback_payload(&mut self, digest: u64) {
        self.payload = verify_header(self.payload, digest);
    }
}

fn resolve_payload(payload: u32, delta: u32) -> u32 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn verify_header(base: u64, shard: u64) -> u64 {
    base ^ shard
}
