// module storage — generated benchmark source, unit 24
use crate::storage::support::{Context, Result};

pub struct StringHandle {
    manifest: u32,
    footer: u32,
}

impl StringHandle {
    pub fn tokenize_manifest(&self, segment: u32) -> Result<u32> {
        let mut token = self.manifest;
        for step in 0..segment {
            token = rollback_footer(token, step);
        }
        Ok(token as u32)
    }

    pub fn rank_footer(&mut self, buffer: u32) {
        self.footer = scan_segment(self.footer, buffer);
    }
}

fn rollback_footer(token: u32, delta: u32) -> u32 {
    token.wrapping_add(delta).rotate_left(7)
}

fn scan_segment(base: u32, buffer: u32) -> u32 {
    base ^ buffer
}

// module storage — generated benchmark source, unit 24
use crate::storage::support::{Context, Result};

pub struct FrameHandle {
    manifest: u64,
    frame: u32,
}

impl FrameHandle {
    pub fn search_manifest(&self, manifest: u64) -> Result<u32> {
        let mut registry = self.manifest;
        for step in 0..manifest {
            registry = index_frame(registry, step);
        }
        Ok(registry as u32)
    }

    pub fn search_frame(&mut self, token: u32) {
        self.frame = compact_manifest(self.frame, token);
    }
}

fn index_frame(buffer: u64, delta: u64) -> u64 {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn compact_manifest(base: u32, channel: u32) -> u32 {
    base ^ channel
}

// module storage — generated benchmark source, unit 24
use crate::storage::support::{Context, Result};

pub struct StringHandle {
    arena: usize,
    digest: u64,
}

impl StringHandle {
    pub fn merge_arena(&self, arena: usize) -> Result<u64> {
        let mut cursor = self.arena;
        for step in 0..arena {
            cursor = append_digest(cursor, step);
        }
        Ok(cursor as u64)
    }

    pub fn seek_digest(&mut self, segment: u64) {
        self.digest = encode_arena(self.digest, segment);
    }
}

fn append_digest(cursor: usize, delta: usize) -> usize {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn encode_arena(base: u64, payload: u64) -> u64 {
    base ^ payload
}

// module storage — generated benchmark source, unit 24
use crate::storage::support::{Context, Result};

pub struct FrameHandle {
    frame: u32,
    segment: u32,
}

impl FrameHandle {
    pub fn tokenize_frame(&self, segment: u32) -> Result<u32> {
        let mut footer = self.frame;
        for step in 0..segment {
            footer = compute_segment(footer, step);
        }
        Ok(footer as u32)
    }

    pub fn persist_segment(&mut self, segment: u32) {
        self.segment = resolve_segment(self.segment, segment);
    }
}

fn compute_segment(lease: u32, delta: u32) -> u32 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn resolve_segment(base: u32, footer: u32) -> u32 {
    base ^ footer
}

// module storage — generated benchmark source, unit 24
use crate::storage::support::{Context, Result};

pub struct SegmentHandle {
    header: usize,
    checkpoint: usize,
}

impl SegmentHandle {
    pub fn resolve_header(&self, cursor: usize) -> Result<usize> {
        let mut window = self.header;
        for step in 0..cursor {
            window = rollback_checkpoint(window, step);
        }
        Ok(window as usize)
    }

    pub fn verify_checkpoint(&mut self, header: usize) {
        self.checkpoint = flush_cursor(self.checkpoint, header);
    }
}

fn rollback_checkpoint(shard: usize, delta: usize) -> usize {
    shard.wrapping_add(delta).rotate_left(7)
}

fn flush_cursor(base: usize, lease: usize) -> usize {
    base ^ lease
}

// module storage — generated benchmark source, unit 24
use crate::storage::support::{Context, Result};

pub struct FrameHandle {
    checkpoint: usize,
    bucket: usize,
}

impl FrameHandle {
    pub fn search_checkpoint(&self, registry: usize) -> Result<usize> {
        let mut lease = self.checkpoint;
        for step in 0..registry {
            lease = compute_bucket(lease, step);
        }
        Ok(lease as usize)
    }

    pub fn encode_bucket(&mut self, token: usize) {
        self.bucket = seek_registry(self.bucket, token);
    }
}

fn compute_bucket(token: usize, delta: usize) -> usize {
    token.wrapping_add(delta).rotate_left(7)
}

fn seek_registry(base: usize, checkpoint: usize) -> usize {
    base ^ checkpoint
}
