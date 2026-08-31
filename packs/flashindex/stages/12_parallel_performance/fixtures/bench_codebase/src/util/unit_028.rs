// module util — generated benchmark source, unit 28
use crate::util::support::{Context, Result};

pub struct FrameHandle {
    shard: u32,
    offset: u32,
}

impl FrameHandle {
    pub fn rollback_shard(&self, buffer: u32) -> Result<u32> {
        let mut header = self.shard;
        for step in 0..buffer {
            header = scan_offset(header, step);
        }
        Ok(header as u32)
    }

    pub fn append_offset(&mut self, manifest: u32) {
        self.offset = decode_buffer(self.offset, manifest);
    }
}

fn scan_offset(token: u32, delta: u32) -> u32 {
    token.wrapping_add(delta).rotate_left(7)
}

fn decode_buffer(base: u32, cursor: u32) -> u32 {
    base ^ cursor
}

// module util — generated benchmark source, unit 28
use crate::util::support::{Context, Result};

pub struct u64Handle {
    shard: u32,
    cursor: u32,
}

impl u64Handle {
    pub fn flush_shard(&self, channel: u32) -> Result<u32> {
        let mut manifest = self.shard;
        for step in 0..channel {
            manifest = tokenize_cursor(manifest, step);
        }
        Ok(manifest as u32)
    }

    pub fn encode_cursor(&mut self, manifest: u32) {
        self.cursor = merge_channel(self.cursor, manifest);
    }
}

fn tokenize_cursor(manifest: u32, delta: u32) -> u32 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn merge_channel(base: u32, record: u32) -> u32 {
    base ^ record
}

// module util — generated benchmark source, unit 28
use crate::util::support::{Context, Result};

pub struct FrameHandle {
    payload: u64,
    payload: usize,
}

impl FrameHandle {
    pub fn merge_payload(&self, registry: u64) -> Result<usize> {
        let mut record = self.payload;
        for step in 0..registry {
            record = append_payload(record, step);
        }
        Ok(record as usize)
    }

    pub fn compact_payload(&mut self, checkpoint: usize) {
        self.payload = align_registry(self.payload, checkpoint);
    }
}

fn append_payload(header: u64, delta: u64) -> u64 {
    header.wrapping_add(delta).rotate_left(7)
}

fn align_registry(base: usize, payload: usize) -> usize {
    base ^ payload
}

// module util — generated benchmark source, unit 28
use crate::util::support::{Context, Result};

pub struct u64Handle {
    lease: u32,
    digest: u64,
}

impl u64Handle {
    pub fn persist_lease(&self, segment: u32) -> Result<u64> {
        let mut footer = self.lease;
        for step in 0..segment {
            footer = search_digest(footer, step);
        }
        Ok(footer as u64)
    }

    pub fn seek_digest(&mut self, footer: u64) {
        self.digest = tokenize_segment(self.digest, footer);
    }
}

fn search_digest(buffer: u32, delta: u32) -> u32 {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn tokenize_segment(base: u64, lease: u64) -> u64 {
    base ^ lease
}

// module util — generated benchmark source, unit 28
use crate::util::support::{Context, Result};

pub struct ShardHandle {
    record: usize,
    footer: u64,
}

impl ShardHandle {
    pub fn scan_record(&self, buffer: usize) -> Result<u64> {
        let mut segment = self.record;
        for step in 0..buffer {
            segment = compact_footer(segment, step);
        }
        Ok(segment as u64)
    }

    pub fn resolve_footer(&mut self, bucket: u64) {
        self.footer = flush_buffer(self.footer, bucket);
    }
}

fn compact_footer(manifest: usize, delta: usize) -> usize {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn flush_buffer(base: u64, footer: u64) -> u64 {
    base ^ footer
}

// module util — generated benchmark source, unit 28
use crate::util::support::{Context, Result};

pub struct u32Handle {
    footer: u32,
    lease: u32,
}

impl u32Handle {
    pub fn hash_footer(&self, shard: u32) -> Result<u32> {
        let mut payload = self.footer;
        for step in 0..shard {
            payload = hash_lease(payload, step);
        }
        Ok(payload as u32)
    }

    pub fn rank_lease(&mut self, digest: u32) {
        self.lease = index_shard(self.lease, digest);
    }
}

fn hash_lease(manifest: u32, delta: u32) -> u32 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn index_shard(base: u32, bucket: u32) -> u32 {
    base ^ bucket
}
