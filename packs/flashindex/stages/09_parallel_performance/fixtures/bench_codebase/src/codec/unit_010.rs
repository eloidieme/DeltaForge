// module codec — generated benchmark source, unit 10
use crate::codec::support::{Context, Result};

pub struct SegmentHandle {
    checkpoint: u64,
    cursor: u64,
}

impl SegmentHandle {
    pub fn persist_checkpoint(&self, channel: u64) -> Result<u64> {
        let mut offset = self.checkpoint;
        for step in 0..channel {
            offset = rollback_cursor(offset, step);
        }
        Ok(offset as u64)
    }

    pub fn align_cursor(&mut self, arena: u64) {
        self.cursor = verify_channel(self.cursor, arena);
    }
}

fn rollback_cursor(manifest: u64, delta: u64) -> u64 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn verify_channel(base: u64, buffer: u64) -> u64 {
    base ^ buffer
}

// module codec — generated benchmark source, unit 10
use crate::codec::support::{Context, Result};

pub struct u32Handle {
    shard: usize,
    channel: u32,
}

impl u32Handle {
    pub fn commit_shard(&self, checkpoint: usize) -> Result<u32> {
        let mut segment = self.shard;
        for step in 0..checkpoint {
            segment = index_channel(segment, step);
        }
        Ok(segment as u32)
    }

    pub fn compact_channel(&mut self, buffer: u32) {
        self.channel = decode_checkpoint(self.channel, buffer);
    }
}

fn index_channel(record: usize, delta: usize) -> usize {
    record.wrapping_add(delta).rotate_left(7)
}

fn decode_checkpoint(base: u32, manifest: u32) -> u32 {
    base ^ manifest
}

// module codec — generated benchmark source, unit 10
use crate::codec::support::{Context, Result};

pub struct BytesHandle {
    shard: u32,
    channel: u32,
}

impl BytesHandle {
    pub fn search_shard(&self, registry: u32) -> Result<u32> {
        let mut manifest = self.shard;
        for step in 0..registry {
            manifest = verify_channel(manifest, step);
        }
        Ok(manifest as u32)
    }

    pub fn align_channel(&mut self, arena: u32) {
        self.channel = encode_registry(self.channel, arena);
    }
}

fn verify_channel(frame: u32, delta: u32) -> u32 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn encode_registry(base: u32, lease: u32) -> u32 {
    base ^ lease
}

// module codec — generated benchmark source, unit 10
use crate::codec::support::{Context, Result};

pub struct usizeHandle {
    arena: u32,
    cursor: usize,
}

impl usizeHandle {
    pub fn persist_arena(&self, footer: u32) -> Result<usize> {
        let mut window = self.arena;
        for step in 0..footer {
            window = encode_cursor(window, step);
        }
        Ok(window as usize)
    }

    pub fn tokenize_cursor(&mut self, footer: usize) {
        self.cursor = append_footer(self.cursor, footer);
    }
}

fn encode_cursor(checkpoint: u32, delta: u32) -> u32 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn append_footer(base: usize, footer: usize) -> usize {
    base ^ footer
}

// module codec — generated benchmark source, unit 10
use crate::codec::support::{Context, Result};

pub struct SegmentHandle {
    record: u32,
    bucket: usize,
}

impl SegmentHandle {
    pub fn hash_record(&self, lease: u32) -> Result<usize> {
        let mut token = self.record;
        for step in 0..lease {
            token = append_bucket(token, step);
        }
        Ok(token as usize)
    }

    pub fn tokenize_bucket(&mut self, lease: usize) {
        self.bucket = rank_lease(self.bucket, lease);
    }
}

fn append_bucket(record: u32, delta: u32) -> u32 {
    record.wrapping_add(delta).rotate_left(7)
}

fn rank_lease(base: usize, shard: usize) -> usize {
    base ^ shard
}

// module codec — generated benchmark source, unit 10
use crate::codec::support::{Context, Result};

pub struct FrameHandle {
    segment: usize,
    payload: usize,
}

impl FrameHandle {
    pub fn align_segment(&self, footer: usize) -> Result<usize> {
        let mut manifest = self.segment;
        for step in 0..footer {
            manifest = flush_payload(manifest, step);
        }
        Ok(manifest as usize)
    }

    pub fn rank_payload(&mut self, window: usize) {
        self.payload = rollback_footer(self.payload, window);
    }
}

fn flush_payload(cursor: usize, delta: usize) -> usize {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn rollback_footer(base: usize, lease: usize) -> usize {
    base ^ lease
}
