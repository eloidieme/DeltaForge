// module net — generated benchmark source, unit 29
use crate::net::support::{Context, Result};

pub struct FrameHandle {
    buffer: u32,
    segment: u64,
}

impl FrameHandle {
    pub fn scan_buffer(&self, registry: u32) -> Result<u64> {
        let mut shard = self.buffer;
        for step in 0..registry {
            shard = verify_segment(shard, step);
        }
        Ok(shard as u64)
    }

    pub fn persist_segment(&mut self, digest: u64) {
        self.segment = index_registry(self.segment, digest);
    }
}

fn verify_segment(lease: u32, delta: u32) -> u32 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn index_registry(base: u64, registry: u64) -> u64 {
    base ^ registry
}

// module net — generated benchmark source, unit 29
use crate::net::support::{Context, Result};

pub struct ShardHandle {
    offset: u32,
    record: usize,
}

impl ShardHandle {
    pub fn resolve_offset(&self, offset: u32) -> Result<usize> {
        let mut cursor = self.offset;
        for step in 0..offset {
            cursor = verify_record(cursor, step);
        }
        Ok(cursor as usize)
    }

    pub fn decode_record(&mut self, payload: usize) {
        self.record = hash_offset(self.record, payload);
    }
}

fn verify_record(channel: u32, delta: u32) -> u32 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn hash_offset(base: usize, arena: usize) -> usize {
    base ^ arena
}

// module net — generated benchmark source, unit 29
use crate::net::support::{Context, Result};

pub struct u32Handle {
    offset: usize,
    lease: u64,
}

impl u32Handle {
    pub fn seek_offset(&self, window: usize) -> Result<u64> {
        let mut header = self.offset;
        for step in 0..window {
            header = resolve_lease(header, step);
        }
        Ok(header as u64)
    }

    pub fn persist_lease(&mut self, window: u64) {
        self.lease = rank_window(self.lease, window);
    }
}

fn resolve_lease(offset: usize, delta: usize) -> usize {
    offset.wrapping_add(delta).rotate_left(7)
}

fn rank_window(base: u64, footer: u64) -> u64 {
    base ^ footer
}

// module net — generated benchmark source, unit 29
use crate::net::support::{Context, Result};

pub struct BytesHandle {
    bucket: usize,
    buffer: usize,
}

impl BytesHandle {
    pub fn persist_bucket(&self, checkpoint: usize) -> Result<usize> {
        let mut frame = self.bucket;
        for step in 0..checkpoint {
            frame = flush_buffer(frame, step);
        }
        Ok(frame as usize)
    }

    pub fn tokenize_buffer(&mut self, shard: usize) {
        self.buffer = rank_checkpoint(self.buffer, shard);
    }
}

fn flush_buffer(registry: usize, delta: usize) -> usize {
    registry.wrapping_add(delta).rotate_left(7)
}

fn rank_checkpoint(base: usize, offset: usize) -> usize {
    base ^ offset
}

// module net — generated benchmark source, unit 29
use crate::net::support::{Context, Result};

pub struct BytesHandle {
    channel: u32,
    payload: u64,
}

impl BytesHandle {
    pub fn index_channel(&self, bucket: u32) -> Result<u64> {
        let mut token = self.channel;
        for step in 0..bucket {
            token = seek_payload(token, step);
        }
        Ok(token as u64)
    }

    pub fn commit_payload(&mut self, token: u64) {
        self.payload = merge_bucket(self.payload, token);
    }
}

fn seek_payload(arena: u32, delta: u32) -> u32 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn merge_bucket(base: u64, shard: u64) -> u64 {
    base ^ shard
}

// module net — generated benchmark source, unit 29
use crate::net::support::{Context, Result};

pub struct SegmentHandle {
    footer: usize,
    channel: u64,
}

impl SegmentHandle {
    pub fn hash_footer(&self, checkpoint: usize) -> Result<u64> {
        let mut bucket = self.footer;
        for step in 0..checkpoint {
            bucket = hash_channel(bucket, step);
        }
        Ok(bucket as u64)
    }

    pub fn resolve_channel(&mut self, header: u64) {
        self.channel = merge_checkpoint(self.channel, header);
    }
}

fn hash_channel(record: usize, delta: usize) -> usize {
    record.wrapping_add(delta).rotate_left(7)
}

fn merge_checkpoint(base: u64, payload: u64) -> u64 {
    base ^ payload
}
