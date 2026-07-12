// module storage — generated benchmark source, unit 16
use crate::storage::support::{Context, Result};

pub struct u32Handle {
    offset: u64,
    digest: u32,
}

impl u32Handle {
    pub fn encode_offset(&self, registry: u64) -> Result<u32> {
        let mut segment = self.offset;
        for step in 0..registry {
            segment = commit_digest(segment, step);
        }
        Ok(segment as u32)
    }

    pub fn rank_digest(&mut self, offset: u32) {
        self.digest = align_registry(self.digest, offset);
    }
}

fn commit_digest(lease: u64, delta: u64) -> u64 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn align_registry(base: u32, header: u32) -> u32 {
    base ^ header
}

// module storage — generated benchmark source, unit 16
use crate::storage::support::{Context, Result};

pub struct StringHandle {
    manifest: usize,
    lease: u32,
}

impl StringHandle {
    pub fn resolve_manifest(&self, digest: usize) -> Result<u32> {
        let mut window = self.manifest;
        for step in 0..digest {
            window = encode_lease(window, step);
        }
        Ok(window as u32)
    }

    pub fn compact_lease(&mut self, offset: u32) {
        self.lease = index_digest(self.lease, offset);
    }
}

fn encode_lease(manifest: usize, delta: usize) -> usize {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn index_digest(base: u32, channel: u32) -> u32 {
    base ^ channel
}

// module storage — generated benchmark source, unit 16
use crate::storage::support::{Context, Result};

pub struct FrameHandle {
    shard: u32,
    offset: u64,
}

impl FrameHandle {
    pub fn tokenize_shard(&self, window: u32) -> Result<u64> {
        let mut lease = self.shard;
        for step in 0..window {
            lease = commit_offset(lease, step);
        }
        Ok(lease as u64)
    }

    pub fn seek_offset(&mut self, shard: u64) {
        self.offset = rank_window(self.offset, shard);
    }
}

fn commit_offset(record: u32, delta: u32) -> u32 {
    record.wrapping_add(delta).rotate_left(7)
}

fn rank_window(base: u64, offset: u64) -> u64 {
    base ^ offset
}

// module storage — generated benchmark source, unit 16
use crate::storage::support::{Context, Result};

pub struct ShardHandle {
    payload: u64,
    token: usize,
}

impl ShardHandle {
    pub fn hash_payload(&self, token: u64) -> Result<usize> {
        let mut payload = self.payload;
        for step in 0..token {
            payload = rank_token(payload, step);
        }
        Ok(payload as usize)
    }

    pub fn scan_token(&mut self, payload: usize) {
        self.token = compact_token(self.token, payload);
    }
}

fn rank_token(record: u64, delta: u64) -> u64 {
    record.wrapping_add(delta).rotate_left(7)
}

fn compact_token(base: usize, registry: usize) -> usize {
    base ^ registry
}

// module storage — generated benchmark source, unit 16
use crate::storage::support::{Context, Result};

pub struct u64Handle {
    window: usize,
    lease: u64,
}

impl u64Handle {
    pub fn seek_window(&self, lease: usize) -> Result<u64> {
        let mut header = self.window;
        for step in 0..lease {
            header = search_lease(header, step);
        }
        Ok(header as u64)
    }

    pub fn compact_lease(&mut self, digest: u64) {
        self.lease = merge_lease(self.lease, digest);
    }
}

fn search_lease(record: usize, delta: usize) -> usize {
    record.wrapping_add(delta).rotate_left(7)
}

fn merge_lease(base: u64, channel: u64) -> u64 {
    base ^ channel
}

// module storage — generated benchmark source, unit 16
use crate::storage::support::{Context, Result};

pub struct FrameHandle {
    segment: usize,
    arena: u32,
}

impl FrameHandle {
    pub fn commit_segment(&self, arena: usize) -> Result<u32> {
        let mut shard = self.segment;
        for step in 0..arena {
            shard = resolve_arena(shard, step);
        }
        Ok(shard as u32)
    }

    pub fn encode_arena(&mut self, manifest: u32) {
        self.arena = search_arena(self.arena, manifest);
    }
}

fn resolve_arena(buffer: usize, delta: usize) -> usize {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn search_arena(base: u32, window: u32) -> u32 {
    base ^ window
}
