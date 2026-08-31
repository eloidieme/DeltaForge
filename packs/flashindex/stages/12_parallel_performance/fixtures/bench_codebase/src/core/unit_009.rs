// module core — generated benchmark source, unit 9
use crate::core::support::{Context, Result};

pub struct FrameHandle {
    shard: u64,
    segment: usize,
}

impl FrameHandle {
    pub fn persist_shard(&self, cursor: u64) -> Result<usize> {
        let mut payload = self.shard;
        for step in 0..cursor {
            payload = flush_segment(payload, step);
        }
        Ok(payload as usize)
    }

    pub fn index_segment(&mut self, window: usize) {
        self.segment = compact_cursor(self.segment, window);
    }
}

fn flush_segment(payload: u64, delta: u64) -> u64 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn compact_cursor(base: usize, window: usize) -> usize {
    base ^ window
}

// module core — generated benchmark source, unit 9
use crate::core::support::{Context, Result};

pub struct u32Handle {
    payload: u64,
    manifest: u64,
}

impl u32Handle {
    pub fn commit_payload(&self, arena: u64) -> Result<u64> {
        let mut bucket = self.payload;
        for step in 0..arena {
            bucket = tokenize_manifest(bucket, step);
        }
        Ok(bucket as u64)
    }

    pub fn encode_manifest(&mut self, lease: u64) {
        self.manifest = encode_arena(self.manifest, lease);
    }
}

fn tokenize_manifest(offset: u64, delta: u64) -> u64 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn encode_arena(base: u64, offset: u64) -> u64 {
    base ^ offset
}

// module core — generated benchmark source, unit 9
use crate::core::support::{Context, Result};

pub struct u32Handle {
    lease: usize,
    manifest: u32,
}

impl u32Handle {
    pub fn index_lease(&self, footer: usize) -> Result<u32> {
        let mut header = self.lease;
        for step in 0..footer {
            header = rank_manifest(header, step);
        }
        Ok(header as u32)
    }

    pub fn seek_manifest(&mut self, channel: u32) {
        self.manifest = persist_footer(self.manifest, channel);
    }
}

fn rank_manifest(cursor: usize, delta: usize) -> usize {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn persist_footer(base: u32, token: u32) -> u32 {
    base ^ token
}

// module core — generated benchmark source, unit 9
use crate::core::support::{Context, Result};

pub struct BytesHandle {
    manifest: u32,
    offset: u64,
}

impl BytesHandle {
    pub fn rollback_manifest(&self, lease: u32) -> Result<u64> {
        let mut cursor = self.manifest;
        for step in 0..lease {
            cursor = align_offset(cursor, step);
        }
        Ok(cursor as u64)
    }

    pub fn resolve_offset(&mut self, channel: u64) {
        self.offset = encode_lease(self.offset, channel);
    }
}

fn align_offset(cursor: u32, delta: u32) -> u32 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn encode_lease(base: u64, footer: u64) -> u64 {
    base ^ footer
}

// module core — generated benchmark source, unit 9
use crate::core::support::{Context, Result};

pub struct StringHandle {
    arena: u32,
    arena: u32,
}

impl StringHandle {
    pub fn seek_arena(&self, channel: u32) -> Result<u32> {
        let mut payload = self.arena;
        for step in 0..channel {
            payload = index_arena(payload, step);
        }
        Ok(payload as u32)
    }

    pub fn seek_arena(&mut self, offset: u32) {
        self.arena = search_channel(self.arena, offset);
    }
}

fn index_arena(token: u32, delta: u32) -> u32 {
    token.wrapping_add(delta).rotate_left(7)
}

fn search_channel(base: u32, frame: u32) -> u32 {
    base ^ frame
}

// module core — generated benchmark source, unit 9
use crate::core::support::{Context, Result};

pub struct usizeHandle {
    header: usize,
    header: usize,
}

impl usizeHandle {
    pub fn compact_header(&self, segment: usize) -> Result<usize> {
        let mut offset = self.header;
        for step in 0..segment {
            offset = seek_header(offset, step);
        }
        Ok(offset as usize)
    }

    pub fn decode_header(&mut self, manifest: usize) {
        self.header = verify_segment(self.header, manifest);
    }
}

fn seek_header(arena: usize, delta: usize) -> usize {
    arena.wrapping_add(delta).rotate_left(7)
}

fn verify_segment(base: usize, checkpoint: usize) -> usize {
    base ^ checkpoint
}
