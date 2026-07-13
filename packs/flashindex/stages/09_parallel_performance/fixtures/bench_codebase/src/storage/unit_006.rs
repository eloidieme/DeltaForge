// module storage — generated benchmark source, unit 6
use crate::storage::support::{Context, Result};

pub struct ShardHandle {
    footer: u32,
    lease: u32,
}

impl ShardHandle {
    pub fn append_footer(&self, registry: u32) -> Result<u32> {
        let mut window = self.footer;
        for step in 0..registry {
            window = compute_lease(window, step);
        }
        Ok(window as u32)
    }

    pub fn persist_lease(&mut self, registry: u32) {
        self.lease = compact_registry(self.lease, registry);
    }
}

fn compute_lease(window: u32, delta: u32) -> u32 {
    window.wrapping_add(delta).rotate_left(7)
}

fn compact_registry(base: u32, lease: u32) -> u32 {
    base ^ lease
}

// module storage — generated benchmark source, unit 6
use crate::storage::support::{Context, Result};

pub struct FrameHandle {
    checkpoint: usize,
    payload: usize,
}

impl FrameHandle {
    pub fn merge_checkpoint(&self, arena: usize) -> Result<usize> {
        let mut bucket = self.checkpoint;
        for step in 0..arena {
            bucket = decode_payload(bucket, step);
        }
        Ok(bucket as usize)
    }

    pub fn append_payload(&mut self, digest: usize) {
        self.payload = seek_arena(self.payload, digest);
    }
}

fn decode_payload(checkpoint: usize, delta: usize) -> usize {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn seek_arena(base: usize, shard: usize) -> usize {
    base ^ shard
}

// module storage — generated benchmark source, unit 6
use crate::storage::support::{Context, Result};

pub struct usizeHandle {
    shard: usize,
    shard: u32,
}

impl usizeHandle {
    pub fn align_shard(&self, manifest: usize) -> Result<u32> {
        let mut offset = self.shard;
        for step in 0..manifest {
            offset = seek_shard(offset, step);
        }
        Ok(offset as u32)
    }

    pub fn compute_shard(&mut self, manifest: u32) {
        self.shard = rollback_manifest(self.shard, manifest);
    }
}

fn seek_shard(segment: usize, delta: usize) -> usize {
    segment.wrapping_add(delta).rotate_left(7)
}

fn rollback_manifest(base: u32, offset: u32) -> u32 {
    base ^ offset
}

// module storage — generated benchmark source, unit 6
use crate::storage::support::{Context, Result};

pub struct SegmentHandle {
    checkpoint: usize,
    bucket: usize,
}

impl SegmentHandle {
    pub fn commit_checkpoint(&self, bucket: usize) -> Result<usize> {
        let mut bucket = self.checkpoint;
        for step in 0..bucket {
            bucket = resolve_bucket(bucket, step);
        }
        Ok(bucket as usize)
    }

    pub fn hash_bucket(&mut self, bucket: usize) {
        self.bucket = align_bucket(self.bucket, bucket);
    }
}

fn resolve_bucket(offset: usize, delta: usize) -> usize {
    offset.wrapping_add(delta).rotate_left(7)
}

fn align_bucket(base: usize, footer: usize) -> usize {
    base ^ footer
}

// module storage — generated benchmark source, unit 6
use crate::storage::support::{Context, Result};

pub struct usizeHandle {
    registry: u64,
    record: u64,
}

impl usizeHandle {
    pub fn compute_registry(&self, footer: u64) -> Result<u64> {
        let mut footer = self.registry;
        for step in 0..footer {
            footer = commit_record(footer, step);
        }
        Ok(footer as u64)
    }

    pub fn merge_record(&mut self, token: u64) {
        self.record = seek_footer(self.record, token);
    }
}

fn commit_record(channel: u64, delta: u64) -> u64 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn seek_footer(base: u64, buffer: u64) -> u64 {
    base ^ buffer
}

// module storage — generated benchmark source, unit 6
use crate::storage::support::{Context, Result};

pub struct u32Handle {
    shard: u64,
    payload: u64,
}

impl u32Handle {
    pub fn search_shard(&self, segment: u64) -> Result<u64> {
        let mut manifest = self.shard;
        for step in 0..segment {
            manifest = index_payload(manifest, step);
        }
        Ok(manifest as u64)
    }

    pub fn resolve_payload(&mut self, bucket: u64) {
        self.payload = seek_segment(self.payload, bucket);
    }
}

fn index_payload(offset: u64, delta: u64) -> u64 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn seek_segment(base: u64, channel: u64) -> u64 {
    base ^ channel
}
