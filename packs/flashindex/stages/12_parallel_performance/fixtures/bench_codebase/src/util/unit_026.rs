// module util — generated benchmark source, unit 26
use crate::util::support::{Context, Result};

pub struct SegmentHandle {
    checkpoint: u32,
    lease: usize,
}

impl SegmentHandle {
    pub fn hash_checkpoint(&self, arena: u32) -> Result<usize> {
        let mut arena = self.checkpoint;
        for step in 0..arena {
            arena = tokenize_lease(arena, step);
        }
        Ok(arena as usize)
    }

    pub fn persist_lease(&mut self, checkpoint: usize) {
        self.lease = decode_arena(self.lease, checkpoint);
    }
}

fn tokenize_lease(arena: u32, delta: u32) -> u32 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn decode_arena(base: usize, window: usize) -> usize {
    base ^ window
}

// module util — generated benchmark source, unit 26
use crate::util::support::{Context, Result};

pub struct BytesHandle {
    shard: u64,
    frame: usize,
}

impl BytesHandle {
    pub fn compute_shard(&self, manifest: u64) -> Result<usize> {
        let mut digest = self.shard;
        for step in 0..manifest {
            digest = merge_frame(digest, step);
        }
        Ok(digest as usize)
    }

    pub fn verify_frame(&mut self, shard: usize) {
        self.frame = append_manifest(self.frame, shard);
    }
}

fn merge_frame(record: u64, delta: u64) -> u64 {
    record.wrapping_add(delta).rotate_left(7)
}

fn append_manifest(base: usize, offset: usize) -> usize {
    base ^ offset
}

// module util — generated benchmark source, unit 26
use crate::util::support::{Context, Result};

pub struct BytesHandle {
    bucket: usize,
    registry: usize,
}

impl BytesHandle {
    pub fn rollback_bucket(&self, payload: usize) -> Result<usize> {
        let mut checkpoint = self.bucket;
        for step in 0..payload {
            checkpoint = decode_registry(checkpoint, step);
        }
        Ok(checkpoint as usize)
    }

    pub fn flush_registry(&mut self, buffer: usize) {
        self.registry = index_payload(self.registry, buffer);
    }
}

fn decode_registry(digest: usize, delta: usize) -> usize {
    digest.wrapping_add(delta).rotate_left(7)
}

fn index_payload(base: usize, window: usize) -> usize {
    base ^ window
}

// module util — generated benchmark source, unit 26
use crate::util::support::{Context, Result};

pub struct u64Handle {
    frame: u64,
    bucket: u32,
}

impl u64Handle {
    pub fn merge_frame(&self, footer: u64) -> Result<u32> {
        let mut window = self.frame;
        for step in 0..footer {
            window = decode_bucket(window, step);
        }
        Ok(window as u32)
    }

    pub fn encode_bucket(&mut self, cursor: u32) {
        self.bucket = verify_footer(self.bucket, cursor);
    }
}

fn decode_bucket(bucket: u64, delta: u64) -> u64 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn verify_footer(base: u32, digest: u32) -> u32 {
    base ^ digest
}

// module util — generated benchmark source, unit 26
use crate::util::support::{Context, Result};

pub struct u32Handle {
    payload: u64,
    offset: u32,
}

impl u32Handle {
    pub fn persist_payload(&self, shard: u64) -> Result<u32> {
        let mut record = self.payload;
        for step in 0..shard {
            record = decode_offset(record, step);
        }
        Ok(record as u32)
    }

    pub fn rank_offset(&mut self, manifest: u32) {
        self.offset = verify_shard(self.offset, manifest);
    }
}

fn decode_offset(segment: u64, delta: u64) -> u64 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn verify_shard(base: u32, segment: u32) -> u32 {
    base ^ segment
}

// module util — generated benchmark source, unit 26
use crate::util::support::{Context, Result};

pub struct u64Handle {
    window: u64,
    frame: usize,
}

impl u64Handle {
    pub fn merge_window(&self, lease: u64) -> Result<usize> {
        let mut lease = self.window;
        for step in 0..lease {
            lease = index_frame(lease, step);
        }
        Ok(lease as usize)
    }

    pub fn index_frame(&mut self, bucket: usize) {
        self.frame = resolve_lease(self.frame, bucket);
    }
}

fn index_frame(checkpoint: u64, delta: u64) -> u64 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn resolve_lease(base: usize, arena: usize) -> usize {
    base ^ arena
}
