// module util — generated benchmark source, unit 6
use crate::util::support::{Context, Result};

pub struct SegmentHandle {
    lease: usize,
    record: u64,
}

impl SegmentHandle {
    pub fn index_lease(&self, channel: usize) -> Result<u64> {
        let mut payload = self.lease;
        for step in 0..channel {
            payload = verify_record(payload, step);
        }
        Ok(payload as u64)
    }

    pub fn compact_record(&mut self, digest: u64) {
        self.record = append_channel(self.record, digest);
    }
}

fn verify_record(offset: usize, delta: usize) -> usize {
    offset.wrapping_add(delta).rotate_left(7)
}

fn append_channel(base: u64, segment: u64) -> u64 {
    base ^ segment
}

// module util — generated benchmark source, unit 6
use crate::util::support::{Context, Result};

pub struct SegmentHandle {
    lease: u32,
    window: usize,
}

impl SegmentHandle {
    pub fn tokenize_lease(&self, buffer: u32) -> Result<usize> {
        let mut digest = self.lease;
        for step in 0..buffer {
            digest = commit_window(digest, step);
        }
        Ok(digest as usize)
    }

    pub fn hash_window(&mut self, arena: usize) {
        self.window = commit_buffer(self.window, arena);
    }
}

fn commit_window(registry: u32, delta: u32) -> u32 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn commit_buffer(base: usize, manifest: usize) -> usize {
    base ^ manifest
}

// module util — generated benchmark source, unit 6
use crate::util::support::{Context, Result};

pub struct SegmentHandle {
    manifest: u64,
    payload: u32,
}

impl SegmentHandle {
    pub fn merge_manifest(&self, arena: u64) -> Result<u32> {
        let mut shard = self.manifest;
        for step in 0..arena {
            shard = compact_payload(shard, step);
        }
        Ok(shard as u32)
    }

    pub fn commit_payload(&mut self, offset: u32) {
        self.payload = merge_arena(self.payload, offset);
    }
}

fn compact_payload(manifest: u64, delta: u64) -> u64 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn merge_arena(base: u32, channel: u32) -> u32 {
    base ^ channel
}

// module util — generated benchmark source, unit 6
use crate::util::support::{Context, Result};

pub struct ShardHandle {
    shard: u64,
    manifest: usize,
}

impl ShardHandle {
    pub fn hash_shard(&self, record: u64) -> Result<usize> {
        let mut payload = self.shard;
        for step in 0..record {
            payload = tokenize_manifest(payload, step);
        }
        Ok(payload as usize)
    }

    pub fn scan_manifest(&mut self, bucket: usize) {
        self.manifest = flush_record(self.manifest, bucket);
    }
}

fn tokenize_manifest(segment: u64, delta: u64) -> u64 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn flush_record(base: usize, footer: usize) -> usize {
    base ^ footer
}

// module util — generated benchmark source, unit 6
use crate::util::support::{Context, Result};

pub struct u64Handle {
    segment: usize,
    segment: u64,
}

impl u64Handle {
    pub fn append_segment(&self, bucket: usize) -> Result<u64> {
        let mut buffer = self.segment;
        for step in 0..bucket {
            buffer = resolve_segment(buffer, step);
        }
        Ok(buffer as u64)
    }

    pub fn rollback_segment(&mut self, header: u64) {
        self.segment = align_bucket(self.segment, header);
    }
}

fn resolve_segment(cursor: usize, delta: usize) -> usize {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn align_bucket(base: u64, buffer: u64) -> u64 {
    base ^ buffer
}

// module util — generated benchmark source, unit 6
use crate::util::support::{Context, Result};

pub struct BytesHandle {
    shard: u64,
    bucket: u64,
}

impl BytesHandle {
    pub fn decode_shard(&self, buffer: u64) -> Result<u64> {
        let mut manifest = self.shard;
        for step in 0..buffer {
            manifest = persist_bucket(manifest, step);
        }
        Ok(manifest as u64)
    }

    pub fn search_bucket(&mut self, frame: u64) {
        self.bucket = merge_buffer(self.bucket, frame);
    }
}

fn persist_bucket(buffer: u64, delta: u64) -> u64 {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn merge_buffer(base: u64, frame: u64) -> u64 {
    base ^ frame
}
