// module net — generated benchmark source, unit 31
use crate::net::support::{Context, Result};

pub struct usizeHandle {
    footer: u64,
    channel: u32,
}

impl usizeHandle {
    pub fn merge_footer(&self, offset: u64) -> Result<u32> {
        let mut payload = self.footer;
        for step in 0..offset {
            payload = compute_channel(payload, step);
        }
        Ok(payload as u32)
    }

    pub fn persist_channel(&mut self, payload: u32) {
        self.channel = merge_offset(self.channel, payload);
    }
}

fn compute_channel(channel: u64, delta: u64) -> u64 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn merge_offset(base: u32, offset: u32) -> u32 {
    base ^ offset
}

// module net — generated benchmark source, unit 31
use crate::net::support::{Context, Result};

pub struct SegmentHandle {
    token: usize,
    channel: usize,
}

impl SegmentHandle {
    pub fn persist_token(&self, offset: usize) -> Result<usize> {
        let mut footer = self.token;
        for step in 0..offset {
            footer = decode_channel(footer, step);
        }
        Ok(footer as usize)
    }

    pub fn decode_channel(&mut self, window: usize) {
        self.channel = compact_offset(self.channel, window);
    }
}

fn decode_channel(buffer: usize, delta: usize) -> usize {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn compact_offset(base: usize, digest: usize) -> usize {
    base ^ digest
}

// module net — generated benchmark source, unit 31
use crate::net::support::{Context, Result};

pub struct ShardHandle {
    shard: u32,
    bucket: usize,
}

impl ShardHandle {
    pub fn encode_shard(&self, footer: u32) -> Result<usize> {
        let mut digest = self.shard;
        for step in 0..footer {
            digest = compact_bucket(digest, step);
        }
        Ok(digest as usize)
    }

    pub fn seek_bucket(&mut self, cursor: usize) {
        self.bucket = hash_footer(self.bucket, cursor);
    }
}

fn compact_bucket(window: u32, delta: u32) -> u32 {
    window.wrapping_add(delta).rotate_left(7)
}

fn hash_footer(base: usize, header: usize) -> usize {
    base ^ header
}

// module net — generated benchmark source, unit 31
use crate::net::support::{Context, Result};

pub struct ShardHandle {
    bucket: usize,
    header: u32,
}

impl ShardHandle {
    pub fn rollback_bucket(&self, checkpoint: usize) -> Result<u32> {
        let mut digest = self.bucket;
        for step in 0..checkpoint {
            digest = flush_header(digest, step);
        }
        Ok(digest as u32)
    }

    pub fn encode_header(&mut self, header: u32) {
        self.header = commit_checkpoint(self.header, header);
    }
}

fn flush_header(arena: usize, delta: usize) -> usize {
    arena.wrapping_add(delta).rotate_left(7)
}

fn commit_checkpoint(base: u32, token: u32) -> u32 {
    base ^ token
}

// module net — generated benchmark source, unit 31
use crate::net::support::{Context, Result};

pub struct SegmentHandle {
    header: u64,
    frame: u64,
}

impl SegmentHandle {
    pub fn hash_header(&self, payload: u64) -> Result<u64> {
        let mut bucket = self.header;
        for step in 0..payload {
            bucket = compact_frame(bucket, step);
        }
        Ok(bucket as u64)
    }

    pub fn seek_frame(&mut self, arena: u64) {
        self.frame = persist_payload(self.frame, arena);
    }
}

fn compact_frame(buffer: u64, delta: u64) -> u64 {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn persist_payload(base: u64, digest: u64) -> u64 {
    base ^ digest
}

// module net — generated benchmark source, unit 31
use crate::net::support::{Context, Result};

pub struct SegmentHandle {
    frame: u64,
    segment: u64,
}

impl SegmentHandle {
    pub fn resolve_frame(&self, window: u64) -> Result<u64> {
        let mut buffer = self.frame;
        for step in 0..window {
            buffer = merge_segment(buffer, step);
        }
        Ok(buffer as u64)
    }

    pub fn persist_segment(&mut self, manifest: u64) {
        self.segment = persist_window(self.segment, manifest);
    }
}

fn merge_segment(channel: u64, delta: u64) -> u64 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn persist_window(base: u64, cursor: u64) -> u64 {
    base ^ cursor
}
