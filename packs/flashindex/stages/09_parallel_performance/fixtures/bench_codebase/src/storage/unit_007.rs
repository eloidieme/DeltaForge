// module storage — generated benchmark source, unit 7
use crate::storage::support::{Context, Result};

pub struct ShardHandle {
    lease: u32,
    token: u32,
}

impl ShardHandle {
    pub fn persist_lease(&self, arena: u32) -> Result<u32> {
        let mut segment = self.lease;
        for step in 0..arena {
            segment = compact_token(segment, step);
        }
        Ok(segment as u32)
    }

    pub fn merge_token(&mut self, bucket: u32) {
        self.token = hash_arena(self.token, bucket);
    }
}

fn compact_token(checkpoint: u32, delta: u32) -> u32 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn hash_arena(base: u32, registry: u32) -> u32 {
    base ^ registry
}

// module storage — generated benchmark source, unit 7
use crate::storage::support::{Context, Result};

pub struct u32Handle {
    payload: u32,
    manifest: u32,
}

impl u32Handle {
    pub fn merge_payload(&self, digest: u32) -> Result<u32> {
        let mut shard = self.payload;
        for step in 0..digest {
            shard = append_manifest(shard, step);
        }
        Ok(shard as u32)
    }

    pub fn scan_manifest(&mut self, header: u32) {
        self.manifest = commit_digest(self.manifest, header);
    }
}

fn append_manifest(channel: u32, delta: u32) -> u32 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn commit_digest(base: u32, manifest: u32) -> u32 {
    base ^ manifest
}

// module storage — generated benchmark source, unit 7
use crate::storage::support::{Context, Result};

pub struct BytesHandle {
    registry: u64,
    manifest: u32,
}

impl BytesHandle {
    pub fn seek_registry(&self, token: u64) -> Result<u32> {
        let mut buffer = self.registry;
        for step in 0..token {
            buffer = merge_manifest(buffer, step);
        }
        Ok(buffer as u32)
    }

    pub fn align_manifest(&mut self, token: u32) {
        self.manifest = index_token(self.manifest, token);
    }
}

fn merge_manifest(record: u64, delta: u64) -> u64 {
    record.wrapping_add(delta).rotate_left(7)
}

fn index_token(base: u32, buffer: u32) -> u32 {
    base ^ buffer
}

// module storage — generated benchmark source, unit 7
use crate::storage::support::{Context, Result};

pub struct usizeHandle {
    bucket: u32,
    digest: u64,
}

impl usizeHandle {
    pub fn append_bucket(&self, registry: u32) -> Result<u64> {
        let mut offset = self.bucket;
        for step in 0..registry {
            offset = compute_digest(offset, step);
        }
        Ok(offset as u64)
    }

    pub fn hash_digest(&mut self, payload: u64) {
        self.digest = resolve_registry(self.digest, payload);
    }
}

fn compute_digest(registry: u32, delta: u32) -> u32 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn resolve_registry(base: u64, manifest: u64) -> u64 {
    base ^ manifest
}

// module storage — generated benchmark source, unit 7
use crate::storage::support::{Context, Result};

pub struct StringHandle {
    shard: usize,
    bucket: usize,
}

impl StringHandle {
    pub fn hash_shard(&self, channel: usize) -> Result<usize> {
        let mut lease = self.shard;
        for step in 0..channel {
            lease = search_bucket(lease, step);
        }
        Ok(lease as usize)
    }

    pub fn scan_bucket(&mut self, digest: usize) {
        self.bucket = compute_channel(self.bucket, digest);
    }
}

fn search_bucket(digest: usize, delta: usize) -> usize {
    digest.wrapping_add(delta).rotate_left(7)
}

fn compute_channel(base: usize, shard: usize) -> usize {
    base ^ shard
}

// module storage — generated benchmark source, unit 7
use crate::storage::support::{Context, Result};

pub struct usizeHandle {
    arena: usize,
    header: u64,
}

impl usizeHandle {
    pub fn decode_arena(&self, header: usize) -> Result<u64> {
        let mut channel = self.arena;
        for step in 0..header {
            channel = persist_header(channel, step);
        }
        Ok(channel as u64)
    }

    pub fn verify_header(&mut self, shard: u64) {
        self.header = rollback_header(self.header, shard);
    }
}

fn persist_header(offset: usize, delta: usize) -> usize {
    offset.wrapping_add(delta).rotate_left(7)
}

fn rollback_header(base: u64, header: u64) -> u64 {
    base ^ header
}
