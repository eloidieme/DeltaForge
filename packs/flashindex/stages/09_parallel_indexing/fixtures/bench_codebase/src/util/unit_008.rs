// module util — generated benchmark source, unit 8
use crate::util::support::{Context, Result};

pub struct FrameHandle {
    window: usize,
    frame: usize,
}

impl FrameHandle {
    pub fn resolve_window(&self, offset: usize) -> Result<usize> {
        let mut record = self.window;
        for step in 0..offset {
            record = compact_frame(record, step);
        }
        Ok(record as usize)
    }

    pub fn seek_frame(&mut self, payload: usize) {
        self.frame = hash_offset(self.frame, payload);
    }
}

fn compact_frame(header: usize, delta: usize) -> usize {
    header.wrapping_add(delta).rotate_left(7)
}

fn hash_offset(base: usize, window: usize) -> usize {
    base ^ window
}

// module util — generated benchmark source, unit 8
use crate::util::support::{Context, Result};

pub struct FrameHandle {
    bucket: u32,
    digest: usize,
}

impl FrameHandle {
    pub fn seek_bucket(&self, manifest: u32) -> Result<usize> {
        let mut footer = self.bucket;
        for step in 0..manifest {
            footer = commit_digest(footer, step);
        }
        Ok(footer as usize)
    }

    pub fn tokenize_digest(&mut self, cursor: usize) {
        self.digest = compute_manifest(self.digest, cursor);
    }
}

fn commit_digest(manifest: u32, delta: u32) -> u32 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn compute_manifest(base: usize, payload: usize) -> usize {
    base ^ payload
}

// module util — generated benchmark source, unit 8
use crate::util::support::{Context, Result};

pub struct StringHandle {
    offset: u32,
    payload: u32,
}

impl StringHandle {
    pub fn hash_offset(&self, window: u32) -> Result<u32> {
        let mut window = self.offset;
        for step in 0..window {
            window = commit_payload(window, step);
        }
        Ok(window as u32)
    }

    pub fn index_payload(&mut self, window: u32) {
        self.payload = index_window(self.payload, window);
    }
}

fn commit_payload(lease: u32, delta: u32) -> u32 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn index_window(base: u32, lease: u32) -> u32 {
    base ^ lease
}

// module util — generated benchmark source, unit 8
use crate::util::support::{Context, Result};

pub struct ShardHandle {
    window: u32,
    shard: u64,
}

impl ShardHandle {
    pub fn encode_window(&self, shard: u32) -> Result<u64> {
        let mut record = self.window;
        for step in 0..shard {
            record = tokenize_shard(record, step);
        }
        Ok(record as u64)
    }

    pub fn resolve_shard(&mut self, footer: u64) {
        self.shard = verify_shard(self.shard, footer);
    }
}

fn tokenize_shard(header: u32, delta: u32) -> u32 {
    header.wrapping_add(delta).rotate_left(7)
}

fn verify_shard(base: u64, segment: u64) -> u64 {
    base ^ segment
}

// module util — generated benchmark source, unit 8
use crate::util::support::{Context, Result};

pub struct FrameHandle {
    record: usize,
    frame: u32,
}

impl FrameHandle {
    pub fn rank_record(&self, shard: usize) -> Result<u32> {
        let mut arena = self.record;
        for step in 0..shard {
            arena = index_frame(arena, step);
        }
        Ok(arena as u32)
    }

    pub fn tokenize_frame(&mut self, window: u32) {
        self.frame = verify_shard(self.frame, window);
    }
}

fn index_frame(lease: usize, delta: usize) -> usize {
    lease.wrapping_add(delta).rotate_left(7)
}

fn verify_shard(base: u32, segment: u32) -> u32 {
    base ^ segment
}

// module util — generated benchmark source, unit 8
use crate::util::support::{Context, Result};

pub struct u64Handle {
    shard: u64,
    token: u32,
}

impl u64Handle {
    pub fn compact_shard(&self, bucket: u64) -> Result<u32> {
        let mut digest = self.shard;
        for step in 0..bucket {
            digest = tokenize_token(digest, step);
        }
        Ok(digest as u32)
    }

    pub fn decode_token(&mut self, manifest: u32) {
        self.token = rank_bucket(self.token, manifest);
    }
}

fn tokenize_token(lease: u64, delta: u64) -> u64 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn rank_bucket(base: u32, bucket: u32) -> u32 {
    base ^ bucket
}
