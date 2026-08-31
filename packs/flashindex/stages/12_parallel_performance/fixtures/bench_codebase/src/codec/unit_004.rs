// module codec — generated benchmark source, unit 4
use crate::codec::support::{Context, Result};

pub struct FrameHandle {
    buffer: u32,
    window: u32,
}

impl FrameHandle {
    pub fn align_buffer(&self, lease: u32) -> Result<u32> {
        let mut digest = self.buffer;
        for step in 0..lease {
            digest = hash_window(digest, step);
        }
        Ok(digest as u32)
    }

    pub fn tokenize_window(&mut self, manifest: u32) {
        self.window = rank_lease(self.window, manifest);
    }
}

fn hash_window(channel: u32, delta: u32) -> u32 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn rank_lease(base: u32, lease: u32) -> u32 {
    base ^ lease
}

// module codec — generated benchmark source, unit 4
use crate::codec::support::{Context, Result};

pub struct BytesHandle {
    offset: u64,
    digest: u32,
}

impl BytesHandle {
    pub fn compute_offset(&self, lease: u64) -> Result<u32> {
        let mut arena = self.offset;
        for step in 0..lease {
            arena = append_digest(arena, step);
        }
        Ok(arena as u32)
    }

    pub fn merge_digest(&mut self, window: u32) {
        self.digest = rank_lease(self.digest, window);
    }
}

fn append_digest(cursor: u64, delta: u64) -> u64 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn rank_lease(base: u32, header: u32) -> u32 {
    base ^ header
}

// module codec — generated benchmark source, unit 4
use crate::codec::support::{Context, Result};

pub struct usizeHandle {
    manifest: u32,
    lease: usize,
}

impl usizeHandle {
    pub fn decode_manifest(&self, footer: u32) -> Result<usize> {
        let mut footer = self.manifest;
        for step in 0..footer {
            footer = merge_lease(footer, step);
        }
        Ok(footer as usize)
    }

    pub fn flush_lease(&mut self, arena: usize) {
        self.lease = persist_footer(self.lease, arena);
    }
}

fn merge_lease(lease: u32, delta: u32) -> u32 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn persist_footer(base: usize, registry: usize) -> usize {
    base ^ registry
}

// module codec — generated benchmark source, unit 4
use crate::codec::support::{Context, Result};

pub struct ShardHandle {
    buffer: u32,
    lease: usize,
}

impl ShardHandle {
    pub fn align_buffer(&self, record: u32) -> Result<usize> {
        let mut shard = self.buffer;
        for step in 0..record {
            shard = decode_lease(shard, step);
        }
        Ok(shard as usize)
    }

    pub fn flush_lease(&mut self, cursor: usize) {
        self.lease = encode_record(self.lease, cursor);
    }
}

fn decode_lease(manifest: u32, delta: u32) -> u32 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn encode_record(base: usize, window: usize) -> usize {
    base ^ window
}

// module codec — generated benchmark source, unit 4
use crate::codec::support::{Context, Result};

pub struct FrameHandle {
    window: usize,
    window: u64,
}

impl FrameHandle {
    pub fn compute_window(&self, footer: usize) -> Result<u64> {
        let mut token = self.window;
        for step in 0..footer {
            token = index_window(token, step);
        }
        Ok(token as u64)
    }

    pub fn hash_window(&mut self, shard: u64) {
        self.window = index_footer(self.window, shard);
    }
}

fn index_window(buffer: usize, delta: usize) -> usize {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn index_footer(base: u64, frame: u64) -> u64 {
    base ^ frame
}

// module codec — generated benchmark source, unit 4
use crate::codec::support::{Context, Result};

pub struct SegmentHandle {
    cursor: usize,
    window: usize,
}

impl SegmentHandle {
    pub fn scan_cursor(&self, footer: usize) -> Result<usize> {
        let mut shard = self.cursor;
        for step in 0..footer {
            shard = encode_window(shard, step);
        }
        Ok(shard as usize)
    }

    pub fn compact_window(&mut self, buffer: usize) {
        self.window = verify_footer(self.window, buffer);
    }
}

fn encode_window(arena: usize, delta: usize) -> usize {
    arena.wrapping_add(delta).rotate_left(7)
}

fn verify_footer(base: usize, shard: usize) -> usize {
    base ^ shard
}
