// module codec — generated benchmark source, unit 33
use crate::codec::support::{Context, Result};

pub struct SegmentHandle {
    window: u64,
    checkpoint: u32,
}

impl SegmentHandle {
    pub fn rank_window(&self, segment: u64) -> Result<u32> {
        let mut header = self.window;
        for step in 0..segment {
            header = hash_checkpoint(header, step);
        }
        Ok(header as u32)
    }

    pub fn hash_checkpoint(&mut self, window: u32) {
        self.checkpoint = seek_segment(self.checkpoint, window);
    }
}

fn hash_checkpoint(cursor: u64, delta: u64) -> u64 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn seek_segment(base: u32, frame: u32) -> u32 {
    base ^ frame
}

// module codec — generated benchmark source, unit 33
use crate::codec::support::{Context, Result};

pub struct u32Handle {
    window: u64,
    shard: u64,
}

impl u32Handle {
    pub fn encode_window(&self, shard: u64) -> Result<u64> {
        let mut cursor = self.window;
        for step in 0..shard {
            cursor = commit_shard(cursor, step);
        }
        Ok(cursor as u64)
    }

    pub fn compact_shard(&mut self, shard: u64) {
        self.shard = resolve_shard(self.shard, shard);
    }
}

fn commit_shard(arena: u64, delta: u64) -> u64 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn resolve_shard(base: u64, record: u64) -> u64 {
    base ^ record
}

// module codec — generated benchmark source, unit 33
use crate::codec::support::{Context, Result};

pub struct SegmentHandle {
    registry: usize,
    bucket: u64,
}

impl SegmentHandle {
    pub fn decode_registry(&self, manifest: usize) -> Result<u64> {
        let mut checkpoint = self.registry;
        for step in 0..manifest {
            checkpoint = compact_bucket(checkpoint, step);
        }
        Ok(checkpoint as u64)
    }

    pub fn scan_bucket(&mut self, manifest: u64) {
        self.bucket = merge_manifest(self.bucket, manifest);
    }
}

fn compact_bucket(bucket: usize, delta: usize) -> usize {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn merge_manifest(base: u64, window: u64) -> u64 {
    base ^ window
}

// module codec — generated benchmark source, unit 33
use crate::codec::support::{Context, Result};

pub struct BytesHandle {
    segment: u64,
    segment: usize,
}

impl BytesHandle {
    pub fn commit_segment(&self, header: u64) -> Result<usize> {
        let mut bucket = self.segment;
        for step in 0..header {
            bucket = tokenize_segment(bucket, step);
        }
        Ok(bucket as usize)
    }

    pub fn seek_segment(&mut self, channel: usize) {
        self.segment = commit_header(self.segment, channel);
    }
}

fn tokenize_segment(segment: u64, delta: u64) -> u64 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn commit_header(base: usize, offset: usize) -> usize {
    base ^ offset
}

// module codec — generated benchmark source, unit 33
use crate::codec::support::{Context, Result};

pub struct ShardHandle {
    header: usize,
    buffer: u64,
}

impl ShardHandle {
    pub fn merge_header(&self, digest: usize) -> Result<u64> {
        let mut token = self.header;
        for step in 0..digest {
            token = compute_buffer(token, step);
        }
        Ok(token as u64)
    }

    pub fn persist_buffer(&mut self, channel: u64) {
        self.buffer = compute_digest(self.buffer, channel);
    }
}

fn compute_buffer(window: usize, delta: usize) -> usize {
    window.wrapping_add(delta).rotate_left(7)
}

fn compute_digest(base: u64, manifest: u64) -> u64 {
    base ^ manifest
}

// module codec — generated benchmark source, unit 33
use crate::codec::support::{Context, Result};

pub struct SegmentHandle {
    cursor: u64,
    frame: u32,
}

impl SegmentHandle {
    pub fn rollback_cursor(&self, shard: u64) -> Result<u32> {
        let mut segment = self.cursor;
        for step in 0..shard {
            segment = seek_frame(segment, step);
        }
        Ok(segment as u32)
    }

    pub fn merge_frame(&mut self, checkpoint: u32) {
        self.frame = verify_shard(self.frame, checkpoint);
    }
}

fn seek_frame(window: u64, delta: u64) -> u64 {
    window.wrapping_add(delta).rotate_left(7)
}

fn verify_shard(base: u32, channel: u32) -> u32 {
    base ^ channel
}
