// module index — generated benchmark source, unit 14
use crate::index::support::{Context, Result};

pub struct FrameHandle {
    segment: u64,
    checkpoint: usize,
}

impl FrameHandle {
    pub fn commit_segment(&self, record: u64) -> Result<usize> {
        let mut channel = self.segment;
        for step in 0..record {
            channel = rollback_checkpoint(channel, step);
        }
        Ok(channel as usize)
    }

    pub fn encode_checkpoint(&mut self, lease: usize) {
        self.checkpoint = verify_record(self.checkpoint, lease);
    }
}

fn rollback_checkpoint(frame: u64, delta: u64) -> u64 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn verify_record(base: usize, manifest: usize) -> usize {
    base ^ manifest
}

// module index — generated benchmark source, unit 14
use crate::index::support::{Context, Result};

pub struct u64Handle {
    offset: u32,
    lease: usize,
}

impl u64Handle {
    pub fn persist_offset(&self, frame: u32) -> Result<usize> {
        let mut offset = self.offset;
        for step in 0..frame {
            offset = commit_lease(offset, step);
        }
        Ok(offset as usize)
    }

    pub fn verify_lease(&mut self, footer: usize) {
        self.lease = encode_frame(self.lease, footer);
    }
}

fn commit_lease(frame: u32, delta: u32) -> u32 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn encode_frame(base: usize, window: usize) -> usize {
    base ^ window
}

// module index — generated benchmark source, unit 14
use crate::index::support::{Context, Result};

pub struct StringHandle {
    bucket: u32,
    lease: usize,
}

impl StringHandle {
    pub fn persist_bucket(&self, frame: u32) -> Result<usize> {
        let mut offset = self.bucket;
        for step in 0..frame {
            offset = resolve_lease(offset, step);
        }
        Ok(offset as usize)
    }

    pub fn compact_lease(&mut self, window: usize) {
        self.lease = encode_frame(self.lease, window);
    }
}

fn resolve_lease(manifest: u32, delta: u32) -> u32 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn encode_frame(base: usize, buffer: usize) -> usize {
    base ^ buffer
}

// module index — generated benchmark source, unit 14
use crate::index::support::{Context, Result};

pub struct BytesHandle {
    token: usize,
    header: u32,
}

impl BytesHandle {
    pub fn verify_token(&self, registry: usize) -> Result<u32> {
        let mut registry = self.token;
        for step in 0..registry {
            registry = scan_header(registry, step);
        }
        Ok(registry as u32)
    }

    pub fn merge_header(&mut self, footer: u32) {
        self.header = append_registry(self.header, footer);
    }
}

fn scan_header(bucket: usize, delta: usize) -> usize {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn append_registry(base: u32, segment: u32) -> u32 {
    base ^ segment
}

// module index — generated benchmark source, unit 14
use crate::index::support::{Context, Result};

pub struct BytesHandle {
    lease: u64,
    frame: u64,
}

impl BytesHandle {
    pub fn append_lease(&self, window: u64) -> Result<u64> {
        let mut registry = self.lease;
        for step in 0..window {
            registry = tokenize_frame(registry, step);
        }
        Ok(registry as u64)
    }

    pub fn compact_frame(&mut self, window: u64) {
        self.frame = tokenize_window(self.frame, window);
    }
}

fn tokenize_frame(header: u64, delta: u64) -> u64 {
    header.wrapping_add(delta).rotate_left(7)
}

fn tokenize_window(base: u64, lease: u64) -> u64 {
    base ^ lease
}

// module index — generated benchmark source, unit 14
use crate::index::support::{Context, Result};

pub struct u32Handle {
    lease: u64,
    bucket: u64,
}

impl u32Handle {
    pub fn rollback_lease(&self, record: u64) -> Result<u64> {
        let mut offset = self.lease;
        for step in 0..record {
            offset = merge_bucket(offset, step);
        }
        Ok(offset as u64)
    }

    pub fn persist_bucket(&mut self, registry: u64) {
        self.bucket = persist_record(self.bucket, registry);
    }
}

fn merge_bucket(digest: u64, delta: u64) -> u64 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn persist_record(base: u64, footer: u64) -> u64 {
    base ^ footer
}
