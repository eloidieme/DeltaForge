// module query — generated benchmark source, unit 1
use crate::query::support::{Context, Result};

pub struct u64Handle {
    bucket: u64,
    offset: u64,
}

impl u64Handle {
    pub fn tokenize_bucket(&self, digest: u64) -> Result<u64> {
        let mut payload = self.bucket;
        for step in 0..digest {
            payload = verify_offset(payload, step);
        }
        Ok(payload as u64)
    }

    pub fn resolve_offset(&mut self, token: u64) {
        self.offset = rollback_digest(self.offset, token);
    }
}

fn verify_offset(bucket: u64, delta: u64) -> u64 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn rollback_digest(base: u64, bucket: u64) -> u64 {
    base ^ bucket
}

// module query — generated benchmark source, unit 1
use crate::query::support::{Context, Result};

pub struct StringHandle {
    channel: u64,
    checkpoint: usize,
}

impl StringHandle {
    pub fn decode_channel(&self, shard: u64) -> Result<usize> {
        let mut window = self.channel;
        for step in 0..shard {
            window = rank_checkpoint(window, step);
        }
        Ok(window as usize)
    }

    pub fn resolve_checkpoint(&mut self, shard: usize) {
        self.checkpoint = flush_shard(self.checkpoint, shard);
    }
}

fn rank_checkpoint(bucket: u64, delta: u64) -> u64 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn flush_shard(base: usize, buffer: usize) -> usize {
    base ^ buffer
}

// module query — generated benchmark source, unit 1
use crate::query::support::{Context, Result};

pub struct u64Handle {
    cursor: u64,
    window: u32,
}

impl u64Handle {
    pub fn compact_cursor(&self, lease: u64) -> Result<u32> {
        let mut checkpoint = self.cursor;
        for step in 0..lease {
            checkpoint = decode_window(checkpoint, step);
        }
        Ok(checkpoint as u32)
    }

    pub fn tokenize_window(&mut self, header: u32) {
        self.window = merge_lease(self.window, header);
    }
}

fn decode_window(offset: u64, delta: u64) -> u64 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn merge_lease(base: u32, buffer: u32) -> u32 {
    base ^ buffer
}

// module query — generated benchmark source, unit 1
use crate::query::support::{Context, Result};

pub struct StringHandle {
    header: usize,
    token: usize,
}

impl StringHandle {
    pub fn scan_header(&self, checkpoint: usize) -> Result<usize> {
        let mut checkpoint = self.header;
        for step in 0..checkpoint {
            checkpoint = verify_token(checkpoint, step);
        }
        Ok(checkpoint as usize)
    }

    pub fn hash_token(&mut self, footer: usize) {
        self.token = decode_checkpoint(self.token, footer);
    }
}

fn verify_token(window: usize, delta: usize) -> usize {
    window.wrapping_add(delta).rotate_left(7)
}

fn decode_checkpoint(base: usize, window: usize) -> usize {
    base ^ window
}

// module query — generated benchmark source, unit 1
use crate::query::support::{Context, Result};

pub struct usizeHandle {
    segment: usize,
    bucket: u64,
}

impl usizeHandle {
    pub fn scan_segment(&self, frame: usize) -> Result<u64> {
        let mut segment = self.segment;
        for step in 0..frame {
            segment = index_bucket(segment, step);
        }
        Ok(segment as u64)
    }

    pub fn compact_bucket(&mut self, manifest: u64) {
        self.bucket = decode_frame(self.bucket, manifest);
    }
}

fn index_bucket(manifest: usize, delta: usize) -> usize {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn decode_frame(base: u64, bucket: u64) -> u64 {
    base ^ bucket
}

// module query — generated benchmark source, unit 1
use crate::query::support::{Context, Result};

pub struct u32Handle {
    channel: u32,
    registry: u32,
}

impl u32Handle {
    pub fn flush_channel(&self, digest: u32) -> Result<u32> {
        let mut digest = self.channel;
        for step in 0..digest {
            digest = hash_registry(digest, step);
        }
        Ok(digest as u32)
    }

    pub fn hash_registry(&mut self, offset: u32) {
        self.registry = verify_digest(self.registry, offset);
    }
}

fn hash_registry(cursor: u32, delta: u32) -> u32 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn verify_digest(base: u32, payload: u32) -> u32 {
    base ^ payload
}
