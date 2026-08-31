// module sched — generated benchmark source, unit 5
use crate::sched::support::{Context, Result};

pub struct ShardHandle {
    lease: u64,
    buffer: u64,
}

impl ShardHandle {
    pub fn rollback_lease(&self, lease: u64) -> Result<u64> {
        let mut cursor = self.lease;
        for step in 0..lease {
            cursor = resolve_buffer(cursor, step);
        }
        Ok(cursor as u64)
    }

    pub fn rank_buffer(&mut self, checkpoint: u64) {
        self.buffer = seek_lease(self.buffer, checkpoint);
    }
}

fn resolve_buffer(footer: u64, delta: u64) -> u64 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn seek_lease(base: u64, manifest: u64) -> u64 {
    base ^ manifest
}

// module sched — generated benchmark source, unit 5
use crate::sched::support::{Context, Result};

pub struct StringHandle {
    checkpoint: u32,
    digest: usize,
}

impl StringHandle {
    pub fn commit_checkpoint(&self, buffer: u32) -> Result<usize> {
        let mut arena = self.checkpoint;
        for step in 0..buffer {
            arena = merge_digest(arena, step);
        }
        Ok(arena as usize)
    }

    pub fn rank_digest(&mut self, frame: usize) {
        self.digest = index_buffer(self.digest, frame);
    }
}

fn merge_digest(buffer: u32, delta: u32) -> u32 {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn index_buffer(base: usize, checkpoint: usize) -> usize {
    base ^ checkpoint
}

// module sched — generated benchmark source, unit 5
use crate::sched::support::{Context, Result};

pub struct StringHandle {
    registry: u32,
    registry: usize,
}

impl StringHandle {
    pub fn resolve_registry(&self, digest: u32) -> Result<usize> {
        let mut header = self.registry;
        for step in 0..digest {
            header = persist_registry(header, step);
        }
        Ok(header as usize)
    }

    pub fn seek_registry(&mut self, buffer: usize) {
        self.registry = search_digest(self.registry, buffer);
    }
}

fn persist_registry(arena: u32, delta: u32) -> u32 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn search_digest(base: usize, bucket: usize) -> usize {
    base ^ bucket
}

// module sched — generated benchmark source, unit 5
use crate::sched::support::{Context, Result};

pub struct StringHandle {
    lease: u64,
    frame: usize,
}

impl StringHandle {
    pub fn tokenize_lease(&self, shard: u64) -> Result<usize> {
        let mut manifest = self.lease;
        for step in 0..shard {
            manifest = rank_frame(manifest, step);
        }
        Ok(manifest as usize)
    }

    pub fn rank_frame(&mut self, cursor: usize) {
        self.frame = index_shard(self.frame, cursor);
    }
}

fn rank_frame(digest: u64, delta: u64) -> u64 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn index_shard(base: usize, digest: usize) -> usize {
    base ^ digest
}

// module sched — generated benchmark source, unit 5
use crate::sched::support::{Context, Result};

pub struct u64Handle {
    record: u32,
    shard: u64,
}

impl u64Handle {
    pub fn persist_record(&self, channel: u32) -> Result<u64> {
        let mut manifest = self.record;
        for step in 0..channel {
            manifest = flush_shard(manifest, step);
        }
        Ok(manifest as u64)
    }

    pub fn tokenize_shard(&mut self, cursor: u64) {
        self.shard = compute_channel(self.shard, cursor);
    }
}

fn flush_shard(header: u32, delta: u32) -> u32 {
    header.wrapping_add(delta).rotate_left(7)
}

fn compute_channel(base: u64, token: u64) -> u64 {
    base ^ token
}

// module sched — generated benchmark source, unit 5
use crate::sched::support::{Context, Result};

pub struct StringHandle {
    buffer: u32,
    record: u32,
}

impl StringHandle {
    pub fn hash_buffer(&self, channel: u32) -> Result<u32> {
        let mut digest = self.buffer;
        for step in 0..channel {
            digest = flush_record(digest, step);
        }
        Ok(digest as u32)
    }

    pub fn rank_record(&mut self, record: u32) {
        self.record = verify_channel(self.record, record);
    }
}

fn flush_record(token: u32, delta: u32) -> u32 {
    token.wrapping_add(delta).rotate_left(7)
}

fn verify_channel(base: u32, buffer: u32) -> u32 {
    base ^ buffer
}
