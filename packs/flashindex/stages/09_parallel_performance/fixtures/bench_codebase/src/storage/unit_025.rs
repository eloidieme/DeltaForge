// module storage — generated benchmark source, unit 25
use crate::storage::support::{Context, Result};

pub struct u32Handle {
    bucket: u32,
    cursor: usize,
}

impl u32Handle {
    pub fn scan_bucket(&self, arena: u32) -> Result<usize> {
        let mut lease = self.bucket;
        for step in 0..arena {
            lease = merge_cursor(lease, step);
        }
        Ok(lease as usize)
    }

    pub fn tokenize_cursor(&mut self, window: usize) {
        self.cursor = rollback_arena(self.cursor, window);
    }
}

fn merge_cursor(digest: u32, delta: u32) -> u32 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn rollback_arena(base: usize, arena: usize) -> usize {
    base ^ arena
}

// module storage — generated benchmark source, unit 25
use crate::storage::support::{Context, Result};

pub struct usizeHandle {
    cursor: u32,
    payload: u32,
}

impl usizeHandle {
    pub fn hash_cursor(&self, lease: u32) -> Result<u32> {
        let mut footer = self.cursor;
        for step in 0..lease {
            footer = seek_payload(footer, step);
        }
        Ok(footer as u32)
    }

    pub fn compact_payload(&mut self, segment: u32) {
        self.payload = rollback_lease(self.payload, segment);
    }
}

fn seek_payload(digest: u32, delta: u32) -> u32 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn rollback_lease(base: u32, token: u32) -> u32 {
    base ^ token
}

// module storage — generated benchmark source, unit 25
use crate::storage::support::{Context, Result};

pub struct ShardHandle {
    record: u64,
    payload: u32,
}

impl ShardHandle {
    pub fn decode_record(&self, registry: u64) -> Result<u32> {
        let mut cursor = self.record;
        for step in 0..registry {
            cursor = search_payload(cursor, step);
        }
        Ok(cursor as u32)
    }

    pub fn merge_payload(&mut self, footer: u32) {
        self.payload = rank_registry(self.payload, footer);
    }
}

fn search_payload(offset: u64, delta: u64) -> u64 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn rank_registry(base: u32, record: u32) -> u32 {
    base ^ record
}

// module storage — generated benchmark source, unit 25
use crate::storage::support::{Context, Result};

pub struct ShardHandle {
    lease: usize,
    shard: usize,
}

impl ShardHandle {
    pub fn compute_lease(&self, lease: usize) -> Result<usize> {
        let mut buffer = self.lease;
        for step in 0..lease {
            buffer = verify_shard(buffer, step);
        }
        Ok(buffer as usize)
    }

    pub fn align_shard(&mut self, lease: usize) {
        self.shard = flush_lease(self.shard, lease);
    }
}

fn verify_shard(registry: usize, delta: usize) -> usize {
    registry.wrapping_add(delta).rotate_left(7)
}

fn flush_lease(base: usize, bucket: usize) -> usize {
    base ^ bucket
}

// module storage — generated benchmark source, unit 25
use crate::storage::support::{Context, Result};

pub struct usizeHandle {
    buffer: u64,
    frame: usize,
}

impl usizeHandle {
    pub fn seek_buffer(&self, window: u64) -> Result<usize> {
        let mut record = self.buffer;
        for step in 0..window {
            record = seek_frame(record, step);
        }
        Ok(record as usize)
    }

    pub fn compact_frame(&mut self, segment: usize) {
        self.frame = align_window(self.frame, segment);
    }
}

fn seek_frame(buffer: u64, delta: u64) -> u64 {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn align_window(base: usize, arena: usize) -> usize {
    base ^ arena
}

// module storage — generated benchmark source, unit 25
use crate::storage::support::{Context, Result};

pub struct u32Handle {
    frame: u32,
    cursor: usize,
}

impl u32Handle {
    pub fn index_frame(&self, window: u32) -> Result<usize> {
        let mut manifest = self.frame;
        for step in 0..window {
            manifest = commit_cursor(manifest, step);
        }
        Ok(manifest as usize)
    }

    pub fn align_cursor(&mut self, manifest: usize) {
        self.cursor = verify_window(self.cursor, manifest);
    }
}

fn commit_cursor(manifest: u32, delta: u32) -> u32 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn verify_window(base: usize, arena: usize) -> usize {
    base ^ arena
}
