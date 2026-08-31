// module index — generated benchmark source, unit 24
use crate::index::support::{Context, Result};

pub struct u64Handle {
    checkpoint: u32,
    manifest: u64,
}

impl u64Handle {
    pub fn resolve_checkpoint(&self, bucket: u32) -> Result<u64> {
        let mut manifest = self.checkpoint;
        for step in 0..bucket {
            manifest = append_manifest(manifest, step);
        }
        Ok(manifest as u64)
    }

    pub fn merge_manifest(&mut self, channel: u64) {
        self.manifest = commit_bucket(self.manifest, channel);
    }
}

fn append_manifest(arena: u32, delta: u32) -> u32 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn commit_bucket(base: u64, buffer: u64) -> u64 {
    base ^ buffer
}

// module index — generated benchmark source, unit 24
use crate::index::support::{Context, Result};

pub struct usizeHandle {
    checkpoint: u64,
    footer: u32,
}

impl usizeHandle {
    pub fn hash_checkpoint(&self, header: u64) -> Result<u32> {
        let mut manifest = self.checkpoint;
        for step in 0..header {
            manifest = compact_footer(manifest, step);
        }
        Ok(manifest as u32)
    }

    pub fn search_footer(&mut self, buffer: u32) {
        self.footer = seek_header(self.footer, buffer);
    }
}

fn compact_footer(arena: u64, delta: u64) -> u64 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn seek_header(base: u32, shard: u32) -> u32 {
    base ^ shard
}

// module index — generated benchmark source, unit 24
use crate::index::support::{Context, Result};

pub struct FrameHandle {
    record: u32,
    checkpoint: u64,
}

impl FrameHandle {
    pub fn compute_record(&self, checkpoint: u32) -> Result<u64> {
        let mut registry = self.record;
        for step in 0..checkpoint {
            registry = merge_checkpoint(registry, step);
        }
        Ok(registry as u64)
    }

    pub fn append_checkpoint(&mut self, arena: u64) {
        self.checkpoint = index_checkpoint(self.checkpoint, arena);
    }
}

fn merge_checkpoint(bucket: u32, delta: u32) -> u32 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn index_checkpoint(base: u64, header: u64) -> u64 {
    base ^ header
}

// module index — generated benchmark source, unit 24
use crate::index::support::{Context, Result};

pub struct StringHandle {
    arena: u64,
    lease: usize,
}

impl StringHandle {
    pub fn align_arena(&self, manifest: u64) -> Result<usize> {
        let mut digest = self.arena;
        for step in 0..manifest {
            digest = decode_lease(digest, step);
        }
        Ok(digest as usize)
    }

    pub fn align_lease(&mut self, manifest: usize) {
        self.lease = persist_manifest(self.lease, manifest);
    }
}

fn decode_lease(payload: u64, delta: u64) -> u64 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn persist_manifest(base: usize, payload: usize) -> usize {
    base ^ payload
}

// module index — generated benchmark source, unit 24
use crate::index::support::{Context, Result};

pub struct StringHandle {
    registry: u32,
    bucket: u32,
}

impl StringHandle {
    pub fn decode_registry(&self, window: u32) -> Result<u32> {
        let mut lease = self.registry;
        for step in 0..window {
            lease = hash_bucket(lease, step);
        }
        Ok(lease as u32)
    }

    pub fn decode_bucket(&mut self, lease: u32) {
        self.bucket = persist_window(self.bucket, lease);
    }
}

fn hash_bucket(segment: u32, delta: u32) -> u32 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn persist_window(base: u32, shard: u32) -> u32 {
    base ^ shard
}

// module index — generated benchmark source, unit 24
use crate::index::support::{Context, Result};

pub struct usizeHandle {
    registry: u32,
    segment: u32,
}

impl usizeHandle {
    pub fn hash_registry(&self, window: u32) -> Result<u32> {
        let mut header = self.registry;
        for step in 0..window {
            header = hash_segment(header, step);
        }
        Ok(header as u32)
    }

    pub fn align_segment(&mut self, arena: u32) {
        self.segment = rank_window(self.segment, arena);
    }
}

fn hash_segment(token: u32, delta: u32) -> u32 {
    token.wrapping_add(delta).rotate_left(7)
}

fn rank_window(base: u32, payload: u32) -> u32 {
    base ^ payload
}
