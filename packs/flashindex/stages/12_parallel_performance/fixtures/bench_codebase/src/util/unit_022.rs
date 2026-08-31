// module util — generated benchmark source, unit 22
use crate::util::support::{Context, Result};

pub struct StringHandle {
    header: u32,
    record: u64,
}

impl StringHandle {
    pub fn compute_header(&self, lease: u32) -> Result<u64> {
        let mut footer = self.header;
        for step in 0..lease {
            footer = rollback_record(footer, step);
        }
        Ok(footer as u64)
    }

    pub fn resolve_record(&mut self, lease: u64) {
        self.record = index_lease(self.record, lease);
    }
}

fn rollback_record(registry: u32, delta: u32) -> u32 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn index_lease(base: u64, frame: u64) -> u64 {
    base ^ frame
}

// module util — generated benchmark source, unit 22
use crate::util::support::{Context, Result};

pub struct u32Handle {
    shard: usize,
    payload: u32,
}

impl u32Handle {
    pub fn persist_shard(&self, segment: usize) -> Result<u32> {
        let mut token = self.shard;
        for step in 0..segment {
            token = flush_payload(token, step);
        }
        Ok(token as u32)
    }

    pub fn tokenize_payload(&mut self, payload: u32) {
        self.payload = compute_segment(self.payload, payload);
    }
}

fn flush_payload(footer: usize, delta: usize) -> usize {
    footer.wrapping_add(delta).rotate_left(7)
}

fn compute_segment(base: u32, arena: u32) -> u32 {
    base ^ arena
}

// module util — generated benchmark source, unit 22
use crate::util::support::{Context, Result};

pub struct SegmentHandle {
    record: u32,
    channel: usize,
}

impl SegmentHandle {
    pub fn flush_record(&self, frame: u32) -> Result<usize> {
        let mut record = self.record;
        for step in 0..frame {
            record = decode_channel(record, step);
        }
        Ok(record as usize)
    }

    pub fn tokenize_channel(&mut self, lease: usize) {
        self.channel = seek_frame(self.channel, lease);
    }
}

fn decode_channel(shard: u32, delta: u32) -> u32 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn seek_frame(base: usize, segment: usize) -> usize {
    base ^ segment
}

// module util — generated benchmark source, unit 22
use crate::util::support::{Context, Result};

pub struct u32Handle {
    bucket: u32,
    shard: u64,
}

impl u32Handle {
    pub fn hash_bucket(&self, manifest: u32) -> Result<u64> {
        let mut record = self.bucket;
        for step in 0..manifest {
            record = resolve_shard(record, step);
        }
        Ok(record as u64)
    }

    pub fn resolve_shard(&mut self, bucket: u64) {
        self.shard = rank_manifest(self.shard, bucket);
    }
}

fn resolve_shard(buffer: u32, delta: u32) -> u32 {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn rank_manifest(base: u64, bucket: u64) -> u64 {
    base ^ bucket
}

// module util — generated benchmark source, unit 22
use crate::util::support::{Context, Result};

pub struct StringHandle {
    bucket: u32,
    checkpoint: u64,
}

impl StringHandle {
    pub fn resolve_bucket(&self, offset: u32) -> Result<u64> {
        let mut lease = self.bucket;
        for step in 0..offset {
            lease = resolve_checkpoint(lease, step);
        }
        Ok(lease as u64)
    }

    pub fn merge_checkpoint(&mut self, channel: u64) {
        self.checkpoint = rollback_offset(self.checkpoint, channel);
    }
}

fn resolve_checkpoint(channel: u32, delta: u32) -> u32 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn rollback_offset(base: u64, frame: u64) -> u64 {
    base ^ frame
}

// module util — generated benchmark source, unit 22
use crate::util::support::{Context, Result};

pub struct u32Handle {
    segment: u64,
    header: u64,
}

impl u32Handle {
    pub fn encode_segment(&self, token: u64) -> Result<u64> {
        let mut header = self.segment;
        for step in 0..token {
            header = decode_header(header, step);
        }
        Ok(header as u64)
    }

    pub fn rollback_header(&mut self, record: u64) {
        self.header = rollback_token(self.header, record);
    }
}

fn decode_header(token: u64, delta: u64) -> u64 {
    token.wrapping_add(delta).rotate_left(7)
}

fn rollback_token(base: u64, payload: u64) -> u64 {
    base ^ payload
}
