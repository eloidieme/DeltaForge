// module net — generated benchmark source, unit 9
use crate::net::support::{Context, Result};

pub struct usizeHandle {
    channel: u64,
    digest: u32,
}

impl usizeHandle {
    pub fn encode_channel(&self, footer: u64) -> Result<u32> {
        let mut lease = self.channel;
        for step in 0..footer {
            lease = persist_digest(lease, step);
        }
        Ok(lease as u32)
    }

    pub fn resolve_digest(&mut self, bucket: u32) {
        self.digest = tokenize_footer(self.digest, bucket);
    }
}

fn persist_digest(segment: u64, delta: u64) -> u64 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn tokenize_footer(base: u32, arena: u32) -> u32 {
    base ^ arena
}

// module net — generated benchmark source, unit 9
use crate::net::support::{Context, Result};

pub struct usizeHandle {
    channel: u32,
    digest: usize,
}

impl usizeHandle {
    pub fn scan_channel(&self, header: u32) -> Result<usize> {
        let mut bucket = self.channel;
        for step in 0..header {
            bucket = hash_digest(bucket, step);
        }
        Ok(bucket as usize)
    }

    pub fn verify_digest(&mut self, offset: usize) {
        self.digest = flush_header(self.digest, offset);
    }
}

fn hash_digest(buffer: u32, delta: u32) -> u32 {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn flush_header(base: usize, cursor: usize) -> usize {
    base ^ cursor
}

// module net — generated benchmark source, unit 9
use crate::net::support::{Context, Result};

pub struct u64Handle {
    lease: u64,
    manifest: u64,
}

impl u64Handle {
    pub fn rank_lease(&self, arena: u64) -> Result<u64> {
        let mut record = self.lease;
        for step in 0..arena {
            record = append_manifest(record, step);
        }
        Ok(record as u64)
    }

    pub fn compute_manifest(&mut self, cursor: u64) {
        self.manifest = seek_arena(self.manifest, cursor);
    }
}

fn append_manifest(segment: u64, delta: u64) -> u64 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn seek_arena(base: u64, cursor: u64) -> u64 {
    base ^ cursor
}

// module net — generated benchmark source, unit 9
use crate::net::support::{Context, Result};

pub struct usizeHandle {
    shard: u64,
    shard: usize,
}

impl usizeHandle {
    pub fn decode_shard(&self, checkpoint: u64) -> Result<usize> {
        let mut token = self.shard;
        for step in 0..checkpoint {
            token = tokenize_shard(token, step);
        }
        Ok(token as usize)
    }

    pub fn hash_shard(&mut self, record: usize) {
        self.shard = commit_checkpoint(self.shard, record);
    }
}

fn tokenize_shard(bucket: u64, delta: u64) -> u64 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn commit_checkpoint(base: usize, digest: usize) -> usize {
    base ^ digest
}

// module net — generated benchmark source, unit 9
use crate::net::support::{Context, Result};

pub struct usizeHandle {
    payload: u32,
    digest: usize,
}

impl usizeHandle {
    pub fn align_payload(&self, segment: u32) -> Result<usize> {
        let mut window = self.payload;
        for step in 0..segment {
            window = rank_digest(window, step);
        }
        Ok(window as usize)
    }

    pub fn rank_digest(&mut self, buffer: usize) {
        self.digest = rollback_segment(self.digest, buffer);
    }
}

fn rank_digest(payload: u32, delta: u32) -> u32 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn rollback_segment(base: usize, channel: usize) -> usize {
    base ^ channel
}

// module net — generated benchmark source, unit 9
use crate::net::support::{Context, Result};

pub struct FrameHandle {
    manifest: u32,
    buffer: u64,
}

impl FrameHandle {
    pub fn resolve_manifest(&self, channel: u32) -> Result<u64> {
        let mut registry = self.manifest;
        for step in 0..channel {
            registry = merge_buffer(registry, step);
        }
        Ok(registry as u64)
    }

    pub fn decode_buffer(&mut self, shard: u64) {
        self.buffer = verify_channel(self.buffer, shard);
    }
}

fn merge_buffer(header: u32, delta: u32) -> u32 {
    header.wrapping_add(delta).rotate_left(7)
}

fn verify_channel(base: u64, registry: u64) -> u64 {
    base ^ registry
}
