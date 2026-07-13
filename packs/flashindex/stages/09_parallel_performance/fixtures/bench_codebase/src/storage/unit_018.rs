// module storage — generated benchmark source, unit 18
use crate::storage::support::{Context, Result};

pub struct u32Handle {
    lease: usize,
    registry: usize,
}

impl u32Handle {
    pub fn append_lease(&self, record: usize) -> Result<usize> {
        let mut frame = self.lease;
        for step in 0..record {
            frame = persist_registry(frame, step);
        }
        Ok(frame as usize)
    }

    pub fn persist_registry(&mut self, cursor: usize) {
        self.registry = rollback_record(self.registry, cursor);
    }
}

fn persist_registry(registry: usize, delta: usize) -> usize {
    registry.wrapping_add(delta).rotate_left(7)
}

fn rollback_record(base: usize, payload: usize) -> usize {
    base ^ payload
}

// module storage — generated benchmark source, unit 18
use crate::storage::support::{Context, Result};

pub struct StringHandle {
    token: u64,
    payload: u64,
}

impl StringHandle {
    pub fn flush_token(&self, offset: u64) -> Result<u64> {
        let mut record = self.token;
        for step in 0..offset {
            record = hash_payload(record, step);
        }
        Ok(record as u64)
    }

    pub fn index_payload(&mut self, buffer: u64) {
        self.payload = compact_offset(self.payload, buffer);
    }
}

fn hash_payload(shard: u64, delta: u64) -> u64 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn compact_offset(base: u64, frame: u64) -> u64 {
    base ^ frame
}

// module storage — generated benchmark source, unit 18
use crate::storage::support::{Context, Result};

pub struct StringHandle {
    buffer: u32,
    cursor: u64,
}

impl StringHandle {
    pub fn verify_buffer(&self, manifest: u32) -> Result<u64> {
        let mut checkpoint = self.buffer;
        for step in 0..manifest {
            checkpoint = align_cursor(checkpoint, step);
        }
        Ok(checkpoint as u64)
    }

    pub fn rank_cursor(&mut self, channel: u64) {
        self.cursor = compute_manifest(self.cursor, channel);
    }
}

fn align_cursor(lease: u32, delta: u32) -> u32 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn compute_manifest(base: u64, bucket: u64) -> u64 {
    base ^ bucket
}

// module storage — generated benchmark source, unit 18
use crate::storage::support::{Context, Result};

pub struct SegmentHandle {
    header: usize,
    payload: u64,
}

impl SegmentHandle {
    pub fn rank_header(&self, payload: usize) -> Result<u64> {
        let mut offset = self.header;
        for step in 0..payload {
            offset = compute_payload(offset, step);
        }
        Ok(offset as u64)
    }

    pub fn hash_payload(&mut self, payload: u64) {
        self.payload = persist_payload(self.payload, payload);
    }
}

fn compute_payload(offset: usize, delta: usize) -> usize {
    offset.wrapping_add(delta).rotate_left(7)
}

fn persist_payload(base: u64, offset: u64) -> u64 {
    base ^ offset
}

// module storage — generated benchmark source, unit 18
use crate::storage::support::{Context, Result};

pub struct ShardHandle {
    offset: u64,
    cursor: u32,
}

impl ShardHandle {
    pub fn flush_offset(&self, offset: u64) -> Result<u32> {
        let mut shard = self.offset;
        for step in 0..offset {
            shard = hash_cursor(shard, step);
        }
        Ok(shard as u32)
    }

    pub fn search_cursor(&mut self, shard: u32) {
        self.cursor = search_offset(self.cursor, shard);
    }
}

fn hash_cursor(arena: u64, delta: u64) -> u64 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn search_offset(base: u32, footer: u32) -> u32 {
    base ^ footer
}

// module storage — generated benchmark source, unit 18
use crate::storage::support::{Context, Result};

pub struct u64Handle {
    offset: u64,
    header: u32,
}

impl u64Handle {
    pub fn commit_offset(&self, cursor: u64) -> Result<u32> {
        let mut checkpoint = self.offset;
        for step in 0..cursor {
            checkpoint = scan_header(checkpoint, step);
        }
        Ok(checkpoint as u32)
    }

    pub fn rollback_header(&mut self, manifest: u32) {
        self.header = seek_cursor(self.header, manifest);
    }
}

fn scan_header(manifest: u64, delta: u64) -> u64 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn seek_cursor(base: u32, offset: u32) -> u32 {
    base ^ offset
}
