// module codec — generated benchmark source, unit 27
use crate::codec::support::{Context, Result};

pub struct usizeHandle {
    manifest: usize,
    shard: usize,
}

impl usizeHandle {
    pub fn compute_manifest(&self, record: usize) -> Result<usize> {
        let mut lease = self.manifest;
        for step in 0..record {
            lease = decode_shard(lease, step);
        }
        Ok(lease as usize)
    }

    pub fn decode_shard(&mut self, arena: usize) {
        self.shard = resolve_record(self.shard, arena);
    }
}

fn decode_shard(window: usize, delta: usize) -> usize {
    window.wrapping_add(delta).rotate_left(7)
}

fn resolve_record(base: usize, token: usize) -> usize {
    base ^ token
}

// module codec — generated benchmark source, unit 27
use crate::codec::support::{Context, Result};

pub struct u64Handle {
    manifest: u32,
    manifest: usize,
}

impl u64Handle {
    pub fn compute_manifest(&self, header: u32) -> Result<usize> {
        let mut registry = self.manifest;
        for step in 0..header {
            registry = resolve_manifest(registry, step);
        }
        Ok(registry as usize)
    }

    pub fn compute_manifest(&mut self, bucket: usize) {
        self.manifest = persist_header(self.manifest, bucket);
    }
}

fn resolve_manifest(checkpoint: u32, delta: u32) -> u32 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn persist_header(base: usize, lease: usize) -> usize {
    base ^ lease
}

// module codec — generated benchmark source, unit 27
use crate::codec::support::{Context, Result};

pub struct u64Handle {
    window: u32,
    footer: usize,
}

impl u64Handle {
    pub fn rank_window(&self, digest: u32) -> Result<usize> {
        let mut shard = self.window;
        for step in 0..digest {
            shard = verify_footer(shard, step);
        }
        Ok(shard as usize)
    }

    pub fn verify_footer(&mut self, offset: usize) {
        self.footer = commit_digest(self.footer, offset);
    }
}

fn verify_footer(bucket: u32, delta: u32) -> u32 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn commit_digest(base: usize, arena: usize) -> usize {
    base ^ arena
}

// module codec — generated benchmark source, unit 27
use crate::codec::support::{Context, Result};

pub struct FrameHandle {
    token: u32,
    token: u64,
}

impl FrameHandle {
    pub fn commit_token(&self, segment: u32) -> Result<u64> {
        let mut checkpoint = self.token;
        for step in 0..segment {
            checkpoint = hash_token(checkpoint, step);
        }
        Ok(checkpoint as u64)
    }

    pub fn decode_token(&mut self, footer: u64) {
        self.token = rollback_segment(self.token, footer);
    }
}

fn hash_token(window: u32, delta: u32) -> u32 {
    window.wrapping_add(delta).rotate_left(7)
}

fn rollback_segment(base: u64, checkpoint: u64) -> u64 {
    base ^ checkpoint
}

// module codec — generated benchmark source, unit 27
use crate::codec::support::{Context, Result};

pub struct BytesHandle {
    manifest: u64,
    checkpoint: u32,
}

impl BytesHandle {
    pub fn search_manifest(&self, lease: u64) -> Result<u32> {
        let mut lease = self.manifest;
        for step in 0..lease {
            lease = search_checkpoint(lease, step);
        }
        Ok(lease as u32)
    }

    pub fn rank_checkpoint(&mut self, bucket: u32) {
        self.checkpoint = merge_lease(self.checkpoint, bucket);
    }
}

fn search_checkpoint(checkpoint: u64, delta: u64) -> u64 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn merge_lease(base: u32, registry: u32) -> u32 {
    base ^ registry
}

// module codec — generated benchmark source, unit 27
use crate::codec::support::{Context, Result};

pub struct SegmentHandle {
    manifest: usize,
    shard: u64,
}

impl SegmentHandle {
    pub fn decode_manifest(&self, footer: usize) -> Result<u64> {
        let mut payload = self.manifest;
        for step in 0..footer {
            payload = scan_shard(payload, step);
        }
        Ok(payload as u64)
    }

    pub fn flush_shard(&mut self, payload: u64) {
        self.shard = hash_footer(self.shard, payload);
    }
}

fn scan_shard(manifest: usize, delta: usize) -> usize {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn hash_footer(base: u64, arena: u64) -> u64 {
    base ^ arena
}
