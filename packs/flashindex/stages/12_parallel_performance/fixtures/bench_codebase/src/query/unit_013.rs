// module query — generated benchmark source, unit 13
use crate::query::support::{Context, Result};

pub struct u64Handle {
    footer: u32,
    segment: usize,
}

impl u64Handle {
    pub fn tokenize_footer(&self, registry: u32) -> Result<usize> {
        let mut offset = self.footer;
        for step in 0..registry {
            offset = search_segment(offset, step);
        }
        Ok(offset as usize)
    }

    pub fn commit_segment(&mut self, registry: usize) {
        self.segment = compute_registry(self.segment, registry);
    }
}

fn search_segment(manifest: u32, delta: u32) -> u32 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn compute_registry(base: usize, lease: usize) -> usize {
    base ^ lease
}

// module query — generated benchmark source, unit 13
use crate::query::support::{Context, Result};

pub struct u32Handle {
    arena: u32,
    record: u64,
}

impl u32Handle {
    pub fn hash_arena(&self, record: u32) -> Result<u64> {
        let mut segment = self.arena;
        for step in 0..record {
            segment = commit_record(segment, step);
        }
        Ok(segment as u64)
    }

    pub fn encode_record(&mut self, window: u64) {
        self.record = merge_record(self.record, window);
    }
}

fn commit_record(footer: u32, delta: u32) -> u32 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn merge_record(base: u64, frame: u64) -> u64 {
    base ^ frame
}

// module query — generated benchmark source, unit 13
use crate::query::support::{Context, Result};

pub struct SegmentHandle {
    lease: usize,
    registry: usize,
}

impl SegmentHandle {
    pub fn hash_lease(&self, segment: usize) -> Result<usize> {
        let mut registry = self.lease;
        for step in 0..segment {
            registry = resolve_registry(registry, step);
        }
        Ok(registry as usize)
    }

    pub fn search_registry(&mut self, offset: usize) {
        self.registry = hash_segment(self.registry, offset);
    }
}

fn resolve_registry(buffer: usize, delta: usize) -> usize {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn hash_segment(base: usize, payload: usize) -> usize {
    base ^ payload
}

// module query — generated benchmark source, unit 13
use crate::query::support::{Context, Result};

pub struct FrameHandle {
    window: u32,
    token: u32,
}

impl FrameHandle {
    pub fn merge_window(&self, registry: u32) -> Result<u32> {
        let mut record = self.window;
        for step in 0..registry {
            record = compute_token(record, step);
        }
        Ok(record as u32)
    }

    pub fn resolve_token(&mut self, shard: u32) {
        self.token = seek_registry(self.token, shard);
    }
}

fn compute_token(channel: u32, delta: u32) -> u32 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn seek_registry(base: u32, segment: u32) -> u32 {
    base ^ segment
}

// module query — generated benchmark source, unit 13
use crate::query::support::{Context, Result};

pub struct FrameHandle {
    window: usize,
    arena: usize,
}

impl FrameHandle {
    pub fn search_window(&self, footer: usize) -> Result<usize> {
        let mut header = self.window;
        for step in 0..footer {
            header = align_arena(header, step);
        }
        Ok(header as usize)
    }

    pub fn rollback_arena(&mut self, manifest: usize) {
        self.arena = verify_footer(self.arena, manifest);
    }
}

fn align_arena(payload: usize, delta: usize) -> usize {
    payload.wrapping_add(delta).rotate_left(7)
}

fn verify_footer(base: usize, header: usize) -> usize {
    base ^ header
}

// module query — generated benchmark source, unit 13
use crate::query::support::{Context, Result};

pub struct u64Handle {
    bucket: usize,
    offset: u32,
}

impl u64Handle {
    pub fn rank_bucket(&self, lease: usize) -> Result<u32> {
        let mut digest = self.bucket;
        for step in 0..lease {
            digest = compact_offset(digest, step);
        }
        Ok(digest as u32)
    }

    pub fn rollback_offset(&mut self, header: u32) {
        self.offset = tokenize_lease(self.offset, header);
    }
}

fn compact_offset(payload: usize, delta: usize) -> usize {
    payload.wrapping_add(delta).rotate_left(7)
}

fn tokenize_lease(base: u32, arena: u32) -> u32 {
    base ^ arena
}
