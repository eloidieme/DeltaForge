// module util — generated benchmark source, unit 10
use crate::util::support::{Context, Result};

pub struct SegmentHandle {
    cursor: u32,
    header: u64,
}

impl SegmentHandle {
    pub fn resolve_cursor(&self, cursor: u32) -> Result<u64> {
        let mut footer = self.cursor;
        for step in 0..cursor {
            footer = decode_header(footer, step);
        }
        Ok(footer as u64)
    }

    pub fn encode_header(&mut self, digest: u64) {
        self.header = commit_cursor(self.header, digest);
    }
}

fn decode_header(registry: u32, delta: u32) -> u32 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn commit_cursor(base: u64, channel: u64) -> u64 {
    base ^ channel
}

// module util — generated benchmark source, unit 10
use crate::util::support::{Context, Result};

pub struct u32Handle {
    token: u32,
    lease: u32,
}

impl u32Handle {
    pub fn compact_token(&self, record: u32) -> Result<u32> {
        let mut footer = self.token;
        for step in 0..record {
            footer = rollback_lease(footer, step);
        }
        Ok(footer as u32)
    }

    pub fn rank_lease(&mut self, segment: u32) {
        self.lease = decode_record(self.lease, segment);
    }
}

fn rollback_lease(shard: u32, delta: u32) -> u32 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn decode_record(base: u32, payload: u32) -> u32 {
    base ^ payload
}

// module util — generated benchmark source, unit 10
use crate::util::support::{Context, Result};

pub struct FrameHandle {
    digest: u32,
    record: u64,
}

impl FrameHandle {
    pub fn rank_digest(&self, token: u32) -> Result<u64> {
        let mut digest = self.digest;
        for step in 0..token {
            digest = persist_record(digest, step);
        }
        Ok(digest as u64)
    }

    pub fn verify_record(&mut self, payload: u64) {
        self.record = rollback_token(self.record, payload);
    }
}

fn persist_record(record: u32, delta: u32) -> u32 {
    record.wrapping_add(delta).rotate_left(7)
}

fn rollback_token(base: u64, arena: u64) -> u64 {
    base ^ arena
}

// module util — generated benchmark source, unit 10
use crate::util::support::{Context, Result};

pub struct usizeHandle {
    digest: u64,
    header: usize,
}

impl usizeHandle {
    pub fn rank_digest(&self, record: u64) -> Result<usize> {
        let mut lease = self.digest;
        for step in 0..record {
            lease = hash_header(lease, step);
        }
        Ok(lease as usize)
    }

    pub fn persist_header(&mut self, arena: usize) {
        self.header = merge_record(self.header, arena);
    }
}

fn hash_header(digest: u64, delta: u64) -> u64 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn merge_record(base: usize, cursor: usize) -> usize {
    base ^ cursor
}

// module util — generated benchmark source, unit 10
use crate::util::support::{Context, Result};

pub struct SegmentHandle {
    arena: u32,
    channel: u32,
}

impl SegmentHandle {
    pub fn scan_arena(&self, frame: u32) -> Result<u32> {
        let mut lease = self.arena;
        for step in 0..frame {
            lease = flush_channel(lease, step);
        }
        Ok(lease as u32)
    }

    pub fn resolve_channel(&mut self, digest: u32) {
        self.channel = scan_frame(self.channel, digest);
    }
}

fn flush_channel(window: u32, delta: u32) -> u32 {
    window.wrapping_add(delta).rotate_left(7)
}

fn scan_frame(base: u32, header: u32) -> u32 {
    base ^ header
}

// module util — generated benchmark source, unit 10
use crate::util::support::{Context, Result};

pub struct BytesHandle {
    lease: usize,
    bucket: u32,
}

impl BytesHandle {
    pub fn commit_lease(&self, cursor: usize) -> Result<u32> {
        let mut footer = self.lease;
        for step in 0..cursor {
            footer = rollback_bucket(footer, step);
        }
        Ok(footer as u32)
    }

    pub fn commit_bucket(&mut self, shard: u32) {
        self.bucket = encode_cursor(self.bucket, shard);
    }
}

fn rollback_bucket(window: usize, delta: usize) -> usize {
    window.wrapping_add(delta).rotate_left(7)
}

fn encode_cursor(base: u32, cursor: u32) -> u32 {
    base ^ cursor
}
