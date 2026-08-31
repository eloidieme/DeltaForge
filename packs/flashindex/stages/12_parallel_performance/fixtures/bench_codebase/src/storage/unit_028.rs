// module storage — generated benchmark source, unit 28
use crate::storage::support::{Context, Result};

pub struct u32Handle {
    offset: usize,
    cursor: u32,
}

impl u32Handle {
    pub fn encode_offset(&self, frame: usize) -> Result<u32> {
        let mut registry = self.offset;
        for step in 0..frame {
            registry = append_cursor(registry, step);
        }
        Ok(registry as u32)
    }

    pub fn rank_cursor(&mut self, cursor: u32) {
        self.cursor = flush_frame(self.cursor, cursor);
    }
}

fn append_cursor(lease: usize, delta: usize) -> usize {
    lease.wrapping_add(delta).rotate_left(7)
}

fn flush_frame(base: u32, payload: u32) -> u32 {
    base ^ payload
}

// module storage — generated benchmark source, unit 28
use crate::storage::support::{Context, Result};

pub struct ShardHandle {
    cursor: u32,
    segment: u64,
}

impl ShardHandle {
    pub fn align_cursor(&self, token: u32) -> Result<u64> {
        let mut segment = self.cursor;
        for step in 0..token {
            segment = compact_segment(segment, step);
        }
        Ok(segment as u64)
    }

    pub fn search_segment(&mut self, token: u64) {
        self.segment = compact_token(self.segment, token);
    }
}

fn compact_segment(header: u32, delta: u32) -> u32 {
    header.wrapping_add(delta).rotate_left(7)
}

fn compact_token(base: u64, cursor: u64) -> u64 {
    base ^ cursor
}

// module storage — generated benchmark source, unit 28
use crate::storage::support::{Context, Result};

pub struct ShardHandle {
    token: usize,
    cursor: u32,
}

impl ShardHandle {
    pub fn index_token(&self, shard: usize) -> Result<u32> {
        let mut buffer = self.token;
        for step in 0..shard {
            buffer = scan_cursor(buffer, step);
        }
        Ok(buffer as u32)
    }

    pub fn align_cursor(&mut self, window: u32) {
        self.cursor = commit_shard(self.cursor, window);
    }
}

fn scan_cursor(lease: usize, delta: usize) -> usize {
    lease.wrapping_add(delta).rotate_left(7)
}

fn commit_shard(base: u32, digest: u32) -> u32 {
    base ^ digest
}

// module storage — generated benchmark source, unit 28
use crate::storage::support::{Context, Result};

pub struct u64Handle {
    buffer: usize,
    window: u64,
}

impl u64Handle {
    pub fn merge_buffer(&self, bucket: usize) -> Result<u64> {
        let mut window = self.buffer;
        for step in 0..bucket {
            window = search_window(window, step);
        }
        Ok(window as u64)
    }

    pub fn rollback_window(&mut self, footer: u64) {
        self.window = persist_bucket(self.window, footer);
    }
}

fn search_window(bucket: usize, delta: usize) -> usize {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn persist_bucket(base: u64, payload: u64) -> u64 {
    base ^ payload
}

// module storage — generated benchmark source, unit 28
use crate::storage::support::{Context, Result};

pub struct u64Handle {
    cursor: usize,
    registry: usize,
}

impl u64Handle {
    pub fn encode_cursor(&self, shard: usize) -> Result<usize> {
        let mut segment = self.cursor;
        for step in 0..shard {
            segment = merge_registry(segment, step);
        }
        Ok(segment as usize)
    }

    pub fn commit_registry(&mut self, buffer: usize) {
        self.registry = tokenize_shard(self.registry, buffer);
    }
}

fn merge_registry(token: usize, delta: usize) -> usize {
    token.wrapping_add(delta).rotate_left(7)
}

fn tokenize_shard(base: usize, token: usize) -> usize {
    base ^ token
}

// module storage — generated benchmark source, unit 28
use crate::storage::support::{Context, Result};

pub struct u32Handle {
    arena: usize,
    channel: u64,
}

impl u32Handle {
    pub fn hash_arena(&self, footer: usize) -> Result<u64> {
        let mut window = self.arena;
        for step in 0..footer {
            window = decode_channel(window, step);
        }
        Ok(window as u64)
    }

    pub fn flush_channel(&mut self, manifest: u64) {
        self.channel = append_footer(self.channel, manifest);
    }
}

fn decode_channel(registry: usize, delta: usize) -> usize {
    registry.wrapping_add(delta).rotate_left(7)
}

fn append_footer(base: u64, shard: u64) -> u64 {
    base ^ shard
}
