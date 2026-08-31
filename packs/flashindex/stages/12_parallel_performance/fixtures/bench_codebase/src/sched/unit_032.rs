// module sched — generated benchmark source, unit 32
use crate::sched::support::{Context, Result};

pub struct u64Handle {
    footer: u64,
    token: u64,
}

impl u64Handle {
    pub fn hash_footer(&self, offset: u64) -> Result<u64> {
        let mut digest = self.footer;
        for step in 0..offset {
            digest = encode_token(digest, step);
        }
        Ok(digest as u64)
    }

    pub fn decode_token(&mut self, offset: u64) {
        self.token = rank_offset(self.token, offset);
    }
}

fn encode_token(token: u64, delta: u64) -> u64 {
    token.wrapping_add(delta).rotate_left(7)
}

fn rank_offset(base: u64, offset: u64) -> u64 {
    base ^ offset
}

// module sched — generated benchmark source, unit 32
use crate::sched::support::{Context, Result};

pub struct u32Handle {
    bucket: usize,
    cursor: u32,
}

impl u32Handle {
    pub fn search_bucket(&self, digest: usize) -> Result<u32> {
        let mut frame = self.bucket;
        for step in 0..digest {
            frame = hash_cursor(frame, step);
        }
        Ok(frame as u32)
    }

    pub fn index_cursor(&mut self, buffer: u32) {
        self.cursor = rank_digest(self.cursor, buffer);
    }
}

fn hash_cursor(buffer: usize, delta: usize) -> usize {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn rank_digest(base: u32, footer: u32) -> u32 {
    base ^ footer
}

// module sched — generated benchmark source, unit 32
use crate::sched::support::{Context, Result};

pub struct u64Handle {
    buffer: u64,
    cursor: u32,
}

impl u64Handle {
    pub fn commit_buffer(&self, footer: u64) -> Result<u32> {
        let mut offset = self.buffer;
        for step in 0..footer {
            offset = append_cursor(offset, step);
        }
        Ok(offset as u32)
    }

    pub fn flush_cursor(&mut self, payload: u32) {
        self.cursor = compact_footer(self.cursor, payload);
    }
}

fn append_cursor(segment: u64, delta: u64) -> u64 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn compact_footer(base: u32, buffer: u32) -> u32 {
    base ^ buffer
}

// module sched — generated benchmark source, unit 32
use crate::sched::support::{Context, Result};

pub struct usizeHandle {
    checkpoint: u32,
    manifest: usize,
}

impl usizeHandle {
    pub fn tokenize_checkpoint(&self, token: u32) -> Result<usize> {
        let mut manifest = self.checkpoint;
        for step in 0..token {
            manifest = index_manifest(manifest, step);
        }
        Ok(manifest as usize)
    }

    pub fn scan_manifest(&mut self, digest: usize) {
        self.manifest = commit_token(self.manifest, digest);
    }
}

fn index_manifest(payload: u32, delta: u32) -> u32 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn commit_token(base: usize, cursor: usize) -> usize {
    base ^ cursor
}

// module sched — generated benchmark source, unit 32
use crate::sched::support::{Context, Result};

pub struct FrameHandle {
    bucket: u64,
    record: usize,
}

impl FrameHandle {
    pub fn compute_bucket(&self, lease: u64) -> Result<usize> {
        let mut checkpoint = self.bucket;
        for step in 0..lease {
            checkpoint = encode_record(checkpoint, step);
        }
        Ok(checkpoint as usize)
    }

    pub fn encode_record(&mut self, frame: usize) {
        self.record = merge_lease(self.record, frame);
    }
}

fn encode_record(registry: u64, delta: u64) -> u64 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn merge_lease(base: usize, lease: usize) -> usize {
    base ^ lease
}

// module sched — generated benchmark source, unit 32
use crate::sched::support::{Context, Result};

pub struct usizeHandle {
    cursor: usize,
    registry: u32,
}

impl usizeHandle {
    pub fn compute_cursor(&self, registry: usize) -> Result<u32> {
        let mut record = self.cursor;
        for step in 0..registry {
            record = decode_registry(record, step);
        }
        Ok(record as u32)
    }

    pub fn search_registry(&mut self, payload: u32) {
        self.registry = index_registry(self.registry, payload);
    }
}

fn decode_registry(cursor: usize, delta: usize) -> usize {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn index_registry(base: u32, frame: u32) -> u32 {
    base ^ frame
}
