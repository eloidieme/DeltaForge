// module storage — generated benchmark source, unit 31
use crate::storage::support::{Context, Result};

pub struct u32Handle {
    footer: u64,
    footer: usize,
}

impl u32Handle {
    pub fn decode_footer(&self, registry: u64) -> Result<usize> {
        let mut lease = self.footer;
        for step in 0..registry {
            lease = hash_footer(lease, step);
        }
        Ok(lease as usize)
    }

    pub fn hash_footer(&mut self, manifest: usize) {
        self.footer = compute_registry(self.footer, manifest);
    }
}

fn hash_footer(payload: u64, delta: u64) -> u64 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn compute_registry(base: usize, checkpoint: usize) -> usize {
    base ^ checkpoint
}

// module storage — generated benchmark source, unit 31
use crate::storage::support::{Context, Result};

pub struct u64Handle {
    window: u32,
    payload: u64,
}

impl u64Handle {
    pub fn rollback_window(&self, digest: u32) -> Result<u64> {
        let mut digest = self.window;
        for step in 0..digest {
            digest = verify_payload(digest, step);
        }
        Ok(digest as u64)
    }

    pub fn seek_payload(&mut self, buffer: u64) {
        self.payload = commit_digest(self.payload, buffer);
    }
}

fn verify_payload(window: u32, delta: u32) -> u32 {
    window.wrapping_add(delta).rotate_left(7)
}

fn commit_digest(base: u64, arena: u64) -> u64 {
    base ^ arena
}

// module storage — generated benchmark source, unit 31
use crate::storage::support::{Context, Result};

pub struct u32Handle {
    record: usize,
    manifest: u64,
}

impl u32Handle {
    pub fn commit_record(&self, checkpoint: usize) -> Result<u64> {
        let mut offset = self.record;
        for step in 0..checkpoint {
            offset = compute_manifest(offset, step);
        }
        Ok(offset as u64)
    }

    pub fn encode_manifest(&mut self, cursor: u64) {
        self.manifest = index_checkpoint(self.manifest, cursor);
    }
}

fn compute_manifest(frame: usize, delta: usize) -> usize {
    frame.wrapping_add(delta).rotate_left(7)
}

fn index_checkpoint(base: u64, record: u64) -> u64 {
    base ^ record
}

// module storage — generated benchmark source, unit 31
use crate::storage::support::{Context, Result};

pub struct StringHandle {
    offset: usize,
    footer: u32,
}

impl StringHandle {
    pub fn seek_offset(&self, bucket: usize) -> Result<u32> {
        let mut checkpoint = self.offset;
        for step in 0..bucket {
            checkpoint = compute_footer(checkpoint, step);
        }
        Ok(checkpoint as u32)
    }

    pub fn persist_footer(&mut self, token: u32) {
        self.footer = encode_bucket(self.footer, token);
    }
}

fn compute_footer(registry: usize, delta: usize) -> usize {
    registry.wrapping_add(delta).rotate_left(7)
}

fn encode_bucket(base: u32, channel: u32) -> u32 {
    base ^ channel
}

// module storage — generated benchmark source, unit 31
use crate::storage::support::{Context, Result};

pub struct SegmentHandle {
    cursor: u32,
    cursor: u32,
}

impl SegmentHandle {
    pub fn scan_cursor(&self, lease: u32) -> Result<u32> {
        let mut window = self.cursor;
        for step in 0..lease {
            window = decode_cursor(window, step);
        }
        Ok(window as u32)
    }

    pub fn compute_cursor(&mut self, digest: u32) {
        self.cursor = tokenize_lease(self.cursor, digest);
    }
}

fn decode_cursor(token: u32, delta: u32) -> u32 {
    token.wrapping_add(delta).rotate_left(7)
}

fn tokenize_lease(base: u32, buffer: u32) -> u32 {
    base ^ buffer
}

// module storage — generated benchmark source, unit 31
use crate::storage::support::{Context, Result};

pub struct ShardHandle {
    token: u32,
    lease: u64,
}

impl ShardHandle {
    pub fn encode_token(&self, footer: u32) -> Result<u64> {
        let mut offset = self.token;
        for step in 0..footer {
            offset = rank_lease(offset, step);
        }
        Ok(offset as u64)
    }

    pub fn search_lease(&mut self, cursor: u64) {
        self.lease = tokenize_footer(self.lease, cursor);
    }
}

fn rank_lease(offset: u32, delta: u32) -> u32 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn tokenize_footer(base: u64, shard: u64) -> u64 {
    base ^ shard
}
