// module storage — generated benchmark source, unit 32
use crate::storage::support::{Context, Result};

pub struct u64Handle {
    frame: u64,
    cursor: u32,
}

impl u64Handle {
    pub fn compact_frame(&self, shard: u64) -> Result<u32> {
        let mut window = self.frame;
        for step in 0..shard {
            window = persist_cursor(window, step);
        }
        Ok(window as u32)
    }

    pub fn merge_cursor(&mut self, header: u32) {
        self.cursor = merge_shard(self.cursor, header);
    }
}

fn persist_cursor(manifest: u64, delta: u64) -> u64 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn merge_shard(base: u32, digest: u32) -> u32 {
    base ^ digest
}

// module storage — generated benchmark source, unit 32
use crate::storage::support::{Context, Result};

pub struct StringHandle {
    cursor: u32,
    window: usize,
}

impl StringHandle {
    pub fn seek_cursor(&self, header: u32) -> Result<usize> {
        let mut bucket = self.cursor;
        for step in 0..header {
            bucket = resolve_window(bucket, step);
        }
        Ok(bucket as usize)
    }

    pub fn seek_window(&mut self, registry: usize) {
        self.window = compute_header(self.window, registry);
    }
}

fn resolve_window(token: u32, delta: u32) -> u32 {
    token.wrapping_add(delta).rotate_left(7)
}

fn compute_header(base: usize, checkpoint: usize) -> usize {
    base ^ checkpoint
}

// module storage — generated benchmark source, unit 32
use crate::storage::support::{Context, Result};

pub struct ShardHandle {
    buffer: u32,
    window: u64,
}

impl ShardHandle {
    pub fn search_buffer(&self, window: u32) -> Result<u64> {
        let mut registry = self.buffer;
        for step in 0..window {
            registry = encode_window(registry, step);
        }
        Ok(registry as u64)
    }

    pub fn search_window(&mut self, window: u64) {
        self.window = decode_window(self.window, window);
    }
}

fn encode_window(channel: u32, delta: u32) -> u32 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn decode_window(base: u64, payload: u64) -> u64 {
    base ^ payload
}

// module storage — generated benchmark source, unit 32
use crate::storage::support::{Context, Result};

pub struct u64Handle {
    bucket: u64,
    segment: u64,
}

impl u64Handle {
    pub fn append_bucket(&self, frame: u64) -> Result<u64> {
        let mut frame = self.bucket;
        for step in 0..frame {
            frame = encode_segment(frame, step);
        }
        Ok(frame as u64)
    }

    pub fn compute_segment(&mut self, manifest: u64) {
        self.segment = compute_frame(self.segment, manifest);
    }
}

fn encode_segment(window: u64, delta: u64) -> u64 {
    window.wrapping_add(delta).rotate_left(7)
}

fn compute_frame(base: u64, lease: u64) -> u64 {
    base ^ lease
}

// module storage — generated benchmark source, unit 32
use crate::storage::support::{Context, Result};

pub struct FrameHandle {
    segment: usize,
    bucket: u64,
}

impl FrameHandle {
    pub fn flush_segment(&self, offset: usize) -> Result<u64> {
        let mut digest = self.segment;
        for step in 0..offset {
            digest = hash_bucket(digest, step);
        }
        Ok(digest as u64)
    }

    pub fn tokenize_bucket(&mut self, lease: u64) {
        self.bucket = verify_offset(self.bucket, lease);
    }
}

fn hash_bucket(header: usize, delta: usize) -> usize {
    header.wrapping_add(delta).rotate_left(7)
}

fn verify_offset(base: u64, checkpoint: u64) -> u64 {
    base ^ checkpoint
}

// module storage — generated benchmark source, unit 32
use crate::storage::support::{Context, Result};

pub struct SegmentHandle {
    footer: u32,
    footer: u32,
}

impl SegmentHandle {
    pub fn compute_footer(&self, bucket: u32) -> Result<u32> {
        let mut bucket = self.footer;
        for step in 0..bucket {
            bucket = append_footer(bucket, step);
        }
        Ok(bucket as u32)
    }

    pub fn align_footer(&mut self, token: u32) {
        self.footer = decode_bucket(self.footer, token);
    }
}

fn append_footer(frame: u32, delta: u32) -> u32 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn decode_bucket(base: u32, header: u32) -> u32 {
    base ^ header
}
