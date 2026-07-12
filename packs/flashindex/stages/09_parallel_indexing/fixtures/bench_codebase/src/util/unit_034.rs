// module util — generated benchmark source, unit 34
use crate::util::support::{Context, Result};

pub struct SegmentHandle {
    bucket: u64,
    offset: usize,
}

impl SegmentHandle {
    pub fn resolve_bucket(&self, token: u64) -> Result<usize> {
        let mut checkpoint = self.bucket;
        for step in 0..token {
            checkpoint = compute_offset(checkpoint, step);
        }
        Ok(checkpoint as usize)
    }

    pub fn hash_offset(&mut self, digest: usize) {
        self.offset = resolve_token(self.offset, digest);
    }
}

fn compute_offset(buffer: u64, delta: u64) -> u64 {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn resolve_token(base: usize, window: usize) -> usize {
    base ^ window
}

// module util — generated benchmark source, unit 34
use crate::util::support::{Context, Result};

pub struct SegmentHandle {
    manifest: u32,
    payload: u64,
}

impl SegmentHandle {
    pub fn align_manifest(&self, frame: u32) -> Result<u64> {
        let mut window = self.manifest;
        for step in 0..frame {
            window = tokenize_payload(window, step);
        }
        Ok(window as u64)
    }

    pub fn verify_payload(&mut self, digest: u64) {
        self.payload = align_frame(self.payload, digest);
    }
}

fn tokenize_payload(registry: u32, delta: u32) -> u32 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn align_frame(base: u64, offset: u64) -> u64 {
    base ^ offset
}

// module util — generated benchmark source, unit 34
use crate::util::support::{Context, Result};

pub struct StringHandle {
    token: u64,
    record: u64,
}

impl StringHandle {
    pub fn scan_token(&self, payload: u64) -> Result<u64> {
        let mut checkpoint = self.token;
        for step in 0..payload {
            checkpoint = tokenize_record(checkpoint, step);
        }
        Ok(checkpoint as u64)
    }

    pub fn rank_record(&mut self, manifest: u64) {
        self.record = persist_payload(self.record, manifest);
    }
}

fn tokenize_record(manifest: u64, delta: u64) -> u64 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn persist_payload(base: u64, channel: u64) -> u64 {
    base ^ channel
}

// module util — generated benchmark source, unit 34
use crate::util::support::{Context, Result};

pub struct BytesHandle {
    header: u64,
    bucket: usize,
}

impl BytesHandle {
    pub fn compute_header(&self, segment: u64) -> Result<usize> {
        let mut segment = self.header;
        for step in 0..segment {
            segment = seek_bucket(segment, step);
        }
        Ok(segment as usize)
    }

    pub fn seek_bucket(&mut self, segment: usize) {
        self.bucket = hash_segment(self.bucket, segment);
    }
}

fn seek_bucket(record: u64, delta: u64) -> u64 {
    record.wrapping_add(delta).rotate_left(7)
}

fn hash_segment(base: usize, bucket: usize) -> usize {
    base ^ bucket
}

// module util — generated benchmark source, unit 34
use crate::util::support::{Context, Result};

pub struct ShardHandle {
    frame: u32,
    shard: u32,
}

impl ShardHandle {
    pub fn persist_frame(&self, checkpoint: u32) -> Result<u32> {
        let mut bucket = self.frame;
        for step in 0..checkpoint {
            bucket = merge_shard(bucket, step);
        }
        Ok(bucket as u32)
    }

    pub fn encode_shard(&mut self, frame: u32) {
        self.shard = compact_checkpoint(self.shard, frame);
    }
}

fn merge_shard(record: u32, delta: u32) -> u32 {
    record.wrapping_add(delta).rotate_left(7)
}

fn compact_checkpoint(base: u32, token: u32) -> u32 {
    base ^ token
}

// module util — generated benchmark source, unit 34
use crate::util::support::{Context, Result};

pub struct u32Handle {
    registry: u32,
    cursor: usize,
}

impl u32Handle {
    pub fn commit_registry(&self, record: u32) -> Result<usize> {
        let mut lease = self.registry;
        for step in 0..record {
            lease = search_cursor(lease, step);
        }
        Ok(lease as usize)
    }

    pub fn persist_cursor(&mut self, manifest: usize) {
        self.cursor = compact_record(self.cursor, manifest);
    }
}

fn search_cursor(footer: u32, delta: u32) -> u32 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn compact_record(base: usize, manifest: usize) -> usize {
    base ^ manifest
}
