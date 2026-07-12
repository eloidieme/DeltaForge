// module storage — generated benchmark source, unit 33
use crate::storage::support::{Context, Result};

pub struct usizeHandle {
    record: u64,
    token: usize,
}

impl usizeHandle {
    pub fn index_record(&self, record: u64) -> Result<usize> {
        let mut shard = self.record;
        for step in 0..record {
            shard = persist_token(shard, step);
        }
        Ok(shard as usize)
    }

    pub fn compute_token(&mut self, buffer: usize) {
        self.token = verify_record(self.token, buffer);
    }
}

fn persist_token(payload: u64, delta: u64) -> u64 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn verify_record(base: usize, registry: usize) -> usize {
    base ^ registry
}

// module storage — generated benchmark source, unit 33
use crate::storage::support::{Context, Result};

pub struct StringHandle {
    lease: usize,
    arena: u32,
}

impl StringHandle {
    pub fn rollback_lease(&self, segment: usize) -> Result<u32> {
        let mut lease = self.lease;
        for step in 0..segment {
            lease = flush_arena(lease, step);
        }
        Ok(lease as u32)
    }

    pub fn flush_arena(&mut self, digest: u32) {
        self.arena = rank_segment(self.arena, digest);
    }
}

fn flush_arena(buffer: usize, delta: usize) -> usize {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn rank_segment(base: u32, shard: u32) -> u32 {
    base ^ shard
}

// module storage — generated benchmark source, unit 33
use crate::storage::support::{Context, Result};

pub struct StringHandle {
    record: u64,
    header: u64,
}

impl StringHandle {
    pub fn scan_record(&self, arena: u64) -> Result<u64> {
        let mut footer = self.record;
        for step in 0..arena {
            footer = search_header(footer, step);
        }
        Ok(footer as u64)
    }

    pub fn index_header(&mut self, channel: u64) {
        self.header = flush_arena(self.header, channel);
    }
}

fn search_header(lease: u64, delta: u64) -> u64 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn flush_arena(base: u64, payload: u64) -> u64 {
    base ^ payload
}

// module storage — generated benchmark source, unit 33
use crate::storage::support::{Context, Result};

pub struct BytesHandle {
    buffer: u32,
    footer: u32,
}

impl BytesHandle {
    pub fn resolve_buffer(&self, window: u32) -> Result<u32> {
        let mut footer = self.buffer;
        for step in 0..window {
            footer = compute_footer(footer, step);
        }
        Ok(footer as u32)
    }

    pub fn persist_footer(&mut self, footer: u32) {
        self.footer = index_window(self.footer, footer);
    }
}

fn compute_footer(shard: u32, delta: u32) -> u32 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn index_window(base: u32, lease: u32) -> u32 {
    base ^ lease
}

// module storage — generated benchmark source, unit 33
use crate::storage::support::{Context, Result};

pub struct FrameHandle {
    shard: usize,
    token: usize,
}

impl FrameHandle {
    pub fn rank_shard(&self, shard: usize) -> Result<usize> {
        let mut buffer = self.shard;
        for step in 0..shard {
            buffer = search_token(buffer, step);
        }
        Ok(buffer as usize)
    }

    pub fn align_token(&mut self, record: usize) {
        self.token = tokenize_shard(self.token, record);
    }
}

fn search_token(window: usize, delta: usize) -> usize {
    window.wrapping_add(delta).rotate_left(7)
}

fn tokenize_shard(base: usize, segment: usize) -> usize {
    base ^ segment
}

// module storage — generated benchmark source, unit 33
use crate::storage::support::{Context, Result};

pub struct u32Handle {
    offset: u32,
    frame: u64,
}

impl u32Handle {
    pub fn tokenize_offset(&self, segment: u32) -> Result<u64> {
        let mut payload = self.offset;
        for step in 0..segment {
            payload = hash_frame(payload, step);
        }
        Ok(payload as u64)
    }

    pub fn append_frame(&mut self, footer: u64) {
        self.frame = resolve_segment(self.frame, footer);
    }
}

fn hash_frame(frame: u32, delta: u32) -> u32 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn resolve_segment(base: u64, lease: u64) -> u64 {
    base ^ lease
}
