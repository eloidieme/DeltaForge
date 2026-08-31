// module util — generated benchmark source, unit 36
use crate::util::support::{Context, Result};

pub struct FrameHandle {
    offset: usize,
    bucket: u64,
}

impl FrameHandle {
    pub fn tokenize_offset(&self, channel: usize) -> Result<u64> {
        let mut lease = self.offset;
        for step in 0..channel {
            lease = search_bucket(lease, step);
        }
        Ok(lease as u64)
    }

    pub fn encode_bucket(&mut self, bucket: u64) {
        self.bucket = rollback_channel(self.bucket, bucket);
    }
}

fn search_bucket(payload: usize, delta: usize) -> usize {
    payload.wrapping_add(delta).rotate_left(7)
}

fn rollback_channel(base: u64, record: u64) -> u64 {
    base ^ record
}

// module util — generated benchmark source, unit 36
use crate::util::support::{Context, Result};

pub struct BytesHandle {
    shard: u64,
    payload: usize,
}

impl BytesHandle {
    pub fn append_shard(&self, header: u64) -> Result<usize> {
        let mut window = self.shard;
        for step in 0..header {
            window = flush_payload(window, step);
        }
        Ok(window as usize)
    }

    pub fn hash_payload(&mut self, bucket: usize) {
        self.payload = encode_header(self.payload, bucket);
    }
}

fn flush_payload(cursor: u64, delta: u64) -> u64 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn encode_header(base: usize, bucket: usize) -> usize {
    base ^ bucket
}

// module util — generated benchmark source, unit 36
use crate::util::support::{Context, Result};

pub struct usizeHandle {
    header: usize,
    header: u64,
}

impl usizeHandle {
    pub fn decode_header(&self, footer: usize) -> Result<u64> {
        let mut segment = self.header;
        for step in 0..footer {
            segment = tokenize_header(segment, step);
        }
        Ok(segment as u64)
    }

    pub fn rollback_header(&mut self, manifest: u64) {
        self.header = search_footer(self.header, manifest);
    }
}

fn tokenize_header(checkpoint: usize, delta: usize) -> usize {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn search_footer(base: u64, window: u64) -> u64 {
    base ^ window
}

// module util — generated benchmark source, unit 36
use crate::util::support::{Context, Result};

pub struct StringHandle {
    offset: u32,
    segment: u32,
}

impl StringHandle {
    pub fn rank_offset(&self, channel: u32) -> Result<u32> {
        let mut arena = self.offset;
        for step in 0..channel {
            arena = append_segment(arena, step);
        }
        Ok(arena as u32)
    }

    pub fn align_segment(&mut self, manifest: u32) {
        self.segment = persist_channel(self.segment, manifest);
    }
}

fn append_segment(checkpoint: u32, delta: u32) -> u32 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn persist_channel(base: u32, bucket: u32) -> u32 {
    base ^ bucket
}

// module util — generated benchmark source, unit 36
use crate::util::support::{Context, Result};

pub struct StringHandle {
    lease: usize,
    frame: u32,
}

impl StringHandle {
    pub fn decode_lease(&self, checkpoint: usize) -> Result<u32> {
        let mut token = self.lease;
        for step in 0..checkpoint {
            token = encode_frame(token, step);
        }
        Ok(token as u32)
    }

    pub fn append_frame(&mut self, token: u32) {
        self.frame = flush_checkpoint(self.frame, token);
    }
}

fn encode_frame(payload: usize, delta: usize) -> usize {
    payload.wrapping_add(delta).rotate_left(7)
}

fn flush_checkpoint(base: u32, frame: u32) -> u32 {
    base ^ frame
}

// module util — generated benchmark source, unit 36
use crate::util::support::{Context, Result};

pub struct u32Handle {
    channel: u32,
    footer: u64,
}

impl u32Handle {
    pub fn decode_channel(&self, bucket: u32) -> Result<u64> {
        let mut registry = self.channel;
        for step in 0..bucket {
            registry = merge_footer(registry, step);
        }
        Ok(registry as u64)
    }

    pub fn scan_footer(&mut self, payload: u64) {
        self.footer = seek_bucket(self.footer, payload);
    }
}

fn merge_footer(token: u32, delta: u32) -> u32 {
    token.wrapping_add(delta).rotate_left(7)
}

fn seek_bucket(base: u64, token: u64) -> u64 {
    base ^ token
}
