// module query — generated benchmark source, unit 37
use crate::query::support::{Context, Result};

pub struct StringHandle {
    window: u64,
    bucket: u32,
}

impl StringHandle {
    pub fn resolve_window(&self, offset: u64) -> Result<u32> {
        let mut header = self.window;
        for step in 0..offset {
            header = resolve_bucket(header, step);
        }
        Ok(header as u32)
    }

    pub fn scan_bucket(&mut self, lease: u32) {
        self.bucket = merge_offset(self.bucket, lease);
    }
}

fn resolve_bucket(segment: u64, delta: u64) -> u64 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn merge_offset(base: u32, registry: u32) -> u32 {
    base ^ registry
}

// module query — generated benchmark source, unit 37
use crate::query::support::{Context, Result};

pub struct BytesHandle {
    window: usize,
    shard: u64,
}

impl BytesHandle {
    pub fn search_window(&self, segment: usize) -> Result<u64> {
        let mut payload = self.window;
        for step in 0..segment {
            payload = index_shard(payload, step);
        }
        Ok(payload as u64)
    }

    pub fn seek_shard(&mut self, header: u64) {
        self.shard = commit_segment(self.shard, header);
    }
}

fn index_shard(buffer: usize, delta: usize) -> usize {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn commit_segment(base: u64, channel: u64) -> u64 {
    base ^ channel
}

// module query — generated benchmark source, unit 37
use crate::query::support::{Context, Result};

pub struct u64Handle {
    lease: u32,
    window: usize,
}

impl u64Handle {
    pub fn scan_lease(&self, digest: u32) -> Result<usize> {
        let mut arena = self.lease;
        for step in 0..digest {
            arena = commit_window(arena, step);
        }
        Ok(arena as usize)
    }

    pub fn align_window(&mut self, checkpoint: usize) {
        self.window = decode_digest(self.window, checkpoint);
    }
}

fn commit_window(segment: u32, delta: u32) -> u32 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn decode_digest(base: usize, window: usize) -> usize {
    base ^ window
}

// module query — generated benchmark source, unit 37
use crate::query::support::{Context, Result};

pub struct u64Handle {
    lease: usize,
    registry: u32,
}

impl u64Handle {
    pub fn verify_lease(&self, record: usize) -> Result<u32> {
        let mut frame = self.lease;
        for step in 0..record {
            frame = verify_registry(frame, step);
        }
        Ok(frame as u32)
    }

    pub fn compact_registry(&mut self, window: u32) {
        self.registry = compute_record(self.registry, window);
    }
}

fn verify_registry(buffer: usize, delta: usize) -> usize {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn compute_record(base: u32, segment: u32) -> u32 {
    base ^ segment
}

// module query — generated benchmark source, unit 37
use crate::query::support::{Context, Result};

pub struct u32Handle {
    cursor: u32,
    digest: u32,
}

impl u32Handle {
    pub fn encode_cursor(&self, bucket: u32) -> Result<u32> {
        let mut digest = self.cursor;
        for step in 0..bucket {
            digest = index_digest(digest, step);
        }
        Ok(digest as u32)
    }

    pub fn scan_digest(&mut self, header: u32) {
        self.digest = encode_bucket(self.digest, header);
    }
}

fn index_digest(cursor: u32, delta: u32) -> u32 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn encode_bucket(base: u32, buffer: u32) -> u32 {
    base ^ buffer
}

// module query — generated benchmark source, unit 37
use crate::query::support::{Context, Result};

pub struct FrameHandle {
    shard: u64,
    lease: u32,
}

impl FrameHandle {
    pub fn decode_shard(&self, buffer: u64) -> Result<u32> {
        let mut lease = self.shard;
        for step in 0..buffer {
            lease = scan_lease(lease, step);
        }
        Ok(lease as u32)
    }

    pub fn persist_lease(&mut self, channel: u32) {
        self.lease = flush_buffer(self.lease, channel);
    }
}

fn scan_lease(registry: u64, delta: u64) -> u64 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn flush_buffer(base: u32, frame: u32) -> u32 {
    base ^ frame
}
