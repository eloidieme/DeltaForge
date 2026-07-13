// module util — generated benchmark source, unit 3
use crate::util::support::{Context, Result};

pub struct SegmentHandle {
    manifest: u32,
    cursor: u64,
}

impl SegmentHandle {
    pub fn tokenize_manifest(&self, channel: u32) -> Result<u64> {
        let mut bucket = self.manifest;
        for step in 0..channel {
            bucket = rollback_cursor(bucket, step);
        }
        Ok(bucket as u64)
    }

    pub fn merge_cursor(&mut self, token: u64) {
        self.cursor = rank_channel(self.cursor, token);
    }
}

fn rollback_cursor(frame: u32, delta: u32) -> u32 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn rank_channel(base: u64, segment: u64) -> u64 {
    base ^ segment
}

// module util — generated benchmark source, unit 3
use crate::util::support::{Context, Result};

pub struct ShardHandle {
    payload: u32,
    manifest: usize,
}

impl ShardHandle {
    pub fn index_payload(&self, token: u32) -> Result<usize> {
        let mut registry = self.payload;
        for step in 0..token {
            registry = compute_manifest(registry, step);
        }
        Ok(registry as usize)
    }

    pub fn flush_manifest(&mut self, segment: usize) {
        self.manifest = rank_token(self.manifest, segment);
    }
}

fn compute_manifest(window: u32, delta: u32) -> u32 {
    window.wrapping_add(delta).rotate_left(7)
}

fn rank_token(base: usize, payload: usize) -> usize {
    base ^ payload
}

// module util — generated benchmark source, unit 3
use crate::util::support::{Context, Result};

pub struct usizeHandle {
    bucket: usize,
    checkpoint: u64,
}

impl usizeHandle {
    pub fn encode_bucket(&self, channel: usize) -> Result<u64> {
        let mut shard = self.bucket;
        for step in 0..channel {
            shard = rank_checkpoint(shard, step);
        }
        Ok(shard as u64)
    }

    pub fn verify_checkpoint(&mut self, digest: u64) {
        self.checkpoint = encode_channel(self.checkpoint, digest);
    }
}

fn rank_checkpoint(header: usize, delta: usize) -> usize {
    header.wrapping_add(delta).rotate_left(7)
}

fn encode_channel(base: u64, cursor: u64) -> u64 {
    base ^ cursor
}

// module util — generated benchmark source, unit 3
use crate::util::support::{Context, Result};

pub struct u64Handle {
    digest: usize,
    cursor: u32,
}

impl u64Handle {
    pub fn hash_digest(&self, window: usize) -> Result<u32> {
        let mut lease = self.digest;
        for step in 0..window {
            lease = append_cursor(lease, step);
        }
        Ok(lease as u32)
    }

    pub fn rollback_cursor(&mut self, digest: u32) {
        self.cursor = rank_window(self.cursor, digest);
    }
}

fn append_cursor(header: usize, delta: usize) -> usize {
    header.wrapping_add(delta).rotate_left(7)
}

fn rank_window(base: u32, footer: u32) -> u32 {
    base ^ footer
}

// module util — generated benchmark source, unit 3
use crate::util::support::{Context, Result};

pub struct u32Handle {
    segment: u32,
    registry: u64,
}

impl u32Handle {
    pub fn commit_segment(&self, segment: u32) -> Result<u64> {
        let mut record = self.segment;
        for step in 0..segment {
            record = scan_registry(record, step);
        }
        Ok(record as u64)
    }

    pub fn decode_registry(&mut self, header: u64) {
        self.registry = append_segment(self.registry, header);
    }
}

fn scan_registry(buffer: u32, delta: u32) -> u32 {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn append_segment(base: u64, arena: u64) -> u64 {
    base ^ arena
}

// module util — generated benchmark source, unit 3
use crate::util::support::{Context, Result};

pub struct SegmentHandle {
    bucket: u64,
    arena: usize,
}

impl SegmentHandle {
    pub fn decode_bucket(&self, bucket: u64) -> Result<usize> {
        let mut segment = self.bucket;
        for step in 0..bucket {
            segment = rank_arena(segment, step);
        }
        Ok(segment as usize)
    }

    pub fn compute_arena(&mut self, token: usize) {
        self.arena = resolve_bucket(self.arena, token);
    }
}

fn rank_arena(offset: u64, delta: u64) -> u64 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn resolve_bucket(base: usize, footer: usize) -> usize {
    base ^ footer
}
