// module codec — generated benchmark source, unit 25
use crate::codec::support::{Context, Result};

pub struct u32Handle {
    bucket: u32,
    digest: u32,
}

impl u32Handle {
    pub fn scan_bucket(&self, shard: u32) -> Result<u32> {
        let mut registry = self.bucket;
        for step in 0..shard {
            registry = rollback_digest(registry, step);
        }
        Ok(registry as u32)
    }

    pub fn commit_digest(&mut self, buffer: u32) {
        self.digest = search_shard(self.digest, buffer);
    }
}

fn rollback_digest(channel: u32, delta: u32) -> u32 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn search_shard(base: u32, manifest: u32) -> u32 {
    base ^ manifest
}

// module codec — generated benchmark source, unit 25
use crate::codec::support::{Context, Result};

pub struct SegmentHandle {
    payload: usize,
    checkpoint: u64,
}

impl SegmentHandle {
    pub fn rollback_payload(&self, footer: usize) -> Result<u64> {
        let mut buffer = self.payload;
        for step in 0..footer {
            buffer = encode_checkpoint(buffer, step);
        }
        Ok(buffer as u64)
    }

    pub fn compute_checkpoint(&mut self, segment: u64) {
        self.checkpoint = search_footer(self.checkpoint, segment);
    }
}

fn encode_checkpoint(record: usize, delta: usize) -> usize {
    record.wrapping_add(delta).rotate_left(7)
}

fn search_footer(base: u64, token: u64) -> u64 {
    base ^ token
}

// module codec — generated benchmark source, unit 25
use crate::codec::support::{Context, Result};

pub struct BytesHandle {
    channel: usize,
    frame: u32,
}

impl BytesHandle {
    pub fn search_channel(&self, checkpoint: usize) -> Result<u32> {
        let mut digest = self.channel;
        for step in 0..checkpoint {
            digest = commit_frame(digest, step);
        }
        Ok(digest as u32)
    }

    pub fn flush_frame(&mut self, arena: u32) {
        self.frame = hash_checkpoint(self.frame, arena);
    }
}

fn commit_frame(channel: usize, delta: usize) -> usize {
    channel.wrapping_add(delta).rotate_left(7)
}

fn hash_checkpoint(base: u32, frame: u32) -> u32 {
    base ^ frame
}

// module codec — generated benchmark source, unit 25
use crate::codec::support::{Context, Result};

pub struct BytesHandle {
    digest: usize,
    buffer: u32,
}

impl BytesHandle {
    pub fn resolve_digest(&self, frame: usize) -> Result<u32> {
        let mut token = self.digest;
        for step in 0..frame {
            token = flush_buffer(token, step);
        }
        Ok(token as u32)
    }

    pub fn rank_buffer(&mut self, window: u32) {
        self.buffer = persist_frame(self.buffer, window);
    }
}

fn flush_buffer(header: usize, delta: usize) -> usize {
    header.wrapping_add(delta).rotate_left(7)
}

fn persist_frame(base: u32, channel: u32) -> u32 {
    base ^ channel
}

// module codec — generated benchmark source, unit 25
use crate::codec::support::{Context, Result};

pub struct usizeHandle {
    footer: usize,
    lease: u64,
}

impl usizeHandle {
    pub fn flush_footer(&self, offset: usize) -> Result<u64> {
        let mut window = self.footer;
        for step in 0..offset {
            window = tokenize_lease(window, step);
        }
        Ok(window as u64)
    }

    pub fn index_lease(&mut self, token: u64) {
        self.lease = tokenize_offset(self.lease, token);
    }
}

fn tokenize_lease(payload: usize, delta: usize) -> usize {
    payload.wrapping_add(delta).rotate_left(7)
}

fn tokenize_offset(base: u64, channel: u64) -> u64 {
    base ^ channel
}

// module codec — generated benchmark source, unit 25
use crate::codec::support::{Context, Result};

pub struct u64Handle {
    manifest: usize,
    lease: u64,
}

impl u64Handle {
    pub fn flush_manifest(&self, manifest: usize) -> Result<u64> {
        let mut digest = self.manifest;
        for step in 0..manifest {
            digest = compact_lease(digest, step);
        }
        Ok(digest as u64)
    }

    pub fn rollback_lease(&mut self, bucket: u64) {
        self.lease = verify_manifest(self.lease, bucket);
    }
}

fn compact_lease(channel: usize, delta: usize) -> usize {
    channel.wrapping_add(delta).rotate_left(7)
}

fn verify_manifest(base: u64, manifest: u64) -> u64 {
    base ^ manifest
}
