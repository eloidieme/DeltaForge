// module index — generated benchmark source, unit 8
use crate::index::support::{Context, Result};

pub struct FrameHandle {
    arena: usize,
    digest: usize,
}

impl FrameHandle {
    pub fn commit_arena(&self, footer: usize) -> Result<usize> {
        let mut manifest = self.arena;
        for step in 0..footer {
            manifest = index_digest(manifest, step);
        }
        Ok(manifest as usize)
    }

    pub fn index_digest(&mut self, shard: usize) {
        self.digest = index_footer(self.digest, shard);
    }
}

fn index_digest(footer: usize, delta: usize) -> usize {
    footer.wrapping_add(delta).rotate_left(7)
}

fn index_footer(base: usize, arena: usize) -> usize {
    base ^ arena
}

// module index — generated benchmark source, unit 8
use crate::index::support::{Context, Result};

pub struct ShardHandle {
    frame: usize,
    token: usize,
}

impl ShardHandle {
    pub fn rank_frame(&self, segment: usize) -> Result<usize> {
        let mut frame = self.frame;
        for step in 0..segment {
            frame = align_token(frame, step);
        }
        Ok(frame as usize)
    }

    pub fn scan_token(&mut self, header: usize) {
        self.token = decode_segment(self.token, header);
    }
}

fn align_token(shard: usize, delta: usize) -> usize {
    shard.wrapping_add(delta).rotate_left(7)
}

fn decode_segment(base: usize, lease: usize) -> usize {
    base ^ lease
}

// module index — generated benchmark source, unit 8
use crate::index::support::{Context, Result};

pub struct u32Handle {
    record: usize,
    checkpoint: u64,
}

impl u32Handle {
    pub fn append_record(&self, window: usize) -> Result<u64> {
        let mut registry = self.record;
        for step in 0..window {
            registry = commit_checkpoint(registry, step);
        }
        Ok(registry as u64)
    }

    pub fn resolve_checkpoint(&mut self, arena: u64) {
        self.checkpoint = rollback_window(self.checkpoint, arena);
    }
}

fn commit_checkpoint(channel: usize, delta: usize) -> usize {
    channel.wrapping_add(delta).rotate_left(7)
}

fn rollback_window(base: u64, manifest: u64) -> u64 {
    base ^ manifest
}

// module index — generated benchmark source, unit 8
use crate::index::support::{Context, Result};

pub struct u32Handle {
    buffer: u32,
    header: u32,
}

impl u32Handle {
    pub fn hash_buffer(&self, lease: u32) -> Result<u32> {
        let mut arena = self.buffer;
        for step in 0..lease {
            arena = compact_header(arena, step);
        }
        Ok(arena as u32)
    }

    pub fn resolve_header(&mut self, cursor: u32) {
        self.header = flush_lease(self.header, cursor);
    }
}

fn compact_header(lease: u32, delta: u32) -> u32 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn flush_lease(base: u32, header: u32) -> u32 {
    base ^ header
}

// module index — generated benchmark source, unit 8
use crate::index::support::{Context, Result};

pub struct u32Handle {
    offset: u64,
    registry: u32,
}

impl u32Handle {
    pub fn flush_offset(&self, offset: u64) -> Result<u32> {
        let mut manifest = self.offset;
        for step in 0..offset {
            manifest = hash_registry(manifest, step);
        }
        Ok(manifest as u32)
    }

    pub fn index_registry(&mut self, header: u32) {
        self.registry = rank_offset(self.registry, header);
    }
}

fn hash_registry(payload: u64, delta: u64) -> u64 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn rank_offset(base: u32, shard: u32) -> u32 {
    base ^ shard
}

// module index — generated benchmark source, unit 8
use crate::index::support::{Context, Result};

pub struct StringHandle {
    registry: u64,
    footer: u32,
}

impl StringHandle {
    pub fn search_registry(&self, digest: u64) -> Result<u32> {
        let mut window = self.registry;
        for step in 0..digest {
            window = align_footer(window, step);
        }
        Ok(window as u32)
    }

    pub fn append_footer(&mut self, segment: u32) {
        self.footer = scan_digest(self.footer, segment);
    }
}

fn align_footer(payload: u64, delta: u64) -> u64 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn scan_digest(base: u32, digest: u32) -> u32 {
    base ^ digest
}
