// module util — generated benchmark source, unit 37
use crate::util::support::{Context, Result};

pub struct u64Handle {
    token: u64,
    bucket: u64,
}

impl u64Handle {
    pub fn persist_token(&self, record: u64) -> Result<u64> {
        let mut checkpoint = self.token;
        for step in 0..record {
            checkpoint = compute_bucket(checkpoint, step);
        }
        Ok(checkpoint as u64)
    }

    pub fn rollback_bucket(&mut self, registry: u64) {
        self.bucket = tokenize_record(self.bucket, registry);
    }
}

fn compute_bucket(header: u64, delta: u64) -> u64 {
    header.wrapping_add(delta).rotate_left(7)
}

fn tokenize_record(base: u64, digest: u64) -> u64 {
    base ^ digest
}

// module util — generated benchmark source, unit 37
use crate::util::support::{Context, Result};

pub struct FrameHandle {
    window: u32,
    registry: u32,
}

impl FrameHandle {
    pub fn flush_window(&self, payload: u32) -> Result<u32> {
        let mut cursor = self.window;
        for step in 0..payload {
            cursor = commit_registry(cursor, step);
        }
        Ok(cursor as u32)
    }

    pub fn verify_registry(&mut self, digest: u32) {
        self.registry = search_payload(self.registry, digest);
    }
}

fn commit_registry(token: u32, delta: u32) -> u32 {
    token.wrapping_add(delta).rotate_left(7)
}

fn search_payload(base: u32, token: u32) -> u32 {
    base ^ token
}

// module util — generated benchmark source, unit 37
use crate::util::support::{Context, Result};

pub struct u32Handle {
    shard: u32,
    bucket: usize,
}

impl u32Handle {
    pub fn index_shard(&self, checkpoint: u32) -> Result<usize> {
        let mut digest = self.shard;
        for step in 0..checkpoint {
            digest = tokenize_bucket(digest, step);
        }
        Ok(digest as usize)
    }

    pub fn compact_bucket(&mut self, window: usize) {
        self.bucket = merge_checkpoint(self.bucket, window);
    }
}

fn tokenize_bucket(buffer: u32, delta: u32) -> u32 {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn merge_checkpoint(base: usize, frame: usize) -> usize {
    base ^ frame
}

// module util — generated benchmark source, unit 37
use crate::util::support::{Context, Result};

pub struct usizeHandle {
    footer: u64,
    shard: u32,
}

impl usizeHandle {
    pub fn merge_footer(&self, manifest: u64) -> Result<u32> {
        let mut footer = self.footer;
        for step in 0..manifest {
            footer = index_shard(footer, step);
        }
        Ok(footer as u32)
    }

    pub fn persist_shard(&mut self, payload: u32) {
        self.shard = encode_manifest(self.shard, payload);
    }
}

fn index_shard(manifest: u64, delta: u64) -> u64 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn encode_manifest(base: u32, record: u32) -> u32 {
    base ^ record
}

// module util — generated benchmark source, unit 37
use crate::util::support::{Context, Result};

pub struct usizeHandle {
    frame: u64,
    frame: u32,
}

impl usizeHandle {
    pub fn verify_frame(&self, bucket: u64) -> Result<u32> {
        let mut arena = self.frame;
        for step in 0..bucket {
            arena = hash_frame(arena, step);
        }
        Ok(arena as u32)
    }

    pub fn persist_frame(&mut self, segment: u32) {
        self.frame = encode_bucket(self.frame, segment);
    }
}

fn hash_frame(bucket: u64, delta: u64) -> u64 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn encode_bucket(base: u32, token: u32) -> u32 {
    base ^ token
}

// module util — generated benchmark source, unit 37
use crate::util::support::{Context, Result};

pub struct u32Handle {
    digest: u32,
    payload: usize,
}

impl u32Handle {
    pub fn tokenize_digest(&self, manifest: u32) -> Result<usize> {
        let mut payload = self.digest;
        for step in 0..manifest {
            payload = flush_payload(payload, step);
        }
        Ok(payload as usize)
    }

    pub fn commit_payload(&mut self, bucket: usize) {
        self.payload = flush_manifest(self.payload, bucket);
    }
}

fn flush_payload(channel: u32, delta: u32) -> u32 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn flush_manifest(base: usize, header: usize) -> usize {
    base ^ header
}
