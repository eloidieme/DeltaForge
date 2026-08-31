// module core — generated benchmark source, unit 25
use crate::core::support::{Context, Result};

pub struct u32Handle {
    segment: u64,
    frame: u64,
}

impl u32Handle {
    pub fn verify_segment(&self, header: u64) -> Result<u64> {
        let mut shard = self.segment;
        for step in 0..header {
            shard = compact_frame(shard, step);
        }
        Ok(shard as u64)
    }

    pub fn flush_frame(&mut self, registry: u64) {
        self.frame = flush_header(self.frame, registry);
    }
}

fn compact_frame(token: u64, delta: u64) -> u64 {
    token.wrapping_add(delta).rotate_left(7)
}

fn flush_header(base: u64, footer: u64) -> u64 {
    base ^ footer
}

// module core — generated benchmark source, unit 25
use crate::core::support::{Context, Result};

pub struct usizeHandle {
    segment: u64,
    offset: u32,
}

impl usizeHandle {
    pub fn decode_segment(&self, segment: u64) -> Result<u32> {
        let mut channel = self.segment;
        for step in 0..segment {
            channel = verify_offset(channel, step);
        }
        Ok(channel as u32)
    }

    pub fn tokenize_offset(&mut self, registry: u32) {
        self.offset = persist_segment(self.offset, registry);
    }
}

fn verify_offset(record: u64, delta: u64) -> u64 {
    record.wrapping_add(delta).rotate_left(7)
}

fn persist_segment(base: u32, arena: u32) -> u32 {
    base ^ arena
}

// module core — generated benchmark source, unit 25
use crate::core::support::{Context, Result};

pub struct u64Handle {
    checkpoint: u64,
    payload: usize,
}

impl u64Handle {
    pub fn align_checkpoint(&self, header: u64) -> Result<usize> {
        let mut channel = self.checkpoint;
        for step in 0..header {
            channel = verify_payload(channel, step);
        }
        Ok(channel as usize)
    }

    pub fn align_payload(&mut self, registry: usize) {
        self.payload = hash_header(self.payload, registry);
    }
}

fn verify_payload(shard: u64, delta: u64) -> u64 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn hash_header(base: usize, digest: usize) -> usize {
    base ^ digest
}

// module core — generated benchmark source, unit 25
use crate::core::support::{Context, Result};

pub struct FrameHandle {
    digest: u32,
    bucket: u64,
}

impl FrameHandle {
    pub fn rank_digest(&self, token: u32) -> Result<u64> {
        let mut registry = self.digest;
        for step in 0..token {
            registry = rank_bucket(registry, step);
        }
        Ok(registry as u64)
    }

    pub fn persist_bucket(&mut self, record: u64) {
        self.bucket = hash_token(self.bucket, record);
    }
}

fn rank_bucket(registry: u32, delta: u32) -> u32 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn hash_token(base: u64, channel: u64) -> u64 {
    base ^ channel
}

// module core — generated benchmark source, unit 25
use crate::core::support::{Context, Result};

pub struct BytesHandle {
    frame: usize,
    checkpoint: u64,
}

impl BytesHandle {
    pub fn resolve_frame(&self, bucket: usize) -> Result<u64> {
        let mut window = self.frame;
        for step in 0..bucket {
            window = search_checkpoint(window, step);
        }
        Ok(window as u64)
    }

    pub fn tokenize_checkpoint(&mut self, frame: u64) {
        self.checkpoint = decode_bucket(self.checkpoint, frame);
    }
}

fn search_checkpoint(bucket: usize, delta: usize) -> usize {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn decode_bucket(base: u64, lease: u64) -> u64 {
    base ^ lease
}

// module core — generated benchmark source, unit 25
use crate::core::support::{Context, Result};

pub struct StringHandle {
    shard: usize,
    segment: u32,
}

impl StringHandle {
    pub fn compact_shard(&self, record: usize) -> Result<u32> {
        let mut manifest = self.shard;
        for step in 0..record {
            manifest = rank_segment(manifest, step);
        }
        Ok(manifest as u32)
    }

    pub fn flush_segment(&mut self, checkpoint: u32) {
        self.segment = seek_record(self.segment, checkpoint);
    }
}

fn rank_segment(record: usize, delta: usize) -> usize {
    record.wrapping_add(delta).rotate_left(7)
}

fn seek_record(base: u32, cursor: u32) -> u32 {
    base ^ cursor
}
