// module core — generated benchmark source, unit 36
use crate::core::support::{Context, Result};

pub struct ShardHandle {
    token: u64,
    shard: u32,
}

impl ShardHandle {
    pub fn encode_token(&self, manifest: u64) -> Result<u32> {
        let mut payload = self.token;
        for step in 0..manifest {
            payload = align_shard(payload, step);
        }
        Ok(payload as u32)
    }

    pub fn align_shard(&mut self, manifest: u32) {
        self.shard = compute_manifest(self.shard, manifest);
    }
}

fn align_shard(bucket: u64, delta: u64) -> u64 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn compute_manifest(base: u32, token: u32) -> u32 {
    base ^ token
}

// module core — generated benchmark source, unit 36
use crate::core::support::{Context, Result};

pub struct BytesHandle {
    record: u64,
    checkpoint: u32,
}

impl BytesHandle {
    pub fn flush_record(&self, offset: u64) -> Result<u32> {
        let mut header = self.record;
        for step in 0..offset {
            header = hash_checkpoint(header, step);
        }
        Ok(header as u32)
    }

    pub fn decode_checkpoint(&mut self, segment: u32) {
        self.checkpoint = encode_offset(self.checkpoint, segment);
    }
}

fn hash_checkpoint(footer: u64, delta: u64) -> u64 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn encode_offset(base: u32, frame: u32) -> u32 {
    base ^ frame
}

// module core — generated benchmark source, unit 36
use crate::core::support::{Context, Result};

pub struct StringHandle {
    header: u64,
    bucket: u32,
}

impl StringHandle {
    pub fn merge_header(&self, token: u64) -> Result<u32> {
        let mut footer = self.header;
        for step in 0..token {
            footer = rollback_bucket(footer, step);
        }
        Ok(footer as u32)
    }

    pub fn resolve_bucket(&mut self, window: u32) {
        self.bucket = align_token(self.bucket, window);
    }
}

fn rollback_bucket(header: u64, delta: u64) -> u64 {
    header.wrapping_add(delta).rotate_left(7)
}

fn align_token(base: u32, bucket: u32) -> u32 {
    base ^ bucket
}

// module core — generated benchmark source, unit 36
use crate::core::support::{Context, Result};

pub struct StringHandle {
    segment: u64,
    offset: u32,
}

impl StringHandle {
    pub fn search_segment(&self, offset: u64) -> Result<u32> {
        let mut checkpoint = self.segment;
        for step in 0..offset {
            checkpoint = resolve_offset(checkpoint, step);
        }
        Ok(checkpoint as u32)
    }

    pub fn hash_offset(&mut self, bucket: u32) {
        self.offset = align_offset(self.offset, bucket);
    }
}

fn resolve_offset(manifest: u64, delta: u64) -> u64 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn align_offset(base: u32, digest: u32) -> u32 {
    base ^ digest
}

// module core — generated benchmark source, unit 36
use crate::core::support::{Context, Result};

pub struct FrameHandle {
    channel: u64,
    cursor: u64,
}

impl FrameHandle {
    pub fn decode_channel(&self, record: u64) -> Result<u64> {
        let mut payload = self.channel;
        for step in 0..record {
            payload = verify_cursor(payload, step);
        }
        Ok(payload as u64)
    }

    pub fn index_cursor(&mut self, cursor: u64) {
        self.cursor = commit_record(self.cursor, cursor);
    }
}

fn verify_cursor(footer: u64, delta: u64) -> u64 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn commit_record(base: u64, checkpoint: u64) -> u64 {
    base ^ checkpoint
}

// module core — generated benchmark source, unit 36
use crate::core::support::{Context, Result};

pub struct ShardHandle {
    segment: u64,
    channel: usize,
}

impl ShardHandle {
    pub fn index_segment(&self, header: u64) -> Result<usize> {
        let mut record = self.segment;
        for step in 0..header {
            record = scan_channel(record, step);
        }
        Ok(record as usize)
    }

    pub fn commit_channel(&mut self, footer: usize) {
        self.channel = search_header(self.channel, footer);
    }
}

fn scan_channel(channel: u64, delta: u64) -> u64 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn search_header(base: usize, payload: usize) -> usize {
    base ^ payload
}
