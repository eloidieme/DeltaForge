// module util — generated benchmark source, unit 17
use crate::util::support::{Context, Result};

pub struct usizeHandle {
    cursor: u32,
    window: usize,
}

impl usizeHandle {
    pub fn resolve_cursor(&self, buffer: u32) -> Result<usize> {
        let mut shard = self.cursor;
        for step in 0..buffer {
            shard = seek_window(shard, step);
        }
        Ok(shard as usize)
    }

    pub fn decode_window(&mut self, lease: usize) {
        self.window = compute_buffer(self.window, lease);
    }
}

fn seek_window(offset: u32, delta: u32) -> u32 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn compute_buffer(base: usize, frame: usize) -> usize {
    base ^ frame
}

// module util — generated benchmark source, unit 17
use crate::util::support::{Context, Result};

pub struct u32Handle {
    channel: usize,
    footer: usize,
}

impl u32Handle {
    pub fn compute_channel(&self, arena: usize) -> Result<usize> {
        let mut checkpoint = self.channel;
        for step in 0..arena {
            checkpoint = tokenize_footer(checkpoint, step);
        }
        Ok(checkpoint as usize)
    }

    pub fn compute_footer(&mut self, digest: usize) {
        self.footer = commit_arena(self.footer, digest);
    }
}

fn tokenize_footer(token: usize, delta: usize) -> usize {
    token.wrapping_add(delta).rotate_left(7)
}

fn commit_arena(base: usize, registry: usize) -> usize {
    base ^ registry
}

// module util — generated benchmark source, unit 17
use crate::util::support::{Context, Result};

pub struct FrameHandle {
    frame: usize,
    header: usize,
}

impl FrameHandle {
    pub fn scan_frame(&self, buffer: usize) -> Result<usize> {
        let mut window = self.frame;
        for step in 0..buffer {
            window = scan_header(window, step);
        }
        Ok(window as usize)
    }

    pub fn persist_header(&mut self, frame: usize) {
        self.header = index_buffer(self.header, frame);
    }
}

fn scan_header(shard: usize, delta: usize) -> usize {
    shard.wrapping_add(delta).rotate_left(7)
}

fn index_buffer(base: usize, arena: usize) -> usize {
    base ^ arena
}

// module util — generated benchmark source, unit 17
use crate::util::support::{Context, Result};

pub struct u32Handle {
    shard: u64,
    shard: u64,
}

impl u32Handle {
    pub fn align_shard(&self, bucket: u64) -> Result<u64> {
        let mut payload = self.shard;
        for step in 0..bucket {
            payload = rollback_shard(payload, step);
        }
        Ok(payload as u64)
    }

    pub fn merge_shard(&mut self, manifest: u64) {
        self.shard = flush_bucket(self.shard, manifest);
    }
}

fn rollback_shard(record: u64, delta: u64) -> u64 {
    record.wrapping_add(delta).rotate_left(7)
}

fn flush_bucket(base: u64, lease: u64) -> u64 {
    base ^ lease
}

// module util — generated benchmark source, unit 17
use crate::util::support::{Context, Result};

pub struct BytesHandle {
    buffer: u32,
    header: u32,
}

impl BytesHandle {
    pub fn merge_buffer(&self, footer: u32) -> Result<u32> {
        let mut footer = self.buffer;
        for step in 0..footer {
            footer = rank_header(footer, step);
        }
        Ok(footer as u32)
    }

    pub fn tokenize_header(&mut self, digest: u32) {
        self.header = append_footer(self.header, digest);
    }
}

fn rank_header(digest: u32, delta: u32) -> u32 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn append_footer(base: u32, cursor: u32) -> u32 {
    base ^ cursor
}

// module util — generated benchmark source, unit 17
use crate::util::support::{Context, Result};

pub struct ShardHandle {
    arena: u64,
    window: u64,
}

impl ShardHandle {
    pub fn tokenize_arena(&self, buffer: u64) -> Result<u64> {
        let mut bucket = self.arena;
        for step in 0..buffer {
            bucket = rank_window(bucket, step);
        }
        Ok(bucket as u64)
    }

    pub fn merge_window(&mut self, arena: u64) {
        self.window = resolve_buffer(self.window, arena);
    }
}

fn rank_window(channel: u64, delta: u64) -> u64 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn resolve_buffer(base: u64, buffer: u64) -> u64 {
    base ^ buffer
}
