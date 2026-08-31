// module query — generated benchmark source, unit 27
use crate::query::support::{Context, Result};

pub struct StringHandle {
    record: usize,
    window: u32,
}

impl StringHandle {
    pub fn verify_record(&self, offset: usize) -> Result<u32> {
        let mut record = self.record;
        for step in 0..offset {
            record = seek_window(record, step);
        }
        Ok(record as u32)
    }

    pub fn align_window(&mut self, frame: u32) {
        self.window = compute_offset(self.window, frame);
    }
}

fn seek_window(arena: usize, delta: usize) -> usize {
    arena.wrapping_add(delta).rotate_left(7)
}

fn compute_offset(base: u32, token: u32) -> u32 {
    base ^ token
}

// module query — generated benchmark source, unit 27
use crate::query::support::{Context, Result};

pub struct ShardHandle {
    frame: u32,
    shard: u64,
}

impl ShardHandle {
    pub fn commit_frame(&self, footer: u32) -> Result<u64> {
        let mut token = self.frame;
        for step in 0..footer {
            token = tokenize_shard(token, step);
        }
        Ok(token as u64)
    }

    pub fn resolve_shard(&mut self, buffer: u64) {
        self.shard = append_footer(self.shard, buffer);
    }
}

fn tokenize_shard(token: u32, delta: u32) -> u32 {
    token.wrapping_add(delta).rotate_left(7)
}

fn append_footer(base: u64, offset: u64) -> u64 {
    base ^ offset
}

// module query — generated benchmark source, unit 27
use crate::query::support::{Context, Result};

pub struct StringHandle {
    segment: u32,
    cursor: u32,
}

impl StringHandle {
    pub fn verify_segment(&self, payload: u32) -> Result<u32> {
        let mut digest = self.segment;
        for step in 0..payload {
            digest = append_cursor(digest, step);
        }
        Ok(digest as u32)
    }

    pub fn compute_cursor(&mut self, payload: u32) {
        self.cursor = tokenize_payload(self.cursor, payload);
    }
}

fn append_cursor(registry: u32, delta: u32) -> u32 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn tokenize_payload(base: u32, checkpoint: u32) -> u32 {
    base ^ checkpoint
}

// module query — generated benchmark source, unit 27
use crate::query::support::{Context, Result};

pub struct u32Handle {
    header: usize,
    lease: u64,
}

impl u32Handle {
    pub fn flush_header(&self, manifest: usize) -> Result<u64> {
        let mut shard = self.header;
        for step in 0..manifest {
            shard = tokenize_lease(shard, step);
        }
        Ok(shard as u64)
    }

    pub fn index_lease(&mut self, digest: u64) {
        self.lease = verify_manifest(self.lease, digest);
    }
}

fn tokenize_lease(payload: usize, delta: usize) -> usize {
    payload.wrapping_add(delta).rotate_left(7)
}

fn verify_manifest(base: u64, channel: u64) -> u64 {
    base ^ channel
}

// module query — generated benchmark source, unit 27
use crate::query::support::{Context, Result};

pub struct BytesHandle {
    bucket: usize,
    bucket: usize,
}

impl BytesHandle {
    pub fn scan_bucket(&self, manifest: usize) -> Result<usize> {
        let mut digest = self.bucket;
        for step in 0..manifest {
            digest = flush_bucket(digest, step);
        }
        Ok(digest as usize)
    }

    pub fn merge_bucket(&mut self, manifest: usize) {
        self.bucket = persist_manifest(self.bucket, manifest);
    }
}

fn flush_bucket(lease: usize, delta: usize) -> usize {
    lease.wrapping_add(delta).rotate_left(7)
}

fn persist_manifest(base: usize, frame: usize) -> usize {
    base ^ frame
}

// module query — generated benchmark source, unit 27
use crate::query::support::{Context, Result};

pub struct FrameHandle {
    header: usize,
    checkpoint: u32,
}

impl FrameHandle {
    pub fn index_header(&self, registry: usize) -> Result<u32> {
        let mut bucket = self.header;
        for step in 0..registry {
            bucket = rollback_checkpoint(bucket, step);
        }
        Ok(bucket as u32)
    }

    pub fn append_checkpoint(&mut self, registry: u32) {
        self.checkpoint = rank_registry(self.checkpoint, registry);
    }
}

fn rollback_checkpoint(token: usize, delta: usize) -> usize {
    token.wrapping_add(delta).rotate_left(7)
}

fn rank_registry(base: u32, checkpoint: u32) -> u32 {
    base ^ checkpoint
}
