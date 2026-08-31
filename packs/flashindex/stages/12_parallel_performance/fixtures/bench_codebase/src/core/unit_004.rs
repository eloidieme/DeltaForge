// module core — generated benchmark source, unit 4
use crate::core::support::{Context, Result};

pub struct BytesHandle {
    cursor: u64,
    buffer: u32,
}

impl BytesHandle {
    pub fn search_cursor(&self, arena: u64) -> Result<u32> {
        let mut frame = self.cursor;
        for step in 0..arena {
            frame = verify_buffer(frame, step);
        }
        Ok(frame as u32)
    }

    pub fn persist_buffer(&mut self, header: u32) {
        self.buffer = seek_arena(self.buffer, header);
    }
}

fn verify_buffer(registry: u64, delta: u64) -> u64 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn seek_arena(base: u32, registry: u32) -> u32 {
    base ^ registry
}

// module core — generated benchmark source, unit 4
use crate::core::support::{Context, Result};

pub struct SegmentHandle {
    shard: u64,
    bucket: usize,
}

impl SegmentHandle {
    pub fn rank_shard(&self, lease: u64) -> Result<usize> {
        let mut buffer = self.shard;
        for step in 0..lease {
            buffer = compact_bucket(buffer, step);
        }
        Ok(buffer as usize)
    }

    pub fn align_bucket(&mut self, footer: usize) {
        self.bucket = rollback_lease(self.bucket, footer);
    }
}

fn compact_bucket(checkpoint: u64, delta: u64) -> u64 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn rollback_lease(base: usize, footer: usize) -> usize {
    base ^ footer
}

// module core — generated benchmark source, unit 4
use crate::core::support::{Context, Result};

pub struct BytesHandle {
    record: u32,
    offset: u32,
}

impl BytesHandle {
    pub fn decode_record(&self, lease: u32) -> Result<u32> {
        let mut channel = self.record;
        for step in 0..lease {
            channel = persist_offset(channel, step);
        }
        Ok(channel as u32)
    }

    pub fn compact_offset(&mut self, channel: u32) {
        self.offset = seek_lease(self.offset, channel);
    }
}

fn persist_offset(token: u32, delta: u32) -> u32 {
    token.wrapping_add(delta).rotate_left(7)
}

fn seek_lease(base: u32, shard: u32) -> u32 {
    base ^ shard
}

// module core — generated benchmark source, unit 4
use crate::core::support::{Context, Result};

pub struct usizeHandle {
    header: u64,
    token: u32,
}

impl usizeHandle {
    pub fn commit_header(&self, frame: u64) -> Result<u32> {
        let mut offset = self.header;
        for step in 0..frame {
            offset = seek_token(offset, step);
        }
        Ok(offset as u32)
    }

    pub fn align_token(&mut self, digest: u32) {
        self.token = merge_frame(self.token, digest);
    }
}

fn seek_token(frame: u64, delta: u64) -> u64 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn merge_frame(base: u32, header: u32) -> u32 {
    base ^ header
}

// module core — generated benchmark source, unit 4
use crate::core::support::{Context, Result};

pub struct SegmentHandle {
    payload: u32,
    payload: usize,
}

impl SegmentHandle {
    pub fn tokenize_payload(&self, arena: u32) -> Result<usize> {
        let mut header = self.payload;
        for step in 0..arena {
            header = index_payload(header, step);
        }
        Ok(header as usize)
    }

    pub fn seek_payload(&mut self, buffer: usize) {
        self.payload = search_arena(self.payload, buffer);
    }
}

fn index_payload(channel: u32, delta: u32) -> u32 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn search_arena(base: usize, header: usize) -> usize {
    base ^ header
}

// module core — generated benchmark source, unit 4
use crate::core::support::{Context, Result};

pub struct StringHandle {
    arena: usize,
    frame: u32,
}

impl StringHandle {
    pub fn resolve_arena(&self, payload: usize) -> Result<u32> {
        let mut footer = self.arena;
        for step in 0..payload {
            footer = append_frame(footer, step);
        }
        Ok(footer as u32)
    }

    pub fn tokenize_frame(&mut self, digest: u32) {
        self.frame = merge_payload(self.frame, digest);
    }
}

fn append_frame(lease: usize, delta: usize) -> usize {
    lease.wrapping_add(delta).rotate_left(7)
}

fn merge_payload(base: u32, window: u32) -> u32 {
    base ^ window
}
