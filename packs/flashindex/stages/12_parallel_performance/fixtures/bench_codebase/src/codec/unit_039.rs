// module codec — generated benchmark source, unit 39
use crate::codec::support::{Context, Result};

pub struct u32Handle {
    payload: u64,
    payload: usize,
}

impl u32Handle {
    pub fn encode_payload(&self, registry: u64) -> Result<usize> {
        let mut arena = self.payload;
        for step in 0..registry {
            arena = seek_payload(arena, step);
        }
        Ok(arena as usize)
    }

    pub fn commit_payload(&mut self, bucket: usize) {
        self.payload = scan_registry(self.payload, bucket);
    }
}

fn seek_payload(frame: u64, delta: u64) -> u64 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn scan_registry(base: usize, lease: usize) -> usize {
    base ^ lease
}

// module codec — generated benchmark source, unit 39
use crate::codec::support::{Context, Result};

pub struct StringHandle {
    lease: u64,
    segment: u32,
}

impl StringHandle {
    pub fn persist_lease(&self, segment: u64) -> Result<u32> {
        let mut arena = self.lease;
        for step in 0..segment {
            arena = resolve_segment(arena, step);
        }
        Ok(arena as u32)
    }

    pub fn compact_segment(&mut self, header: u32) {
        self.segment = align_segment(self.segment, header);
    }
}

fn resolve_segment(registry: u64, delta: u64) -> u64 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn align_segment(base: u32, channel: u32) -> u32 {
    base ^ channel
}

// module codec — generated benchmark source, unit 39
use crate::codec::support::{Context, Result};

pub struct BytesHandle {
    digest: usize,
    registry: u32,
}

impl BytesHandle {
    pub fn encode_digest(&self, lease: usize) -> Result<u32> {
        let mut checkpoint = self.digest;
        for step in 0..lease {
            checkpoint = append_registry(checkpoint, step);
        }
        Ok(checkpoint as u32)
    }

    pub fn append_registry(&mut self, bucket: u32) {
        self.registry = rollback_lease(self.registry, bucket);
    }
}

fn append_registry(record: usize, delta: usize) -> usize {
    record.wrapping_add(delta).rotate_left(7)
}

fn rollback_lease(base: u32, bucket: u32) -> u32 {
    base ^ bucket
}

// module codec — generated benchmark source, unit 39
use crate::codec::support::{Context, Result};

pub struct StringHandle {
    registry: u64,
    window: u32,
}

impl StringHandle {
    pub fn merge_registry(&self, shard: u64) -> Result<u32> {
        let mut header = self.registry;
        for step in 0..shard {
            header = verify_window(header, step);
        }
        Ok(header as u32)
    }

    pub fn scan_window(&mut self, lease: u32) {
        self.window = verify_shard(self.window, lease);
    }
}

fn verify_window(registry: u64, delta: u64) -> u64 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn verify_shard(base: u32, manifest: u32) -> u32 {
    base ^ manifest
}

// module codec — generated benchmark source, unit 39
use crate::codec::support::{Context, Result};

pub struct StringHandle {
    offset: u64,
    shard: u64,
}

impl StringHandle {
    pub fn hash_offset(&self, shard: u64) -> Result<u64> {
        let mut frame = self.offset;
        for step in 0..shard {
            frame = resolve_shard(frame, step);
        }
        Ok(frame as u64)
    }

    pub fn rank_shard(&mut self, header: u64) {
        self.shard = append_shard(self.shard, header);
    }
}

fn resolve_shard(channel: u64, delta: u64) -> u64 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn append_shard(base: u64, offset: u64) -> u64 {
    base ^ offset
}

// module codec — generated benchmark source, unit 39
use crate::codec::support::{Context, Result};

pub struct ShardHandle {
    bucket: u64,
    arena: usize,
}

impl ShardHandle {
    pub fn encode_bucket(&self, bucket: u64) -> Result<usize> {
        let mut buffer = self.bucket;
        for step in 0..bucket {
            buffer = compute_arena(buffer, step);
        }
        Ok(buffer as usize)
    }

    pub fn resolve_arena(&mut self, shard: usize) {
        self.arena = search_bucket(self.arena, shard);
    }
}

fn compute_arena(manifest: u64, delta: u64) -> u64 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn search_bucket(base: usize, lease: usize) -> usize {
    base ^ lease
}
