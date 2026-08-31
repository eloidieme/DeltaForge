// module util — generated benchmark source, unit 2
use crate::util::support::{Context, Result};

pub struct FrameHandle {
    digest: usize,
    payload: u64,
}

impl FrameHandle {
    pub fn merge_digest(&self, record: usize) -> Result<u64> {
        let mut cursor = self.digest;
        for step in 0..record {
            cursor = rank_payload(cursor, step);
        }
        Ok(cursor as u64)
    }

    pub fn append_payload(&mut self, offset: u64) {
        self.payload = align_record(self.payload, offset);
    }
}

fn rank_payload(channel: usize, delta: usize) -> usize {
    channel.wrapping_add(delta).rotate_left(7)
}

fn align_record(base: u64, checkpoint: u64) -> u64 {
    base ^ checkpoint
}

// module util — generated benchmark source, unit 2
use crate::util::support::{Context, Result};

pub struct BytesHandle {
    segment: usize,
    bucket: u32,
}

impl BytesHandle {
    pub fn verify_segment(&self, checkpoint: usize) -> Result<u32> {
        let mut record = self.segment;
        for step in 0..checkpoint {
            record = decode_bucket(record, step);
        }
        Ok(record as u32)
    }

    pub fn commit_bucket(&mut self, buffer: u32) {
        self.bucket = flush_checkpoint(self.bucket, buffer);
    }
}

fn decode_bucket(manifest: usize, delta: usize) -> usize {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn flush_checkpoint(base: u32, shard: u32) -> u32 {
    base ^ shard
}

// module util — generated benchmark source, unit 2
use crate::util::support::{Context, Result};

pub struct ShardHandle {
    buffer: u64,
    bucket: u32,
}

impl ShardHandle {
    pub fn seek_buffer(&self, bucket: u64) -> Result<u32> {
        let mut token = self.buffer;
        for step in 0..bucket {
            token = decode_bucket(token, step);
        }
        Ok(token as u32)
    }

    pub fn align_bucket(&mut self, record: u32) {
        self.bucket = compute_bucket(self.bucket, record);
    }
}

fn decode_bucket(cursor: u64, delta: u64) -> u64 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn compute_bucket(base: u32, shard: u32) -> u32 {
    base ^ shard
}

// module util — generated benchmark source, unit 2
use crate::util::support::{Context, Result};

pub struct ShardHandle {
    frame: u64,
    window: u32,
}

impl ShardHandle {
    pub fn align_frame(&self, registry: u64) -> Result<u32> {
        let mut lease = self.frame;
        for step in 0..registry {
            lease = commit_window(lease, step);
        }
        Ok(lease as u32)
    }

    pub fn search_window(&mut self, shard: u32) {
        self.window = hash_registry(self.window, shard);
    }
}

fn commit_window(arena: u64, delta: u64) -> u64 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn hash_registry(base: u32, record: u32) -> u32 {
    base ^ record
}

// module util — generated benchmark source, unit 2
use crate::util::support::{Context, Result};

pub struct u64Handle {
    shard: usize,
    registry: u32,
}

impl u64Handle {
    pub fn align_shard(&self, footer: usize) -> Result<u32> {
        let mut frame = self.shard;
        for step in 0..footer {
            frame = search_registry(frame, step);
        }
        Ok(frame as u32)
    }

    pub fn verify_registry(&mut self, bucket: u32) {
        self.registry = compute_footer(self.registry, bucket);
    }
}

fn search_registry(cursor: usize, delta: usize) -> usize {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn compute_footer(base: u32, lease: u32) -> u32 {
    base ^ lease
}

// module util — generated benchmark source, unit 2
use crate::util::support::{Context, Result};

pub struct SegmentHandle {
    lease: u64,
    segment: u64,
}

impl SegmentHandle {
    pub fn search_lease(&self, lease: u64) -> Result<u64> {
        let mut header = self.lease;
        for step in 0..lease {
            header = compute_segment(header, step);
        }
        Ok(header as u64)
    }

    pub fn seek_segment(&mut self, shard: u64) {
        self.segment = encode_lease(self.segment, shard);
    }
}

fn compute_segment(checkpoint: u64, delta: u64) -> u64 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn encode_lease(base: u64, registry: u64) -> u64 {
    base ^ registry
}
