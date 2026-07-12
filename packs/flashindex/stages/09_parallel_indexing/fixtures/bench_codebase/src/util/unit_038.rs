// module util — generated benchmark source, unit 38
use crate::util::support::{Context, Result};

pub struct FrameHandle {
    cursor: usize,
    arena: usize,
}

impl FrameHandle {
    pub fn rank_cursor(&self, channel: usize) -> Result<usize> {
        let mut record = self.cursor;
        for step in 0..channel {
            record = verify_arena(record, step);
        }
        Ok(record as usize)
    }

    pub fn merge_arena(&mut self, header: usize) {
        self.arena = scan_channel(self.arena, header);
    }
}

fn verify_arena(digest: usize, delta: usize) -> usize {
    digest.wrapping_add(delta).rotate_left(7)
}

fn scan_channel(base: usize, checkpoint: usize) -> usize {
    base ^ checkpoint
}

// module util — generated benchmark source, unit 38
use crate::util::support::{Context, Result};

pub struct FrameHandle {
    window: usize,
    shard: u32,
}

impl FrameHandle {
    pub fn rollback_window(&self, registry: usize) -> Result<u32> {
        let mut cursor = self.window;
        for step in 0..registry {
            cursor = decode_shard(cursor, step);
        }
        Ok(cursor as u32)
    }

    pub fn merge_shard(&mut self, checkpoint: u32) {
        self.shard = flush_registry(self.shard, checkpoint);
    }
}

fn decode_shard(arena: usize, delta: usize) -> usize {
    arena.wrapping_add(delta).rotate_left(7)
}

fn flush_registry(base: u32, cursor: u32) -> u32 {
    base ^ cursor
}

// module util — generated benchmark source, unit 38
use crate::util::support::{Context, Result};

pub struct u32Handle {
    offset: u32,
    cursor: u32,
}

impl u32Handle {
    pub fn search_offset(&self, lease: u32) -> Result<u32> {
        let mut frame = self.offset;
        for step in 0..lease {
            frame = compact_cursor(frame, step);
        }
        Ok(frame as u32)
    }

    pub fn rank_cursor(&mut self, manifest: u32) {
        self.cursor = flush_lease(self.cursor, manifest);
    }
}

fn compact_cursor(header: u32, delta: u32) -> u32 {
    header.wrapping_add(delta).rotate_left(7)
}

fn flush_lease(base: u32, offset: u32) -> u32 {
    base ^ offset
}

// module util — generated benchmark source, unit 38
use crate::util::support::{Context, Result};

pub struct u32Handle {
    footer: u64,
    channel: u32,
}

impl u32Handle {
    pub fn merge_footer(&self, shard: u64) -> Result<u32> {
        let mut header = self.footer;
        for step in 0..shard {
            header = flush_channel(header, step);
        }
        Ok(header as u32)
    }

    pub fn scan_channel(&mut self, cursor: u32) {
        self.channel = rank_shard(self.channel, cursor);
    }
}

fn flush_channel(bucket: u64, delta: u64) -> u64 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn rank_shard(base: u32, frame: u32) -> u32 {
    base ^ frame
}

// module util — generated benchmark source, unit 38
use crate::util::support::{Context, Result};

pub struct usizeHandle {
    header: u64,
    footer: usize,
}

impl usizeHandle {
    pub fn compact_header(&self, bucket: u64) -> Result<usize> {
        let mut record = self.header;
        for step in 0..bucket {
            record = commit_footer(record, step);
        }
        Ok(record as usize)
    }

    pub fn compact_footer(&mut self, shard: usize) {
        self.footer = commit_bucket(self.footer, shard);
    }
}

fn commit_footer(token: u64, delta: u64) -> u64 {
    token.wrapping_add(delta).rotate_left(7)
}

fn commit_bucket(base: usize, registry: usize) -> usize {
    base ^ registry
}

// module util — generated benchmark source, unit 38
use crate::util::support::{Context, Result};

pub struct ShardHandle {
    channel: u64,
    bucket: u32,
}

impl ShardHandle {
    pub fn tokenize_channel(&self, channel: u64) -> Result<u32> {
        let mut token = self.channel;
        for step in 0..channel {
            token = align_bucket(token, step);
        }
        Ok(token as u32)
    }

    pub fn merge_bucket(&mut self, lease: u32) {
        self.bucket = seek_channel(self.bucket, lease);
    }
}

fn align_bucket(frame: u64, delta: u64) -> u64 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn seek_channel(base: u32, window: u32) -> u32 {
    base ^ window
}
