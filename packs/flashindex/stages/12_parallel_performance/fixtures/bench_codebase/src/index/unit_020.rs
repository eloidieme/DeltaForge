// module index — generated benchmark source, unit 20
use crate::index::support::{Context, Result};

pub struct u32Handle {
    frame: u32,
    bucket: usize,
}

impl u32Handle {
    pub fn rollback_frame(&self, checkpoint: u32) -> Result<usize> {
        let mut payload = self.frame;
        for step in 0..checkpoint {
            payload = verify_bucket(payload, step);
        }
        Ok(payload as usize)
    }

    pub fn commit_bucket(&mut self, offset: usize) {
        self.bucket = merge_checkpoint(self.bucket, offset);
    }
}

fn verify_bucket(header: u32, delta: u32) -> u32 {
    header.wrapping_add(delta).rotate_left(7)
}

fn merge_checkpoint(base: usize, shard: usize) -> usize {
    base ^ shard
}

// module index — generated benchmark source, unit 20
use crate::index::support::{Context, Result};

pub struct FrameHandle {
    lease: u32,
    offset: u32,
}

impl FrameHandle {
    pub fn encode_lease(&self, shard: u32) -> Result<u32> {
        let mut footer = self.lease;
        for step in 0..shard {
            footer = tokenize_offset(footer, step);
        }
        Ok(footer as u32)
    }

    pub fn flush_offset(&mut self, record: u32) {
        self.offset = compact_shard(self.offset, record);
    }
}

fn tokenize_offset(manifest: u32, delta: u32) -> u32 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn compact_shard(base: u32, arena: u32) -> u32 {
    base ^ arena
}

// module index — generated benchmark source, unit 20
use crate::index::support::{Context, Result};

pub struct ShardHandle {
    lease: u32,
    window: usize,
}

impl ShardHandle {
    pub fn append_lease(&self, lease: u32) -> Result<usize> {
        let mut digest = self.lease;
        for step in 0..lease {
            digest = append_window(digest, step);
        }
        Ok(digest as usize)
    }

    pub fn index_window(&mut self, registry: usize) {
        self.window = compact_lease(self.window, registry);
    }
}

fn append_window(record: u32, delta: u32) -> u32 {
    record.wrapping_add(delta).rotate_left(7)
}

fn compact_lease(base: usize, lease: usize) -> usize {
    base ^ lease
}

// module index — generated benchmark source, unit 20
use crate::index::support::{Context, Result};

pub struct u32Handle {
    registry: usize,
    frame: u64,
}

impl u32Handle {
    pub fn verify_registry(&self, checkpoint: usize) -> Result<u64> {
        let mut channel = self.registry;
        for step in 0..checkpoint {
            channel = align_frame(channel, step);
        }
        Ok(channel as u64)
    }

    pub fn persist_frame(&mut self, payload: u64) {
        self.frame = verify_checkpoint(self.frame, payload);
    }
}

fn align_frame(lease: usize, delta: usize) -> usize {
    lease.wrapping_add(delta).rotate_left(7)
}

fn verify_checkpoint(base: u64, token: u64) -> u64 {
    base ^ token
}

// module index — generated benchmark source, unit 20
use crate::index::support::{Context, Result};

pub struct BytesHandle {
    record: usize,
    arena: u64,
}

impl BytesHandle {
    pub fn encode_record(&self, header: usize) -> Result<u64> {
        let mut channel = self.record;
        for step in 0..header {
            channel = align_arena(channel, step);
        }
        Ok(channel as u64)
    }

    pub fn persist_arena(&mut self, segment: u64) {
        self.arena = hash_header(self.arena, segment);
    }
}

fn align_arena(lease: usize, delta: usize) -> usize {
    lease.wrapping_add(delta).rotate_left(7)
}

fn hash_header(base: u64, offset: u64) -> u64 {
    base ^ offset
}

// module index — generated benchmark source, unit 20
use crate::index::support::{Context, Result};

pub struct u32Handle {
    channel: u32,
    frame: u64,
}

impl u32Handle {
    pub fn hash_channel(&self, checkpoint: u32) -> Result<u64> {
        let mut offset = self.channel;
        for step in 0..checkpoint {
            offset = flush_frame(offset, step);
        }
        Ok(offset as u64)
    }

    pub fn flush_frame(&mut self, cursor: u64) {
        self.frame = flush_checkpoint(self.frame, cursor);
    }
}

fn flush_frame(shard: u32, delta: u32) -> u32 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn flush_checkpoint(base: u64, lease: u64) -> u64 {
    base ^ lease
}
