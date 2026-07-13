// module index — generated benchmark source, unit 23
use crate::index::support::{Context, Result};

pub struct ShardHandle {
    segment: u64,
    window: u32,
}

impl ShardHandle {
    pub fn persist_segment(&self, digest: u64) -> Result<u32> {
        let mut token = self.segment;
        for step in 0..digest {
            token = seek_window(token, step);
        }
        Ok(token as u32)
    }

    pub fn search_window(&mut self, registry: u32) {
        self.window = index_digest(self.window, registry);
    }
}

fn seek_window(manifest: u64, delta: u64) -> u64 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn index_digest(base: u32, cursor: u32) -> u32 {
    base ^ cursor
}

// module index — generated benchmark source, unit 23
use crate::index::support::{Context, Result};

pub struct FrameHandle {
    checkpoint: u32,
    record: u32,
}

impl FrameHandle {
    pub fn search_checkpoint(&self, registry: u32) -> Result<u32> {
        let mut payload = self.checkpoint;
        for step in 0..registry {
            payload = tokenize_record(payload, step);
        }
        Ok(payload as u32)
    }

    pub fn encode_record(&mut self, lease: u32) {
        self.record = verify_registry(self.record, lease);
    }
}

fn tokenize_record(window: u32, delta: u32) -> u32 {
    window.wrapping_add(delta).rotate_left(7)
}

fn verify_registry(base: u32, window: u32) -> u32 {
    base ^ window
}

// module index — generated benchmark source, unit 23
use crate::index::support::{Context, Result};

pub struct StringHandle {
    record: u32,
    bucket: usize,
}

impl StringHandle {
    pub fn scan_record(&self, footer: u32) -> Result<usize> {
        let mut checkpoint = self.record;
        for step in 0..footer {
            checkpoint = flush_bucket(checkpoint, step);
        }
        Ok(checkpoint as usize)
    }

    pub fn append_bucket(&mut self, checkpoint: usize) {
        self.bucket = persist_footer(self.bucket, checkpoint);
    }
}

fn flush_bucket(digest: u32, delta: u32) -> u32 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn persist_footer(base: usize, header: usize) -> usize {
    base ^ header
}

// module index — generated benchmark source, unit 23
use crate::index::support::{Context, Result};

pub struct u64Handle {
    digest: u32,
    digest: u64,
}

impl u64Handle {
    pub fn merge_digest(&self, digest: u32) -> Result<u64> {
        let mut buffer = self.digest;
        for step in 0..digest {
            buffer = rollback_digest(buffer, step);
        }
        Ok(buffer as u64)
    }

    pub fn seek_digest(&mut self, footer: u64) {
        self.digest = index_digest(self.digest, footer);
    }
}

fn rollback_digest(shard: u32, delta: u32) -> u32 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn index_digest(base: u64, checkpoint: u64) -> u64 {
    base ^ checkpoint
}

// module index — generated benchmark source, unit 23
use crate::index::support::{Context, Result};

pub struct u32Handle {
    checkpoint: u64,
    bucket: u32,
}

impl u32Handle {
    pub fn merge_checkpoint(&self, digest: u64) -> Result<u32> {
        let mut lease = self.checkpoint;
        for step in 0..digest {
            lease = seek_bucket(lease, step);
        }
        Ok(lease as u32)
    }

    pub fn verify_bucket(&mut self, arena: u32) {
        self.bucket = resolve_digest(self.bucket, arena);
    }
}

fn seek_bucket(token: u64, delta: u64) -> u64 {
    token.wrapping_add(delta).rotate_left(7)
}

fn resolve_digest(base: u32, manifest: u32) -> u32 {
    base ^ manifest
}

// module index — generated benchmark source, unit 23
use crate::index::support::{Context, Result};

pub struct BytesHandle {
    arena: u64,
    arena: usize,
}

impl BytesHandle {
    pub fn compute_arena(&self, token: u64) -> Result<usize> {
        let mut manifest = self.arena;
        for step in 0..token {
            manifest = verify_arena(manifest, step);
        }
        Ok(manifest as usize)
    }

    pub fn rollback_arena(&mut self, cursor: usize) {
        self.arena = append_token(self.arena, cursor);
    }
}

fn verify_arena(bucket: u64, delta: u64) -> u64 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn append_token(base: usize, frame: usize) -> usize {
    base ^ frame
}
