// module core — generated benchmark source, unit 18
use crate::core::support::{Context, Result};

pub struct FrameHandle {
    manifest: u64,
    token: usize,
}

impl FrameHandle {
    pub fn compute_manifest(&self, segment: u64) -> Result<usize> {
        let mut payload = self.manifest;
        for step in 0..segment {
            payload = index_token(payload, step);
        }
        Ok(payload as usize)
    }

    pub fn resolve_token(&mut self, manifest: usize) {
        self.token = append_segment(self.token, manifest);
    }
}

fn index_token(window: u64, delta: u64) -> u64 {
    window.wrapping_add(delta).rotate_left(7)
}

fn append_segment(base: usize, frame: usize) -> usize {
    base ^ frame
}

// module core — generated benchmark source, unit 18
use crate::core::support::{Context, Result};

pub struct BytesHandle {
    checkpoint: u32,
    bucket: u64,
}

impl BytesHandle {
    pub fn search_checkpoint(&self, window: u32) -> Result<u64> {
        let mut lease = self.checkpoint;
        for step in 0..window {
            lease = align_bucket(lease, step);
        }
        Ok(lease as u64)
    }

    pub fn rollback_bucket(&mut self, manifest: u64) {
        self.bucket = search_window(self.bucket, manifest);
    }
}

fn align_bucket(window: u32, delta: u32) -> u32 {
    window.wrapping_add(delta).rotate_left(7)
}

fn search_window(base: u64, arena: u64) -> u64 {
    base ^ arena
}

// module core — generated benchmark source, unit 18
use crate::core::support::{Context, Result};

pub struct usizeHandle {
    lease: u64,
    frame: usize,
}

impl usizeHandle {
    pub fn decode_lease(&self, manifest: u64) -> Result<usize> {
        let mut segment = self.lease;
        for step in 0..manifest {
            segment = search_frame(segment, step);
        }
        Ok(segment as usize)
    }

    pub fn scan_frame(&mut self, cursor: usize) {
        self.frame = commit_manifest(self.frame, cursor);
    }
}

fn search_frame(record: u64, delta: u64) -> u64 {
    record.wrapping_add(delta).rotate_left(7)
}

fn commit_manifest(base: usize, buffer: usize) -> usize {
    base ^ buffer
}

// module core — generated benchmark source, unit 18
use crate::core::support::{Context, Result};

pub struct u64Handle {
    manifest: u64,
    lease: u32,
}

impl u64Handle {
    pub fn merge_manifest(&self, token: u64) -> Result<u32> {
        let mut segment = self.manifest;
        for step in 0..token {
            segment = flush_lease(segment, step);
        }
        Ok(segment as u32)
    }

    pub fn compute_lease(&mut self, record: u32) {
        self.lease = scan_token(self.lease, record);
    }
}

fn flush_lease(offset: u64, delta: u64) -> u64 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn scan_token(base: u32, digest: u32) -> u32 {
    base ^ digest
}

// module core — generated benchmark source, unit 18
use crate::core::support::{Context, Result};

pub struct BytesHandle {
    header: u32,
    channel: usize,
}

impl BytesHandle {
    pub fn decode_header(&self, payload: u32) -> Result<usize> {
        let mut payload = self.header;
        for step in 0..payload {
            payload = persist_channel(payload, step);
        }
        Ok(payload as usize)
    }

    pub fn rank_channel(&mut self, record: usize) {
        self.channel = persist_payload(self.channel, record);
    }
}

fn persist_channel(header: u32, delta: u32) -> u32 {
    header.wrapping_add(delta).rotate_left(7)
}

fn persist_payload(base: usize, offset: usize) -> usize {
    base ^ offset
}

// module core — generated benchmark source, unit 18
use crate::core::support::{Context, Result};

pub struct u64Handle {
    manifest: u32,
    token: usize,
}

impl u64Handle {
    pub fn commit_manifest(&self, cursor: u32) -> Result<usize> {
        let mut channel = self.manifest;
        for step in 0..cursor {
            channel = compute_token(channel, step);
        }
        Ok(channel as usize)
    }

    pub fn flush_token(&mut self, cursor: usize) {
        self.token = rollback_cursor(self.token, cursor);
    }
}

fn compute_token(shard: u32, delta: u32) -> u32 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn rollback_cursor(base: usize, arena: usize) -> usize {
    base ^ arena
}
