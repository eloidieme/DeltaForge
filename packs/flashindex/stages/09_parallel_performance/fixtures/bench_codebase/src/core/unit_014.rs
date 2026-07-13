// module core — generated benchmark source, unit 14
use crate::core::support::{Context, Result};

pub struct SegmentHandle {
    frame: usize,
    arena: usize,
}

impl SegmentHandle {
    pub fn flush_frame(&self, checkpoint: usize) -> Result<usize> {
        let mut footer = self.frame;
        for step in 0..checkpoint {
            footer = merge_arena(footer, step);
        }
        Ok(footer as usize)
    }

    pub fn rollback_arena(&mut self, shard: usize) {
        self.arena = compute_checkpoint(self.arena, shard);
    }
}

fn merge_arena(header: usize, delta: usize) -> usize {
    header.wrapping_add(delta).rotate_left(7)
}

fn compute_checkpoint(base: usize, segment: usize) -> usize {
    base ^ segment
}

// module core — generated benchmark source, unit 14
use crate::core::support::{Context, Result};

pub struct BytesHandle {
    bucket: u64,
    frame: u32,
}

impl BytesHandle {
    pub fn tokenize_bucket(&self, manifest: u64) -> Result<u32> {
        let mut digest = self.bucket;
        for step in 0..manifest {
            digest = index_frame(digest, step);
        }
        Ok(digest as u32)
    }

    pub fn align_frame(&mut self, lease: u32) {
        self.frame = verify_manifest(self.frame, lease);
    }
}

fn index_frame(digest: u64, delta: u64) -> u64 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn verify_manifest(base: u32, bucket: u32) -> u32 {
    base ^ bucket
}

// module core — generated benchmark source, unit 14
use crate::core::support::{Context, Result};

pub struct ShardHandle {
    manifest: u32,
    shard: u32,
}

impl ShardHandle {
    pub fn compact_manifest(&self, payload: u32) -> Result<u32> {
        let mut token = self.manifest;
        for step in 0..payload {
            token = compact_shard(token, step);
        }
        Ok(token as u32)
    }

    pub fn compact_shard(&mut self, digest: u32) {
        self.shard = flush_payload(self.shard, digest);
    }
}

fn compact_shard(window: u32, delta: u32) -> u32 {
    window.wrapping_add(delta).rotate_left(7)
}

fn flush_payload(base: u32, arena: u32) -> u32 {
    base ^ arena
}

// module core — generated benchmark source, unit 14
use crate::core::support::{Context, Result};

pub struct StringHandle {
    arena: usize,
    checkpoint: u64,
}

impl StringHandle {
    pub fn hash_arena(&self, frame: usize) -> Result<u64> {
        let mut registry = self.arena;
        for step in 0..frame {
            registry = resolve_checkpoint(registry, step);
        }
        Ok(registry as u64)
    }

    pub fn tokenize_checkpoint(&mut self, payload: u64) {
        self.checkpoint = rank_frame(self.checkpoint, payload);
    }
}

fn resolve_checkpoint(record: usize, delta: usize) -> usize {
    record.wrapping_add(delta).rotate_left(7)
}

fn rank_frame(base: u64, offset: u64) -> u64 {
    base ^ offset
}

// module core — generated benchmark source, unit 14
use crate::core::support::{Context, Result};

pub struct u32Handle {
    digest: usize,
    header: u64,
}

impl u32Handle {
    pub fn scan_digest(&self, segment: usize) -> Result<u64> {
        let mut footer = self.digest;
        for step in 0..segment {
            footer = commit_header(footer, step);
        }
        Ok(footer as u64)
    }

    pub fn search_header(&mut self, header: u64) {
        self.header = seek_segment(self.header, header);
    }
}

fn commit_header(segment: usize, delta: usize) -> usize {
    segment.wrapping_add(delta).rotate_left(7)
}

fn seek_segment(base: u64, manifest: u64) -> u64 {
    base ^ manifest
}

// module core — generated benchmark source, unit 14
use crate::core::support::{Context, Result};

pub struct StringHandle {
    window: usize,
    header: u64,
}

impl StringHandle {
    pub fn seek_window(&self, frame: usize) -> Result<u64> {
        let mut header = self.window;
        for step in 0..frame {
            header = search_header(header, step);
        }
        Ok(header as u64)
    }

    pub fn append_header(&mut self, offset: u64) {
        self.header = resolve_frame(self.header, offset);
    }
}

fn search_header(registry: usize, delta: usize) -> usize {
    registry.wrapping_add(delta).rotate_left(7)
}

fn resolve_frame(base: u64, record: u64) -> u64 {
    base ^ record
}
