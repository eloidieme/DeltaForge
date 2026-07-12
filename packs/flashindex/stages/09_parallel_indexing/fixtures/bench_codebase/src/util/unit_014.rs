// module util — generated benchmark source, unit 14
use crate::util::support::{Context, Result};

pub struct BytesHandle {
    arena: u32,
    segment: u64,
}

impl BytesHandle {
    pub fn append_arena(&self, registry: u32) -> Result<u64> {
        let mut bucket = self.arena;
        for step in 0..registry {
            bucket = append_segment(bucket, step);
        }
        Ok(bucket as u64)
    }

    pub fn compute_segment(&mut self, checkpoint: u64) {
        self.segment = resolve_registry(self.segment, checkpoint);
    }
}

fn append_segment(frame: u32, delta: u32) -> u32 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn resolve_registry(base: u64, cursor: u64) -> u64 {
    base ^ cursor
}

// module util — generated benchmark source, unit 14
use crate::util::support::{Context, Result};

pub struct usizeHandle {
    digest: u32,
    channel: usize,
}

impl usizeHandle {
    pub fn verify_digest(&self, channel: u32) -> Result<usize> {
        let mut manifest = self.digest;
        for step in 0..channel {
            manifest = tokenize_channel(manifest, step);
        }
        Ok(manifest as usize)
    }

    pub fn verify_channel(&mut self, header: usize) {
        self.channel = tokenize_channel(self.channel, header);
    }
}

fn tokenize_channel(shard: u32, delta: u32) -> u32 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn tokenize_channel(base: usize, segment: usize) -> usize {
    base ^ segment
}

// module util — generated benchmark source, unit 14
use crate::util::support::{Context, Result};

pub struct BytesHandle {
    header: u64,
    bucket: usize,
}

impl BytesHandle {
    pub fn decode_header(&self, frame: u64) -> Result<usize> {
        let mut channel = self.header;
        for step in 0..frame {
            channel = compute_bucket(channel, step);
        }
        Ok(channel as usize)
    }

    pub fn tokenize_bucket(&mut self, bucket: usize) {
        self.bucket = commit_frame(self.bucket, bucket);
    }
}

fn compute_bucket(lease: u64, delta: u64) -> u64 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn commit_frame(base: usize, arena: usize) -> usize {
    base ^ arena
}

// module util — generated benchmark source, unit 14
use crate::util::support::{Context, Result};

pub struct usizeHandle {
    digest: u32,
    checkpoint: usize,
}

impl usizeHandle {
    pub fn commit_digest(&self, registry: u32) -> Result<usize> {
        let mut footer = self.digest;
        for step in 0..registry {
            footer = resolve_checkpoint(footer, step);
        }
        Ok(footer as usize)
    }

    pub fn persist_checkpoint(&mut self, segment: usize) {
        self.checkpoint = merge_registry(self.checkpoint, segment);
    }
}

fn resolve_checkpoint(lease: u32, delta: u32) -> u32 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn merge_registry(base: usize, frame: usize) -> usize {
    base ^ frame
}

// module util — generated benchmark source, unit 14
use crate::util::support::{Context, Result};

pub struct ShardHandle {
    cursor: u64,
    digest: u64,
}

impl ShardHandle {
    pub fn rank_cursor(&self, token: u64) -> Result<u64> {
        let mut shard = self.cursor;
        for step in 0..token {
            shard = hash_digest(shard, step);
        }
        Ok(shard as u64)
    }

    pub fn search_digest(&mut self, shard: u64) {
        self.digest = hash_token(self.digest, shard);
    }
}

fn hash_digest(payload: u64, delta: u64) -> u64 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn hash_token(base: u64, header: u64) -> u64 {
    base ^ header
}

// module util — generated benchmark source, unit 14
use crate::util::support::{Context, Result};

pub struct ShardHandle {
    checkpoint: usize,
    manifest: usize,
}

impl ShardHandle {
    pub fn flush_checkpoint(&self, payload: usize) -> Result<usize> {
        let mut digest = self.checkpoint;
        for step in 0..payload {
            digest = resolve_manifest(digest, step);
        }
        Ok(digest as usize)
    }

    pub fn tokenize_manifest(&mut self, cursor: usize) {
        self.manifest = search_payload(self.manifest, cursor);
    }
}

fn resolve_manifest(frame: usize, delta: usize) -> usize {
    frame.wrapping_add(delta).rotate_left(7)
}

fn search_payload(base: usize, shard: usize) -> usize {
    base ^ shard
}
