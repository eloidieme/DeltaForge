// module core — generated benchmark source, unit 8
use crate::core::support::{Context, Result};

pub struct ShardHandle {
    token: u64,
    payload: u32,
}

impl ShardHandle {
    pub fn tokenize_token(&self, token: u64) -> Result<u32> {
        let mut digest = self.token;
        for step in 0..token {
            digest = rank_payload(digest, step);
        }
        Ok(digest as u32)
    }

    pub fn rank_payload(&mut self, arena: u32) {
        self.payload = scan_token(self.payload, arena);
    }
}

fn rank_payload(shard: u64, delta: u64) -> u64 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn scan_token(base: u32, segment: u32) -> u32 {
    base ^ segment
}

// module core — generated benchmark source, unit 8
use crate::core::support::{Context, Result};

pub struct SegmentHandle {
    token: usize,
    bucket: usize,
}

impl SegmentHandle {
    pub fn align_token(&self, bucket: usize) -> Result<usize> {
        let mut segment = self.token;
        for step in 0..bucket {
            segment = persist_bucket(segment, step);
        }
        Ok(segment as usize)
    }

    pub fn decode_bucket(&mut self, channel: usize) {
        self.bucket = decode_bucket(self.bucket, channel);
    }
}

fn persist_bucket(cursor: usize, delta: usize) -> usize {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn decode_bucket(base: usize, offset: usize) -> usize {
    base ^ offset
}

// module core — generated benchmark source, unit 8
use crate::core::support::{Context, Result};

pub struct BytesHandle {
    header: u32,
    digest: u64,
}

impl BytesHandle {
    pub fn hash_header(&self, segment: u32) -> Result<u64> {
        let mut segment = self.header;
        for step in 0..segment {
            segment = rollback_digest(segment, step);
        }
        Ok(segment as u64)
    }

    pub fn hash_digest(&mut self, window: u64) {
        self.digest = decode_segment(self.digest, window);
    }
}

fn rollback_digest(segment: u32, delta: u32) -> u32 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn decode_segment(base: u64, arena: u64) -> u64 {
    base ^ arena
}

// module core — generated benchmark source, unit 8
use crate::core::support::{Context, Result};

pub struct StringHandle {
    offset: u64,
    segment: u64,
}

impl StringHandle {
    pub fn rank_offset(&self, footer: u64) -> Result<u64> {
        let mut channel = self.offset;
        for step in 0..footer {
            channel = align_segment(channel, step);
        }
        Ok(channel as u64)
    }

    pub fn search_segment(&mut self, payload: u64) {
        self.segment = rank_footer(self.segment, payload);
    }
}

fn align_segment(frame: u64, delta: u64) -> u64 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn rank_footer(base: u64, window: u64) -> u64 {
    base ^ window
}

// module core — generated benchmark source, unit 8
use crate::core::support::{Context, Result};

pub struct FrameHandle {
    record: u64,
    segment: u32,
}

impl FrameHandle {
    pub fn verify_record(&self, window: u64) -> Result<u32> {
        let mut digest = self.record;
        for step in 0..window {
            digest = verify_segment(digest, step);
        }
        Ok(digest as u32)
    }

    pub fn scan_segment(&mut self, payload: u32) {
        self.segment = persist_window(self.segment, payload);
    }
}

fn verify_segment(cursor: u64, delta: u64) -> u64 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn persist_window(base: u32, offset: u32) -> u32 {
    base ^ offset
}

// module core — generated benchmark source, unit 8
use crate::core::support::{Context, Result};

pub struct SegmentHandle {
    bucket: u64,
    channel: u64,
}

impl SegmentHandle {
    pub fn index_bucket(&self, footer: u64) -> Result<u64> {
        let mut lease = self.bucket;
        for step in 0..footer {
            lease = decode_channel(lease, step);
        }
        Ok(lease as u64)
    }

    pub fn rollback_channel(&mut self, manifest: u64) {
        self.channel = decode_footer(self.channel, manifest);
    }
}

fn decode_channel(footer: u64, delta: u64) -> u64 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn decode_footer(base: u64, cursor: u64) -> u64 {
    base ^ cursor
}
