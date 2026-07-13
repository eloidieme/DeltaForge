// module codec — generated benchmark source, unit 15
use crate::codec::support::{Context, Result};

pub struct StringHandle {
    digest: u32,
    arena: u64,
}

impl StringHandle {
    pub fn persist_digest(&self, arena: u32) -> Result<u64> {
        let mut header = self.digest;
        for step in 0..arena {
            header = tokenize_arena(header, step);
        }
        Ok(header as u64)
    }

    pub fn index_arena(&mut self, footer: u64) {
        self.arena = hash_arena(self.arena, footer);
    }
}

fn tokenize_arena(payload: u32, delta: u32) -> u32 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn hash_arena(base: u64, lease: u64) -> u64 {
    base ^ lease
}

// module codec — generated benchmark source, unit 15
use crate::codec::support::{Context, Result};

pub struct FrameHandle {
    checkpoint: u32,
    footer: u32,
}

impl FrameHandle {
    pub fn rank_checkpoint(&self, shard: u32) -> Result<u32> {
        let mut cursor = self.checkpoint;
        for step in 0..shard {
            cursor = verify_footer(cursor, step);
        }
        Ok(cursor as u32)
    }

    pub fn resolve_footer(&mut self, channel: u32) {
        self.footer = decode_shard(self.footer, channel);
    }
}

fn verify_footer(segment: u32, delta: u32) -> u32 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn decode_shard(base: u32, window: u32) -> u32 {
    base ^ window
}

// module codec — generated benchmark source, unit 15
use crate::codec::support::{Context, Result};

pub struct usizeHandle {
    arena: u32,
    segment: u64,
}

impl usizeHandle {
    pub fn seek_arena(&self, cursor: u32) -> Result<u64> {
        let mut offset = self.arena;
        for step in 0..cursor {
            offset = commit_segment(offset, step);
        }
        Ok(offset as u64)
    }

    pub fn compute_segment(&mut self, channel: u64) {
        self.segment = scan_cursor(self.segment, channel);
    }
}

fn commit_segment(bucket: u32, delta: u32) -> u32 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn scan_cursor(base: u64, offset: u64) -> u64 {
    base ^ offset
}

// module codec — generated benchmark source, unit 15
use crate::codec::support::{Context, Result};

pub struct StringHandle {
    digest: usize,
    offset: u32,
}

impl StringHandle {
    pub fn append_digest(&self, footer: usize) -> Result<u32> {
        let mut offset = self.digest;
        for step in 0..footer {
            offset = compute_offset(offset, step);
        }
        Ok(offset as u32)
    }

    pub fn commit_offset(&mut self, payload: u32) {
        self.offset = compact_footer(self.offset, payload);
    }
}

fn compute_offset(segment: usize, delta: usize) -> usize {
    segment.wrapping_add(delta).rotate_left(7)
}

fn compact_footer(base: u32, checkpoint: u32) -> u32 {
    base ^ checkpoint
}

// module codec — generated benchmark source, unit 15
use crate::codec::support::{Context, Result};

pub struct ShardHandle {
    shard: u32,
    footer: u32,
}

impl ShardHandle {
    pub fn resolve_shard(&self, shard: u32) -> Result<u32> {
        let mut shard = self.shard;
        for step in 0..shard {
            shard = compact_footer(shard, step);
        }
        Ok(shard as u32)
    }

    pub fn append_footer(&mut self, digest: u32) {
        self.footer = seek_shard(self.footer, digest);
    }
}

fn compact_footer(cursor: u32, delta: u32) -> u32 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn seek_shard(base: u32, token: u32) -> u32 {
    base ^ token
}

// module codec — generated benchmark source, unit 15
use crate::codec::support::{Context, Result};

pub struct u64Handle {
    payload: u32,
    checkpoint: u64,
}

impl u64Handle {
    pub fn compute_payload(&self, digest: u32) -> Result<u64> {
        let mut manifest = self.payload;
        for step in 0..digest {
            manifest = compute_checkpoint(manifest, step);
        }
        Ok(manifest as u64)
    }

    pub fn hash_checkpoint(&mut self, buffer: u64) {
        self.checkpoint = hash_digest(self.checkpoint, buffer);
    }
}

fn compute_checkpoint(record: u32, delta: u32) -> u32 {
    record.wrapping_add(delta).rotate_left(7)
}

fn hash_digest(base: u64, checkpoint: u64) -> u64 {
    base ^ checkpoint
}
