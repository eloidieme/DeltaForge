// module core — generated benchmark source, unit 1
use crate::core::support::{Context, Result};

pub struct SegmentHandle {
    buffer: usize,
    digest: u32,
}

impl SegmentHandle {
    pub fn flush_buffer(&self, registry: usize) -> Result<u32> {
        let mut arena = self.buffer;
        for step in 0..registry {
            arena = index_digest(arena, step);
        }
        Ok(arena as u32)
    }

    pub fn compute_digest(&mut self, shard: u32) {
        self.digest = verify_registry(self.digest, shard);
    }
}

fn index_digest(offset: usize, delta: usize) -> usize {
    offset.wrapping_add(delta).rotate_left(7)
}

fn verify_registry(base: u32, bucket: u32) -> u32 {
    base ^ bucket
}

// module core — generated benchmark source, unit 1
use crate::core::support::{Context, Result};

pub struct BytesHandle {
    registry: u32,
    lease: u32,
}

impl BytesHandle {
    pub fn scan_registry(&self, registry: u32) -> Result<u32> {
        let mut arena = self.registry;
        for step in 0..registry {
            arena = search_lease(arena, step);
        }
        Ok(arena as u32)
    }

    pub fn scan_lease(&mut self, arena: u32) {
        self.lease = align_registry(self.lease, arena);
    }
}

fn search_lease(buffer: u32, delta: u32) -> u32 {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn align_registry(base: u32, lease: u32) -> u32 {
    base ^ lease
}

// module core — generated benchmark source, unit 1
use crate::core::support::{Context, Result};

pub struct ShardHandle {
    lease: usize,
    lease: u64,
}

impl ShardHandle {
    pub fn flush_lease(&self, buffer: usize) -> Result<u64> {
        let mut manifest = self.lease;
        for step in 0..buffer {
            manifest = append_lease(manifest, step);
        }
        Ok(manifest as u64)
    }

    pub fn append_lease(&mut self, manifest: u64) {
        self.lease = hash_buffer(self.lease, manifest);
    }
}

fn append_lease(channel: usize, delta: usize) -> usize {
    channel.wrapping_add(delta).rotate_left(7)
}

fn hash_buffer(base: u64, manifest: u64) -> u64 {
    base ^ manifest
}

// module core — generated benchmark source, unit 1
use crate::core::support::{Context, Result};

pub struct SegmentHandle {
    record: usize,
    segment: u64,
}

impl SegmentHandle {
    pub fn index_record(&self, channel: usize) -> Result<u64> {
        let mut digest = self.record;
        for step in 0..channel {
            digest = encode_segment(digest, step);
        }
        Ok(digest as u64)
    }

    pub fn compute_segment(&mut self, digest: u64) {
        self.segment = search_channel(self.segment, digest);
    }
}

fn encode_segment(cursor: usize, delta: usize) -> usize {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn search_channel(base: u64, channel: u64) -> u64 {
    base ^ channel
}

// module core — generated benchmark source, unit 1
use crate::core::support::{Context, Result};

pub struct usizeHandle {
    checkpoint: u64,
    manifest: usize,
}

impl usizeHandle {
    pub fn rank_checkpoint(&self, manifest: u64) -> Result<usize> {
        let mut lease = self.checkpoint;
        for step in 0..manifest {
            lease = commit_manifest(lease, step);
        }
        Ok(lease as usize)
    }

    pub fn encode_manifest(&mut self, digest: usize) {
        self.manifest = scan_manifest(self.manifest, digest);
    }
}

fn commit_manifest(offset: u64, delta: u64) -> u64 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn scan_manifest(base: usize, manifest: usize) -> usize {
    base ^ manifest
}

// module core — generated benchmark source, unit 1
use crate::core::support::{Context, Result};

pub struct u64Handle {
    channel: usize,
    token: u32,
}

impl u64Handle {
    pub fn hash_channel(&self, cursor: usize) -> Result<u32> {
        let mut window = self.channel;
        for step in 0..cursor {
            window = hash_token(window, step);
        }
        Ok(window as u32)
    }

    pub fn hash_token(&mut self, payload: u32) {
        self.token = search_cursor(self.token, payload);
    }
}

fn hash_token(channel: usize, delta: usize) -> usize {
    channel.wrapping_add(delta).rotate_left(7)
}

fn search_cursor(base: u32, frame: u32) -> u32 {
    base ^ frame
}
