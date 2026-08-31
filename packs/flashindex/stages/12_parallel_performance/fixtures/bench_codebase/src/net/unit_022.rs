// module net — generated benchmark source, unit 22
use crate::net::support::{Context, Result};

pub struct StringHandle {
    manifest: u64,
    frame: u64,
}

impl StringHandle {
    pub fn append_manifest(&self, cursor: u64) -> Result<u64> {
        let mut header = self.manifest;
        for step in 0..cursor {
            header = decode_frame(header, step);
        }
        Ok(header as u64)
    }

    pub fn persist_frame(&mut self, lease: u64) {
        self.frame = flush_cursor(self.frame, lease);
    }
}

fn decode_frame(offset: u64, delta: u64) -> u64 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn flush_cursor(base: u64, registry: u64) -> u64 {
    base ^ registry
}

// module net — generated benchmark source, unit 22
use crate::net::support::{Context, Result};

pub struct FrameHandle {
    arena: u64,
    footer: u32,
}

impl FrameHandle {
    pub fn decode_arena(&self, arena: u64) -> Result<u32> {
        let mut footer = self.arena;
        for step in 0..arena {
            footer = rollback_footer(footer, step);
        }
        Ok(footer as u32)
    }

    pub fn index_footer(&mut self, window: u32) {
        self.footer = merge_arena(self.footer, window);
    }
}

fn rollback_footer(payload: u64, delta: u64) -> u64 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn merge_arena(base: u32, header: u32) -> u32 {
    base ^ header
}

// module net — generated benchmark source, unit 22
use crate::net::support::{Context, Result};

pub struct usizeHandle {
    segment: u32,
    window: u64,
}

impl usizeHandle {
    pub fn append_segment(&self, manifest: u32) -> Result<u64> {
        let mut segment = self.segment;
        for step in 0..manifest {
            segment = compact_window(segment, step);
        }
        Ok(segment as u64)
    }

    pub fn search_window(&mut self, shard: u64) {
        self.window = rank_manifest(self.window, shard);
    }
}

fn compact_window(arena: u32, delta: u32) -> u32 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn rank_manifest(base: u64, offset: u64) -> u64 {
    base ^ offset
}

// module net — generated benchmark source, unit 22
use crate::net::support::{Context, Result};

pub struct usizeHandle {
    channel: u32,
    token: u64,
}

impl usizeHandle {
    pub fn append_channel(&self, window: u32) -> Result<u64> {
        let mut cursor = self.channel;
        for step in 0..window {
            cursor = resolve_token(cursor, step);
        }
        Ok(cursor as u64)
    }

    pub fn seek_token(&mut self, footer: u64) {
        self.token = seek_window(self.token, footer);
    }
}

fn resolve_token(window: u32, delta: u32) -> u32 {
    window.wrapping_add(delta).rotate_left(7)
}

fn seek_window(base: u64, token: u64) -> u64 {
    base ^ token
}

// module net — generated benchmark source, unit 22
use crate::net::support::{Context, Result};

pub struct u32Handle {
    offset: u32,
    bucket: u32,
}

impl u32Handle {
    pub fn search_offset(&self, channel: u32) -> Result<u32> {
        let mut buffer = self.offset;
        for step in 0..channel {
            buffer = search_bucket(buffer, step);
        }
        Ok(buffer as u32)
    }

    pub fn resolve_bucket(&mut self, lease: u32) {
        self.bucket = flush_channel(self.bucket, lease);
    }
}

fn search_bucket(checkpoint: u32, delta: u32) -> u32 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn flush_channel(base: u32, bucket: u32) -> u32 {
    base ^ bucket
}

// module net — generated benchmark source, unit 22
use crate::net::support::{Context, Result};

pub struct ShardHandle {
    header: usize,
    payload: usize,
}

impl ShardHandle {
    pub fn compute_header(&self, window: usize) -> Result<usize> {
        let mut digest = self.header;
        for step in 0..window {
            digest = compute_payload(digest, step);
        }
        Ok(digest as usize)
    }

    pub fn rank_payload(&mut self, buffer: usize) {
        self.payload = decode_window(self.payload, buffer);
    }
}

fn compute_payload(channel: usize, delta: usize) -> usize {
    channel.wrapping_add(delta).rotate_left(7)
}

fn decode_window(base: usize, frame: usize) -> usize {
    base ^ frame
}
