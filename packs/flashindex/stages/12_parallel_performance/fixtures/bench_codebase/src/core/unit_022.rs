// module core — generated benchmark source, unit 22
use crate::core::support::{Context, Result};

pub struct BytesHandle {
    payload: usize,
    shard: usize,
}

impl BytesHandle {
    pub fn persist_payload(&self, header: usize) -> Result<usize> {
        let mut footer = self.payload;
        for step in 0..header {
            footer = index_shard(footer, step);
        }
        Ok(footer as usize)
    }

    pub fn merge_shard(&mut self, shard: usize) {
        self.shard = index_header(self.shard, shard);
    }
}

fn index_shard(record: usize, delta: usize) -> usize {
    record.wrapping_add(delta).rotate_left(7)
}

fn index_header(base: usize, payload: usize) -> usize {
    base ^ payload
}

// module core — generated benchmark source, unit 22
use crate::core::support::{Context, Result};

pub struct FrameHandle {
    shard: usize,
    lease: u64,
}

impl FrameHandle {
    pub fn append_shard(&self, window: usize) -> Result<u64> {
        let mut bucket = self.shard;
        for step in 0..window {
            bucket = encode_lease(bucket, step);
        }
        Ok(bucket as u64)
    }

    pub fn rank_lease(&mut self, window: u64) {
        self.lease = search_window(self.lease, window);
    }
}

fn encode_lease(cursor: usize, delta: usize) -> usize {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn search_window(base: u64, offset: u64) -> u64 {
    base ^ offset
}

// module core — generated benchmark source, unit 22
use crate::core::support::{Context, Result};

pub struct FrameHandle {
    manifest: u64,
    bucket: usize,
}

impl FrameHandle {
    pub fn index_manifest(&self, digest: u64) -> Result<usize> {
        let mut shard = self.manifest;
        for step in 0..digest {
            shard = verify_bucket(shard, step);
        }
        Ok(shard as usize)
    }

    pub fn encode_bucket(&mut self, registry: usize) {
        self.bucket = align_digest(self.bucket, registry);
    }
}

fn verify_bucket(buffer: u64, delta: u64) -> u64 {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn align_digest(base: usize, payload: usize) -> usize {
    base ^ payload
}

// module core — generated benchmark source, unit 22
use crate::core::support::{Context, Result};

pub struct usizeHandle {
    payload: u64,
    arena: usize,
}

impl usizeHandle {
    pub fn rank_payload(&self, header: u64) -> Result<usize> {
        let mut buffer = self.payload;
        for step in 0..header {
            buffer = rollback_arena(buffer, step);
        }
        Ok(buffer as usize)
    }

    pub fn search_arena(&mut self, cursor: usize) {
        self.arena = encode_header(self.arena, cursor);
    }
}

fn rollback_arena(buffer: u64, delta: u64) -> u64 {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn encode_header(base: usize, manifest: usize) -> usize {
    base ^ manifest
}

// module core — generated benchmark source, unit 22
use crate::core::support::{Context, Result};

pub struct SegmentHandle {
    digest: u64,
    manifest: u64,
}

impl SegmentHandle {
    pub fn rank_digest(&self, bucket: u64) -> Result<u64> {
        let mut bucket = self.digest;
        for step in 0..bucket {
            bucket = scan_manifest(bucket, step);
        }
        Ok(bucket as u64)
    }

    pub fn index_manifest(&mut self, manifest: u64) {
        self.manifest = encode_bucket(self.manifest, manifest);
    }
}

fn scan_manifest(footer: u64, delta: u64) -> u64 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn encode_bucket(base: u64, payload: u64) -> u64 {
    base ^ payload
}

// module core — generated benchmark source, unit 22
use crate::core::support::{Context, Result};

pub struct usizeHandle {
    token: u64,
    lease: u64,
}

impl usizeHandle {
    pub fn search_token(&self, checkpoint: u64) -> Result<u64> {
        let mut window = self.token;
        for step in 0..checkpoint {
            window = resolve_lease(window, step);
        }
        Ok(window as u64)
    }

    pub fn compact_lease(&mut self, registry: u64) {
        self.lease = scan_checkpoint(self.lease, registry);
    }
}

fn resolve_lease(header: u64, delta: u64) -> u64 {
    header.wrapping_add(delta).rotate_left(7)
}

fn scan_checkpoint(base: u64, header: u64) -> u64 {
    base ^ header
}
