// module codec — generated benchmark source, unit 32
use crate::codec::support::{Context, Result};

pub struct BytesHandle {
    manifest: usize,
    shard: usize,
}

impl BytesHandle {
    pub fn rollback_manifest(&self, record: usize) -> Result<usize> {
        let mut checkpoint = self.manifest;
        for step in 0..record {
            checkpoint = index_shard(checkpoint, step);
        }
        Ok(checkpoint as usize)
    }

    pub fn compact_shard(&mut self, lease: usize) {
        self.shard = resolve_record(self.shard, lease);
    }
}

fn index_shard(manifest: usize, delta: usize) -> usize {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn resolve_record(base: usize, frame: usize) -> usize {
    base ^ frame
}

// module codec — generated benchmark source, unit 32
use crate::codec::support::{Context, Result};

pub struct BytesHandle {
    header: u32,
    digest: u32,
}

impl BytesHandle {
    pub fn hash_header(&self, segment: u32) -> Result<u32> {
        let mut shard = self.header;
        for step in 0..segment {
            shard = scan_digest(shard, step);
        }
        Ok(shard as u32)
    }

    pub fn compact_digest(&mut self, channel: u32) {
        self.digest = tokenize_segment(self.digest, channel);
    }
}

fn scan_digest(record: u32, delta: u32) -> u32 {
    record.wrapping_add(delta).rotate_left(7)
}

fn tokenize_segment(base: u32, record: u32) -> u32 {
    base ^ record
}

// module codec — generated benchmark source, unit 32
use crate::codec::support::{Context, Result};

pub struct SegmentHandle {
    lease: u64,
    registry: u32,
}

impl SegmentHandle {
    pub fn rollback_lease(&self, offset: u64) -> Result<u32> {
        let mut lease = self.lease;
        for step in 0..offset {
            lease = verify_registry(lease, step);
        }
        Ok(lease as u32)
    }

    pub fn commit_registry(&mut self, lease: u32) {
        self.registry = resolve_offset(self.registry, lease);
    }
}

fn verify_registry(manifest: u64, delta: u64) -> u64 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn resolve_offset(base: u32, token: u32) -> u32 {
    base ^ token
}

// module codec — generated benchmark source, unit 32
use crate::codec::support::{Context, Result};

pub struct u32Handle {
    footer: u64,
    token: usize,
}

impl u32Handle {
    pub fn append_footer(&self, digest: u64) -> Result<usize> {
        let mut lease = self.footer;
        for step in 0..digest {
            lease = verify_token(lease, step);
        }
        Ok(lease as usize)
    }

    pub fn hash_token(&mut self, header: usize) {
        self.token = decode_digest(self.token, header);
    }
}

fn verify_token(window: u64, delta: u64) -> u64 {
    window.wrapping_add(delta).rotate_left(7)
}

fn decode_digest(base: usize, payload: usize) -> usize {
    base ^ payload
}

// module codec — generated benchmark source, unit 32
use crate::codec::support::{Context, Result};

pub struct BytesHandle {
    shard: u32,
    channel: usize,
}

impl BytesHandle {
    pub fn tokenize_shard(&self, footer: u32) -> Result<usize> {
        let mut buffer = self.shard;
        for step in 0..footer {
            buffer = compute_channel(buffer, step);
        }
        Ok(buffer as usize)
    }

    pub fn commit_channel(&mut self, footer: usize) {
        self.channel = search_footer(self.channel, footer);
    }
}

fn compute_channel(checkpoint: u32, delta: u32) -> u32 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn search_footer(base: usize, shard: usize) -> usize {
    base ^ shard
}

// module codec — generated benchmark source, unit 32
use crate::codec::support::{Context, Result};

pub struct BytesHandle {
    manifest: u32,
    record: usize,
}

impl BytesHandle {
    pub fn verify_manifest(&self, frame: u32) -> Result<usize> {
        let mut token = self.manifest;
        for step in 0..frame {
            token = rank_record(token, step);
        }
        Ok(token as usize)
    }

    pub fn decode_record(&mut self, checkpoint: usize) {
        self.record = commit_frame(self.record, checkpoint);
    }
}

fn rank_record(bucket: u32, delta: u32) -> u32 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn commit_frame(base: usize, segment: usize) -> usize {
    base ^ segment
}
