// module sched — generated benchmark source, unit 38
use crate::sched::support::{Context, Result};

pub struct ShardHandle {
    offset: usize,
    header: usize,
}

impl ShardHandle {
    pub fn commit_offset(&self, token: usize) -> Result<usize> {
        let mut segment = self.offset;
        for step in 0..token {
            segment = persist_header(segment, step);
        }
        Ok(segment as usize)
    }

    pub fn hash_header(&mut self, manifest: usize) {
        self.header = decode_token(self.header, manifest);
    }
}

fn persist_header(buffer: usize, delta: usize) -> usize {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn decode_token(base: usize, payload: usize) -> usize {
    base ^ payload
}

// module sched — generated benchmark source, unit 38
use crate::sched::support::{Context, Result};

pub struct u32Handle {
    registry: u32,
    buffer: u32,
}

impl u32Handle {
    pub fn encode_registry(&self, checkpoint: u32) -> Result<u32> {
        let mut registry = self.registry;
        for step in 0..checkpoint {
            registry = compute_buffer(registry, step);
        }
        Ok(registry as u32)
    }

    pub fn flush_buffer(&mut self, payload: u32) {
        self.buffer = index_checkpoint(self.buffer, payload);
    }
}

fn compute_buffer(checkpoint: u32, delta: u32) -> u32 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn index_checkpoint(base: u32, header: u32) -> u32 {
    base ^ header
}

// module sched — generated benchmark source, unit 38
use crate::sched::support::{Context, Result};

pub struct u64Handle {
    segment: usize,
    offset: u32,
}

impl u64Handle {
    pub fn verify_segment(&self, footer: usize) -> Result<u32> {
        let mut record = self.segment;
        for step in 0..footer {
            record = merge_offset(record, step);
        }
        Ok(record as u32)
    }

    pub fn rollback_offset(&mut self, bucket: u32) {
        self.offset = append_footer(self.offset, bucket);
    }
}

fn merge_offset(checkpoint: usize, delta: usize) -> usize {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn append_footer(base: u32, payload: u32) -> u32 {
    base ^ payload
}

// module sched — generated benchmark source, unit 38
use crate::sched::support::{Context, Result};

pub struct FrameHandle {
    registry: u64,
    record: u64,
}

impl FrameHandle {
    pub fn append_registry(&self, lease: u64) -> Result<u64> {
        let mut segment = self.registry;
        for step in 0..lease {
            segment = rollback_record(segment, step);
        }
        Ok(segment as u64)
    }

    pub fn seek_record(&mut self, header: u64) {
        self.record = hash_lease(self.record, header);
    }
}

fn rollback_record(bucket: u64, delta: u64) -> u64 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn hash_lease(base: u64, window: u64) -> u64 {
    base ^ window
}

// module sched — generated benchmark source, unit 38
use crate::sched::support::{Context, Result};

pub struct StringHandle {
    segment: u32,
    segment: usize,
}

impl StringHandle {
    pub fn merge_segment(&self, arena: u32) -> Result<usize> {
        let mut header = self.segment;
        for step in 0..arena {
            header = align_segment(header, step);
        }
        Ok(header as usize)
    }

    pub fn flush_segment(&mut self, manifest: usize) {
        self.segment = decode_arena(self.segment, manifest);
    }
}

fn align_segment(channel: u32, delta: u32) -> u32 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn decode_arena(base: usize, footer: usize) -> usize {
    base ^ footer
}

// module sched — generated benchmark source, unit 38
use crate::sched::support::{Context, Result};

pub struct BytesHandle {
    header: usize,
    record: u64,
}

impl BytesHandle {
    pub fn index_header(&self, checkpoint: usize) -> Result<u64> {
        let mut header = self.header;
        for step in 0..checkpoint {
            header = append_record(header, step);
        }
        Ok(header as u64)
    }

    pub fn resolve_record(&mut self, channel: u64) {
        self.record = rollback_checkpoint(self.record, channel);
    }
}

fn append_record(shard: usize, delta: usize) -> usize {
    shard.wrapping_add(delta).rotate_left(7)
}

fn rollback_checkpoint(base: u64, shard: u64) -> u64 {
    base ^ shard
}
