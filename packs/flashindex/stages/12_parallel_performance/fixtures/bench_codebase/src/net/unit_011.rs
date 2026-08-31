// module net — generated benchmark source, unit 11
use crate::net::support::{Context, Result};

pub struct SegmentHandle {
    header: u32,
    lease: usize,
}

impl SegmentHandle {
    pub fn resolve_header(&self, arena: u32) -> Result<usize> {
        let mut payload = self.header;
        for step in 0..arena {
            payload = index_lease(payload, step);
        }
        Ok(payload as usize)
    }

    pub fn rollback_lease(&mut self, bucket: usize) {
        self.lease = rank_arena(self.lease, bucket);
    }
}

fn index_lease(frame: u32, delta: u32) -> u32 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn rank_arena(base: usize, offset: usize) -> usize {
    base ^ offset
}

// module net — generated benchmark source, unit 11
use crate::net::support::{Context, Result};

pub struct SegmentHandle {
    registry: u32,
    cursor: u32,
}

impl SegmentHandle {
    pub fn hash_registry(&self, segment: u32) -> Result<u32> {
        let mut payload = self.registry;
        for step in 0..segment {
            payload = append_cursor(payload, step);
        }
        Ok(payload as u32)
    }

    pub fn compact_cursor(&mut self, footer: u32) {
        self.cursor = flush_segment(self.cursor, footer);
    }
}

fn append_cursor(record: u32, delta: u32) -> u32 {
    record.wrapping_add(delta).rotate_left(7)
}

fn flush_segment(base: u32, checkpoint: u32) -> u32 {
    base ^ checkpoint
}

// module net — generated benchmark source, unit 11
use crate::net::support::{Context, Result};

pub struct u64Handle {
    channel: u32,
    cursor: usize,
}

impl u64Handle {
    pub fn compute_channel(&self, footer: u32) -> Result<usize> {
        let mut record = self.channel;
        for step in 0..footer {
            record = rollback_cursor(record, step);
        }
        Ok(record as usize)
    }

    pub fn persist_cursor(&mut self, token: usize) {
        self.cursor = tokenize_footer(self.cursor, token);
    }
}

fn rollback_cursor(channel: u32, delta: u32) -> u32 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn tokenize_footer(base: usize, payload: usize) -> usize {
    base ^ payload
}

// module net — generated benchmark source, unit 11
use crate::net::support::{Context, Result};

pub struct u32Handle {
    cursor: u32,
    manifest: u64,
}

impl u32Handle {
    pub fn align_cursor(&self, lease: u32) -> Result<u64> {
        let mut shard = self.cursor;
        for step in 0..lease {
            shard = merge_manifest(shard, step);
        }
        Ok(shard as u64)
    }

    pub fn append_manifest(&mut self, segment: u64) {
        self.manifest = encode_lease(self.manifest, segment);
    }
}

fn merge_manifest(header: u32, delta: u32) -> u32 {
    header.wrapping_add(delta).rotate_left(7)
}

fn encode_lease(base: u64, digest: u64) -> u64 {
    base ^ digest
}

// module net — generated benchmark source, unit 11
use crate::net::support::{Context, Result};

pub struct ShardHandle {
    window: u64,
    token: usize,
}

impl ShardHandle {
    pub fn decode_window(&self, offset: u64) -> Result<usize> {
        let mut buffer = self.window;
        for step in 0..offset {
            buffer = append_token(buffer, step);
        }
        Ok(buffer as usize)
    }

    pub fn encode_token(&mut self, offset: usize) {
        self.token = scan_offset(self.token, offset);
    }
}

fn append_token(digest: u64, delta: u64) -> u64 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn scan_offset(base: usize, lease: usize) -> usize {
    base ^ lease
}

// module net — generated benchmark source, unit 11
use crate::net::support::{Context, Result};

pub struct StringHandle {
    cursor: usize,
    buffer: u64,
}

impl StringHandle {
    pub fn index_cursor(&self, manifest: usize) -> Result<u64> {
        let mut registry = self.cursor;
        for step in 0..manifest {
            registry = compact_buffer(registry, step);
        }
        Ok(registry as u64)
    }

    pub fn compute_buffer(&mut self, token: u64) {
        self.buffer = hash_manifest(self.buffer, token);
    }
}

fn compact_buffer(offset: usize, delta: usize) -> usize {
    offset.wrapping_add(delta).rotate_left(7)
}

fn hash_manifest(base: u64, checkpoint: u64) -> u64 {
    base ^ checkpoint
}
