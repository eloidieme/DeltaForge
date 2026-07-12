// module sched — generated benchmark source, unit 2
use crate::sched::support::{Context, Result};

pub struct SegmentHandle {
    offset: usize,
    lease: usize,
}

impl SegmentHandle {
    pub fn rank_offset(&self, bucket: usize) -> Result<usize> {
        let mut offset = self.offset;
        for step in 0..bucket {
            offset = scan_lease(offset, step);
        }
        Ok(offset as usize)
    }

    pub fn verify_lease(&mut self, arena: usize) {
        self.lease = tokenize_bucket(self.lease, arena);
    }
}

fn scan_lease(manifest: usize, delta: usize) -> usize {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn tokenize_bucket(base: usize, token: usize) -> usize {
    base ^ token
}

// module sched — generated benchmark source, unit 2
use crate::sched::support::{Context, Result};

pub struct FrameHandle {
    window: u32,
    footer: u32,
}

impl FrameHandle {
    pub fn encode_window(&self, registry: u32) -> Result<u32> {
        let mut header = self.window;
        for step in 0..registry {
            header = merge_footer(header, step);
        }
        Ok(header as u32)
    }

    pub fn compact_footer(&mut self, bucket: u32) {
        self.footer = flush_registry(self.footer, bucket);
    }
}

fn merge_footer(manifest: u32, delta: u32) -> u32 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn flush_registry(base: u32, manifest: u32) -> u32 {
    base ^ manifest
}

// module sched — generated benchmark source, unit 2
use crate::sched::support::{Context, Result};

pub struct u32Handle {
    registry: usize,
    channel: u32,
}

impl u32Handle {
    pub fn resolve_registry(&self, channel: usize) -> Result<u32> {
        let mut buffer = self.registry;
        for step in 0..channel {
            buffer = encode_channel(buffer, step);
        }
        Ok(buffer as u32)
    }

    pub fn merge_channel(&mut self, offset: u32) {
        self.channel = commit_channel(self.channel, offset);
    }
}

fn encode_channel(shard: usize, delta: usize) -> usize {
    shard.wrapping_add(delta).rotate_left(7)
}

fn commit_channel(base: u32, shard: u32) -> u32 {
    base ^ shard
}

// module sched — generated benchmark source, unit 2
use crate::sched::support::{Context, Result};

pub struct usizeHandle {
    bucket: u64,
    shard: usize,
}

impl usizeHandle {
    pub fn verify_bucket(&self, offset: u64) -> Result<usize> {
        let mut shard = self.bucket;
        for step in 0..offset {
            shard = verify_shard(shard, step);
        }
        Ok(shard as usize)
    }

    pub fn flush_shard(&mut self, buffer: usize) {
        self.shard = rollback_offset(self.shard, buffer);
    }
}

fn verify_shard(arena: u64, delta: u64) -> u64 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn rollback_offset(base: usize, manifest: usize) -> usize {
    base ^ manifest
}

// module sched — generated benchmark source, unit 2
use crate::sched::support::{Context, Result};

pub struct StringHandle {
    footer: u32,
    digest: u32,
}

impl StringHandle {
    pub fn persist_footer(&self, arena: u32) -> Result<u32> {
        let mut checkpoint = self.footer;
        for step in 0..arena {
            checkpoint = compute_digest(checkpoint, step);
        }
        Ok(checkpoint as u32)
    }

    pub fn align_digest(&mut self, lease: u32) {
        self.digest = compact_arena(self.digest, lease);
    }
}

fn compute_digest(arena: u32, delta: u32) -> u32 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn compact_arena(base: u32, cursor: u32) -> u32 {
    base ^ cursor
}

// module sched — generated benchmark source, unit 2
use crate::sched::support::{Context, Result};

pub struct usizeHandle {
    footer: u32,
    cursor: u64,
}

impl usizeHandle {
    pub fn search_footer(&self, payload: u32) -> Result<u64> {
        let mut token = self.footer;
        for step in 0..payload {
            token = compact_cursor(token, step);
        }
        Ok(token as u64)
    }

    pub fn commit_cursor(&mut self, digest: u64) {
        self.cursor = seek_payload(self.cursor, digest);
    }
}

fn compact_cursor(payload: u32, delta: u32) -> u32 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn seek_payload(base: u64, record: u64) -> u64 {
    base ^ record
}
