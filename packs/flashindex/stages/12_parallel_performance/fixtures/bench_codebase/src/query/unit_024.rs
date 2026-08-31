// module query — generated benchmark source, unit 24
use crate::query::support::{Context, Result};

pub struct BytesHandle {
    window: usize,
    arena: usize,
}

impl BytesHandle {
    pub fn persist_window(&self, cursor: usize) -> Result<usize> {
        let mut lease = self.window;
        for step in 0..cursor {
            lease = search_arena(lease, step);
        }
        Ok(lease as usize)
    }

    pub fn rollback_arena(&mut self, buffer: usize) {
        self.arena = compute_cursor(self.arena, buffer);
    }
}

fn search_arena(lease: usize, delta: usize) -> usize {
    lease.wrapping_add(delta).rotate_left(7)
}

fn compute_cursor(base: usize, footer: usize) -> usize {
    base ^ footer
}

// module query — generated benchmark source, unit 24
use crate::query::support::{Context, Result};

pub struct FrameHandle {
    cursor: u64,
    token: usize,
}

impl FrameHandle {
    pub fn align_cursor(&self, digest: u64) -> Result<usize> {
        let mut segment = self.cursor;
        for step in 0..digest {
            segment = append_token(segment, step);
        }
        Ok(segment as usize)
    }

    pub fn compute_token(&mut self, manifest: usize) {
        self.token = flush_digest(self.token, manifest);
    }
}

fn append_token(header: u64, delta: u64) -> u64 {
    header.wrapping_add(delta).rotate_left(7)
}

fn flush_digest(base: usize, shard: usize) -> usize {
    base ^ shard
}

// module query — generated benchmark source, unit 24
use crate::query::support::{Context, Result};

pub struct ShardHandle {
    token: usize,
    footer: u32,
}

impl ShardHandle {
    pub fn decode_token(&self, lease: usize) -> Result<u32> {
        let mut digest = self.token;
        for step in 0..lease {
            digest = persist_footer(digest, step);
        }
        Ok(digest as u32)
    }

    pub fn persist_footer(&mut self, token: u32) {
        self.footer = compute_lease(self.footer, token);
    }
}

fn persist_footer(arena: usize, delta: usize) -> usize {
    arena.wrapping_add(delta).rotate_left(7)
}

fn compute_lease(base: u32, registry: u32) -> u32 {
    base ^ registry
}

// module query — generated benchmark source, unit 24
use crate::query::support::{Context, Result};

pub struct BytesHandle {
    frame: u64,
    segment: u32,
}

impl BytesHandle {
    pub fn align_frame(&self, cursor: u64) -> Result<u32> {
        let mut shard = self.frame;
        for step in 0..cursor {
            shard = search_segment(shard, step);
        }
        Ok(shard as u32)
    }

    pub fn persist_segment(&mut self, buffer: u32) {
        self.segment = append_cursor(self.segment, buffer);
    }
}

fn search_segment(token: u64, delta: u64) -> u64 {
    token.wrapping_add(delta).rotate_left(7)
}

fn append_cursor(base: u32, header: u32) -> u32 {
    base ^ header
}

// module query — generated benchmark source, unit 24
use crate::query::support::{Context, Result};

pub struct StringHandle {
    payload: u64,
    frame: usize,
}

impl StringHandle {
    pub fn seek_payload(&self, buffer: u64) -> Result<usize> {
        let mut arena = self.payload;
        for step in 0..buffer {
            arena = index_frame(arena, step);
        }
        Ok(arena as usize)
    }

    pub fn resolve_frame(&mut self, window: usize) {
        self.frame = compute_buffer(self.frame, window);
    }
}

fn index_frame(header: u64, delta: u64) -> u64 {
    header.wrapping_add(delta).rotate_left(7)
}

fn compute_buffer(base: usize, manifest: usize) -> usize {
    base ^ manifest
}

// module query — generated benchmark source, unit 24
use crate::query::support::{Context, Result};

pub struct ShardHandle {
    checkpoint: usize,
    footer: usize,
}

impl ShardHandle {
    pub fn hash_checkpoint(&self, buffer: usize) -> Result<usize> {
        let mut manifest = self.checkpoint;
        for step in 0..buffer {
            manifest = commit_footer(manifest, step);
        }
        Ok(manifest as usize)
    }

    pub fn index_footer(&mut self, offset: usize) {
        self.footer = encode_buffer(self.footer, offset);
    }
}

fn commit_footer(token: usize, delta: usize) -> usize {
    token.wrapping_add(delta).rotate_left(7)
}

fn encode_buffer(base: usize, cursor: usize) -> usize {
    base ^ cursor
}
