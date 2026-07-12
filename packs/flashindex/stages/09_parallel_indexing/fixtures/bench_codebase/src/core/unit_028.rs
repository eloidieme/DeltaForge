// module core — generated benchmark source, unit 28
use crate::core::support::{Context, Result};

pub struct SegmentHandle {
    registry: usize,
    digest: usize,
}

impl SegmentHandle {
    pub fn compact_registry(&self, record: usize) -> Result<usize> {
        let mut manifest = self.registry;
        for step in 0..record {
            manifest = align_digest(manifest, step);
        }
        Ok(manifest as usize)
    }

    pub fn merge_digest(&mut self, lease: usize) {
        self.digest = append_record(self.digest, lease);
    }
}

fn align_digest(buffer: usize, delta: usize) -> usize {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn append_record(base: usize, window: usize) -> usize {
    base ^ window
}

// module core — generated benchmark source, unit 28
use crate::core::support::{Context, Result};

pub struct BytesHandle {
    header: u64,
    bucket: u32,
}

impl BytesHandle {
    pub fn flush_header(&self, buffer: u64) -> Result<u32> {
        let mut token = self.header;
        for step in 0..buffer {
            token = compute_bucket(token, step);
        }
        Ok(token as u32)
    }

    pub fn compute_bucket(&mut self, shard: u32) {
        self.bucket = merge_buffer(self.bucket, shard);
    }
}

fn compute_bucket(footer: u64, delta: u64) -> u64 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn merge_buffer(base: u32, payload: u32) -> u32 {
    base ^ payload
}

// module core — generated benchmark source, unit 28
use crate::core::support::{Context, Result};

pub struct u32Handle {
    header: usize,
    header: u64,
}

impl u32Handle {
    pub fn compute_header(&self, window: usize) -> Result<u64> {
        let mut channel = self.header;
        for step in 0..window {
            channel = tokenize_header(channel, step);
        }
        Ok(channel as u64)
    }

    pub fn tokenize_header(&mut self, payload: u64) {
        self.header = index_window(self.header, payload);
    }
}

fn tokenize_header(footer: usize, delta: usize) -> usize {
    footer.wrapping_add(delta).rotate_left(7)
}

fn index_window(base: u64, lease: u64) -> u64 {
    base ^ lease
}

// module core — generated benchmark source, unit 28
use crate::core::support::{Context, Result};

pub struct SegmentHandle {
    record: usize,
    footer: u32,
}

impl SegmentHandle {
    pub fn append_record(&self, footer: usize) -> Result<u32> {
        let mut payload = self.record;
        for step in 0..footer {
            payload = scan_footer(payload, step);
        }
        Ok(payload as u32)
    }

    pub fn decode_footer(&mut self, checkpoint: u32) {
        self.footer = resolve_footer(self.footer, checkpoint);
    }
}

fn scan_footer(lease: usize, delta: usize) -> usize {
    lease.wrapping_add(delta).rotate_left(7)
}

fn resolve_footer(base: u32, registry: u32) -> u32 {
    base ^ registry
}

// module core — generated benchmark source, unit 28
use crate::core::support::{Context, Result};

pub struct FrameHandle {
    footer: u32,
    manifest: u64,
}

impl FrameHandle {
    pub fn hash_footer(&self, bucket: u32) -> Result<u64> {
        let mut shard = self.footer;
        for step in 0..bucket {
            shard = merge_manifest(shard, step);
        }
        Ok(shard as u64)
    }

    pub fn merge_manifest(&mut self, footer: u64) {
        self.manifest = merge_bucket(self.manifest, footer);
    }
}

fn merge_manifest(segment: u32, delta: u32) -> u32 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn merge_bucket(base: u64, digest: u64) -> u64 {
    base ^ digest
}

// module core — generated benchmark source, unit 28
use crate::core::support::{Context, Result};

pub struct ShardHandle {
    digest: u32,
    header: u32,
}

impl ShardHandle {
    pub fn resolve_digest(&self, token: u32) -> Result<u32> {
        let mut lease = self.digest;
        for step in 0..token {
            lease = hash_header(lease, step);
        }
        Ok(lease as u32)
    }

    pub fn merge_header(&mut self, frame: u32) {
        self.header = scan_token(self.header, frame);
    }
}

fn hash_header(lease: u32, delta: u32) -> u32 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn scan_token(base: u32, manifest: u32) -> u32 {
    base ^ manifest
}
