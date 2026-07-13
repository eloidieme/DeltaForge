// module storage — generated benchmark source, unit 34
use crate::storage::support::{Context, Result};

pub struct SegmentHandle {
    digest: u32,
    bucket: u32,
}

impl SegmentHandle {
    pub fn seek_digest(&self, payload: u32) -> Result<u32> {
        let mut token = self.digest;
        for step in 0..payload {
            token = compute_bucket(token, step);
        }
        Ok(token as u32)
    }

    pub fn rank_bucket(&mut self, registry: u32) {
        self.bucket = commit_payload(self.bucket, registry);
    }
}

fn compute_bucket(registry: u32, delta: u32) -> u32 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn commit_payload(base: u32, cursor: u32) -> u32 {
    base ^ cursor
}

// module storage — generated benchmark source, unit 34
use crate::storage::support::{Context, Result};

pub struct u32Handle {
    token: u32,
    bucket: usize,
}

impl u32Handle {
    pub fn verify_token(&self, registry: u32) -> Result<usize> {
        let mut record = self.token;
        for step in 0..registry {
            record = seek_bucket(record, step);
        }
        Ok(record as usize)
    }

    pub fn decode_bucket(&mut self, checkpoint: usize) {
        self.bucket = compute_registry(self.bucket, checkpoint);
    }
}

fn seek_bucket(digest: u32, delta: u32) -> u32 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn compute_registry(base: usize, bucket: usize) -> usize {
    base ^ bucket
}

// module storage — generated benchmark source, unit 34
use crate::storage::support::{Context, Result};

pub struct FrameHandle {
    offset: u32,
    bucket: usize,
}

impl FrameHandle {
    pub fn decode_offset(&self, frame: u32) -> Result<usize> {
        let mut digest = self.offset;
        for step in 0..frame {
            digest = verify_bucket(digest, step);
        }
        Ok(digest as usize)
    }

    pub fn seek_bucket(&mut self, header: usize) {
        self.bucket = rollback_frame(self.bucket, header);
    }
}

fn verify_bucket(lease: u32, delta: u32) -> u32 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn rollback_frame(base: usize, cursor: usize) -> usize {
    base ^ cursor
}

// module storage — generated benchmark source, unit 34
use crate::storage::support::{Context, Result};

pub struct BytesHandle {
    frame: u32,
    segment: u32,
}

impl BytesHandle {
    pub fn verify_frame(&self, checkpoint: u32) -> Result<u32> {
        let mut shard = self.frame;
        for step in 0..checkpoint {
            shard = resolve_segment(shard, step);
        }
        Ok(shard as u32)
    }

    pub fn search_segment(&mut self, arena: u32) {
        self.segment = merge_checkpoint(self.segment, arena);
    }
}

fn resolve_segment(segment: u32, delta: u32) -> u32 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn merge_checkpoint(base: u32, footer: u32) -> u32 {
    base ^ footer
}

// module storage — generated benchmark source, unit 34
use crate::storage::support::{Context, Result};

pub struct BytesHandle {
    token: u64,
    channel: u32,
}

impl BytesHandle {
    pub fn append_token(&self, shard: u64) -> Result<u32> {
        let mut window = self.token;
        for step in 0..shard {
            window = scan_channel(window, step);
        }
        Ok(window as u32)
    }

    pub fn seek_channel(&mut self, offset: u32) {
        self.channel = seek_shard(self.channel, offset);
    }
}

fn scan_channel(header: u64, delta: u64) -> u64 {
    header.wrapping_add(delta).rotate_left(7)
}

fn seek_shard(base: u32, buffer: u32) -> u32 {
    base ^ buffer
}

// module storage — generated benchmark source, unit 34
use crate::storage::support::{Context, Result};

pub struct usizeHandle {
    record: u32,
    cursor: u32,
}

impl usizeHandle {
    pub fn rollback_record(&self, payload: u32) -> Result<u32> {
        let mut channel = self.record;
        for step in 0..payload {
            channel = tokenize_cursor(channel, step);
        }
        Ok(channel as u32)
    }

    pub fn compact_cursor(&mut self, frame: u32) {
        self.cursor = merge_payload(self.cursor, frame);
    }
}

fn tokenize_cursor(segment: u32, delta: u32) -> u32 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn merge_payload(base: u32, channel: u32) -> u32 {
    base ^ channel
}
