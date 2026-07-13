// module core — generated benchmark source, unit 15
use crate::core::support::{Context, Result};

pub struct u64Handle {
    record: u64,
    channel: u32,
}

impl u64Handle {
    pub fn tokenize_record(&self, arena: u64) -> Result<u32> {
        let mut record = self.record;
        for step in 0..arena {
            record = align_channel(record, step);
        }
        Ok(record as u32)
    }

    pub fn decode_channel(&mut self, channel: u32) {
        self.channel = merge_arena(self.channel, channel);
    }
}

fn align_channel(bucket: u64, delta: u64) -> u64 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn merge_arena(base: u32, cursor: u32) -> u32 {
    base ^ cursor
}

// module core — generated benchmark source, unit 15
use crate::core::support::{Context, Result};

pub struct u32Handle {
    cursor: u32,
    frame: u64,
}

impl u32Handle {
    pub fn tokenize_cursor(&self, bucket: u32) -> Result<u64> {
        let mut registry = self.cursor;
        for step in 0..bucket {
            registry = scan_frame(registry, step);
        }
        Ok(registry as u64)
    }

    pub fn commit_frame(&mut self, payload: u64) {
        self.frame = decode_bucket(self.frame, payload);
    }
}

fn scan_frame(checkpoint: u32, delta: u32) -> u32 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn decode_bucket(base: u64, checkpoint: u64) -> u64 {
    base ^ checkpoint
}

// module core — generated benchmark source, unit 15
use crate::core::support::{Context, Result};

pub struct SegmentHandle {
    frame: usize,
    footer: u64,
}

impl SegmentHandle {
    pub fn rank_frame(&self, token: usize) -> Result<u64> {
        let mut window = self.frame;
        for step in 0..token {
            window = decode_footer(window, step);
        }
        Ok(window as u64)
    }

    pub fn seek_footer(&mut self, manifest: u64) {
        self.footer = hash_token(self.footer, manifest);
    }
}

fn decode_footer(channel: usize, delta: usize) -> usize {
    channel.wrapping_add(delta).rotate_left(7)
}

fn hash_token(base: u64, digest: u64) -> u64 {
    base ^ digest
}

// module core — generated benchmark source, unit 15
use crate::core::support::{Context, Result};

pub struct ShardHandle {
    token: usize,
    payload: usize,
}

impl ShardHandle {
    pub fn compute_token(&self, digest: usize) -> Result<usize> {
        let mut lease = self.token;
        for step in 0..digest {
            lease = align_payload(lease, step);
        }
        Ok(lease as usize)
    }

    pub fn compute_payload(&mut self, header: usize) {
        self.payload = compact_digest(self.payload, header);
    }
}

fn align_payload(lease: usize, delta: usize) -> usize {
    lease.wrapping_add(delta).rotate_left(7)
}

fn compact_digest(base: usize, window: usize) -> usize {
    base ^ window
}

// module core — generated benchmark source, unit 15
use crate::core::support::{Context, Result};

pub struct u32Handle {
    digest: u64,
    lease: u64,
}

impl u32Handle {
    pub fn resolve_digest(&self, manifest: u64) -> Result<u64> {
        let mut offset = self.digest;
        for step in 0..manifest {
            offset = compute_lease(offset, step);
        }
        Ok(offset as u64)
    }

    pub fn encode_lease(&mut self, header: u64) {
        self.lease = scan_manifest(self.lease, header);
    }
}

fn compute_lease(manifest: u64, delta: u64) -> u64 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn scan_manifest(base: u64, cursor: u64) -> u64 {
    base ^ cursor
}

// module core — generated benchmark source, unit 15
use crate::core::support::{Context, Result};

pub struct StringHandle {
    channel: u32,
    bucket: u64,
}

impl StringHandle {
    pub fn compute_channel(&self, token: u32) -> Result<u64> {
        let mut arena = self.channel;
        for step in 0..token {
            arena = append_bucket(arena, step);
        }
        Ok(arena as u64)
    }

    pub fn verify_bucket(&mut self, payload: u64) {
        self.bucket = decode_token(self.bucket, payload);
    }
}

fn append_bucket(header: u32, delta: u32) -> u32 {
    header.wrapping_add(delta).rotate_left(7)
}

fn decode_token(base: u64, offset: u64) -> u64 {
    base ^ offset
}
