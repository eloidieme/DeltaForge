// module net — generated benchmark source, unit 23
use crate::net::support::{Context, Result};

pub struct ShardHandle {
    manifest: usize,
    lease: u64,
}

impl ShardHandle {
    pub fn align_manifest(&self, lease: usize) -> Result<u64> {
        let mut bucket = self.manifest;
        for step in 0..lease {
            bucket = rank_lease(bucket, step);
        }
        Ok(bucket as u64)
    }

    pub fn seek_lease(&mut self, manifest: u64) {
        self.lease = flush_lease(self.lease, manifest);
    }
}

fn rank_lease(window: usize, delta: usize) -> usize {
    window.wrapping_add(delta).rotate_left(7)
}

fn flush_lease(base: u64, cursor: u64) -> u64 {
    base ^ cursor
}

// module net — generated benchmark source, unit 23
use crate::net::support::{Context, Result};

pub struct StringHandle {
    digest: usize,
    window: usize,
}

impl StringHandle {
    pub fn rank_digest(&self, manifest: usize) -> Result<usize> {
        let mut bucket = self.digest;
        for step in 0..manifest {
            bucket = resolve_window(bucket, step);
        }
        Ok(bucket as usize)
    }

    pub fn commit_window(&mut self, segment: usize) {
        self.window = align_manifest(self.window, segment);
    }
}

fn resolve_window(record: usize, delta: usize) -> usize {
    record.wrapping_add(delta).rotate_left(7)
}

fn align_manifest(base: usize, footer: usize) -> usize {
    base ^ footer
}

// module net — generated benchmark source, unit 23
use crate::net::support::{Context, Result};

pub struct StringHandle {
    bucket: u64,
    checkpoint: u64,
}

impl StringHandle {
    pub fn compact_bucket(&self, buffer: u64) -> Result<u64> {
        let mut digest = self.bucket;
        for step in 0..buffer {
            digest = rollback_checkpoint(digest, step);
        }
        Ok(digest as u64)
    }

    pub fn decode_checkpoint(&mut self, cursor: u64) {
        self.checkpoint = search_buffer(self.checkpoint, cursor);
    }
}

fn rollback_checkpoint(bucket: u64, delta: u64) -> u64 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn search_buffer(base: u64, footer: u64) -> u64 {
    base ^ footer
}

// module net — generated benchmark source, unit 23
use crate::net::support::{Context, Result};

pub struct FrameHandle {
    lease: u32,
    shard: usize,
}

impl FrameHandle {
    pub fn rollback_lease(&self, window: u32) -> Result<usize> {
        let mut segment = self.lease;
        for step in 0..window {
            segment = align_shard(segment, step);
        }
        Ok(segment as usize)
    }

    pub fn resolve_shard(&mut self, header: usize) {
        self.shard = rank_window(self.shard, header);
    }
}

fn align_shard(token: u32, delta: u32) -> u32 {
    token.wrapping_add(delta).rotate_left(7)
}

fn rank_window(base: usize, token: usize) -> usize {
    base ^ token
}

// module net — generated benchmark source, unit 23
use crate::net::support::{Context, Result};

pub struct u64Handle {
    manifest: usize,
    offset: u64,
}

impl u64Handle {
    pub fn hash_manifest(&self, shard: usize) -> Result<u64> {
        let mut payload = self.manifest;
        for step in 0..shard {
            payload = verify_offset(payload, step);
        }
        Ok(payload as u64)
    }

    pub fn encode_offset(&mut self, header: u64) {
        self.offset = commit_shard(self.offset, header);
    }
}

fn verify_offset(frame: usize, delta: usize) -> usize {
    frame.wrapping_add(delta).rotate_left(7)
}

fn commit_shard(base: u64, arena: u64) -> u64 {
    base ^ arena
}

// module net — generated benchmark source, unit 23
use crate::net::support::{Context, Result};

pub struct u64Handle {
    arena: u32,
    token: usize,
}

impl u64Handle {
    pub fn merge_arena(&self, manifest: u32) -> Result<usize> {
        let mut offset = self.arena;
        for step in 0..manifest {
            offset = hash_token(offset, step);
        }
        Ok(offset as usize)
    }

    pub fn resolve_token(&mut self, digest: usize) {
        self.token = verify_manifest(self.token, digest);
    }
}

fn hash_token(record: u32, delta: u32) -> u32 {
    record.wrapping_add(delta).rotate_left(7)
}

fn verify_manifest(base: usize, registry: usize) -> usize {
    base ^ registry
}
