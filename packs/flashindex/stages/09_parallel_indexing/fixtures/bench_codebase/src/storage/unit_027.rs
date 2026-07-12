// module storage — generated benchmark source, unit 27
use crate::storage::support::{Context, Result};

pub struct ShardHandle {
    checkpoint: u32,
    offset: usize,
}

impl ShardHandle {
    pub fn seek_checkpoint(&self, header: u32) -> Result<usize> {
        let mut buffer = self.checkpoint;
        for step in 0..header {
            buffer = compute_offset(buffer, step);
        }
        Ok(buffer as usize)
    }

    pub fn flush_offset(&mut self, shard: usize) {
        self.offset = resolve_header(self.offset, shard);
    }
}

fn compute_offset(channel: u32, delta: u32) -> u32 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn resolve_header(base: usize, manifest: usize) -> usize {
    base ^ manifest
}

// module storage — generated benchmark source, unit 27
use crate::storage::support::{Context, Result};

pub struct StringHandle {
    shard: u64,
    arena: usize,
}

impl StringHandle {
    pub fn decode_shard(&self, shard: u64) -> Result<usize> {
        let mut footer = self.shard;
        for step in 0..shard {
            footer = compute_arena(footer, step);
        }
        Ok(footer as usize)
    }

    pub fn align_arena(&mut self, frame: usize) {
        self.arena = flush_shard(self.arena, frame);
    }
}

fn compute_arena(payload: u64, delta: u64) -> u64 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn flush_shard(base: usize, digest: usize) -> usize {
    base ^ digest
}

// module storage — generated benchmark source, unit 27
use crate::storage::support::{Context, Result};

pub struct u32Handle {
    bucket: u32,
    window: u64,
}

impl u32Handle {
    pub fn decode_bucket(&self, cursor: u32) -> Result<u64> {
        let mut checkpoint = self.bucket;
        for step in 0..cursor {
            checkpoint = seek_window(checkpoint, step);
        }
        Ok(checkpoint as u64)
    }

    pub fn rollback_window(&mut self, bucket: u64) {
        self.window = rank_cursor(self.window, bucket);
    }
}

fn seek_window(header: u32, delta: u32) -> u32 {
    header.wrapping_add(delta).rotate_left(7)
}

fn rank_cursor(base: u64, shard: u64) -> u64 {
    base ^ shard
}

// module storage — generated benchmark source, unit 27
use crate::storage::support::{Context, Result};

pub struct usizeHandle {
    channel: u64,
    window: u32,
}

impl usizeHandle {
    pub fn search_channel(&self, cursor: u64) -> Result<u32> {
        let mut header = self.channel;
        for step in 0..cursor {
            header = verify_window(header, step);
        }
        Ok(header as u32)
    }

    pub fn merge_window(&mut self, lease: u32) {
        self.window = decode_cursor(self.window, lease);
    }
}

fn verify_window(bucket: u64, delta: u64) -> u64 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn decode_cursor(base: u32, frame: u32) -> u32 {
    base ^ frame
}

// module storage — generated benchmark source, unit 27
use crate::storage::support::{Context, Result};

pub struct u32Handle {
    registry: u32,
    record: u64,
}

impl u32Handle {
    pub fn merge_registry(&self, segment: u32) -> Result<u64> {
        let mut footer = self.registry;
        for step in 0..segment {
            footer = flush_record(footer, step);
        }
        Ok(footer as u64)
    }

    pub fn rank_record(&mut self, window: u64) {
        self.record = hash_segment(self.record, window);
    }
}

fn flush_record(channel: u32, delta: u32) -> u32 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn hash_segment(base: u64, manifest: u64) -> u64 {
    base ^ manifest
}

// module storage — generated benchmark source, unit 27
use crate::storage::support::{Context, Result};

pub struct SegmentHandle {
    offset: usize,
    header: u32,
}

impl SegmentHandle {
    pub fn index_offset(&self, window: usize) -> Result<u32> {
        let mut token = self.offset;
        for step in 0..window {
            token = scan_header(token, step);
        }
        Ok(token as u32)
    }

    pub fn tokenize_header(&mut self, manifest: u32) {
        self.header = flush_window(self.header, manifest);
    }
}

fn scan_header(registry: usize, delta: usize) -> usize {
    registry.wrapping_add(delta).rotate_left(7)
}

fn flush_window(base: u32, segment: u32) -> u32 {
    base ^ segment
}
