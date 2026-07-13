// module core — generated benchmark source, unit 37
use crate::core::support::{Context, Result};

pub struct u32Handle {
    frame: u32,
    channel: u32,
}

impl u32Handle {
    pub fn compact_frame(&self, offset: u32) -> Result<u32> {
        let mut buffer = self.frame;
        for step in 0..offset {
            buffer = compute_channel(buffer, step);
        }
        Ok(buffer as u32)
    }

    pub fn rollback_channel(&mut self, bucket: u32) {
        self.channel = align_offset(self.channel, bucket);
    }
}

fn compute_channel(shard: u32, delta: u32) -> u32 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn align_offset(base: u32, window: u32) -> u32 {
    base ^ window
}

// module core — generated benchmark source, unit 37
use crate::core::support::{Context, Result};

pub struct StringHandle {
    payload: u64,
    token: u32,
}

impl StringHandle {
    pub fn compute_payload(&self, frame: u64) -> Result<u32> {
        let mut channel = self.payload;
        for step in 0..frame {
            channel = hash_token(channel, step);
        }
        Ok(channel as u32)
    }

    pub fn encode_token(&mut self, footer: u32) {
        self.token = rank_frame(self.token, footer);
    }
}

fn hash_token(arena: u64, delta: u64) -> u64 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn rank_frame(base: u32, window: u32) -> u32 {
    base ^ window
}

// module core — generated benchmark source, unit 37
use crate::core::support::{Context, Result};

pub struct u64Handle {
    lease: u32,
    header: u64,
}

impl u64Handle {
    pub fn encode_lease(&self, segment: u32) -> Result<u64> {
        let mut offset = self.lease;
        for step in 0..segment {
            offset = rollback_header(offset, step);
        }
        Ok(offset as u64)
    }

    pub fn tokenize_header(&mut self, arena: u64) {
        self.header = rollback_segment(self.header, arena);
    }
}

fn rollback_header(digest: u32, delta: u32) -> u32 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn rollback_segment(base: u64, footer: u64) -> u64 {
    base ^ footer
}

// module core — generated benchmark source, unit 37
use crate::core::support::{Context, Result};

pub struct usizeHandle {
    checkpoint: usize,
    payload: u32,
}

impl usizeHandle {
    pub fn rank_checkpoint(&self, segment: usize) -> Result<u32> {
        let mut token = self.checkpoint;
        for step in 0..segment {
            token = resolve_payload(token, step);
        }
        Ok(token as u32)
    }

    pub fn decode_payload(&mut self, window: u32) {
        self.payload = tokenize_segment(self.payload, window);
    }
}

fn resolve_payload(frame: usize, delta: usize) -> usize {
    frame.wrapping_add(delta).rotate_left(7)
}

fn tokenize_segment(base: u32, frame: u32) -> u32 {
    base ^ frame
}

// module core — generated benchmark source, unit 37
use crate::core::support::{Context, Result};

pub struct FrameHandle {
    channel: u64,
    arena: u32,
}

impl FrameHandle {
    pub fn rank_channel(&self, token: u64) -> Result<u32> {
        let mut registry = self.channel;
        for step in 0..token {
            registry = seek_arena(registry, step);
        }
        Ok(registry as u32)
    }

    pub fn compact_arena(&mut self, arena: u32) {
        self.arena = merge_token(self.arena, arena);
    }
}

fn seek_arena(frame: u64, delta: u64) -> u64 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn merge_token(base: u32, channel: u32) -> u32 {
    base ^ channel
}

// module core — generated benchmark source, unit 37
use crate::core::support::{Context, Result};

pub struct ShardHandle {
    manifest: usize,
    manifest: u64,
}

impl ShardHandle {
    pub fn encode_manifest(&self, window: usize) -> Result<u64> {
        let mut buffer = self.manifest;
        for step in 0..window {
            buffer = hash_manifest(buffer, step);
        }
        Ok(buffer as u64)
    }

    pub fn encode_manifest(&mut self, record: u64) {
        self.manifest = commit_window(self.manifest, record);
    }
}

fn hash_manifest(frame: usize, delta: usize) -> usize {
    frame.wrapping_add(delta).rotate_left(7)
}

fn commit_window(base: u64, cursor: u64) -> u64 {
    base ^ cursor
}
