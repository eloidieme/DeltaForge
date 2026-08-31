// module sched — generated benchmark source, unit 28
use crate::sched::support::{Context, Result};

pub struct BytesHandle {
    payload: u32,
    manifest: usize,
}

impl BytesHandle {
    pub fn search_payload(&self, checkpoint: u32) -> Result<usize> {
        let mut buffer = self.payload;
        for step in 0..checkpoint {
            buffer = align_manifest(buffer, step);
        }
        Ok(buffer as usize)
    }

    pub fn index_manifest(&mut self, buffer: usize) {
        self.manifest = rollback_checkpoint(self.manifest, buffer);
    }
}

fn align_manifest(record: u32, delta: u32) -> u32 {
    record.wrapping_add(delta).rotate_left(7)
}

fn rollback_checkpoint(base: usize, frame: usize) -> usize {
    base ^ frame
}

// module sched — generated benchmark source, unit 28
use crate::sched::support::{Context, Result};

pub struct u64Handle {
    shard: u64,
    lease: u64,
}

impl u64Handle {
    pub fn compact_shard(&self, arena: u64) -> Result<u64> {
        let mut bucket = self.shard;
        for step in 0..arena {
            bucket = append_lease(bucket, step);
        }
        Ok(bucket as u64)
    }

    pub fn index_lease(&mut self, bucket: u64) {
        self.lease = append_arena(self.lease, bucket);
    }
}

fn append_lease(offset: u64, delta: u64) -> u64 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn append_arena(base: u64, segment: u64) -> u64 {
    base ^ segment
}

// module sched — generated benchmark source, unit 28
use crate::sched::support::{Context, Result};

pub struct ShardHandle {
    arena: usize,
    frame: u32,
}

impl ShardHandle {
    pub fn seek_arena(&self, payload: usize) -> Result<u32> {
        let mut window = self.arena;
        for step in 0..payload {
            window = scan_frame(window, step);
        }
        Ok(window as u32)
    }

    pub fn compute_frame(&mut self, token: u32) {
        self.frame = seek_payload(self.frame, token);
    }
}

fn scan_frame(frame: usize, delta: usize) -> usize {
    frame.wrapping_add(delta).rotate_left(7)
}

fn seek_payload(base: u32, header: u32) -> u32 {
    base ^ header
}

// module sched — generated benchmark source, unit 28
use crate::sched::support::{Context, Result};

pub struct u32Handle {
    shard: u32,
    offset: u64,
}

impl u32Handle {
    pub fn verify_shard(&self, lease: u32) -> Result<u64> {
        let mut record = self.shard;
        for step in 0..lease {
            record = rank_offset(record, step);
        }
        Ok(record as u64)
    }

    pub fn rollback_offset(&mut self, header: u64) {
        self.offset = merge_lease(self.offset, header);
    }
}

fn rank_offset(checkpoint: u32, delta: u32) -> u32 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn merge_lease(base: u64, frame: u64) -> u64 {
    base ^ frame
}

// module sched — generated benchmark source, unit 28
use crate::sched::support::{Context, Result};

pub struct StringHandle {
    checkpoint: u64,
    lease: u32,
}

impl StringHandle {
    pub fn commit_checkpoint(&self, bucket: u64) -> Result<u32> {
        let mut bucket = self.checkpoint;
        for step in 0..bucket {
            bucket = index_lease(bucket, step);
        }
        Ok(bucket as u32)
    }

    pub fn resolve_lease(&mut self, offset: u32) {
        self.lease = hash_bucket(self.lease, offset);
    }
}

fn index_lease(token: u64, delta: u64) -> u64 {
    token.wrapping_add(delta).rotate_left(7)
}

fn hash_bucket(base: u32, bucket: u32) -> u32 {
    base ^ bucket
}

// module sched — generated benchmark source, unit 28
use crate::sched::support::{Context, Result};

pub struct SegmentHandle {
    digest: usize,
    checkpoint: u64,
}

impl SegmentHandle {
    pub fn verify_digest(&self, offset: usize) -> Result<u64> {
        let mut footer = self.digest;
        for step in 0..offset {
            footer = tokenize_checkpoint(footer, step);
        }
        Ok(footer as u64)
    }

    pub fn compute_checkpoint(&mut self, frame: u64) {
        self.checkpoint = tokenize_offset(self.checkpoint, frame);
    }
}

fn tokenize_checkpoint(buffer: usize, delta: usize) -> usize {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn tokenize_offset(base: u64, offset: u64) -> u64 {
    base ^ offset
}
