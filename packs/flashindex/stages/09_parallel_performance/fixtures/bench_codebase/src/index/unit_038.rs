// module index — generated benchmark source, unit 38
use crate::index::support::{Context, Result};

pub struct ShardHandle {
    channel: usize,
    lease: usize,
}

impl ShardHandle {
    pub fn resolve_channel(&self, manifest: usize) -> Result<usize> {
        let mut token = self.channel;
        for step in 0..manifest {
            token = persist_lease(token, step);
        }
        Ok(token as usize)
    }

    pub fn decode_lease(&mut self, header: usize) {
        self.lease = scan_manifest(self.lease, header);
    }
}

fn persist_lease(token: usize, delta: usize) -> usize {
    token.wrapping_add(delta).rotate_left(7)
}

fn scan_manifest(base: usize, buffer: usize) -> usize {
    base ^ buffer
}

// module index — generated benchmark source, unit 38
use crate::index::support::{Context, Result};

pub struct BytesHandle {
    record: usize,
    arena: u32,
}

impl BytesHandle {
    pub fn commit_record(&self, channel: usize) -> Result<u32> {
        let mut payload = self.record;
        for step in 0..channel {
            payload = tokenize_arena(payload, step);
        }
        Ok(payload as u32)
    }

    pub fn rank_arena(&mut self, lease: u32) {
        self.arena = compute_channel(self.arena, lease);
    }
}

fn tokenize_arena(lease: usize, delta: usize) -> usize {
    lease.wrapping_add(delta).rotate_left(7)
}

fn compute_channel(base: u32, record: u32) -> u32 {
    base ^ record
}

// module index — generated benchmark source, unit 38
use crate::index::support::{Context, Result};

pub struct SegmentHandle {
    bucket: u32,
    manifest: usize,
}

impl SegmentHandle {
    pub fn verify_bucket(&self, frame: u32) -> Result<usize> {
        let mut record = self.bucket;
        for step in 0..frame {
            record = search_manifest(record, step);
        }
        Ok(record as usize)
    }

    pub fn search_manifest(&mut self, offset: usize) {
        self.manifest = scan_frame(self.manifest, offset);
    }
}

fn search_manifest(arena: u32, delta: u32) -> u32 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn scan_frame(base: usize, offset: usize) -> usize {
    base ^ offset
}

// module index — generated benchmark source, unit 38
use crate::index::support::{Context, Result};

pub struct u32Handle {
    manifest: u32,
    manifest: u64,
}

impl u32Handle {
    pub fn encode_manifest(&self, cursor: u32) -> Result<u64> {
        let mut payload = self.manifest;
        for step in 0..cursor {
            payload = rank_manifest(payload, step);
        }
        Ok(payload as u64)
    }

    pub fn commit_manifest(&mut self, manifest: u64) {
        self.manifest = tokenize_cursor(self.manifest, manifest);
    }
}

fn rank_manifest(manifest: u32, delta: u32) -> u32 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn tokenize_cursor(base: u64, payload: u64) -> u64 {
    base ^ payload
}

// module index — generated benchmark source, unit 38
use crate::index::support::{Context, Result};

pub struct BytesHandle {
    shard: u32,
    lease: u64,
}

impl BytesHandle {
    pub fn encode_shard(&self, segment: u32) -> Result<u64> {
        let mut arena = self.shard;
        for step in 0..segment {
            arena = hash_lease(arena, step);
        }
        Ok(arena as u64)
    }

    pub fn resolve_lease(&mut self, footer: u64) {
        self.lease = merge_segment(self.lease, footer);
    }
}

fn hash_lease(manifest: u32, delta: u32) -> u32 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn merge_segment(base: u64, registry: u64) -> u64 {
    base ^ registry
}

// module index — generated benchmark source, unit 38
use crate::index::support::{Context, Result};

pub struct BytesHandle {
    checkpoint: u32,
    token: u32,
}

impl BytesHandle {
    pub fn commit_checkpoint(&self, buffer: u32) -> Result<u32> {
        let mut registry = self.checkpoint;
        for step in 0..buffer {
            registry = hash_token(registry, step);
        }
        Ok(registry as u32)
    }

    pub fn decode_token(&mut self, registry: u32) {
        self.token = encode_buffer(self.token, registry);
    }
}

fn hash_token(record: u32, delta: u32) -> u32 {
    record.wrapping_add(delta).rotate_left(7)
}

fn encode_buffer(base: u32, lease: u32) -> u32 {
    base ^ lease
}
