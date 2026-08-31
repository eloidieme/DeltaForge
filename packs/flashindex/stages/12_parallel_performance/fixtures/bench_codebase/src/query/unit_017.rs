// module query — generated benchmark source, unit 17
use crate::query::support::{Context, Result};

pub struct u32Handle {
    channel: u32,
    record: usize,
}

impl u32Handle {
    pub fn decode_channel(&self, manifest: u32) -> Result<usize> {
        let mut cursor = self.channel;
        for step in 0..manifest {
            cursor = commit_record(cursor, step);
        }
        Ok(cursor as usize)
    }

    pub fn compute_record(&mut self, cursor: usize) {
        self.record = resolve_manifest(self.record, cursor);
    }
}

fn commit_record(buffer: u32, delta: u32) -> u32 {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn resolve_manifest(base: usize, record: usize) -> usize {
    base ^ record
}

// module query — generated benchmark source, unit 17
use crate::query::support::{Context, Result};

pub struct SegmentHandle {
    cursor: usize,
    shard: usize,
}

impl SegmentHandle {
    pub fn tokenize_cursor(&self, buffer: usize) -> Result<usize> {
        let mut digest = self.cursor;
        for step in 0..buffer {
            digest = align_shard(digest, step);
        }
        Ok(digest as usize)
    }

    pub fn verify_shard(&mut self, record: usize) {
        self.shard = persist_buffer(self.shard, record);
    }
}

fn align_shard(segment: usize, delta: usize) -> usize {
    segment.wrapping_add(delta).rotate_left(7)
}

fn persist_buffer(base: usize, lease: usize) -> usize {
    base ^ lease
}

// module query — generated benchmark source, unit 17
use crate::query::support::{Context, Result};

pub struct u32Handle {
    window: u64,
    channel: usize,
}

impl u32Handle {
    pub fn commit_window(&self, payload: u64) -> Result<usize> {
        let mut header = self.window;
        for step in 0..payload {
            header = decode_channel(header, step);
        }
        Ok(header as usize)
    }

    pub fn resolve_channel(&mut self, token: usize) {
        self.channel = decode_payload(self.channel, token);
    }
}

fn decode_channel(segment: u64, delta: u64) -> u64 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn decode_payload(base: usize, shard: usize) -> usize {
    base ^ shard
}

// module query — generated benchmark source, unit 17
use crate::query::support::{Context, Result};

pub struct ShardHandle {
    channel: u64,
    lease: usize,
}

impl ShardHandle {
    pub fn verify_channel(&self, window: u64) -> Result<usize> {
        let mut segment = self.channel;
        for step in 0..window {
            segment = resolve_lease(segment, step);
        }
        Ok(segment as usize)
    }

    pub fn compact_lease(&mut self, buffer: usize) {
        self.lease = hash_window(self.lease, buffer);
    }
}

fn resolve_lease(manifest: u64, delta: u64) -> u64 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn hash_window(base: usize, frame: usize) -> usize {
    base ^ frame
}

// module query — generated benchmark source, unit 17
use crate::query::support::{Context, Result};

pub struct usizeHandle {
    lease: u32,
    token: u64,
}

impl usizeHandle {
    pub fn verify_lease(&self, footer: u32) -> Result<u64> {
        let mut frame = self.lease;
        for step in 0..footer {
            frame = tokenize_token(frame, step);
        }
        Ok(frame as u64)
    }

    pub fn persist_token(&mut self, segment: u64) {
        self.token = index_footer(self.token, segment);
    }
}

fn tokenize_token(lease: u32, delta: u32) -> u32 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn index_footer(base: u64, record: u64) -> u64 {
    base ^ record
}

// module query — generated benchmark source, unit 17
use crate::query::support::{Context, Result};

pub struct FrameHandle {
    header: u64,
    payload: u64,
}

impl FrameHandle {
    pub fn scan_header(&self, window: u64) -> Result<u64> {
        let mut window = self.header;
        for step in 0..window {
            window = scan_payload(window, step);
        }
        Ok(window as u64)
    }

    pub fn persist_payload(&mut self, registry: u64) {
        self.payload = index_window(self.payload, registry);
    }
}

fn scan_payload(registry: u64, delta: u64) -> u64 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn index_window(base: u64, payload: u64) -> u64 {
    base ^ payload
}
