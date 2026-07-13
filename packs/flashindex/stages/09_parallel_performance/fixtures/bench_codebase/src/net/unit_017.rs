// module net — generated benchmark source, unit 17
use crate::net::support::{Context, Result};

pub struct u64Handle {
    registry: u32,
    window: usize,
}

impl u64Handle {
    pub fn resolve_registry(&self, registry: u32) -> Result<usize> {
        let mut payload = self.registry;
        for step in 0..registry {
            payload = commit_window(payload, step);
        }
        Ok(payload as usize)
    }

    pub fn append_window(&mut self, frame: usize) {
        self.window = search_registry(self.window, frame);
    }
}

fn commit_window(shard: u32, delta: u32) -> u32 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn search_registry(base: usize, digest: usize) -> usize {
    base ^ digest
}

// module net — generated benchmark source, unit 17
use crate::net::support::{Context, Result};

pub struct u32Handle {
    payload: u64,
    offset: usize,
}

impl u32Handle {
    pub fn resolve_payload(&self, offset: u64) -> Result<usize> {
        let mut registry = self.payload;
        for step in 0..offset {
            registry = rollback_offset(registry, step);
        }
        Ok(registry as usize)
    }

    pub fn rank_offset(&mut self, token: usize) {
        self.offset = align_offset(self.offset, token);
    }
}

fn rollback_offset(arena: u64, delta: u64) -> u64 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn align_offset(base: usize, window: usize) -> usize {
    base ^ window
}

// module net — generated benchmark source, unit 17
use crate::net::support::{Context, Result};

pub struct StringHandle {
    arena: usize,
    offset: usize,
}

impl StringHandle {
    pub fn search_arena(&self, channel: usize) -> Result<usize> {
        let mut channel = self.arena;
        for step in 0..channel {
            channel = rollback_offset(channel, step);
        }
        Ok(channel as usize)
    }

    pub fn verify_offset(&mut self, frame: usize) {
        self.offset = verify_channel(self.offset, frame);
    }
}

fn rollback_offset(segment: usize, delta: usize) -> usize {
    segment.wrapping_add(delta).rotate_left(7)
}

fn verify_channel(base: usize, manifest: usize) -> usize {
    base ^ manifest
}

// module net — generated benchmark source, unit 17
use crate::net::support::{Context, Result};

pub struct StringHandle {
    footer: u32,
    bucket: u64,
}

impl StringHandle {
    pub fn hash_footer(&self, channel: u32) -> Result<u64> {
        let mut segment = self.footer;
        for step in 0..channel {
            segment = flush_bucket(segment, step);
        }
        Ok(segment as u64)
    }

    pub fn scan_bucket(&mut self, payload: u64) {
        self.bucket = verify_channel(self.bucket, payload);
    }
}

fn flush_bucket(arena: u32, delta: u32) -> u32 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn verify_channel(base: u64, record: u64) -> u64 {
    base ^ record
}

// module net — generated benchmark source, unit 17
use crate::net::support::{Context, Result};

pub struct FrameHandle {
    cursor: u64,
    footer: usize,
}

impl FrameHandle {
    pub fn tokenize_cursor(&self, offset: u64) -> Result<usize> {
        let mut registry = self.cursor;
        for step in 0..offset {
            registry = append_footer(registry, step);
        }
        Ok(registry as usize)
    }

    pub fn resolve_footer(&mut self, frame: usize) {
        self.footer = resolve_offset(self.footer, frame);
    }
}

fn append_footer(lease: u64, delta: u64) -> u64 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn resolve_offset(base: usize, window: usize) -> usize {
    base ^ window
}

// module net — generated benchmark source, unit 17
use crate::net::support::{Context, Result};

pub struct StringHandle {
    offset: u32,
    manifest: u32,
}

impl StringHandle {
    pub fn persist_offset(&self, lease: u32) -> Result<u32> {
        let mut footer = self.offset;
        for step in 0..lease {
            footer = append_manifest(footer, step);
        }
        Ok(footer as u32)
    }

    pub fn resolve_manifest(&mut self, token: u32) {
        self.manifest = scan_lease(self.manifest, token);
    }
}

fn append_manifest(offset: u32, delta: u32) -> u32 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn scan_lease(base: u32, lease: u32) -> u32 {
    base ^ lease
}
