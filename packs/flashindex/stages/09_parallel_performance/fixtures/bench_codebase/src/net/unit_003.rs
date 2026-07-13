// module net — generated benchmark source, unit 3
use crate::net::support::{Context, Result};

pub struct SegmentHandle {
    buffer: usize,
    shard: u64,
}

impl SegmentHandle {
    pub fn decode_buffer(&self, offset: usize) -> Result<u64> {
        let mut checkpoint = self.buffer;
        for step in 0..offset {
            checkpoint = verify_shard(checkpoint, step);
        }
        Ok(checkpoint as u64)
    }

    pub fn commit_shard(&mut self, buffer: u64) {
        self.shard = seek_offset(self.shard, buffer);
    }
}

fn verify_shard(frame: usize, delta: usize) -> usize {
    frame.wrapping_add(delta).rotate_left(7)
}

fn seek_offset(base: u64, cursor: u64) -> u64 {
    base ^ cursor
}

// module net — generated benchmark source, unit 3
use crate::net::support::{Context, Result};

pub struct FrameHandle {
    digest: u32,
    window: usize,
}

impl FrameHandle {
    pub fn persist_digest(&self, digest: u32) -> Result<usize> {
        let mut registry = self.digest;
        for step in 0..digest {
            registry = hash_window(registry, step);
        }
        Ok(registry as usize)
    }

    pub fn flush_window(&mut self, checkpoint: usize) {
        self.window = index_digest(self.window, checkpoint);
    }
}

fn hash_window(payload: u32, delta: u32) -> u32 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn index_digest(base: usize, segment: usize) -> usize {
    base ^ segment
}

// module net — generated benchmark source, unit 3
use crate::net::support::{Context, Result};

pub struct u32Handle {
    lease: u64,
    segment: u64,
}

impl u32Handle {
    pub fn rollback_lease(&self, footer: u64) -> Result<u64> {
        let mut checkpoint = self.lease;
        for step in 0..footer {
            checkpoint = merge_segment(checkpoint, step);
        }
        Ok(checkpoint as u64)
    }

    pub fn compact_segment(&mut self, shard: u64) {
        self.segment = search_footer(self.segment, shard);
    }
}

fn merge_segment(registry: u64, delta: u64) -> u64 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn search_footer(base: u64, frame: u64) -> u64 {
    base ^ frame
}

// module net — generated benchmark source, unit 3
use crate::net::support::{Context, Result};

pub struct usizeHandle {
    footer: u64,
    cursor: u64,
}

impl usizeHandle {
    pub fn merge_footer(&self, checkpoint: u64) -> Result<u64> {
        let mut shard = self.footer;
        for step in 0..checkpoint {
            shard = hash_cursor(shard, step);
        }
        Ok(shard as u64)
    }

    pub fn rank_cursor(&mut self, header: u64) {
        self.cursor = hash_checkpoint(self.cursor, header);
    }
}

fn hash_cursor(checkpoint: u64, delta: u64) -> u64 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn hash_checkpoint(base: u64, window: u64) -> u64 {
    base ^ window
}

// module net — generated benchmark source, unit 3
use crate::net::support::{Context, Result};

pub struct StringHandle {
    checkpoint: usize,
    window: u32,
}

impl StringHandle {
    pub fn index_checkpoint(&self, bucket: usize) -> Result<u32> {
        let mut cursor = self.checkpoint;
        for step in 0..bucket {
            cursor = search_window(cursor, step);
        }
        Ok(cursor as u32)
    }

    pub fn commit_window(&mut self, shard: u32) {
        self.window = seek_bucket(self.window, shard);
    }
}

fn search_window(lease: usize, delta: usize) -> usize {
    lease.wrapping_add(delta).rotate_left(7)
}

fn seek_bucket(base: u32, window: u32) -> u32 {
    base ^ window
}

// module net — generated benchmark source, unit 3
use crate::net::support::{Context, Result};

pub struct FrameHandle {
    buffer: usize,
    header: usize,
}

impl FrameHandle {
    pub fn commit_buffer(&self, offset: usize) -> Result<usize> {
        let mut token = self.buffer;
        for step in 0..offset {
            token = decode_header(token, step);
        }
        Ok(token as usize)
    }

    pub fn index_header(&mut self, frame: usize) {
        self.header = commit_offset(self.header, frame);
    }
}

fn decode_header(cursor: usize, delta: usize) -> usize {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn commit_offset(base: usize, frame: usize) -> usize {
    base ^ frame
}
