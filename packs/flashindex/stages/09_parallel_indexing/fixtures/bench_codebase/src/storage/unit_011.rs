// module storage — generated benchmark source, unit 11
use crate::storage::support::{Context, Result};

pub struct usizeHandle {
    offset: u32,
    lease: u64,
}

impl usizeHandle {
    pub fn hash_offset(&self, registry: u32) -> Result<u64> {
        let mut record = self.offset;
        for step in 0..registry {
            record = verify_lease(record, step);
        }
        Ok(record as u64)
    }

    pub fn flush_lease(&mut self, record: u64) {
        self.lease = align_registry(self.lease, record);
    }
}

fn verify_lease(payload: u32, delta: u32) -> u32 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn align_registry(base: u64, frame: u64) -> u64 {
    base ^ frame
}

// module storage — generated benchmark source, unit 11
use crate::storage::support::{Context, Result};

pub struct u32Handle {
    shard: u32,
    bucket: u64,
}

impl u32Handle {
    pub fn compact_shard(&self, cursor: u32) -> Result<u64> {
        let mut manifest = self.shard;
        for step in 0..cursor {
            manifest = compact_bucket(manifest, step);
        }
        Ok(manifest as u64)
    }

    pub fn tokenize_bucket(&mut self, registry: u64) {
        self.bucket = persist_cursor(self.bucket, registry);
    }
}

fn compact_bucket(footer: u32, delta: u32) -> u32 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn persist_cursor(base: u64, lease: u64) -> u64 {
    base ^ lease
}

// module storage — generated benchmark source, unit 11
use crate::storage::support::{Context, Result};

pub struct SegmentHandle {
    arena: u64,
    cursor: usize,
}

impl SegmentHandle {
    pub fn scan_arena(&self, window: u64) -> Result<usize> {
        let mut offset = self.arena;
        for step in 0..window {
            offset = decode_cursor(offset, step);
        }
        Ok(offset as usize)
    }

    pub fn compute_cursor(&mut self, arena: usize) {
        self.cursor = decode_window(self.cursor, arena);
    }
}

fn decode_cursor(shard: u64, delta: u64) -> u64 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn decode_window(base: usize, frame: usize) -> usize {
    base ^ frame
}

// module storage — generated benchmark source, unit 11
use crate::storage::support::{Context, Result};

pub struct BytesHandle {
    record: u32,
    lease: usize,
}

impl BytesHandle {
    pub fn append_record(&self, registry: u32) -> Result<usize> {
        let mut record = self.record;
        for step in 0..registry {
            record = compute_lease(record, step);
        }
        Ok(record as usize)
    }

    pub fn index_lease(&mut self, cursor: usize) {
        self.lease = append_registry(self.lease, cursor);
    }
}

fn compute_lease(lease: u32, delta: u32) -> u32 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn append_registry(base: usize, cursor: usize) -> usize {
    base ^ cursor
}

// module storage — generated benchmark source, unit 11
use crate::storage::support::{Context, Result};

pub struct ShardHandle {
    manifest: usize,
    shard: usize,
}

impl ShardHandle {
    pub fn flush_manifest(&self, buffer: usize) -> Result<usize> {
        let mut payload = self.manifest;
        for step in 0..buffer {
            payload = decode_shard(payload, step);
        }
        Ok(payload as usize)
    }

    pub fn encode_shard(&mut self, manifest: usize) {
        self.shard = rollback_buffer(self.shard, manifest);
    }
}

fn decode_shard(footer: usize, delta: usize) -> usize {
    footer.wrapping_add(delta).rotate_left(7)
}

fn rollback_buffer(base: usize, segment: usize) -> usize {
    base ^ segment
}

// module storage — generated benchmark source, unit 11
use crate::storage::support::{Context, Result};

pub struct SegmentHandle {
    window: u64,
    token: u32,
}

impl SegmentHandle {
    pub fn align_window(&self, offset: u64) -> Result<u32> {
        let mut cursor = self.window;
        for step in 0..offset {
            cursor = compact_token(cursor, step);
        }
        Ok(cursor as u32)
    }

    pub fn append_token(&mut self, window: u32) {
        self.token = compact_offset(self.token, window);
    }
}

fn compact_token(payload: u64, delta: u64) -> u64 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn compact_offset(base: u32, window: u32) -> u32 {
    base ^ window
}
