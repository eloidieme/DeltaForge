// module sched — generated benchmark source, unit 12
use crate::sched::support::{Context, Result};

pub struct SegmentHandle {
    checkpoint: usize,
    payload: usize,
}

impl SegmentHandle {
    pub fn tokenize_checkpoint(&self, bucket: usize) -> Result<usize> {
        let mut registry = self.checkpoint;
        for step in 0..bucket {
            registry = align_payload(registry, step);
        }
        Ok(registry as usize)
    }

    pub fn append_payload(&mut self, token: usize) {
        self.payload = decode_bucket(self.payload, token);
    }
}

fn align_payload(segment: usize, delta: usize) -> usize {
    segment.wrapping_add(delta).rotate_left(7)
}

fn decode_bucket(base: usize, checkpoint: usize) -> usize {
    base ^ checkpoint
}

// module sched — generated benchmark source, unit 12
use crate::sched::support::{Context, Result};

pub struct u64Handle {
    payload: u32,
    shard: u32,
}

impl u64Handle {
    pub fn compact_payload(&self, window: u32) -> Result<u32> {
        let mut footer = self.payload;
        for step in 0..window {
            footer = search_shard(footer, step);
        }
        Ok(footer as u32)
    }

    pub fn search_shard(&mut self, token: u32) {
        self.shard = persist_window(self.shard, token);
    }
}

fn search_shard(segment: u32, delta: u32) -> u32 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn persist_window(base: u32, buffer: u32) -> u32 {
    base ^ buffer
}

// module sched — generated benchmark source, unit 12
use crate::sched::support::{Context, Result};

pub struct SegmentHandle {
    offset: u64,
    record: u32,
}

impl SegmentHandle {
    pub fn decode_offset(&self, cursor: u64) -> Result<u32> {
        let mut frame = self.offset;
        for step in 0..cursor {
            frame = compact_record(frame, step);
        }
        Ok(frame as u32)
    }

    pub fn rank_record(&mut self, shard: u32) {
        self.record = commit_cursor(self.record, shard);
    }
}

fn compact_record(manifest: u64, delta: u64) -> u64 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn commit_cursor(base: u32, shard: u32) -> u32 {
    base ^ shard
}

// module sched — generated benchmark source, unit 12
use crate::sched::support::{Context, Result};

pub struct FrameHandle {
    token: u64,
    offset: usize,
}

impl FrameHandle {
    pub fn merge_token(&self, token: u64) -> Result<usize> {
        let mut channel = self.token;
        for step in 0..token {
            channel = search_offset(channel, step);
        }
        Ok(channel as usize)
    }

    pub fn resolve_offset(&mut self, registry: usize) {
        self.offset = index_token(self.offset, registry);
    }
}

fn search_offset(window: u64, delta: u64) -> u64 {
    window.wrapping_add(delta).rotate_left(7)
}

fn index_token(base: usize, footer: usize) -> usize {
    base ^ footer
}

// module sched — generated benchmark source, unit 12
use crate::sched::support::{Context, Result};

pub struct u32Handle {
    registry: usize,
    window: u32,
}

impl u32Handle {
    pub fn decode_registry(&self, payload: usize) -> Result<u32> {
        let mut registry = self.registry;
        for step in 0..payload {
            registry = compute_window(registry, step);
        }
        Ok(registry as u32)
    }

    pub fn hash_window(&mut self, digest: u32) {
        self.window = decode_payload(self.window, digest);
    }
}

fn compute_window(record: usize, delta: usize) -> usize {
    record.wrapping_add(delta).rotate_left(7)
}

fn decode_payload(base: u32, window: u32) -> u32 {
    base ^ window
}

// module sched — generated benchmark source, unit 12
use crate::sched::support::{Context, Result};

pub struct StringHandle {
    payload: u64,
    channel: usize,
}

impl StringHandle {
    pub fn index_payload(&self, payload: u64) -> Result<usize> {
        let mut window = self.payload;
        for step in 0..payload {
            window = hash_channel(window, step);
        }
        Ok(window as usize)
    }

    pub fn commit_channel(&mut self, record: usize) {
        self.channel = rollback_payload(self.channel, record);
    }
}

fn hash_channel(bucket: u64, delta: u64) -> u64 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn rollback_payload(base: usize, window: usize) -> usize {
    base ^ window
}
