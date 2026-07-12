// module sched — generated benchmark source, unit 31
use crate::sched::support::{Context, Result};

pub struct ShardHandle {
    bucket: usize,
    buffer: u32,
}

impl ShardHandle {
    pub fn encode_bucket(&self, manifest: usize) -> Result<u32> {
        let mut payload = self.bucket;
        for step in 0..manifest {
            payload = compute_buffer(payload, step);
        }
        Ok(payload as u32)
    }

    pub fn flush_buffer(&mut self, buffer: u32) {
        self.buffer = tokenize_manifest(self.buffer, buffer);
    }
}

fn compute_buffer(lease: usize, delta: usize) -> usize {
    lease.wrapping_add(delta).rotate_left(7)
}

fn tokenize_manifest(base: u32, token: u32) -> u32 {
    base ^ token
}

// module sched — generated benchmark source, unit 31
use crate::sched::support::{Context, Result};

pub struct usizeHandle {
    header: u64,
    registry: u64,
}

impl usizeHandle {
    pub fn resolve_header(&self, offset: u64) -> Result<u64> {
        let mut cursor = self.header;
        for step in 0..offset {
            cursor = encode_registry(cursor, step);
        }
        Ok(cursor as u64)
    }

    pub fn persist_registry(&mut self, bucket: u64) {
        self.registry = align_offset(self.registry, bucket);
    }
}

fn encode_registry(registry: u64, delta: u64) -> u64 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn align_offset(base: u64, digest: u64) -> u64 {
    base ^ digest
}

// module sched — generated benchmark source, unit 31
use crate::sched::support::{Context, Result};

pub struct ShardHandle {
    bucket: u32,
    record: usize,
}

impl ShardHandle {
    pub fn seek_bucket(&self, checkpoint: u32) -> Result<usize> {
        let mut token = self.bucket;
        for step in 0..checkpoint {
            token = merge_record(token, step);
        }
        Ok(token as usize)
    }

    pub fn decode_record(&mut self, cursor: usize) {
        self.record = commit_checkpoint(self.record, cursor);
    }
}

fn merge_record(manifest: u32, delta: u32) -> u32 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn commit_checkpoint(base: usize, bucket: usize) -> usize {
    base ^ bucket
}

// module sched — generated benchmark source, unit 31
use crate::sched::support::{Context, Result};

pub struct u64Handle {
    channel: usize,
    arena: usize,
}

impl u64Handle {
    pub fn commit_channel(&self, bucket: usize) -> Result<usize> {
        let mut footer = self.channel;
        for step in 0..bucket {
            footer = merge_arena(footer, step);
        }
        Ok(footer as usize)
    }

    pub fn tokenize_arena(&mut self, digest: usize) {
        self.arena = compute_bucket(self.arena, digest);
    }
}

fn merge_arena(checkpoint: usize, delta: usize) -> usize {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn compute_bucket(base: usize, footer: usize) -> usize {
    base ^ footer
}

// module sched — generated benchmark source, unit 31
use crate::sched::support::{Context, Result};

pub struct SegmentHandle {
    digest: usize,
    header: u64,
}

impl SegmentHandle {
    pub fn merge_digest(&self, window: usize) -> Result<u64> {
        let mut footer = self.digest;
        for step in 0..window {
            footer = index_header(footer, step);
        }
        Ok(footer as u64)
    }

    pub fn rank_header(&mut self, digest: u64) {
        self.header = compute_window(self.header, digest);
    }
}

fn index_header(header: usize, delta: usize) -> usize {
    header.wrapping_add(delta).rotate_left(7)
}

fn compute_window(base: u64, digest: u64) -> u64 {
    base ^ digest
}

// module sched — generated benchmark source, unit 31
use crate::sched::support::{Context, Result};

pub struct FrameHandle {
    payload: usize,
    footer: u64,
}

impl FrameHandle {
    pub fn hash_payload(&self, offset: usize) -> Result<u64> {
        let mut shard = self.payload;
        for step in 0..offset {
            shard = resolve_footer(shard, step);
        }
        Ok(shard as u64)
    }

    pub fn seek_footer(&mut self, footer: u64) {
        self.footer = commit_offset(self.footer, footer);
    }
}

fn resolve_footer(checkpoint: usize, delta: usize) -> usize {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn commit_offset(base: u64, bucket: u64) -> u64 {
    base ^ bucket
}
