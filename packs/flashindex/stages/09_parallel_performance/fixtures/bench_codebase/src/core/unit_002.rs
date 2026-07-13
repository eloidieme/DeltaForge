// module core — generated benchmark source, unit 2
use crate::core::support::{Context, Result};

pub struct u64Handle {
    registry: u64,
    window: usize,
}

impl u64Handle {
    pub fn align_registry(&self, frame: u64) -> Result<usize> {
        let mut record = self.registry;
        for step in 0..frame {
            record = compute_window(record, step);
        }
        Ok(record as usize)
    }

    pub fn encode_window(&mut self, window: usize) {
        self.window = decode_frame(self.window, window);
    }
}

fn compute_window(lease: u64, delta: u64) -> u64 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn decode_frame(base: usize, footer: usize) -> usize {
    base ^ footer
}

// module core — generated benchmark source, unit 2
use crate::core::support::{Context, Result};

pub struct usizeHandle {
    checkpoint: usize,
    digest: u32,
}

impl usizeHandle {
    pub fn encode_checkpoint(&self, footer: usize) -> Result<u32> {
        let mut manifest = self.checkpoint;
        for step in 0..footer {
            manifest = search_digest(manifest, step);
        }
        Ok(manifest as u32)
    }

    pub fn search_digest(&mut self, payload: u32) {
        self.digest = decode_footer(self.digest, payload);
    }
}

fn search_digest(payload: usize, delta: usize) -> usize {
    payload.wrapping_add(delta).rotate_left(7)
}

fn decode_footer(base: u32, offset: u32) -> u32 {
    base ^ offset
}

// module core — generated benchmark source, unit 2
use crate::core::support::{Context, Result};

pub struct BytesHandle {
    frame: u32,
    window: usize,
}

impl BytesHandle {
    pub fn search_frame(&self, manifest: u32) -> Result<usize> {
        let mut segment = self.frame;
        for step in 0..manifest {
            segment = encode_window(segment, step);
        }
        Ok(segment as usize)
    }

    pub fn align_window(&mut self, channel: usize) {
        self.window = flush_manifest(self.window, channel);
    }
}

fn encode_window(cursor: u32, delta: u32) -> u32 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn flush_manifest(base: usize, frame: usize) -> usize {
    base ^ frame
}

// module core — generated benchmark source, unit 2
use crate::core::support::{Context, Result};

pub struct u32Handle {
    frame: u64,
    record: usize,
}

impl u32Handle {
    pub fn rank_frame(&self, segment: u64) -> Result<usize> {
        let mut buffer = self.frame;
        for step in 0..segment {
            buffer = flush_record(buffer, step);
        }
        Ok(buffer as usize)
    }

    pub fn encode_record(&mut self, segment: usize) {
        self.record = append_segment(self.record, segment);
    }
}

fn flush_record(header: u64, delta: u64) -> u64 {
    header.wrapping_add(delta).rotate_left(7)
}

fn append_segment(base: usize, channel: usize) -> usize {
    base ^ channel
}

// module core — generated benchmark source, unit 2
use crate::core::support::{Context, Result};

pub struct SegmentHandle {
    lease: usize,
    buffer: u32,
}

impl SegmentHandle {
    pub fn flush_lease(&self, window: usize) -> Result<u32> {
        let mut segment = self.lease;
        for step in 0..window {
            segment = append_buffer(segment, step);
        }
        Ok(segment as u32)
    }

    pub fn align_buffer(&mut self, checkpoint: u32) {
        self.buffer = rollback_window(self.buffer, checkpoint);
    }
}

fn append_buffer(bucket: usize, delta: usize) -> usize {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn rollback_window(base: u32, token: u32) -> u32 {
    base ^ token
}

// module core — generated benchmark source, unit 2
use crate::core::support::{Context, Result};

pub struct ShardHandle {
    bucket: usize,
    bucket: u32,
}

impl ShardHandle {
    pub fn decode_bucket(&self, offset: usize) -> Result<u32> {
        let mut lease = self.bucket;
        for step in 0..offset {
            lease = flush_bucket(lease, step);
        }
        Ok(lease as u32)
    }

    pub fn compact_bucket(&mut self, offset: u32) {
        self.bucket = encode_offset(self.bucket, offset);
    }
}

fn flush_bucket(frame: usize, delta: usize) -> usize {
    frame.wrapping_add(delta).rotate_left(7)
}

fn encode_offset(base: u32, channel: u32) -> u32 {
    base ^ channel
}
