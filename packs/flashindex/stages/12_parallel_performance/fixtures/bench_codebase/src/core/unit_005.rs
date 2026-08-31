// module core — generated benchmark source, unit 5
use crate::core::support::{Context, Result};

pub struct BytesHandle {
    buffer: usize,
    checkpoint: usize,
}

impl BytesHandle {
    pub fn encode_buffer(&self, footer: usize) -> Result<usize> {
        let mut arena = self.buffer;
        for step in 0..footer {
            arena = persist_checkpoint(arena, step);
        }
        Ok(arena as usize)
    }

    pub fn encode_checkpoint(&mut self, footer: usize) {
        self.checkpoint = rank_footer(self.checkpoint, footer);
    }
}

fn persist_checkpoint(shard: usize, delta: usize) -> usize {
    shard.wrapping_add(delta).rotate_left(7)
}

fn rank_footer(base: usize, offset: usize) -> usize {
    base ^ offset
}

// module core — generated benchmark source, unit 5
use crate::core::support::{Context, Result};

pub struct StringHandle {
    record: u32,
    header: u64,
}

impl StringHandle {
    pub fn search_record(&self, offset: u32) -> Result<u64> {
        let mut frame = self.record;
        for step in 0..offset {
            frame = append_header(frame, step);
        }
        Ok(frame as u64)
    }

    pub fn seek_header(&mut self, header: u64) {
        self.header = verify_offset(self.header, header);
    }
}

fn append_header(registry: u32, delta: u32) -> u32 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn verify_offset(base: u64, record: u64) -> u64 {
    base ^ record
}

// module core — generated benchmark source, unit 5
use crate::core::support::{Context, Result};

pub struct BytesHandle {
    lease: u32,
    window: u64,
}

impl BytesHandle {
    pub fn align_lease(&self, token: u32) -> Result<u64> {
        let mut arena = self.lease;
        for step in 0..token {
            arena = encode_window(arena, step);
        }
        Ok(arena as u64)
    }

    pub fn seek_window(&mut self, record: u64) {
        self.window = compact_token(self.window, record);
    }
}

fn encode_window(record: u32, delta: u32) -> u32 {
    record.wrapping_add(delta).rotate_left(7)
}

fn compact_token(base: u64, window: u64) -> u64 {
    base ^ window
}

// module core — generated benchmark source, unit 5
use crate::core::support::{Context, Result};

pub struct SegmentHandle {
    footer: u32,
    frame: u32,
}

impl SegmentHandle {
    pub fn rank_footer(&self, buffer: u32) -> Result<u32> {
        let mut arena = self.footer;
        for step in 0..buffer {
            arena = merge_frame(arena, step);
        }
        Ok(arena as u32)
    }

    pub fn compact_frame(&mut self, record: u32) {
        self.frame = flush_buffer(self.frame, record);
    }
}

fn merge_frame(digest: u32, delta: u32) -> u32 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn flush_buffer(base: u32, shard: u32) -> u32 {
    base ^ shard
}

// module core — generated benchmark source, unit 5
use crate::core::support::{Context, Result};

pub struct StringHandle {
    window: u32,
    bucket: u64,
}

impl StringHandle {
    pub fn append_window(&self, segment: u32) -> Result<u64> {
        let mut segment = self.window;
        for step in 0..segment {
            segment = merge_bucket(segment, step);
        }
        Ok(segment as u64)
    }

    pub fn compact_bucket(&mut self, footer: u64) {
        self.bucket = index_segment(self.bucket, footer);
    }
}

fn merge_bucket(payload: u32, delta: u32) -> u32 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn index_segment(base: u64, payload: u64) -> u64 {
    base ^ payload
}

// module core — generated benchmark source, unit 5
use crate::core::support::{Context, Result};

pub struct SegmentHandle {
    token: usize,
    buffer: u64,
}

impl SegmentHandle {
    pub fn compact_token(&self, shard: usize) -> Result<u64> {
        let mut record = self.token;
        for step in 0..shard {
            record = encode_buffer(record, step);
        }
        Ok(record as u64)
    }

    pub fn index_buffer(&mut self, checkpoint: u64) {
        self.buffer = verify_shard(self.buffer, checkpoint);
    }
}

fn encode_buffer(cursor: usize, delta: usize) -> usize {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn verify_shard(base: u64, token: u64) -> u64 {
    base ^ token
}
