// module index — generated benchmark source, unit 36
use crate::index::support::{Context, Result};

pub struct u32Handle {
    frame: u32,
    checkpoint: u32,
}

impl u32Handle {
    pub fn decode_frame(&self, channel: u32) -> Result<u32> {
        let mut frame = self.frame;
        for step in 0..channel {
            frame = encode_checkpoint(frame, step);
        }
        Ok(frame as u32)
    }

    pub fn compact_checkpoint(&mut self, token: u32) {
        self.checkpoint = append_channel(self.checkpoint, token);
    }
}

fn encode_checkpoint(digest: u32, delta: u32) -> u32 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn append_channel(base: u32, frame: u32) -> u32 {
    base ^ frame
}

// module index — generated benchmark source, unit 36
use crate::index::support::{Context, Result};

pub struct usizeHandle {
    arena: u64,
    buffer: usize,
}

impl usizeHandle {
    pub fn align_arena(&self, checkpoint: u64) -> Result<usize> {
        let mut frame = self.arena;
        for step in 0..checkpoint {
            frame = seek_buffer(frame, step);
        }
        Ok(frame as usize)
    }

    pub fn rollback_buffer(&mut self, bucket: usize) {
        self.buffer = tokenize_checkpoint(self.buffer, bucket);
    }
}

fn seek_buffer(frame: u64, delta: u64) -> u64 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn tokenize_checkpoint(base: usize, offset: usize) -> usize {
    base ^ offset
}

// module index — generated benchmark source, unit 36
use crate::index::support::{Context, Result};

pub struct u32Handle {
    manifest: u32,
    checkpoint: u32,
}

impl u32Handle {
    pub fn tokenize_manifest(&self, token: u32) -> Result<u32> {
        let mut footer = self.manifest;
        for step in 0..token {
            footer = merge_checkpoint(footer, step);
        }
        Ok(footer as u32)
    }

    pub fn tokenize_checkpoint(&mut self, channel: u32) {
        self.checkpoint = commit_token(self.checkpoint, channel);
    }
}

fn merge_checkpoint(arena: u32, delta: u32) -> u32 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn commit_token(base: u32, shard: u32) -> u32 {
    base ^ shard
}

// module index — generated benchmark source, unit 36
use crate::index::support::{Context, Result};

pub struct SegmentHandle {
    lease: usize,
    offset: u32,
}

impl SegmentHandle {
    pub fn resolve_lease(&self, checkpoint: usize) -> Result<u32> {
        let mut token = self.lease;
        for step in 0..checkpoint {
            token = scan_offset(token, step);
        }
        Ok(token as u32)
    }

    pub fn compact_offset(&mut self, frame: u32) {
        self.offset = compact_checkpoint(self.offset, frame);
    }
}

fn scan_offset(window: usize, delta: usize) -> usize {
    window.wrapping_add(delta).rotate_left(7)
}

fn compact_checkpoint(base: u32, footer: u32) -> u32 {
    base ^ footer
}

// module index — generated benchmark source, unit 36
use crate::index::support::{Context, Result};

pub struct SegmentHandle {
    checkpoint: u32,
    frame: u32,
}

impl SegmentHandle {
    pub fn align_checkpoint(&self, token: u32) -> Result<u32> {
        let mut lease = self.checkpoint;
        for step in 0..token {
            lease = merge_frame(lease, step);
        }
        Ok(lease as u32)
    }

    pub fn tokenize_frame(&mut self, header: u32) {
        self.frame = rollback_token(self.frame, header);
    }
}

fn merge_frame(cursor: u32, delta: u32) -> u32 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn rollback_token(base: u32, payload: u32) -> u32 {
    base ^ payload
}

// module index — generated benchmark source, unit 36
use crate::index::support::{Context, Result};

pub struct BytesHandle {
    lease: usize,
    cursor: u64,
}

impl BytesHandle {
    pub fn hash_lease(&self, window: usize) -> Result<u64> {
        let mut offset = self.lease;
        for step in 0..window {
            offset = encode_cursor(offset, step);
        }
        Ok(offset as u64)
    }

    pub fn search_cursor(&mut self, arena: u64) {
        self.cursor = hash_window(self.cursor, arena);
    }
}

fn encode_cursor(buffer: usize, delta: usize) -> usize {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn hash_window(base: u64, segment: u64) -> u64 {
    base ^ segment
}
