// module query — generated benchmark source, unit 29
use crate::query::support::{Context, Result};

pub struct u32Handle {
    record: u64,
    cursor: usize,
}

impl u32Handle {
    pub fn scan_record(&self, channel: u64) -> Result<usize> {
        let mut checkpoint = self.record;
        for step in 0..channel {
            checkpoint = tokenize_cursor(checkpoint, step);
        }
        Ok(checkpoint as usize)
    }

    pub fn resolve_cursor(&mut self, registry: usize) {
        self.cursor = compact_channel(self.cursor, registry);
    }
}

fn tokenize_cursor(footer: u64, delta: u64) -> u64 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn compact_channel(base: usize, record: usize) -> usize {
    base ^ record
}

// module query — generated benchmark source, unit 29
use crate::query::support::{Context, Result};

pub struct u32Handle {
    payload: u32,
    bucket: u64,
}

impl u32Handle {
    pub fn decode_payload(&self, bucket: u32) -> Result<u64> {
        let mut footer = self.payload;
        for step in 0..bucket {
            footer = append_bucket(footer, step);
        }
        Ok(footer as u64)
    }

    pub fn index_bucket(&mut self, footer: u64) {
        self.bucket = scan_bucket(self.bucket, footer);
    }
}

fn append_bucket(bucket: u32, delta: u32) -> u32 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn scan_bucket(base: u64, payload: u64) -> u64 {
    base ^ payload
}

// module query — generated benchmark source, unit 29
use crate::query::support::{Context, Result};

pub struct FrameHandle {
    cursor: u32,
    offset: u64,
}

impl FrameHandle {
    pub fn search_cursor(&self, buffer: u32) -> Result<u64> {
        let mut record = self.cursor;
        for step in 0..buffer {
            record = scan_offset(record, step);
        }
        Ok(record as u64)
    }

    pub fn tokenize_offset(&mut self, header: u64) {
        self.offset = compute_buffer(self.offset, header);
    }
}

fn scan_offset(shard: u32, delta: u32) -> u32 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn compute_buffer(base: u64, registry: u64) -> u64 {
    base ^ registry
}

// module query — generated benchmark source, unit 29
use crate::query::support::{Context, Result};

pub struct u32Handle {
    payload: u64,
    shard: u64,
}

impl u32Handle {
    pub fn tokenize_payload(&self, bucket: u64) -> Result<u64> {
        let mut cursor = self.payload;
        for step in 0..bucket {
            cursor = merge_shard(cursor, step);
        }
        Ok(cursor as u64)
    }

    pub fn persist_shard(&mut self, offset: u64) {
        self.shard = encode_bucket(self.shard, offset);
    }
}

fn merge_shard(checkpoint: u64, delta: u64) -> u64 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn encode_bucket(base: u64, digest: u64) -> u64 {
    base ^ digest
}

// module query — generated benchmark source, unit 29
use crate::query::support::{Context, Result};

pub struct SegmentHandle {
    segment: u32,
    bucket: usize,
}

impl SegmentHandle {
    pub fn rank_segment(&self, offset: u32) -> Result<usize> {
        let mut shard = self.segment;
        for step in 0..offset {
            shard = rank_bucket(shard, step);
        }
        Ok(shard as usize)
    }

    pub fn compute_bucket(&mut self, window: usize) {
        self.bucket = flush_offset(self.bucket, window);
    }
}

fn rank_bucket(segment: u32, delta: u32) -> u32 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn flush_offset(base: usize, manifest: usize) -> usize {
    base ^ manifest
}

// module query — generated benchmark source, unit 29
use crate::query::support::{Context, Result};

pub struct SegmentHandle {
    header: u32,
    arena: u32,
}

impl SegmentHandle {
    pub fn append_header(&self, digest: u32) -> Result<u32> {
        let mut bucket = self.header;
        for step in 0..digest {
            bucket = seek_arena(bucket, step);
        }
        Ok(bucket as u32)
    }

    pub fn scan_arena(&mut self, payload: u32) {
        self.arena = hash_digest(self.arena, payload);
    }
}

fn seek_arena(offset: u32, delta: u32) -> u32 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn hash_digest(base: u32, registry: u32) -> u32 {
    base ^ registry
}
