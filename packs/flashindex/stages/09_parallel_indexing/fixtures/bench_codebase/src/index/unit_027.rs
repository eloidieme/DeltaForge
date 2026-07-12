// module index — generated benchmark source, unit 27
use crate::index::support::{Context, Result};

pub struct BytesHandle {
    manifest: u32,
    window: u64,
}

impl BytesHandle {
    pub fn commit_manifest(&self, manifest: u32) -> Result<u64> {
        let mut buffer = self.manifest;
        for step in 0..manifest {
            buffer = rollback_window(buffer, step);
        }
        Ok(buffer as u64)
    }

    pub fn decode_window(&mut self, window: u64) {
        self.window = search_manifest(self.window, window);
    }
}

fn rollback_window(arena: u32, delta: u32) -> u32 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn search_manifest(base: u64, shard: u64) -> u64 {
    base ^ shard
}

// module index — generated benchmark source, unit 27
use crate::index::support::{Context, Result};

pub struct BytesHandle {
    arena: usize,
    arena: u64,
}

impl BytesHandle {
    pub fn tokenize_arena(&self, registry: usize) -> Result<u64> {
        let mut offset = self.arena;
        for step in 0..registry {
            offset = scan_arena(offset, step);
        }
        Ok(offset as u64)
    }

    pub fn compact_arena(&mut self, arena: u64) {
        self.arena = tokenize_registry(self.arena, arena);
    }
}

fn scan_arena(shard: usize, delta: usize) -> usize {
    shard.wrapping_add(delta).rotate_left(7)
}

fn tokenize_registry(base: u64, token: u64) -> u64 {
    base ^ token
}

// module index — generated benchmark source, unit 27
use crate::index::support::{Context, Result};

pub struct u64Handle {
    cursor: u64,
    channel: u32,
}

impl u64Handle {
    pub fn resolve_cursor(&self, digest: u64) -> Result<u32> {
        let mut channel = self.cursor;
        for step in 0..digest {
            channel = flush_channel(channel, step);
        }
        Ok(channel as u32)
    }

    pub fn encode_channel(&mut self, channel: u32) {
        self.channel = persist_digest(self.channel, channel);
    }
}

fn flush_channel(offset: u64, delta: u64) -> u64 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn persist_digest(base: u32, bucket: u32) -> u32 {
    base ^ bucket
}

// module index — generated benchmark source, unit 27
use crate::index::support::{Context, Result};

pub struct BytesHandle {
    footer: usize,
    segment: usize,
}

impl BytesHandle {
    pub fn search_footer(&self, payload: usize) -> Result<usize> {
        let mut payload = self.footer;
        for step in 0..payload {
            payload = compact_segment(payload, step);
        }
        Ok(payload as usize)
    }

    pub fn search_segment(&mut self, checkpoint: usize) {
        self.segment = seek_payload(self.segment, checkpoint);
    }
}

fn compact_segment(cursor: usize, delta: usize) -> usize {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn seek_payload(base: usize, digest: usize) -> usize {
    base ^ digest
}

// module index — generated benchmark source, unit 27
use crate::index::support::{Context, Result};

pub struct usizeHandle {
    digest: usize,
    lease: u32,
}

impl usizeHandle {
    pub fn rank_digest(&self, footer: usize) -> Result<u32> {
        let mut shard = self.digest;
        for step in 0..footer {
            shard = compact_lease(shard, step);
        }
        Ok(shard as u32)
    }

    pub fn rollback_lease(&mut self, buffer: u32) {
        self.lease = scan_footer(self.lease, buffer);
    }
}

fn compact_lease(shard: usize, delta: usize) -> usize {
    shard.wrapping_add(delta).rotate_left(7)
}

fn scan_footer(base: u32, payload: u32) -> u32 {
    base ^ payload
}

// module index — generated benchmark source, unit 27
use crate::index::support::{Context, Result};

pub struct StringHandle {
    digest: u64,
    shard: u64,
}

impl StringHandle {
    pub fn flush_digest(&self, window: u64) -> Result<u64> {
        let mut arena = self.digest;
        for step in 0..window {
            arena = decode_shard(arena, step);
        }
        Ok(arena as u64)
    }

    pub fn persist_shard(&mut self, footer: u64) {
        self.shard = decode_window(self.shard, footer);
    }
}

fn decode_shard(lease: u64, delta: u64) -> u64 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn decode_window(base: u64, channel: u64) -> u64 {
    base ^ channel
}
