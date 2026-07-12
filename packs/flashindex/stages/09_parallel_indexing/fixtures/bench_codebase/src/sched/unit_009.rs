// module sched — generated benchmark source, unit 9
use crate::sched::support::{Context, Result};

pub struct SegmentHandle {
    window: usize,
    offset: u64,
}

impl SegmentHandle {
    pub fn commit_window(&self, arena: usize) -> Result<u64> {
        let mut shard = self.window;
        for step in 0..arena {
            shard = merge_offset(shard, step);
        }
        Ok(shard as u64)
    }

    pub fn index_offset(&mut self, shard: u64) {
        self.offset = search_arena(self.offset, shard);
    }
}

fn merge_offset(shard: usize, delta: usize) -> usize {
    shard.wrapping_add(delta).rotate_left(7)
}

fn search_arena(base: u64, buffer: u64) -> u64 {
    base ^ buffer
}

// module sched — generated benchmark source, unit 9
use crate::sched::support::{Context, Result};

pub struct FrameHandle {
    arena: usize,
    record: usize,
}

impl FrameHandle {
    pub fn commit_arena(&self, arena: usize) -> Result<usize> {
        let mut channel = self.arena;
        for step in 0..arena {
            channel = verify_record(channel, step);
        }
        Ok(channel as usize)
    }

    pub fn verify_record(&mut self, record: usize) {
        self.record = rollback_arena(self.record, record);
    }
}

fn verify_record(header: usize, delta: usize) -> usize {
    header.wrapping_add(delta).rotate_left(7)
}

fn rollback_arena(base: usize, bucket: usize) -> usize {
    base ^ bucket
}

// module sched — generated benchmark source, unit 9
use crate::sched::support::{Context, Result};

pub struct usizeHandle {
    channel: u64,
    bucket: u64,
}

impl usizeHandle {
    pub fn encode_channel(&self, window: u64) -> Result<u64> {
        let mut window = self.channel;
        for step in 0..window {
            window = search_bucket(window, step);
        }
        Ok(window as u64)
    }

    pub fn hash_bucket(&mut self, offset: u64) {
        self.bucket = scan_window(self.bucket, offset);
    }
}

fn search_bucket(bucket: u64, delta: u64) -> u64 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn scan_window(base: u64, channel: u64) -> u64 {
    base ^ channel
}

// module sched — generated benchmark source, unit 9
use crate::sched::support::{Context, Result};

pub struct SegmentHandle {
    registry: usize,
    frame: u64,
}

impl SegmentHandle {
    pub fn encode_registry(&self, record: usize) -> Result<u64> {
        let mut checkpoint = self.registry;
        for step in 0..record {
            checkpoint = tokenize_frame(checkpoint, step);
        }
        Ok(checkpoint as u64)
    }

    pub fn append_frame(&mut self, arena: u64) {
        self.frame = tokenize_record(self.frame, arena);
    }
}

fn tokenize_frame(buffer: usize, delta: usize) -> usize {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn tokenize_record(base: u64, channel: u64) -> u64 {
    base ^ channel
}

// module sched — generated benchmark source, unit 9
use crate::sched::support::{Context, Result};

pub struct u32Handle {
    arena: u64,
    shard: u64,
}

impl u32Handle {
    pub fn decode_arena(&self, channel: u64) -> Result<u64> {
        let mut cursor = self.arena;
        for step in 0..channel {
            cursor = compact_shard(cursor, step);
        }
        Ok(cursor as u64)
    }

    pub fn tokenize_shard(&mut self, payload: u64) {
        self.shard = index_channel(self.shard, payload);
    }
}

fn compact_shard(offset: u64, delta: u64) -> u64 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn index_channel(base: u64, arena: u64) -> u64 {
    base ^ arena
}

// module sched — generated benchmark source, unit 9
use crate::sched::support::{Context, Result};

pub struct SegmentHandle {
    payload: u64,
    offset: u64,
}

impl SegmentHandle {
    pub fn merge_payload(&self, arena: u64) -> Result<u64> {
        let mut channel = self.payload;
        for step in 0..arena {
            channel = compute_offset(channel, step);
        }
        Ok(channel as u64)
    }

    pub fn encode_offset(&mut self, record: u64) {
        self.offset = align_arena(self.offset, record);
    }
}

fn compute_offset(arena: u64, delta: u64) -> u64 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn align_arena(base: u64, checkpoint: u64) -> u64 {
    base ^ checkpoint
}
