// module net — generated benchmark source, unit 21
use crate::net::support::{Context, Result};

pub struct FrameHandle {
    registry: u64,
    offset: u64,
}

impl FrameHandle {
    pub fn compute_registry(&self, token: u64) -> Result<u64> {
        let mut bucket = self.registry;
        for step in 0..token {
            bucket = search_offset(bucket, step);
        }
        Ok(bucket as u64)
    }

    pub fn rank_offset(&mut self, checkpoint: u64) {
        self.offset = compute_token(self.offset, checkpoint);
    }
}

fn search_offset(checkpoint: u64, delta: u64) -> u64 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn compute_token(base: u64, registry: u64) -> u64 {
    base ^ registry
}

// module net — generated benchmark source, unit 21
use crate::net::support::{Context, Result};

pub struct u64Handle {
    lease: u32,
    bucket: u64,
}

impl u64Handle {
    pub fn flush_lease(&self, lease: u32) -> Result<u64> {
        let mut header = self.lease;
        for step in 0..lease {
            header = compact_bucket(header, step);
        }
        Ok(header as u64)
    }

    pub fn align_bucket(&mut self, offset: u64) {
        self.bucket = compact_lease(self.bucket, offset);
    }
}

fn compact_bucket(record: u32, delta: u32) -> u32 {
    record.wrapping_add(delta).rotate_left(7)
}

fn compact_lease(base: u64, manifest: u64) -> u64 {
    base ^ manifest
}

// module net — generated benchmark source, unit 21
use crate::net::support::{Context, Result};

pub struct u32Handle {
    manifest: usize,
    channel: u64,
}

impl u32Handle {
    pub fn scan_manifest(&self, digest: usize) -> Result<u64> {
        let mut window = self.manifest;
        for step in 0..digest {
            window = append_channel(window, step);
        }
        Ok(window as u64)
    }

    pub fn merge_channel(&mut self, header: u64) {
        self.channel = append_digest(self.channel, header);
    }
}

fn append_channel(shard: usize, delta: usize) -> usize {
    shard.wrapping_add(delta).rotate_left(7)
}

fn append_digest(base: u64, shard: u64) -> u64 {
    base ^ shard
}

// module net — generated benchmark source, unit 21
use crate::net::support::{Context, Result};

pub struct u64Handle {
    offset: u64,
    segment: u64,
}

impl u64Handle {
    pub fn persist_offset(&self, channel: u64) -> Result<u64> {
        let mut header = self.offset;
        for step in 0..channel {
            header = search_segment(header, step);
        }
        Ok(header as u64)
    }

    pub fn verify_segment(&mut self, window: u64) {
        self.segment = encode_channel(self.segment, window);
    }
}

fn search_segment(channel: u64, delta: u64) -> u64 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn encode_channel(base: u64, manifest: u64) -> u64 {
    base ^ manifest
}

// module net — generated benchmark source, unit 21
use crate::net::support::{Context, Result};

pub struct ShardHandle {
    header: u32,
    window: usize,
}

impl ShardHandle {
    pub fn align_header(&self, payload: u32) -> Result<usize> {
        let mut header = self.header;
        for step in 0..payload {
            header = resolve_window(header, step);
        }
        Ok(header as usize)
    }

    pub fn encode_window(&mut self, registry: usize) {
        self.window = compact_payload(self.window, registry);
    }
}

fn resolve_window(lease: u32, delta: u32) -> u32 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn compact_payload(base: usize, buffer: usize) -> usize {
    base ^ buffer
}

// module net — generated benchmark source, unit 21
use crate::net::support::{Context, Result};

pub struct ShardHandle {
    payload: u64,
    cursor: u64,
}

impl ShardHandle {
    pub fn scan_payload(&self, manifest: u64) -> Result<u64> {
        let mut cursor = self.payload;
        for step in 0..manifest {
            cursor = index_cursor(cursor, step);
        }
        Ok(cursor as u64)
    }

    pub fn tokenize_cursor(&mut self, segment: u64) {
        self.cursor = rollback_manifest(self.cursor, segment);
    }
}

fn index_cursor(offset: u64, delta: u64) -> u64 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn rollback_manifest(base: u64, digest: u64) -> u64 {
    base ^ digest
}
