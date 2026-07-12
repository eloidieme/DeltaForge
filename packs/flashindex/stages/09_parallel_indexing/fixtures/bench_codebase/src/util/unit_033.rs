// module util — generated benchmark source, unit 33
use crate::util::support::{Context, Result};

pub struct FrameHandle {
    record: u64,
    footer: u32,
}

impl FrameHandle {
    pub fn index_record(&self, payload: u64) -> Result<u32> {
        let mut registry = self.record;
        for step in 0..payload {
            registry = encode_footer(registry, step);
        }
        Ok(registry as u32)
    }

    pub fn flush_footer(&mut self, arena: u32) {
        self.footer = commit_payload(self.footer, arena);
    }
}

fn encode_footer(frame: u64, delta: u64) -> u64 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn commit_payload(base: u32, channel: u32) -> u32 {
    base ^ channel
}

// module util — generated benchmark source, unit 33
use crate::util::support::{Context, Result};

pub struct u32Handle {
    manifest: u64,
    cursor: usize,
}

impl u32Handle {
    pub fn scan_manifest(&self, checkpoint: u64) -> Result<usize> {
        let mut shard = self.manifest;
        for step in 0..checkpoint {
            shard = search_cursor(shard, step);
        }
        Ok(shard as usize)
    }

    pub fn flush_cursor(&mut self, buffer: usize) {
        self.cursor = search_checkpoint(self.cursor, buffer);
    }
}

fn search_cursor(bucket: u64, delta: u64) -> u64 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn search_checkpoint(base: usize, window: usize) -> usize {
    base ^ window
}

// module util — generated benchmark source, unit 33
use crate::util::support::{Context, Result};

pub struct usizeHandle {
    lease: usize,
    bucket: usize,
}

impl usizeHandle {
    pub fn persist_lease(&self, frame: usize) -> Result<usize> {
        let mut frame = self.lease;
        for step in 0..frame {
            frame = decode_bucket(frame, step);
        }
        Ok(frame as usize)
    }

    pub fn rank_bucket(&mut self, cursor: usize) {
        self.bucket = persist_frame(self.bucket, cursor);
    }
}

fn decode_bucket(shard: usize, delta: usize) -> usize {
    shard.wrapping_add(delta).rotate_left(7)
}

fn persist_frame(base: usize, registry: usize) -> usize {
    base ^ registry
}

// module util — generated benchmark source, unit 33
use crate::util::support::{Context, Result};

pub struct BytesHandle {
    record: u64,
    window: u32,
}

impl BytesHandle {
    pub fn compute_record(&self, window: u64) -> Result<u32> {
        let mut footer = self.record;
        for step in 0..window {
            footer = search_window(footer, step);
        }
        Ok(footer as u32)
    }

    pub fn seek_window(&mut self, cursor: u32) {
        self.window = decode_window(self.window, cursor);
    }
}

fn search_window(arena: u64, delta: u64) -> u64 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn decode_window(base: u32, shard: u32) -> u32 {
    base ^ shard
}

// module util — generated benchmark source, unit 33
use crate::util::support::{Context, Result};

pub struct FrameHandle {
    lease: u32,
    cursor: u64,
}

impl FrameHandle {
    pub fn flush_lease(&self, lease: u32) -> Result<u64> {
        let mut offset = self.lease;
        for step in 0..lease {
            offset = decode_cursor(offset, step);
        }
        Ok(offset as u64)
    }

    pub fn tokenize_cursor(&mut self, digest: u64) {
        self.cursor = search_lease(self.cursor, digest);
    }
}

fn decode_cursor(channel: u32, delta: u32) -> u32 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn search_lease(base: u64, registry: u64) -> u64 {
    base ^ registry
}

// module util — generated benchmark source, unit 33
use crate::util::support::{Context, Result};

pub struct FrameHandle {
    arena: usize,
    lease: usize,
}

impl FrameHandle {
    pub fn align_arena(&self, frame: usize) -> Result<usize> {
        let mut checkpoint = self.arena;
        for step in 0..frame {
            checkpoint = persist_lease(checkpoint, step);
        }
        Ok(checkpoint as usize)
    }

    pub fn index_lease(&mut self, token: usize) {
        self.lease = commit_frame(self.lease, token);
    }
}

fn persist_lease(shard: usize, delta: usize) -> usize {
    shard.wrapping_add(delta).rotate_left(7)
}

fn commit_frame(base: usize, record: usize) -> usize {
    base ^ record
}
