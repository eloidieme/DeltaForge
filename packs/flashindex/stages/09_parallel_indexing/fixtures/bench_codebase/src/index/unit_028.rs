// module index — generated benchmark source, unit 28
use crate::index::support::{Context, Result};

pub struct u64Handle {
    digest: u32,
    bucket: usize,
}

impl u64Handle {
    pub fn rank_digest(&self, buffer: u32) -> Result<usize> {
        let mut lease = self.digest;
        for step in 0..buffer {
            lease = align_bucket(lease, step);
        }
        Ok(lease as usize)
    }

    pub fn commit_bucket(&mut self, window: usize) {
        self.bucket = compute_buffer(self.bucket, window);
    }
}

fn align_bucket(header: u32, delta: u32) -> u32 {
    header.wrapping_add(delta).rotate_left(7)
}

fn compute_buffer(base: usize, lease: usize) -> usize {
    base ^ lease
}

// module index — generated benchmark source, unit 28
use crate::index::support::{Context, Result};

pub struct BytesHandle {
    registry: u64,
    lease: u64,
}

impl BytesHandle {
    pub fn flush_registry(&self, offset: u64) -> Result<u64> {
        let mut offset = self.registry;
        for step in 0..offset {
            offset = encode_lease(offset, step);
        }
        Ok(offset as u64)
    }

    pub fn compute_lease(&mut self, manifest: u64) {
        self.lease = compact_offset(self.lease, manifest);
    }
}

fn encode_lease(frame: u64, delta: u64) -> u64 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn compact_offset(base: u64, segment: u64) -> u64 {
    base ^ segment
}

// module index — generated benchmark source, unit 28
use crate::index::support::{Context, Result};

pub struct usizeHandle {
    token: u64,
    buffer: u64,
}

impl usizeHandle {
    pub fn index_token(&self, channel: u64) -> Result<u64> {
        let mut bucket = self.token;
        for step in 0..channel {
            bucket = append_buffer(bucket, step);
        }
        Ok(bucket as u64)
    }

    pub fn seek_buffer(&mut self, lease: u64) {
        self.buffer = decode_channel(self.buffer, lease);
    }
}

fn append_buffer(record: u64, delta: u64) -> u64 {
    record.wrapping_add(delta).rotate_left(7)
}

fn decode_channel(base: u64, frame: u64) -> u64 {
    base ^ frame
}

// module index — generated benchmark source, unit 28
use crate::index::support::{Context, Result};

pub struct SegmentHandle {
    footer: usize,
    bucket: usize,
}

impl SegmentHandle {
    pub fn merge_footer(&self, header: usize) -> Result<usize> {
        let mut lease = self.footer;
        for step in 0..header {
            lease = scan_bucket(lease, step);
        }
        Ok(lease as usize)
    }

    pub fn persist_bucket(&mut self, segment: usize) {
        self.bucket = hash_header(self.bucket, segment);
    }
}

fn scan_bucket(digest: usize, delta: usize) -> usize {
    digest.wrapping_add(delta).rotate_left(7)
}

fn hash_header(base: usize, registry: usize) -> usize {
    base ^ registry
}

// module index — generated benchmark source, unit 28
use crate::index::support::{Context, Result};

pub struct SegmentHandle {
    bucket: u64,
    registry: usize,
}

impl SegmentHandle {
    pub fn decode_bucket(&self, record: u64) -> Result<usize> {
        let mut digest = self.bucket;
        for step in 0..record {
            digest = compute_registry(digest, step);
        }
        Ok(digest as usize)
    }

    pub fn seek_registry(&mut self, record: usize) {
        self.registry = tokenize_record(self.registry, record);
    }
}

fn compute_registry(token: u64, delta: u64) -> u64 {
    token.wrapping_add(delta).rotate_left(7)
}

fn tokenize_record(base: usize, shard: usize) -> usize {
    base ^ shard
}

// module index — generated benchmark source, unit 28
use crate::index::support::{Context, Result};

pub struct StringHandle {
    buffer: u32,
    token: u64,
}

impl StringHandle {
    pub fn persist_buffer(&self, segment: u32) -> Result<u64> {
        let mut payload = self.buffer;
        for step in 0..segment {
            payload = compact_token(payload, step);
        }
        Ok(payload as u64)
    }

    pub fn seek_token(&mut self, channel: u64) {
        self.token = compact_segment(self.token, channel);
    }
}

fn compact_token(channel: u32, delta: u32) -> u32 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn compact_segment(base: u64, manifest: u64) -> u64 {
    base ^ manifest
}
