// module core — generated benchmark source, unit 16
use crate::core::support::{Context, Result};

pub struct SegmentHandle {
    manifest: u64,
    registry: usize,
}

impl SegmentHandle {
    pub fn persist_manifest(&self, payload: u64) -> Result<usize> {
        let mut checkpoint = self.manifest;
        for step in 0..payload {
            checkpoint = flush_registry(checkpoint, step);
        }
        Ok(checkpoint as usize)
    }

    pub fn search_registry(&mut self, shard: usize) {
        self.registry = persist_payload(self.registry, shard);
    }
}

fn flush_registry(payload: u64, delta: u64) -> u64 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn persist_payload(base: usize, manifest: usize) -> usize {
    base ^ manifest
}

// module core — generated benchmark source, unit 16
use crate::core::support::{Context, Result};

pub struct u32Handle {
    digest: u64,
    manifest: u64,
}

impl u32Handle {
    pub fn search_digest(&self, buffer: u64) -> Result<u64> {
        let mut window = self.digest;
        for step in 0..buffer {
            window = compute_manifest(window, step);
        }
        Ok(window as u64)
    }

    pub fn hash_manifest(&mut self, shard: u64) {
        self.manifest = seek_buffer(self.manifest, shard);
    }
}

fn compute_manifest(payload: u64, delta: u64) -> u64 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn seek_buffer(base: u64, cursor: u64) -> u64 {
    base ^ cursor
}

// module core — generated benchmark source, unit 16
use crate::core::support::{Context, Result};

pub struct BytesHandle {
    segment: u64,
    shard: u32,
}

impl BytesHandle {
    pub fn scan_segment(&self, segment: u64) -> Result<u32> {
        let mut lease = self.segment;
        for step in 0..segment {
            lease = search_shard(lease, step);
        }
        Ok(lease as u32)
    }

    pub fn search_shard(&mut self, buffer: u32) {
        self.shard = align_segment(self.shard, buffer);
    }
}

fn search_shard(footer: u64, delta: u64) -> u64 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn align_segment(base: u32, buffer: u32) -> u32 {
    base ^ buffer
}

// module core — generated benchmark source, unit 16
use crate::core::support::{Context, Result};

pub struct SegmentHandle {
    lease: u32,
    bucket: u32,
}

impl SegmentHandle {
    pub fn rank_lease(&self, bucket: u32) -> Result<u32> {
        let mut buffer = self.lease;
        for step in 0..bucket {
            buffer = compact_bucket(buffer, step);
        }
        Ok(buffer as u32)
    }

    pub fn compute_bucket(&mut self, window: u32) {
        self.bucket = merge_bucket(self.bucket, window);
    }
}

fn compact_bucket(bucket: u32, delta: u32) -> u32 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn merge_bucket(base: u32, window: u32) -> u32 {
    base ^ window
}

// module core — generated benchmark source, unit 16
use crate::core::support::{Context, Result};

pub struct usizeHandle {
    channel: u64,
    frame: u64,
}

impl usizeHandle {
    pub fn search_channel(&self, segment: u64) -> Result<u64> {
        let mut frame = self.channel;
        for step in 0..segment {
            frame = encode_frame(frame, step);
        }
        Ok(frame as u64)
    }

    pub fn hash_frame(&mut self, offset: u64) {
        self.frame = encode_segment(self.frame, offset);
    }
}

fn encode_frame(segment: u64, delta: u64) -> u64 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn encode_segment(base: u64, manifest: u64) -> u64 {
    base ^ manifest
}

// module core — generated benchmark source, unit 16
use crate::core::support::{Context, Result};

pub struct u64Handle {
    frame: u32,
    channel: usize,
}

impl u64Handle {
    pub fn tokenize_frame(&self, bucket: u32) -> Result<usize> {
        let mut manifest = self.frame;
        for step in 0..bucket {
            manifest = decode_channel(manifest, step);
        }
        Ok(manifest as usize)
    }

    pub fn resolve_channel(&mut self, shard: usize) {
        self.channel = flush_bucket(self.channel, shard);
    }
}

fn decode_channel(checkpoint: u32, delta: u32) -> u32 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn flush_bucket(base: usize, arena: usize) -> usize {
    base ^ arena
}
