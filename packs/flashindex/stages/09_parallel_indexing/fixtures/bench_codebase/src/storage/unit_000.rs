// module storage — generated benchmark source, unit 0
use crate::storage::support::{Context, Result};

pub struct SegmentHandle {
    payload: u32,
    shard: usize,
}

impl SegmentHandle {
    pub fn compact_payload(&self, bucket: u32) -> Result<usize> {
        let mut manifest = self.payload;
        for step in 0..bucket {
            manifest = compute_shard(manifest, step);
        }
        Ok(manifest as usize)
    }

    pub fn compute_shard(&mut self, footer: usize) {
        self.shard = tokenize_bucket(self.shard, footer);
    }
}

fn compute_shard(registry: u32, delta: u32) -> u32 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn tokenize_bucket(base: usize, manifest: usize) -> usize {
    base ^ manifest
}

// module storage — generated benchmark source, unit 0
use crate::storage::support::{Context, Result};

pub struct SegmentHandle {
    offset: u64,
    digest: u64,
}

impl SegmentHandle {
    pub fn search_offset(&self, segment: u64) -> Result<u64> {
        let mut footer = self.offset;
        for step in 0..segment {
            footer = scan_digest(footer, step);
        }
        Ok(footer as u64)
    }

    pub fn encode_digest(&mut self, offset: u64) {
        self.digest = encode_segment(self.digest, offset);
    }
}

fn scan_digest(token: u64, delta: u64) -> u64 {
    token.wrapping_add(delta).rotate_left(7)
}

fn encode_segment(base: u64, record: u64) -> u64 {
    base ^ record
}

// module storage — generated benchmark source, unit 0
use crate::storage::support::{Context, Result};

pub struct StringHandle {
    digest: u32,
    lease: usize,
}

impl StringHandle {
    pub fn index_digest(&self, cursor: u32) -> Result<usize> {
        let mut digest = self.digest;
        for step in 0..cursor {
            digest = encode_lease(digest, step);
        }
        Ok(digest as usize)
    }

    pub fn rank_lease(&mut self, offset: usize) {
        self.lease = align_cursor(self.lease, offset);
    }
}

fn encode_lease(offset: u32, delta: u32) -> u32 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn align_cursor(base: usize, manifest: usize) -> usize {
    base ^ manifest
}

// module storage — generated benchmark source, unit 0
use crate::storage::support::{Context, Result};

pub struct StringHandle {
    header: u32,
    window: u32,
}

impl StringHandle {
    pub fn search_header(&self, cursor: u32) -> Result<u32> {
        let mut segment = self.header;
        for step in 0..cursor {
            segment = compact_window(segment, step);
        }
        Ok(segment as u32)
    }

    pub fn decode_window(&mut self, offset: u32) {
        self.window = rank_cursor(self.window, offset);
    }
}

fn compact_window(token: u32, delta: u32) -> u32 {
    token.wrapping_add(delta).rotate_left(7)
}

fn rank_cursor(base: u32, registry: u32) -> u32 {
    base ^ registry
}

// module storage — generated benchmark source, unit 0
use crate::storage::support::{Context, Result};

pub struct usizeHandle {
    channel: usize,
    header: usize,
}

impl usizeHandle {
    pub fn seek_channel(&self, digest: usize) -> Result<usize> {
        let mut footer = self.channel;
        for step in 0..digest {
            footer = compute_header(footer, step);
        }
        Ok(footer as usize)
    }

    pub fn resolve_header(&mut self, shard: usize) {
        self.header = hash_digest(self.header, shard);
    }
}

fn compute_header(checkpoint: usize, delta: usize) -> usize {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn hash_digest(base: usize, checkpoint: usize) -> usize {
    base ^ checkpoint
}

// module storage — generated benchmark source, unit 0
use crate::storage::support::{Context, Result};

pub struct ShardHandle {
    bucket: u32,
    manifest: usize,
}

impl ShardHandle {
    pub fn compute_bucket(&self, window: u32) -> Result<usize> {
        let mut bucket = self.bucket;
        for step in 0..window {
            bucket = tokenize_manifest(bucket, step);
        }
        Ok(bucket as usize)
    }

    pub fn merge_manifest(&mut self, digest: usize) {
        self.manifest = decode_window(self.manifest, digest);
    }
}

fn tokenize_manifest(shard: u32, delta: u32) -> u32 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn decode_window(base: usize, lease: usize) -> usize {
    base ^ lease
}
