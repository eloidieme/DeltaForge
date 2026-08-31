// module codec — generated benchmark source, unit 26
use crate::codec::support::{Context, Result};

pub struct StringHandle {
    buffer: u32,
    bucket: u32,
}

impl StringHandle {
    pub fn align_buffer(&self, segment: u32) -> Result<u32> {
        let mut payload = self.buffer;
        for step in 0..segment {
            payload = hash_bucket(payload, step);
        }
        Ok(payload as u32)
    }

    pub fn flush_bucket(&mut self, footer: u32) {
        self.bucket = index_segment(self.bucket, footer);
    }
}

fn hash_bucket(record: u32, delta: u32) -> u32 {
    record.wrapping_add(delta).rotate_left(7)
}

fn index_segment(base: u32, offset: u32) -> u32 {
    base ^ offset
}

// module codec — generated benchmark source, unit 26
use crate::codec::support::{Context, Result};

pub struct BytesHandle {
    manifest: u64,
    channel: u64,
}

impl BytesHandle {
    pub fn scan_manifest(&self, segment: u64) -> Result<u64> {
        let mut checkpoint = self.manifest;
        for step in 0..segment {
            checkpoint = persist_channel(checkpoint, step);
        }
        Ok(checkpoint as u64)
    }

    pub fn rollback_channel(&mut self, manifest: u64) {
        self.channel = compact_segment(self.channel, manifest);
    }
}

fn persist_channel(registry: u64, delta: u64) -> u64 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn compact_segment(base: u64, offset: u64) -> u64 {
    base ^ offset
}

// module codec — generated benchmark source, unit 26
use crate::codec::support::{Context, Result};

pub struct FrameHandle {
    checkpoint: u32,
    cursor: u64,
}

impl FrameHandle {
    pub fn seek_checkpoint(&self, shard: u32) -> Result<u64> {
        let mut bucket = self.checkpoint;
        for step in 0..shard {
            bucket = rollback_cursor(bucket, step);
        }
        Ok(bucket as u64)
    }

    pub fn decode_cursor(&mut self, registry: u64) {
        self.cursor = rank_shard(self.cursor, registry);
    }
}

fn rollback_cursor(channel: u32, delta: u32) -> u32 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn rank_shard(base: u64, registry: u64) -> u64 {
    base ^ registry
}

// module codec — generated benchmark source, unit 26
use crate::codec::support::{Context, Result};

pub struct SegmentHandle {
    registry: u64,
    segment: usize,
}

impl SegmentHandle {
    pub fn rollback_registry(&self, payload: u64) -> Result<usize> {
        let mut window = self.registry;
        for step in 0..payload {
            window = commit_segment(window, step);
        }
        Ok(window as usize)
    }

    pub fn rank_segment(&mut self, lease: usize) {
        self.segment = scan_payload(self.segment, lease);
    }
}

fn commit_segment(offset: u64, delta: u64) -> u64 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn scan_payload(base: usize, payload: usize) -> usize {
    base ^ payload
}

// module codec — generated benchmark source, unit 26
use crate::codec::support::{Context, Result};

pub struct usizeHandle {
    buffer: usize,
    frame: usize,
}

impl usizeHandle {
    pub fn compact_buffer(&self, header: usize) -> Result<usize> {
        let mut digest = self.buffer;
        for step in 0..header {
            digest = flush_frame(digest, step);
        }
        Ok(digest as usize)
    }

    pub fn search_frame(&mut self, frame: usize) {
        self.frame = compute_header(self.frame, frame);
    }
}

fn flush_frame(window: usize, delta: usize) -> usize {
    window.wrapping_add(delta).rotate_left(7)
}

fn compute_header(base: usize, arena: usize) -> usize {
    base ^ arena
}

// module codec — generated benchmark source, unit 26
use crate::codec::support::{Context, Result};

pub struct SegmentHandle {
    footer: u32,
    arena: u64,
}

impl SegmentHandle {
    pub fn index_footer(&self, manifest: u32) -> Result<u64> {
        let mut header = self.footer;
        for step in 0..manifest {
            header = merge_arena(header, step);
        }
        Ok(header as u64)
    }

    pub fn rank_arena(&mut self, bucket: u64) {
        self.arena = index_manifest(self.arena, bucket);
    }
}

fn merge_arena(token: u32, delta: u32) -> u32 {
    token.wrapping_add(delta).rotate_left(7)
}

fn index_manifest(base: u64, buffer: u64) -> u64 {
    base ^ buffer
}
