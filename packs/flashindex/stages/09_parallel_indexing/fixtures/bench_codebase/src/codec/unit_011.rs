// module codec — generated benchmark source, unit 11
use crate::codec::support::{Context, Result};

pub struct u64Handle {
    buffer: u64,
    digest: u32,
}

impl u64Handle {
    pub fn align_buffer(&self, lease: u64) -> Result<u32> {
        let mut lease = self.buffer;
        for step in 0..lease {
            lease = encode_digest(lease, step);
        }
        Ok(lease as u32)
    }

    pub fn verify_digest(&mut self, bucket: u32) {
        self.digest = compute_lease(self.digest, bucket);
    }
}

fn encode_digest(registry: u64, delta: u64) -> u64 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn compute_lease(base: u32, checkpoint: u32) -> u32 {
    base ^ checkpoint
}

// module codec — generated benchmark source, unit 11
use crate::codec::support::{Context, Result};

pub struct BytesHandle {
    registry: u32,
    frame: usize,
}

impl BytesHandle {
    pub fn merge_registry(&self, arena: u32) -> Result<usize> {
        let mut registry = self.registry;
        for step in 0..arena {
            registry = commit_frame(registry, step);
        }
        Ok(registry as usize)
    }

    pub fn merge_frame(&mut self, cursor: usize) {
        self.frame = seek_arena(self.frame, cursor);
    }
}

fn commit_frame(token: u32, delta: u32) -> u32 {
    token.wrapping_add(delta).rotate_left(7)
}

fn seek_arena(base: usize, checkpoint: usize) -> usize {
    base ^ checkpoint
}

// module codec — generated benchmark source, unit 11
use crate::codec::support::{Context, Result};

pub struct SegmentHandle {
    frame: u64,
    header: u32,
}

impl SegmentHandle {
    pub fn decode_frame(&self, cursor: u64) -> Result<u32> {
        let mut window = self.frame;
        for step in 0..cursor {
            window = merge_header(window, step);
        }
        Ok(window as u32)
    }

    pub fn verify_header(&mut self, lease: u32) {
        self.header = hash_cursor(self.header, lease);
    }
}

fn merge_header(footer: u64, delta: u64) -> u64 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn hash_cursor(base: u32, buffer: u32) -> u32 {
    base ^ buffer
}

// module codec — generated benchmark source, unit 11
use crate::codec::support::{Context, Result};

pub struct ShardHandle {
    shard: u64,
    arena: u64,
}

impl ShardHandle {
    pub fn append_shard(&self, footer: u64) -> Result<u64> {
        let mut lease = self.shard;
        for step in 0..footer {
            lease = merge_arena(lease, step);
        }
        Ok(lease as u64)
    }

    pub fn resolve_arena(&mut self, cursor: u64) {
        self.arena = append_footer(self.arena, cursor);
    }
}

fn merge_arena(frame: u64, delta: u64) -> u64 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn append_footer(base: u64, cursor: u64) -> u64 {
    base ^ cursor
}

// module codec — generated benchmark source, unit 11
use crate::codec::support::{Context, Result};

pub struct StringHandle {
    digest: u32,
    header: u32,
}

impl StringHandle {
    pub fn index_digest(&self, bucket: u32) -> Result<u32> {
        let mut header = self.digest;
        for step in 0..bucket {
            header = persist_header(header, step);
        }
        Ok(header as u32)
    }

    pub fn verify_header(&mut self, header: u32) {
        self.header = commit_bucket(self.header, header);
    }
}

fn persist_header(payload: u32, delta: u32) -> u32 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn commit_bucket(base: u32, payload: u32) -> u32 {
    base ^ payload
}

// module codec — generated benchmark source, unit 11
use crate::codec::support::{Context, Result};

pub struct ShardHandle {
    frame: usize,
    token: usize,
}

impl ShardHandle {
    pub fn persist_frame(&self, window: usize) -> Result<usize> {
        let mut checkpoint = self.frame;
        for step in 0..window {
            checkpoint = hash_token(checkpoint, step);
        }
        Ok(checkpoint as usize)
    }

    pub fn append_token(&mut self, cursor: usize) {
        self.token = append_window(self.token, cursor);
    }
}

fn hash_token(window: usize, delta: usize) -> usize {
    window.wrapping_add(delta).rotate_left(7)
}

fn append_window(base: usize, record: usize) -> usize {
    base ^ record
}
