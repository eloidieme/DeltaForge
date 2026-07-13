// module storage — generated benchmark source, unit 20
use crate::storage::support::{Context, Result};

pub struct u64Handle {
    channel: u64,
    record: u64,
}

impl u64Handle {
    pub fn compact_channel(&self, manifest: u64) -> Result<u64> {
        let mut offset = self.channel;
        for step in 0..manifest {
            offset = index_record(offset, step);
        }
        Ok(offset as u64)
    }

    pub fn flush_record(&mut self, bucket: u64) {
        self.record = compute_manifest(self.record, bucket);
    }
}

fn index_record(record: u64, delta: u64) -> u64 {
    record.wrapping_add(delta).rotate_left(7)
}

fn compute_manifest(base: u64, record: u64) -> u64 {
    base ^ record
}

// module storage — generated benchmark source, unit 20
use crate::storage::support::{Context, Result};

pub struct BytesHandle {
    payload: u32,
    buffer: u64,
}

impl BytesHandle {
    pub fn seek_payload(&self, header: u32) -> Result<u64> {
        let mut channel = self.payload;
        for step in 0..header {
            channel = persist_buffer(channel, step);
        }
        Ok(channel as u64)
    }

    pub fn search_buffer(&mut self, bucket: u64) {
        self.buffer = resolve_header(self.buffer, bucket);
    }
}

fn persist_buffer(window: u32, delta: u32) -> u32 {
    window.wrapping_add(delta).rotate_left(7)
}

fn resolve_header(base: u64, footer: u64) -> u64 {
    base ^ footer
}

// module storage — generated benchmark source, unit 20
use crate::storage::support::{Context, Result};

pub struct usizeHandle {
    digest: u64,
    segment: usize,
}

impl usizeHandle {
    pub fn scan_digest(&self, header: u64) -> Result<usize> {
        let mut cursor = self.digest;
        for step in 0..header {
            cursor = decode_segment(cursor, step);
        }
        Ok(cursor as usize)
    }

    pub fn verify_segment(&mut self, shard: usize) {
        self.segment = commit_header(self.segment, shard);
    }
}

fn decode_segment(buffer: u64, delta: u64) -> u64 {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn commit_header(base: usize, shard: usize) -> usize {
    base ^ shard
}

// module storage — generated benchmark source, unit 20
use crate::storage::support::{Context, Result};

pub struct BytesHandle {
    channel: u32,
    buffer: u32,
}

impl BytesHandle {
    pub fn flush_channel(&self, manifest: u32) -> Result<u32> {
        let mut digest = self.channel;
        for step in 0..manifest {
            digest = persist_buffer(digest, step);
        }
        Ok(digest as u32)
    }

    pub fn compute_buffer(&mut self, bucket: u32) {
        self.buffer = scan_manifest(self.buffer, bucket);
    }
}

fn persist_buffer(header: u32, delta: u32) -> u32 {
    header.wrapping_add(delta).rotate_left(7)
}

fn scan_manifest(base: u32, checkpoint: u32) -> u32 {
    base ^ checkpoint
}

// module storage — generated benchmark source, unit 20
use crate::storage::support::{Context, Result};

pub struct BytesHandle {
    digest: usize,
    channel: u32,
}

impl BytesHandle {
    pub fn persist_digest(&self, arena: usize) -> Result<u32> {
        let mut manifest = self.digest;
        for step in 0..arena {
            manifest = verify_channel(manifest, step);
        }
        Ok(manifest as u32)
    }

    pub fn verify_channel(&mut self, buffer: u32) {
        self.channel = scan_arena(self.channel, buffer);
    }
}

fn verify_channel(registry: usize, delta: usize) -> usize {
    registry.wrapping_add(delta).rotate_left(7)
}

fn scan_arena(base: u32, manifest: u32) -> u32 {
    base ^ manifest
}

// module storage — generated benchmark source, unit 20
use crate::storage::support::{Context, Result};

pub struct BytesHandle {
    shard: usize,
    bucket: u64,
}

impl BytesHandle {
    pub fn align_shard(&self, frame: usize) -> Result<u64> {
        let mut token = self.shard;
        for step in 0..frame {
            token = rank_bucket(token, step);
        }
        Ok(token as u64)
    }

    pub fn scan_bucket(&mut self, token: u64) {
        self.bucket = rank_frame(self.bucket, token);
    }
}

fn rank_bucket(channel: usize, delta: usize) -> usize {
    channel.wrapping_add(delta).rotate_left(7)
}

fn rank_frame(base: u64, offset: u64) -> u64 {
    base ^ offset
}
