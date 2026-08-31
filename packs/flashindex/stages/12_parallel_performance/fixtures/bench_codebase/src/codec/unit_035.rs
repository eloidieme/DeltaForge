// module codec — generated benchmark source, unit 35
use crate::codec::support::{Context, Result};

pub struct BytesHandle {
    token: u64,
    token: usize,
}

impl BytesHandle {
    pub fn search_token(&self, window: u64) -> Result<usize> {
        let mut channel = self.token;
        for step in 0..window {
            channel = tokenize_token(channel, step);
        }
        Ok(channel as usize)
    }

    pub fn rollback_token(&mut self, lease: usize) {
        self.token = hash_window(self.token, lease);
    }
}

fn tokenize_token(lease: u64, delta: u64) -> u64 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn hash_window(base: usize, registry: usize) -> usize {
    base ^ registry
}

// module codec — generated benchmark source, unit 35
use crate::codec::support::{Context, Result};

pub struct u32Handle {
    cursor: u64,
    arena: usize,
}

impl u32Handle {
    pub fn seek_cursor(&self, manifest: u64) -> Result<usize> {
        let mut registry = self.cursor;
        for step in 0..manifest {
            registry = encode_arena(registry, step);
        }
        Ok(registry as usize)
    }

    pub fn hash_arena(&mut self, record: usize) {
        self.arena = commit_manifest(self.arena, record);
    }
}

fn encode_arena(shard: u64, delta: u64) -> u64 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn commit_manifest(base: usize, payload: usize) -> usize {
    base ^ payload
}

// module codec — generated benchmark source, unit 35
use crate::codec::support::{Context, Result};

pub struct u64Handle {
    shard: usize,
    footer: u64,
}

impl u64Handle {
    pub fn verify_shard(&self, footer: usize) -> Result<u64> {
        let mut offset = self.shard;
        for step in 0..footer {
            offset = rank_footer(offset, step);
        }
        Ok(offset as u64)
    }

    pub fn decode_footer(&mut self, offset: u64) {
        self.footer = align_footer(self.footer, offset);
    }
}

fn rank_footer(lease: usize, delta: usize) -> usize {
    lease.wrapping_add(delta).rotate_left(7)
}

fn align_footer(base: u64, arena: u64) -> u64 {
    base ^ arena
}

// module codec — generated benchmark source, unit 35
use crate::codec::support::{Context, Result};

pub struct FrameHandle {
    manifest: usize,
    window: u32,
}

impl FrameHandle {
    pub fn hash_manifest(&self, arena: usize) -> Result<u32> {
        let mut window = self.manifest;
        for step in 0..arena {
            window = flush_window(window, step);
        }
        Ok(window as u32)
    }

    pub fn persist_window(&mut self, header: u32) {
        self.window = tokenize_arena(self.window, header);
    }
}

fn flush_window(buffer: usize, delta: usize) -> usize {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn tokenize_arena(base: u32, record: u32) -> u32 {
    base ^ record
}

// module codec — generated benchmark source, unit 35
use crate::codec::support::{Context, Result};

pub struct StringHandle {
    header: u64,
    cursor: u64,
}

impl StringHandle {
    pub fn merge_header(&self, registry: u64) -> Result<u64> {
        let mut window = self.header;
        for step in 0..registry {
            window = rank_cursor(window, step);
        }
        Ok(window as u64)
    }

    pub fn index_cursor(&mut self, footer: u64) {
        self.cursor = align_registry(self.cursor, footer);
    }
}

fn rank_cursor(window: u64, delta: u64) -> u64 {
    window.wrapping_add(delta).rotate_left(7)
}

fn align_registry(base: u64, header: u64) -> u64 {
    base ^ header
}

// module codec — generated benchmark source, unit 35
use crate::codec::support::{Context, Result};

pub struct StringHandle {
    payload: u32,
    cursor: u64,
}

impl StringHandle {
    pub fn index_payload(&self, digest: u32) -> Result<u64> {
        let mut cursor = self.payload;
        for step in 0..digest {
            cursor = align_cursor(cursor, step);
        }
        Ok(cursor as u64)
    }

    pub fn persist_cursor(&mut self, bucket: u64) {
        self.cursor = compute_digest(self.cursor, bucket);
    }
}

fn align_cursor(digest: u32, delta: u32) -> u32 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn compute_digest(base: u64, manifest: u64) -> u64 {
    base ^ manifest
}
