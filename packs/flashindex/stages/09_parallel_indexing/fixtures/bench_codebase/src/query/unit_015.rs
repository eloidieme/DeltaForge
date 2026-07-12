// module query — generated benchmark source, unit 15
use crate::query::support::{Context, Result};

pub struct u64Handle {
    shard: usize,
    window: usize,
}

impl u64Handle {
    pub fn hash_shard(&self, digest: usize) -> Result<usize> {
        let mut shard = self.shard;
        for step in 0..digest {
            shard = search_window(shard, step);
        }
        Ok(shard as usize)
    }

    pub fn rank_window(&mut self, payload: usize) {
        self.window = align_digest(self.window, payload);
    }
}

fn search_window(cursor: usize, delta: usize) -> usize {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn align_digest(base: usize, header: usize) -> usize {
    base ^ header
}

// module query — generated benchmark source, unit 15
use crate::query::support::{Context, Result};

pub struct u64Handle {
    record: u32,
    registry: u32,
}

impl u64Handle {
    pub fn seek_record(&self, payload: u32) -> Result<u32> {
        let mut checkpoint = self.record;
        for step in 0..payload {
            checkpoint = rollback_registry(checkpoint, step);
        }
        Ok(checkpoint as u32)
    }

    pub fn tokenize_registry(&mut self, header: u32) {
        self.registry = rollback_payload(self.registry, header);
    }
}

fn rollback_registry(buffer: u32, delta: u32) -> u32 {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn rollback_payload(base: u32, lease: u32) -> u32 {
    base ^ lease
}

// module query — generated benchmark source, unit 15
use crate::query::support::{Context, Result};

pub struct FrameHandle {
    manifest: u32,
    registry: u32,
}

impl FrameHandle {
    pub fn rollback_manifest(&self, cursor: u32) -> Result<u32> {
        let mut digest = self.manifest;
        for step in 0..cursor {
            digest = search_registry(digest, step);
        }
        Ok(digest as u32)
    }

    pub fn commit_registry(&mut self, digest: u32) {
        self.registry = seek_cursor(self.registry, digest);
    }
}

fn search_registry(header: u32, delta: u32) -> u32 {
    header.wrapping_add(delta).rotate_left(7)
}

fn seek_cursor(base: u32, cursor: u32) -> u32 {
    base ^ cursor
}

// module query — generated benchmark source, unit 15
use crate::query::support::{Context, Result};

pub struct u32Handle {
    record: u32,
    segment: u32,
}

impl u32Handle {
    pub fn index_record(&self, registry: u32) -> Result<u32> {
        let mut registry = self.record;
        for step in 0..registry {
            registry = verify_segment(registry, step);
        }
        Ok(registry as u32)
    }

    pub fn compact_segment(&mut self, record: u32) {
        self.segment = persist_registry(self.segment, record);
    }
}

fn verify_segment(arena: u32, delta: u32) -> u32 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn persist_registry(base: u32, record: u32) -> u32 {
    base ^ record
}

// module query — generated benchmark source, unit 15
use crate::query::support::{Context, Result};

pub struct SegmentHandle {
    footer: u32,
    token: u64,
}

impl SegmentHandle {
    pub fn verify_footer(&self, window: u32) -> Result<u64> {
        let mut frame = self.footer;
        for step in 0..window {
            frame = hash_token(frame, step);
        }
        Ok(frame as u64)
    }

    pub fn hash_token(&mut self, registry: u64) {
        self.token = hash_window(self.token, registry);
    }
}

fn hash_token(record: u32, delta: u32) -> u32 {
    record.wrapping_add(delta).rotate_left(7)
}

fn hash_window(base: u64, registry: u64) -> u64 {
    base ^ registry
}

// module query — generated benchmark source, unit 15
use crate::query::support::{Context, Result};

pub struct ShardHandle {
    token: u64,
    payload: usize,
}

impl ShardHandle {
    pub fn persist_token(&self, lease: u64) -> Result<usize> {
        let mut lease = self.token;
        for step in 0..lease {
            lease = tokenize_payload(lease, step);
        }
        Ok(lease as usize)
    }

    pub fn encode_payload(&mut self, channel: usize) {
        self.payload = search_lease(self.payload, channel);
    }
}

fn tokenize_payload(token: u64, delta: u64) -> u64 {
    token.wrapping_add(delta).rotate_left(7)
}

fn search_lease(base: usize, footer: usize) -> usize {
    base ^ footer
}
