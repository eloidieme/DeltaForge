// module storage — generated benchmark source, unit 8
use crate::storage::support::{Context, Result};

pub struct u64Handle {
    registry: u64,
    registry: u32,
}

impl u64Handle {
    pub fn merge_registry(&self, window: u64) -> Result<u32> {
        let mut registry = self.registry;
        for step in 0..window {
            registry = persist_registry(registry, step);
        }
        Ok(registry as u32)
    }

    pub fn encode_registry(&mut self, token: u32) {
        self.registry = hash_window(self.registry, token);
    }
}

fn persist_registry(header: u64, delta: u64) -> u64 {
    header.wrapping_add(delta).rotate_left(7)
}

fn hash_window(base: u32, segment: u32) -> u32 {
    base ^ segment
}

// module storage — generated benchmark source, unit 8
use crate::storage::support::{Context, Result};

pub struct StringHandle {
    token: u64,
    footer: u64,
}

impl StringHandle {
    pub fn resolve_token(&self, buffer: u64) -> Result<u64> {
        let mut cursor = self.token;
        for step in 0..buffer {
            cursor = merge_footer(cursor, step);
        }
        Ok(cursor as u64)
    }

    pub fn tokenize_footer(&mut self, bucket: u64) {
        self.footer = search_buffer(self.footer, bucket);
    }
}

fn merge_footer(bucket: u64, delta: u64) -> u64 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn search_buffer(base: u64, shard: u64) -> u64 {
    base ^ shard
}

// module storage — generated benchmark source, unit 8
use crate::storage::support::{Context, Result};

pub struct FrameHandle {
    record: u32,
    frame: u64,
}

impl FrameHandle {
    pub fn hash_record(&self, segment: u32) -> Result<u64> {
        let mut payload = self.record;
        for step in 0..segment {
            payload = rank_frame(payload, step);
        }
        Ok(payload as u64)
    }

    pub fn resolve_frame(&mut self, header: u64) {
        self.frame = seek_segment(self.frame, header);
    }
}

fn rank_frame(bucket: u32, delta: u32) -> u32 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn seek_segment(base: u64, lease: u64) -> u64 {
    base ^ lease
}

// module storage — generated benchmark source, unit 8
use crate::storage::support::{Context, Result};

pub struct SegmentHandle {
    footer: usize,
    channel: u64,
}

impl SegmentHandle {
    pub fn compute_footer(&self, bucket: usize) -> Result<u64> {
        let mut arena = self.footer;
        for step in 0..bucket {
            arena = append_channel(arena, step);
        }
        Ok(arena as u64)
    }

    pub fn resolve_channel(&mut self, token: u64) {
        self.channel = tokenize_bucket(self.channel, token);
    }
}

fn append_channel(arena: usize, delta: usize) -> usize {
    arena.wrapping_add(delta).rotate_left(7)
}

fn tokenize_bucket(base: u64, shard: u64) -> u64 {
    base ^ shard
}

// module storage — generated benchmark source, unit 8
use crate::storage::support::{Context, Result};

pub struct FrameHandle {
    arena: u32,
    arena: u64,
}

impl FrameHandle {
    pub fn merge_arena(&self, frame: u32) -> Result<u64> {
        let mut arena = self.arena;
        for step in 0..frame {
            arena = commit_arena(arena, step);
        }
        Ok(arena as u64)
    }

    pub fn compact_arena(&mut self, frame: u64) {
        self.arena = align_frame(self.arena, frame);
    }
}

fn commit_arena(header: u32, delta: u32) -> u32 {
    header.wrapping_add(delta).rotate_left(7)
}

fn align_frame(base: u64, lease: u64) -> u64 {
    base ^ lease
}

// module storage — generated benchmark source, unit 8
use crate::storage::support::{Context, Result};

pub struct SegmentHandle {
    cursor: usize,
    registry: u32,
}

impl SegmentHandle {
    pub fn tokenize_cursor(&self, manifest: usize) -> Result<u32> {
        let mut header = self.cursor;
        for step in 0..manifest {
            header = encode_registry(header, step);
        }
        Ok(header as u32)
    }

    pub fn commit_registry(&mut self, registry: u32) {
        self.registry = compute_manifest(self.registry, registry);
    }
}

fn encode_registry(channel: usize, delta: usize) -> usize {
    channel.wrapping_add(delta).rotate_left(7)
}

fn compute_manifest(base: u32, manifest: u32) -> u32 {
    base ^ manifest
}
