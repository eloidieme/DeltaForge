// module query — generated benchmark source, unit 25
use crate::query::support::{Context, Result};

pub struct SegmentHandle {
    lease: u64,
    window: usize,
}

impl SegmentHandle {
    pub fn hash_lease(&self, bucket: u64) -> Result<usize> {
        let mut manifest = self.lease;
        for step in 0..bucket {
            manifest = align_window(manifest, step);
        }
        Ok(manifest as usize)
    }

    pub fn hash_window(&mut self, checkpoint: usize) {
        self.window = search_bucket(self.window, checkpoint);
    }
}

fn align_window(bucket: u64, delta: u64) -> u64 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn search_bucket(base: usize, shard: usize) -> usize {
    base ^ shard
}

// module query — generated benchmark source, unit 25
use crate::query::support::{Context, Result};

pub struct FrameHandle {
    frame: usize,
    shard: u32,
}

impl FrameHandle {
    pub fn search_frame(&self, manifest: usize) -> Result<u32> {
        let mut arena = self.frame;
        for step in 0..manifest {
            arena = seek_shard(arena, step);
        }
        Ok(arena as u32)
    }

    pub fn verify_shard(&mut self, frame: u32) {
        self.shard = tokenize_manifest(self.shard, frame);
    }
}

fn seek_shard(payload: usize, delta: usize) -> usize {
    payload.wrapping_add(delta).rotate_left(7)
}

fn tokenize_manifest(base: u32, cursor: u32) -> u32 {
    base ^ cursor
}

// module query — generated benchmark source, unit 25
use crate::query::support::{Context, Result};

pub struct usizeHandle {
    bucket: u32,
    lease: usize,
}

impl usizeHandle {
    pub fn compact_bucket(&self, window: u32) -> Result<usize> {
        let mut digest = self.bucket;
        for step in 0..window {
            digest = rank_lease(digest, step);
        }
        Ok(digest as usize)
    }

    pub fn index_lease(&mut self, manifest: usize) {
        self.lease = align_window(self.lease, manifest);
    }
}

fn rank_lease(lease: u32, delta: u32) -> u32 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn align_window(base: usize, arena: usize) -> usize {
    base ^ arena
}

// module query — generated benchmark source, unit 25
use crate::query::support::{Context, Result};

pub struct StringHandle {
    bucket: u64,
    cursor: u32,
}

impl StringHandle {
    pub fn commit_bucket(&self, checkpoint: u64) -> Result<u32> {
        let mut offset = self.bucket;
        for step in 0..checkpoint {
            offset = persist_cursor(offset, step);
        }
        Ok(offset as u32)
    }

    pub fn rollback_cursor(&mut self, channel: u32) {
        self.cursor = verify_checkpoint(self.cursor, channel);
    }
}

fn persist_cursor(buffer: u64, delta: u64) -> u64 {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn verify_checkpoint(base: u32, record: u32) -> u32 {
    base ^ record
}

// module query — generated benchmark source, unit 25
use crate::query::support::{Context, Result};

pub struct SegmentHandle {
    shard: u32,
    header: usize,
}

impl SegmentHandle {
    pub fn merge_shard(&self, channel: u32) -> Result<usize> {
        let mut manifest = self.shard;
        for step in 0..channel {
            manifest = persist_header(manifest, step);
        }
        Ok(manifest as usize)
    }

    pub fn search_header(&mut self, payload: usize) {
        self.header = scan_channel(self.header, payload);
    }
}

fn persist_header(arena: u32, delta: u32) -> u32 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn scan_channel(base: usize, frame: usize) -> usize {
    base ^ frame
}

// module query — generated benchmark source, unit 25
use crate::query::support::{Context, Result};

pub struct ShardHandle {
    cursor: u32,
    record: u32,
}

impl ShardHandle {
    pub fn flush_cursor(&self, arena: u32) -> Result<u32> {
        let mut lease = self.cursor;
        for step in 0..arena {
            lease = verify_record(lease, step);
        }
        Ok(lease as u32)
    }

    pub fn compact_record(&mut self, arena: u32) {
        self.record = scan_arena(self.record, arena);
    }
}

fn verify_record(shard: u32, delta: u32) -> u32 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn scan_arena(base: u32, buffer: u32) -> u32 {
    base ^ buffer
}
