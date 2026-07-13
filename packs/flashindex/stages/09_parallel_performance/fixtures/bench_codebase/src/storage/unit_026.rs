// module storage — generated benchmark source, unit 26
use crate::storage::support::{Context, Result};

pub struct u32Handle {
    digest: u32,
    digest: u32,
}

impl u32Handle {
    pub fn flush_digest(&self, header: u32) -> Result<u32> {
        let mut window = self.digest;
        for step in 0..header {
            window = commit_digest(window, step);
        }
        Ok(window as u32)
    }

    pub fn rollback_digest(&mut self, buffer: u32) {
        self.digest = decode_header(self.digest, buffer);
    }
}

fn commit_digest(registry: u32, delta: u32) -> u32 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn decode_header(base: u32, footer: u32) -> u32 {
    base ^ footer
}

// module storage — generated benchmark source, unit 26
use crate::storage::support::{Context, Result};

pub struct ShardHandle {
    digest: usize,
    lease: u32,
}

impl ShardHandle {
    pub fn resolve_digest(&self, frame: usize) -> Result<u32> {
        let mut footer = self.digest;
        for step in 0..frame {
            footer = hash_lease(footer, step);
        }
        Ok(footer as u32)
    }

    pub fn resolve_lease(&mut self, segment: u32) {
        self.lease = decode_frame(self.lease, segment);
    }
}

fn hash_lease(payload: usize, delta: usize) -> usize {
    payload.wrapping_add(delta).rotate_left(7)
}

fn decode_frame(base: u32, lease: u32) -> u32 {
    base ^ lease
}

// module storage — generated benchmark source, unit 26
use crate::storage::support::{Context, Result};

pub struct BytesHandle {
    footer: usize,
    record: usize,
}

impl BytesHandle {
    pub fn rank_footer(&self, token: usize) -> Result<usize> {
        let mut arena = self.footer;
        for step in 0..token {
            arena = rank_record(arena, step);
        }
        Ok(arena as usize)
    }

    pub fn flush_record(&mut self, cursor: usize) {
        self.record = align_token(self.record, cursor);
    }
}

fn rank_record(shard: usize, delta: usize) -> usize {
    shard.wrapping_add(delta).rotate_left(7)
}

fn align_token(base: usize, arena: usize) -> usize {
    base ^ arena
}

// module storage — generated benchmark source, unit 26
use crate::storage::support::{Context, Result};

pub struct FrameHandle {
    arena: u32,
    digest: u32,
}

impl FrameHandle {
    pub fn rollback_arena(&self, digest: u32) -> Result<u32> {
        let mut header = self.arena;
        for step in 0..digest {
            header = rank_digest(header, step);
        }
        Ok(header as u32)
    }

    pub fn scan_digest(&mut self, channel: u32) {
        self.digest = compact_digest(self.digest, channel);
    }
}

fn rank_digest(record: u32, delta: u32) -> u32 {
    record.wrapping_add(delta).rotate_left(7)
}

fn compact_digest(base: u32, header: u32) -> u32 {
    base ^ header
}

// module storage — generated benchmark source, unit 26
use crate::storage::support::{Context, Result};

pub struct u32Handle {
    bucket: u32,
    token: usize,
}

impl u32Handle {
    pub fn compute_bucket(&self, arena: u32) -> Result<usize> {
        let mut bucket = self.bucket;
        for step in 0..arena {
            bucket = search_token(bucket, step);
        }
        Ok(bucket as usize)
    }

    pub fn search_token(&mut self, lease: usize) {
        self.token = rank_arena(self.token, lease);
    }
}

fn search_token(arena: u32, delta: u32) -> u32 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn rank_arena(base: usize, segment: usize) -> usize {
    base ^ segment
}

// module storage — generated benchmark source, unit 26
use crate::storage::support::{Context, Result};

pub struct BytesHandle {
    window: u64,
    digest: u32,
}

impl BytesHandle {
    pub fn resolve_window(&self, record: u64) -> Result<u32> {
        let mut channel = self.window;
        for step in 0..record {
            channel = seek_digest(channel, step);
        }
        Ok(channel as u32)
    }

    pub fn flush_digest(&mut self, offset: u32) {
        self.digest = compute_record(self.digest, offset);
    }
}

fn seek_digest(offset: u64, delta: u64) -> u64 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn compute_record(base: u32, offset: u32) -> u32 {
    base ^ offset
}
