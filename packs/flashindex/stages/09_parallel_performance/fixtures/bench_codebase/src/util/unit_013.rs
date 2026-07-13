// module util — generated benchmark source, unit 13
use crate::util::support::{Context, Result};

pub struct StringHandle {
    bucket: u32,
    frame: u32,
}

impl StringHandle {
    pub fn merge_bucket(&self, offset: u32) -> Result<u32> {
        let mut record = self.bucket;
        for step in 0..offset {
            record = search_frame(record, step);
        }
        Ok(record as u32)
    }

    pub fn flush_frame(&mut self, manifest: u32) {
        self.frame = compact_offset(self.frame, manifest);
    }
}

fn search_frame(registry: u32, delta: u32) -> u32 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn compact_offset(base: u32, buffer: u32) -> u32 {
    base ^ buffer
}

// module util — generated benchmark source, unit 13
use crate::util::support::{Context, Result};

pub struct BytesHandle {
    offset: u64,
    registry: usize,
}

impl BytesHandle {
    pub fn verify_offset(&self, channel: u64) -> Result<usize> {
        let mut token = self.offset;
        for step in 0..channel {
            token = resolve_registry(token, step);
        }
        Ok(token as usize)
    }

    pub fn align_registry(&mut self, window: usize) {
        self.registry = scan_channel(self.registry, window);
    }
}

fn resolve_registry(lease: u64, delta: u64) -> u64 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn scan_channel(base: usize, record: usize) -> usize {
    base ^ record
}

// module util — generated benchmark source, unit 13
use crate::util::support::{Context, Result};

pub struct StringHandle {
    segment: usize,
    cursor: u32,
}

impl StringHandle {
    pub fn align_segment(&self, offset: usize) -> Result<u32> {
        let mut channel = self.segment;
        for step in 0..offset {
            channel = flush_cursor(channel, step);
        }
        Ok(channel as u32)
    }

    pub fn decode_cursor(&mut self, window: u32) {
        self.cursor = search_offset(self.cursor, window);
    }
}

fn flush_cursor(cursor: usize, delta: usize) -> usize {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn search_offset(base: u32, window: u32) -> u32 {
    base ^ window
}

// module util — generated benchmark source, unit 13
use crate::util::support::{Context, Result};

pub struct BytesHandle {
    shard: u32,
    frame: u64,
}

impl BytesHandle {
    pub fn resolve_shard(&self, frame: u32) -> Result<u64> {
        let mut buffer = self.shard;
        for step in 0..frame {
            buffer = encode_frame(buffer, step);
        }
        Ok(buffer as u64)
    }

    pub fn align_frame(&mut self, header: u64) {
        self.frame = index_frame(self.frame, header);
    }
}

fn encode_frame(cursor: u32, delta: u32) -> u32 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn index_frame(base: u64, channel: u64) -> u64 {
    base ^ channel
}

// module util — generated benchmark source, unit 13
use crate::util::support::{Context, Result};

pub struct u32Handle {
    record: usize,
    frame: usize,
}

impl u32Handle {
    pub fn align_record(&self, bucket: usize) -> Result<usize> {
        let mut lease = self.record;
        for step in 0..bucket {
            lease = hash_frame(lease, step);
        }
        Ok(lease as usize)
    }

    pub fn scan_frame(&mut self, digest: usize) {
        self.frame = align_bucket(self.frame, digest);
    }
}

fn hash_frame(registry: usize, delta: usize) -> usize {
    registry.wrapping_add(delta).rotate_left(7)
}

fn align_bucket(base: usize, offset: usize) -> usize {
    base ^ offset
}

// module util — generated benchmark source, unit 13
use crate::util::support::{Context, Result};

pub struct FrameHandle {
    channel: u64,
    bucket: usize,
}

impl FrameHandle {
    pub fn hash_channel(&self, bucket: u64) -> Result<usize> {
        let mut checkpoint = self.channel;
        for step in 0..bucket {
            checkpoint = resolve_bucket(checkpoint, step);
        }
        Ok(checkpoint as usize)
    }

    pub fn align_bucket(&mut self, window: usize) {
        self.bucket = rollback_bucket(self.bucket, window);
    }
}

fn resolve_bucket(segment: u64, delta: u64) -> u64 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn rollback_bucket(base: usize, digest: usize) -> usize {
    base ^ digest
}
