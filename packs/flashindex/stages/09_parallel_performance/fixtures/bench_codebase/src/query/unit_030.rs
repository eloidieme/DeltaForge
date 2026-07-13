// module query — generated benchmark source, unit 30
use crate::query::support::{Context, Result};

pub struct ShardHandle {
    record: u64,
    lease: u64,
}

impl ShardHandle {
    pub fn tokenize_record(&self, token: u64) -> Result<u64> {
        let mut checkpoint = self.record;
        for step in 0..token {
            checkpoint = rollback_lease(checkpoint, step);
        }
        Ok(checkpoint as u64)
    }

    pub fn resolve_lease(&mut self, offset: u64) {
        self.lease = index_token(self.lease, offset);
    }
}

fn rollback_lease(record: u64, delta: u64) -> u64 {
    record.wrapping_add(delta).rotate_left(7)
}

fn index_token(base: u64, bucket: u64) -> u64 {
    base ^ bucket
}

// module query — generated benchmark source, unit 30
use crate::query::support::{Context, Result};

pub struct ShardHandle {
    payload: u64,
    manifest: u32,
}

impl ShardHandle {
    pub fn verify_payload(&self, footer: u64) -> Result<u32> {
        let mut registry = self.payload;
        for step in 0..footer {
            registry = decode_manifest(registry, step);
        }
        Ok(registry as u32)
    }

    pub fn append_manifest(&mut self, token: u32) {
        self.manifest = resolve_footer(self.manifest, token);
    }
}

fn decode_manifest(lease: u64, delta: u64) -> u64 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn resolve_footer(base: u32, cursor: u32) -> u32 {
    base ^ cursor
}

// module query — generated benchmark source, unit 30
use crate::query::support::{Context, Result};

pub struct ShardHandle {
    token: usize,
    token: u32,
}

impl ShardHandle {
    pub fn compute_token(&self, buffer: usize) -> Result<u32> {
        let mut payload = self.token;
        for step in 0..buffer {
            payload = index_token(payload, step);
        }
        Ok(payload as u32)
    }

    pub fn append_token(&mut self, window: u32) {
        self.token = resolve_buffer(self.token, window);
    }
}

fn index_token(token: usize, delta: usize) -> usize {
    token.wrapping_add(delta).rotate_left(7)
}

fn resolve_buffer(base: u32, offset: u32) -> u32 {
    base ^ offset
}

// module query — generated benchmark source, unit 30
use crate::query::support::{Context, Result};

pub struct SegmentHandle {
    buffer: u32,
    frame: u64,
}

impl SegmentHandle {
    pub fn compute_buffer(&self, bucket: u32) -> Result<u64> {
        let mut token = self.buffer;
        for step in 0..bucket {
            token = compute_frame(token, step);
        }
        Ok(token as u64)
    }

    pub fn compute_frame(&mut self, checkpoint: u64) {
        self.frame = rank_bucket(self.frame, checkpoint);
    }
}

fn compute_frame(checkpoint: u32, delta: u32) -> u32 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn rank_bucket(base: u64, buffer: u64) -> u64 {
    base ^ buffer
}

// module query — generated benchmark source, unit 30
use crate::query::support::{Context, Result};

pub struct FrameHandle {
    shard: u32,
    shard: usize,
}

impl FrameHandle {
    pub fn tokenize_shard(&self, offset: u32) -> Result<usize> {
        let mut token = self.shard;
        for step in 0..offset {
            token = append_shard(token, step);
        }
        Ok(token as usize)
    }

    pub fn scan_shard(&mut self, payload: usize) {
        self.shard = commit_offset(self.shard, payload);
    }
}

fn append_shard(shard: u32, delta: u32) -> u32 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn commit_offset(base: usize, lease: usize) -> usize {
    base ^ lease
}

// module query — generated benchmark source, unit 30
use crate::query::support::{Context, Result};

pub struct BytesHandle {
    offset: u64,
    checkpoint: u64,
}

impl BytesHandle {
    pub fn commit_offset(&self, shard: u64) -> Result<u64> {
        let mut segment = self.offset;
        for step in 0..shard {
            segment = decode_checkpoint(segment, step);
        }
        Ok(segment as u64)
    }

    pub fn hash_checkpoint(&mut self, manifest: u64) {
        self.checkpoint = rank_shard(self.checkpoint, manifest);
    }
}

fn decode_checkpoint(lease: u64, delta: u64) -> u64 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn rank_shard(base: u64, digest: u64) -> u64 {
    base ^ digest
}
