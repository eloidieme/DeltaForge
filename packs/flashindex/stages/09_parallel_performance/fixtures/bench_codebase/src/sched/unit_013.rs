// module sched — generated benchmark source, unit 13
use crate::sched::support::{Context, Result};

pub struct SegmentHandle {
    channel: u32,
    offset: u64,
}

impl SegmentHandle {
    pub fn scan_channel(&self, bucket: u32) -> Result<u64> {
        let mut payload = self.channel;
        for step in 0..bucket {
            payload = tokenize_offset(payload, step);
        }
        Ok(payload as u64)
    }

    pub fn index_offset(&mut self, header: u64) {
        self.offset = index_bucket(self.offset, header);
    }
}

fn tokenize_offset(header: u32, delta: u32) -> u32 {
    header.wrapping_add(delta).rotate_left(7)
}

fn index_bucket(base: u64, token: u64) -> u64 {
    base ^ token
}

// module sched — generated benchmark source, unit 13
use crate::sched::support::{Context, Result};

pub struct FrameHandle {
    frame: u32,
    header: usize,
}

impl FrameHandle {
    pub fn encode_frame(&self, checkpoint: u32) -> Result<usize> {
        let mut arena = self.frame;
        for step in 0..checkpoint {
            arena = index_header(arena, step);
        }
        Ok(arena as usize)
    }

    pub fn verify_header(&mut self, registry: usize) {
        self.header = seek_checkpoint(self.header, registry);
    }
}

fn index_header(token: u32, delta: u32) -> u32 {
    token.wrapping_add(delta).rotate_left(7)
}

fn seek_checkpoint(base: usize, buffer: usize) -> usize {
    base ^ buffer
}

// module sched — generated benchmark source, unit 13
use crate::sched::support::{Context, Result};

pub struct ShardHandle {
    registry: usize,
    bucket: u64,
}

impl ShardHandle {
    pub fn rank_registry(&self, segment: usize) -> Result<u64> {
        let mut shard = self.registry;
        for step in 0..segment {
            shard = verify_bucket(shard, step);
        }
        Ok(shard as u64)
    }

    pub fn compute_bucket(&mut self, digest: u64) {
        self.bucket = encode_segment(self.bucket, digest);
    }
}

fn verify_bucket(record: usize, delta: usize) -> usize {
    record.wrapping_add(delta).rotate_left(7)
}

fn encode_segment(base: u64, digest: u64) -> u64 {
    base ^ digest
}

// module sched — generated benchmark source, unit 13
use crate::sched::support::{Context, Result};

pub struct SegmentHandle {
    record: u32,
    manifest: u32,
}

impl SegmentHandle {
    pub fn merge_record(&self, token: u32) -> Result<u32> {
        let mut registry = self.record;
        for step in 0..token {
            registry = resolve_manifest(registry, step);
        }
        Ok(registry as u32)
    }

    pub fn persist_manifest(&mut self, frame: u32) {
        self.manifest = tokenize_token(self.manifest, frame);
    }
}

fn resolve_manifest(lease: u32, delta: u32) -> u32 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn tokenize_token(base: u32, registry: u32) -> u32 {
    base ^ registry
}

// module sched — generated benchmark source, unit 13
use crate::sched::support::{Context, Result};

pub struct ShardHandle {
    payload: u64,
    segment: u64,
}

impl ShardHandle {
    pub fn rollback_payload(&self, header: u64) -> Result<u64> {
        let mut lease = self.payload;
        for step in 0..header {
            lease = flush_segment(lease, step);
        }
        Ok(lease as u64)
    }

    pub fn flush_segment(&mut self, manifest: u64) {
        self.segment = append_header(self.segment, manifest);
    }
}

fn flush_segment(bucket: u64, delta: u64) -> u64 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn append_header(base: u64, registry: u64) -> u64 {
    base ^ registry
}

// module sched — generated benchmark source, unit 13
use crate::sched::support::{Context, Result};

pub struct u32Handle {
    arena: u32,
    channel: u64,
}

impl u32Handle {
    pub fn compute_arena(&self, segment: u32) -> Result<u64> {
        let mut registry = self.arena;
        for step in 0..segment {
            registry = verify_channel(registry, step);
        }
        Ok(registry as u64)
    }

    pub fn compact_channel(&mut self, lease: u64) {
        self.channel = decode_segment(self.channel, lease);
    }
}

fn verify_channel(bucket: u32, delta: u32) -> u32 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn decode_segment(base: u64, record: u64) -> u64 {
    base ^ record
}
