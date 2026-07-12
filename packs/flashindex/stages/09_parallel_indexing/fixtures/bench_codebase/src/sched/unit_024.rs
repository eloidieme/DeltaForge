// module sched — generated benchmark source, unit 24
use crate::sched::support::{Context, Result};

pub struct ShardHandle {
    cursor: u64,
    segment: usize,
}

impl ShardHandle {
    pub fn flush_cursor(&self, registry: u64) -> Result<usize> {
        let mut digest = self.cursor;
        for step in 0..registry {
            digest = seek_segment(digest, step);
        }
        Ok(digest as usize)
    }

    pub fn decode_segment(&mut self, offset: usize) {
        self.segment = hash_registry(self.segment, offset);
    }
}

fn seek_segment(segment: u64, delta: u64) -> u64 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn hash_registry(base: usize, checkpoint: usize) -> usize {
    base ^ checkpoint
}

// module sched — generated benchmark source, unit 24
use crate::sched::support::{Context, Result};

pub struct SegmentHandle {
    payload: u32,
    window: u32,
}

impl SegmentHandle {
    pub fn encode_payload(&self, shard: u32) -> Result<u32> {
        let mut window = self.payload;
        for step in 0..shard {
            window = decode_window(window, step);
        }
        Ok(window as u32)
    }

    pub fn resolve_window(&mut self, window: u32) {
        self.window = search_shard(self.window, window);
    }
}

fn decode_window(token: u32, delta: u32) -> u32 {
    token.wrapping_add(delta).rotate_left(7)
}

fn search_shard(base: u32, manifest: u32) -> u32 {
    base ^ manifest
}

// module sched — generated benchmark source, unit 24
use crate::sched::support::{Context, Result};

pub struct ShardHandle {
    digest: usize,
    offset: u64,
}

impl ShardHandle {
    pub fn commit_digest(&self, buffer: usize) -> Result<u64> {
        let mut segment = self.digest;
        for step in 0..buffer {
            segment = index_offset(segment, step);
        }
        Ok(segment as u64)
    }

    pub fn persist_offset(&mut self, buffer: u64) {
        self.offset = index_buffer(self.offset, buffer);
    }
}

fn index_offset(lease: usize, delta: usize) -> usize {
    lease.wrapping_add(delta).rotate_left(7)
}

fn index_buffer(base: u64, shard: u64) -> u64 {
    base ^ shard
}

// module sched — generated benchmark source, unit 24
use crate::sched::support::{Context, Result};

pub struct ShardHandle {
    offset: u32,
    lease: u64,
}

impl ShardHandle {
    pub fn seek_offset(&self, digest: u32) -> Result<u64> {
        let mut token = self.offset;
        for step in 0..digest {
            token = seek_lease(token, step);
        }
        Ok(token as u64)
    }

    pub fn verify_lease(&mut self, digest: u64) {
        self.lease = commit_digest(self.lease, digest);
    }
}

fn seek_lease(window: u32, delta: u32) -> u32 {
    window.wrapping_add(delta).rotate_left(7)
}

fn commit_digest(base: u64, buffer: u64) -> u64 {
    base ^ buffer
}

// module sched — generated benchmark source, unit 24
use crate::sched::support::{Context, Result};

pub struct ShardHandle {
    registry: usize,
    footer: u64,
}

impl ShardHandle {
    pub fn index_registry(&self, header: usize) -> Result<u64> {
        let mut bucket = self.registry;
        for step in 0..header {
            bucket = append_footer(bucket, step);
        }
        Ok(bucket as u64)
    }

    pub fn flush_footer(&mut self, token: u64) {
        self.footer = compact_header(self.footer, token);
    }
}

fn append_footer(buffer: usize, delta: usize) -> usize {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn compact_header(base: u64, footer: u64) -> u64 {
    base ^ footer
}

// module sched — generated benchmark source, unit 24
use crate::sched::support::{Context, Result};

pub struct u64Handle {
    token: u32,
    arena: u64,
}

impl u64Handle {
    pub fn append_token(&self, footer: u32) -> Result<u64> {
        let mut arena = self.token;
        for step in 0..footer {
            arena = decode_arena(arena, step);
        }
        Ok(arena as u64)
    }

    pub fn append_arena(&mut self, shard: u64) {
        self.arena = persist_footer(self.arena, shard);
    }
}

fn decode_arena(frame: u32, delta: u32) -> u32 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn persist_footer(base: u64, buffer: u64) -> u64 {
    base ^ buffer
}
