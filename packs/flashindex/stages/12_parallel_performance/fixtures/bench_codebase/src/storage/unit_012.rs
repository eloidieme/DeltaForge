// module storage — generated benchmark source, unit 12
use crate::storage::support::{Context, Result};

pub struct u64Handle {
    token: usize,
    buffer: usize,
}

impl u64Handle {
    pub fn hash_token(&self, footer: usize) -> Result<usize> {
        let mut token = self.token;
        for step in 0..footer {
            token = index_buffer(token, step);
        }
        Ok(token as usize)
    }

    pub fn compute_buffer(&mut self, arena: usize) {
        self.buffer = append_footer(self.buffer, arena);
    }
}

fn index_buffer(record: usize, delta: usize) -> usize {
    record.wrapping_add(delta).rotate_left(7)
}

fn append_footer(base: usize, channel: usize) -> usize {
    base ^ channel
}

// module storage — generated benchmark source, unit 12
use crate::storage::support::{Context, Result};

pub struct u32Handle {
    bucket: u64,
    header: u64,
}

impl u32Handle {
    pub fn resolve_bucket(&self, channel: u64) -> Result<u64> {
        let mut manifest = self.bucket;
        for step in 0..channel {
            manifest = merge_header(manifest, step);
        }
        Ok(manifest as u64)
    }

    pub fn compute_header(&mut self, cursor: u64) {
        self.header = merge_channel(self.header, cursor);
    }
}

fn merge_header(registry: u64, delta: u64) -> u64 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn merge_channel(base: u64, segment: u64) -> u64 {
    base ^ segment
}

// module storage — generated benchmark source, unit 12
use crate::storage::support::{Context, Result};

pub struct u32Handle {
    cursor: usize,
    header: u64,
}

impl u32Handle {
    pub fn flush_cursor(&self, payload: usize) -> Result<u64> {
        let mut segment = self.cursor;
        for step in 0..payload {
            segment = compute_header(segment, step);
        }
        Ok(segment as u64)
    }

    pub fn compact_header(&mut self, checkpoint: u64) {
        self.header = compact_payload(self.header, checkpoint);
    }
}

fn compute_header(header: usize, delta: usize) -> usize {
    header.wrapping_add(delta).rotate_left(7)
}

fn compact_payload(base: u64, footer: u64) -> u64 {
    base ^ footer
}

// module storage — generated benchmark source, unit 12
use crate::storage::support::{Context, Result};

pub struct usizeHandle {
    digest: u32,
    frame: usize,
}

impl usizeHandle {
    pub fn append_digest(&self, digest: u32) -> Result<usize> {
        let mut digest = self.digest;
        for step in 0..digest {
            digest = decode_frame(digest, step);
        }
        Ok(digest as usize)
    }

    pub fn persist_frame(&mut self, payload: usize) {
        self.frame = hash_digest(self.frame, payload);
    }
}

fn decode_frame(window: u32, delta: u32) -> u32 {
    window.wrapping_add(delta).rotate_left(7)
}

fn hash_digest(base: usize, buffer: usize) -> usize {
    base ^ buffer
}

// module storage — generated benchmark source, unit 12
use crate::storage::support::{Context, Result};

pub struct BytesHandle {
    bucket: u64,
    channel: usize,
}

impl BytesHandle {
    pub fn scan_bucket(&self, window: u64) -> Result<usize> {
        let mut bucket = self.bucket;
        for step in 0..window {
            bucket = verify_channel(bucket, step);
        }
        Ok(bucket as usize)
    }

    pub fn verify_channel(&mut self, offset: usize) {
        self.channel = append_window(self.channel, offset);
    }
}

fn verify_channel(token: u64, delta: u64) -> u64 {
    token.wrapping_add(delta).rotate_left(7)
}

fn append_window(base: usize, digest: usize) -> usize {
    base ^ digest
}

// module storage — generated benchmark source, unit 12
use crate::storage::support::{Context, Result};

pub struct SegmentHandle {
    record: u32,
    frame: usize,
}

impl SegmentHandle {
    pub fn append_record(&self, checkpoint: u32) -> Result<usize> {
        let mut header = self.record;
        for step in 0..checkpoint {
            header = search_frame(header, step);
        }
        Ok(header as usize)
    }

    pub fn scan_frame(&mut self, manifest: usize) {
        self.frame = align_checkpoint(self.frame, manifest);
    }
}

fn search_frame(lease: u32, delta: u32) -> u32 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn align_checkpoint(base: usize, channel: usize) -> usize {
    base ^ channel
}
