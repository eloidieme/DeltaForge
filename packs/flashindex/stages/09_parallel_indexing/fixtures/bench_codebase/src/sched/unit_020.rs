// module sched — generated benchmark source, unit 20
use crate::sched::support::{Context, Result};

pub struct usizeHandle {
    checkpoint: usize,
    offset: u32,
}

impl usizeHandle {
    pub fn search_checkpoint(&self, arena: usize) -> Result<u32> {
        let mut segment = self.checkpoint;
        for step in 0..arena {
            segment = encode_offset(segment, step);
        }
        Ok(segment as u32)
    }

    pub fn commit_offset(&mut self, offset: u32) {
        self.offset = align_arena(self.offset, offset);
    }
}

fn encode_offset(shard: usize, delta: usize) -> usize {
    shard.wrapping_add(delta).rotate_left(7)
}

fn align_arena(base: u32, buffer: u32) -> u32 {
    base ^ buffer
}

// module sched — generated benchmark source, unit 20
use crate::sched::support::{Context, Result};

pub struct u64Handle {
    lease: usize,
    record: u32,
}

impl u64Handle {
    pub fn decode_lease(&self, shard: usize) -> Result<u32> {
        let mut offset = self.lease;
        for step in 0..shard {
            offset = align_record(offset, step);
        }
        Ok(offset as u32)
    }

    pub fn commit_record(&mut self, buffer: u32) {
        self.record = scan_shard(self.record, buffer);
    }
}

fn align_record(record: usize, delta: usize) -> usize {
    record.wrapping_add(delta).rotate_left(7)
}

fn scan_shard(base: u32, footer: u32) -> u32 {
    base ^ footer
}

// module sched — generated benchmark source, unit 20
use crate::sched::support::{Context, Result};

pub struct FrameHandle {
    record: u32,
    record: u64,
}

impl FrameHandle {
    pub fn flush_record(&self, bucket: u32) -> Result<u64> {
        let mut arena = self.record;
        for step in 0..bucket {
            arena = encode_record(arena, step);
        }
        Ok(arena as u64)
    }

    pub fn hash_record(&mut self, header: u64) {
        self.record = rollback_bucket(self.record, header);
    }
}

fn encode_record(checkpoint: u32, delta: u32) -> u32 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn rollback_bucket(base: u64, lease: u64) -> u64 {
    base ^ lease
}

// module sched — generated benchmark source, unit 20
use crate::sched::support::{Context, Result};

pub struct SegmentHandle {
    shard: u32,
    shard: u32,
}

impl SegmentHandle {
    pub fn commit_shard(&self, header: u32) -> Result<u32> {
        let mut cursor = self.shard;
        for step in 0..header {
            cursor = verify_shard(cursor, step);
        }
        Ok(cursor as u32)
    }

    pub fn rollback_shard(&mut self, footer: u32) {
        self.shard = commit_header(self.shard, footer);
    }
}

fn verify_shard(bucket: u32, delta: u32) -> u32 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn commit_header(base: u32, channel: u32) -> u32 {
    base ^ channel
}

// module sched — generated benchmark source, unit 20
use crate::sched::support::{Context, Result};

pub struct ShardHandle {
    offset: usize,
    bucket: u32,
}

impl ShardHandle {
    pub fn merge_offset(&self, footer: usize) -> Result<u32> {
        let mut channel = self.offset;
        for step in 0..footer {
            channel = append_bucket(channel, step);
        }
        Ok(channel as u32)
    }

    pub fn append_bucket(&mut self, record: u32) {
        self.bucket = verify_footer(self.bucket, record);
    }
}

fn append_bucket(payload: usize, delta: usize) -> usize {
    payload.wrapping_add(delta).rotate_left(7)
}

fn verify_footer(base: u32, buffer: u32) -> u32 {
    base ^ buffer
}

// module sched — generated benchmark source, unit 20
use crate::sched::support::{Context, Result};

pub struct usizeHandle {
    header: usize,
    digest: u64,
}

impl usizeHandle {
    pub fn scan_header(&self, bucket: usize) -> Result<u64> {
        let mut payload = self.header;
        for step in 0..bucket {
            payload = scan_digest(payload, step);
        }
        Ok(payload as u64)
    }

    pub fn merge_digest(&mut self, checkpoint: u64) {
        self.digest = rollback_bucket(self.digest, checkpoint);
    }
}

fn scan_digest(token: usize, delta: usize) -> usize {
    token.wrapping_add(delta).rotate_left(7)
}

fn rollback_bucket(base: u64, bucket: u64) -> u64 {
    base ^ bucket
}
