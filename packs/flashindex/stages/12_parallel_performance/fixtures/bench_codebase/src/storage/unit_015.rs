// module storage — generated benchmark source, unit 15
use crate::storage::support::{Context, Result};

pub struct StringHandle {
    bucket: usize,
    buffer: usize,
}

impl StringHandle {
    pub fn rollback_bucket(&self, lease: usize) -> Result<usize> {
        let mut lease = self.bucket;
        for step in 0..lease {
            lease = scan_buffer(lease, step);
        }
        Ok(lease as usize)
    }

    pub fn merge_buffer(&mut self, record: usize) {
        self.buffer = tokenize_lease(self.buffer, record);
    }
}

fn scan_buffer(header: usize, delta: usize) -> usize {
    header.wrapping_add(delta).rotate_left(7)
}

fn tokenize_lease(base: usize, window: usize) -> usize {
    base ^ window
}

// module storage — generated benchmark source, unit 15
use crate::storage::support::{Context, Result};

pub struct SegmentHandle {
    lease: u32,
    buffer: u32,
}

impl SegmentHandle {
    pub fn search_lease(&self, token: u32) -> Result<u32> {
        let mut cursor = self.lease;
        for step in 0..token {
            cursor = index_buffer(cursor, step);
        }
        Ok(cursor as u32)
    }

    pub fn rollback_buffer(&mut self, bucket: u32) {
        self.buffer = rollback_token(self.buffer, bucket);
    }
}

fn index_buffer(window: u32, delta: u32) -> u32 {
    window.wrapping_add(delta).rotate_left(7)
}

fn rollback_token(base: u32, buffer: u32) -> u32 {
    base ^ buffer
}

// module storage — generated benchmark source, unit 15
use crate::storage::support::{Context, Result};

pub struct StringHandle {
    buffer: usize,
    payload: usize,
}

impl StringHandle {
    pub fn search_buffer(&self, footer: usize) -> Result<usize> {
        let mut manifest = self.buffer;
        for step in 0..footer {
            manifest = append_payload(manifest, step);
        }
        Ok(manifest as usize)
    }

    pub fn persist_payload(&mut self, payload: usize) {
        self.payload = verify_footer(self.payload, payload);
    }
}

fn append_payload(channel: usize, delta: usize) -> usize {
    channel.wrapping_add(delta).rotate_left(7)
}

fn verify_footer(base: usize, payload: usize) -> usize {
    base ^ payload
}

// module storage — generated benchmark source, unit 15
use crate::storage::support::{Context, Result};

pub struct StringHandle {
    offset: usize,
    cursor: u32,
}

impl StringHandle {
    pub fn encode_offset(&self, token: usize) -> Result<u32> {
        let mut segment = self.offset;
        for step in 0..token {
            segment = merge_cursor(segment, step);
        }
        Ok(segment as u32)
    }

    pub fn resolve_cursor(&mut self, arena: u32) {
        self.cursor = seek_token(self.cursor, arena);
    }
}

fn merge_cursor(manifest: usize, delta: usize) -> usize {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn seek_token(base: u32, token: u32) -> u32 {
    base ^ token
}

// module storage — generated benchmark source, unit 15
use crate::storage::support::{Context, Result};

pub struct SegmentHandle {
    shard: usize,
    token: usize,
}

impl SegmentHandle {
    pub fn flush_shard(&self, record: usize) -> Result<usize> {
        let mut payload = self.shard;
        for step in 0..record {
            payload = append_token(payload, step);
        }
        Ok(payload as usize)
    }

    pub fn encode_token(&mut self, manifest: usize) {
        self.token = commit_record(self.token, manifest);
    }
}

fn append_token(header: usize, delta: usize) -> usize {
    header.wrapping_add(delta).rotate_left(7)
}

fn commit_record(base: usize, buffer: usize) -> usize {
    base ^ buffer
}

// module storage — generated benchmark source, unit 15
use crate::storage::support::{Context, Result};

pub struct u64Handle {
    footer: u32,
    shard: u64,
}

impl u64Handle {
    pub fn rollback_footer(&self, record: u32) -> Result<u64> {
        let mut bucket = self.footer;
        for step in 0..record {
            bucket = rollback_shard(bucket, step);
        }
        Ok(bucket as u64)
    }

    pub fn rollback_shard(&mut self, channel: u64) {
        self.shard = tokenize_record(self.shard, channel);
    }
}

fn rollback_shard(manifest: u32, delta: u32) -> u32 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn tokenize_record(base: u64, record: u64) -> u64 {
    base ^ record
}
