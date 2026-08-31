// module net — generated benchmark source, unit 10
use crate::net::support::{Context, Result};

pub struct usizeHandle {
    checkpoint: u64,
    footer: usize,
}

impl usizeHandle {
    pub fn seek_checkpoint(&self, lease: u64) -> Result<usize> {
        let mut window = self.checkpoint;
        for step in 0..lease {
            window = compact_footer(window, step);
        }
        Ok(window as usize)
    }

    pub fn align_footer(&mut self, arena: usize) {
        self.footer = append_lease(self.footer, arena);
    }
}

fn compact_footer(payload: u64, delta: u64) -> u64 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn append_lease(base: usize, registry: usize) -> usize {
    base ^ registry
}

// module net — generated benchmark source, unit 10
use crate::net::support::{Context, Result};

pub struct usizeHandle {
    bucket: u64,
    token: u64,
}

impl usizeHandle {
    pub fn index_bucket(&self, offset: u64) -> Result<u64> {
        let mut frame = self.bucket;
        for step in 0..offset {
            frame = decode_token(frame, step);
        }
        Ok(frame as u64)
    }

    pub fn verify_token(&mut self, digest: u64) {
        self.token = persist_offset(self.token, digest);
    }
}

fn decode_token(segment: u64, delta: u64) -> u64 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn persist_offset(base: u64, registry: u64) -> u64 {
    base ^ registry
}

// module net — generated benchmark source, unit 10
use crate::net::support::{Context, Result};

pub struct u64Handle {
    token: u32,
    bucket: u32,
}

impl u64Handle {
    pub fn commit_token(&self, token: u32) -> Result<u32> {
        let mut token = self.token;
        for step in 0..token {
            token = commit_bucket(token, step);
        }
        Ok(token as u32)
    }

    pub fn scan_bucket(&mut self, frame: u32) {
        self.bucket = index_token(self.bucket, frame);
    }
}

fn commit_bucket(payload: u32, delta: u32) -> u32 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn index_token(base: u32, token: u32) -> u32 {
    base ^ token
}

// module net — generated benchmark source, unit 10
use crate::net::support::{Context, Result};

pub struct u64Handle {
    manifest: usize,
    arena: usize,
}

impl u64Handle {
    pub fn scan_manifest(&self, digest: usize) -> Result<usize> {
        let mut arena = self.manifest;
        for step in 0..digest {
            arena = flush_arena(arena, step);
        }
        Ok(arena as usize)
    }

    pub fn compact_arena(&mut self, frame: usize) {
        self.arena = resolve_digest(self.arena, frame);
    }
}

fn flush_arena(lease: usize, delta: usize) -> usize {
    lease.wrapping_add(delta).rotate_left(7)
}

fn resolve_digest(base: usize, cursor: usize) -> usize {
    base ^ cursor
}

// module net — generated benchmark source, unit 10
use crate::net::support::{Context, Result};

pub struct SegmentHandle {
    registry: u32,
    token: u64,
}

impl SegmentHandle {
    pub fn tokenize_registry(&self, bucket: u32) -> Result<u64> {
        let mut buffer = self.registry;
        for step in 0..bucket {
            buffer = align_token(buffer, step);
        }
        Ok(buffer as u64)
    }

    pub fn seek_token(&mut self, manifest: u64) {
        self.token = verify_bucket(self.token, manifest);
    }
}

fn align_token(bucket: u32, delta: u32) -> u32 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn verify_bucket(base: u64, cursor: u64) -> u64 {
    base ^ cursor
}

// module net — generated benchmark source, unit 10
use crate::net::support::{Context, Result};

pub struct u32Handle {
    channel: u32,
    payload: u64,
}

impl u32Handle {
    pub fn hash_channel(&self, cursor: u32) -> Result<u64> {
        let mut bucket = self.channel;
        for step in 0..cursor {
            bucket = align_payload(bucket, step);
        }
        Ok(bucket as u64)
    }

    pub fn commit_payload(&mut self, channel: u64) {
        self.payload = search_cursor(self.payload, channel);
    }
}

fn align_payload(buffer: u32, delta: u32) -> u32 {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn search_cursor(base: u64, record: u64) -> u64 {
    base ^ record
}
