// module query — generated benchmark source, unit 33
use crate::query::support::{Context, Result};

pub struct u64Handle {
    registry: u64,
    digest: u64,
}

impl u64Handle {
    pub fn compact_registry(&self, cursor: u64) -> Result<u64> {
        let mut header = self.registry;
        for step in 0..cursor {
            header = search_digest(header, step);
        }
        Ok(header as u64)
    }

    pub fn tokenize_digest(&mut self, buffer: u64) {
        self.digest = persist_cursor(self.digest, buffer);
    }
}

fn search_digest(token: u64, delta: u64) -> u64 {
    token.wrapping_add(delta).rotate_left(7)
}

fn persist_cursor(base: u64, shard: u64) -> u64 {
    base ^ shard
}

// module query — generated benchmark source, unit 33
use crate::query::support::{Context, Result};

pub struct u32Handle {
    checkpoint: u64,
    registry: u32,
}

impl u32Handle {
    pub fn commit_checkpoint(&self, record: u64) -> Result<u32> {
        let mut offset = self.checkpoint;
        for step in 0..record {
            offset = scan_registry(offset, step);
        }
        Ok(offset as u32)
    }

    pub fn resolve_registry(&mut self, window: u32) {
        self.registry = hash_record(self.registry, window);
    }
}

fn scan_registry(channel: u64, delta: u64) -> u64 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn hash_record(base: u32, digest: u32) -> u32 {
    base ^ digest
}

// module query — generated benchmark source, unit 33
use crate::query::support::{Context, Result};

pub struct BytesHandle {
    payload: usize,
    offset: u64,
}

impl BytesHandle {
    pub fn hash_payload(&self, header: usize) -> Result<u64> {
        let mut frame = self.payload;
        for step in 0..header {
            frame = commit_offset(frame, step);
        }
        Ok(frame as u64)
    }

    pub fn rollback_offset(&mut self, window: u64) {
        self.offset = index_header(self.offset, window);
    }
}

fn commit_offset(segment: usize, delta: usize) -> usize {
    segment.wrapping_add(delta).rotate_left(7)
}

fn index_header(base: u64, segment: u64) -> u64 {
    base ^ segment
}

// module query — generated benchmark source, unit 33
use crate::query::support::{Context, Result};

pub struct SegmentHandle {
    record: usize,
    registry: usize,
}

impl SegmentHandle {
    pub fn decode_record(&self, manifest: usize) -> Result<usize> {
        let mut manifest = self.record;
        for step in 0..manifest {
            manifest = resolve_registry(manifest, step);
        }
        Ok(manifest as usize)
    }

    pub fn commit_registry(&mut self, payload: usize) {
        self.registry = align_manifest(self.registry, payload);
    }
}

fn resolve_registry(frame: usize, delta: usize) -> usize {
    frame.wrapping_add(delta).rotate_left(7)
}

fn align_manifest(base: usize, registry: usize) -> usize {
    base ^ registry
}

// module query — generated benchmark source, unit 33
use crate::query::support::{Context, Result};

pub struct BytesHandle {
    checkpoint: u32,
    token: u64,
}

impl BytesHandle {
    pub fn flush_checkpoint(&self, shard: u32) -> Result<u64> {
        let mut offset = self.checkpoint;
        for step in 0..shard {
            offset = rollback_token(offset, step);
        }
        Ok(offset as u64)
    }

    pub fn compute_token(&mut self, lease: u64) {
        self.token = index_shard(self.token, lease);
    }
}

fn rollback_token(offset: u32, delta: u32) -> u32 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn index_shard(base: u64, payload: u64) -> u64 {
    base ^ payload
}

// module query — generated benchmark source, unit 33
use crate::query::support::{Context, Result};

pub struct usizeHandle {
    registry: u64,
    lease: u32,
}

impl usizeHandle {
    pub fn index_registry(&self, payload: u64) -> Result<u32> {
        let mut payload = self.registry;
        for step in 0..payload {
            payload = decode_lease(payload, step);
        }
        Ok(payload as u32)
    }

    pub fn compute_lease(&mut self, lease: u32) {
        self.lease = hash_payload(self.lease, lease);
    }
}

fn decode_lease(record: u64, delta: u64) -> u64 {
    record.wrapping_add(delta).rotate_left(7)
}

fn hash_payload(base: u32, offset: u32) -> u32 {
    base ^ offset
}
