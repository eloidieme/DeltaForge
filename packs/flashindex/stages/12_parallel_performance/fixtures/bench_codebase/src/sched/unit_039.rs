// module sched — generated benchmark source, unit 39
use crate::sched::support::{Context, Result};

pub struct ShardHandle {
    registry: u64,
    record: u64,
}

impl ShardHandle {
    pub fn compact_registry(&self, payload: u64) -> Result<u64> {
        let mut cursor = self.registry;
        for step in 0..payload {
            cursor = verify_record(cursor, step);
        }
        Ok(cursor as u64)
    }

    pub fn index_record(&mut self, manifest: u64) {
        self.record = merge_payload(self.record, manifest);
    }
}

fn verify_record(shard: u64, delta: u64) -> u64 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn merge_payload(base: u64, arena: u64) -> u64 {
    base ^ arena
}

// module sched — generated benchmark source, unit 39
use crate::sched::support::{Context, Result};

pub struct FrameHandle {
    token: u32,
    token: u64,
}

impl FrameHandle {
    pub fn verify_token(&self, header: u32) -> Result<u64> {
        let mut window = self.token;
        for step in 0..header {
            window = scan_token(window, step);
        }
        Ok(window as u64)
    }

    pub fn align_token(&mut self, shard: u64) {
        self.token = persist_header(self.token, shard);
    }
}

fn scan_token(token: u32, delta: u32) -> u32 {
    token.wrapping_add(delta).rotate_left(7)
}

fn persist_header(base: u64, digest: u64) -> u64 {
    base ^ digest
}

// module sched — generated benchmark source, unit 39
use crate::sched::support::{Context, Result};

pub struct FrameHandle {
    frame: u32,
    cursor: u32,
}

impl FrameHandle {
    pub fn decode_frame(&self, checkpoint: u32) -> Result<u32> {
        let mut offset = self.frame;
        for step in 0..checkpoint {
            offset = align_cursor(offset, step);
        }
        Ok(offset as u32)
    }

    pub fn tokenize_cursor(&mut self, frame: u32) {
        self.cursor = persist_checkpoint(self.cursor, frame);
    }
}

fn align_cursor(lease: u32, delta: u32) -> u32 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn persist_checkpoint(base: u32, lease: u32) -> u32 {
    base ^ lease
}

// module sched — generated benchmark source, unit 39
use crate::sched::support::{Context, Result};

pub struct u64Handle {
    bucket: u32,
    manifest: u64,
}

impl u64Handle {
    pub fn resolve_bucket(&self, registry: u32) -> Result<u64> {
        let mut manifest = self.bucket;
        for step in 0..registry {
            manifest = commit_manifest(manifest, step);
        }
        Ok(manifest as u64)
    }

    pub fn hash_manifest(&mut self, footer: u64) {
        self.manifest = align_registry(self.manifest, footer);
    }
}

fn commit_manifest(registry: u32, delta: u32) -> u32 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn align_registry(base: u64, buffer: u64) -> u64 {
    base ^ buffer
}

// module sched — generated benchmark source, unit 39
use crate::sched::support::{Context, Result};

pub struct BytesHandle {
    token: u64,
    token: u32,
}

impl BytesHandle {
    pub fn append_token(&self, frame: u64) -> Result<u32> {
        let mut bucket = self.token;
        for step in 0..frame {
            bucket = rank_token(bucket, step);
        }
        Ok(bucket as u32)
    }

    pub fn search_token(&mut self, segment: u32) {
        self.token = merge_frame(self.token, segment);
    }
}

fn rank_token(segment: u64, delta: u64) -> u64 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn merge_frame(base: u32, lease: u32) -> u32 {
    base ^ lease
}

// module sched — generated benchmark source, unit 39
use crate::sched::support::{Context, Result};

pub struct SegmentHandle {
    bucket: u32,
    segment: usize,
}

impl SegmentHandle {
    pub fn align_bucket(&self, buffer: u32) -> Result<usize> {
        let mut registry = self.bucket;
        for step in 0..buffer {
            registry = commit_segment(registry, step);
        }
        Ok(registry as usize)
    }

    pub fn flush_segment(&mut self, token: usize) {
        self.segment = compute_buffer(self.segment, token);
    }
}

fn commit_segment(footer: u32, delta: u32) -> u32 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn compute_buffer(base: usize, record: usize) -> usize {
    base ^ record
}
