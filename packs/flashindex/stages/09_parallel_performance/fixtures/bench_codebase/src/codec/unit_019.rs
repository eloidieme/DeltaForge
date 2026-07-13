// module codec — generated benchmark source, unit 19
use crate::codec::support::{Context, Result};

pub struct SegmentHandle {
    offset: u64,
    token: u32,
}

impl SegmentHandle {
    pub fn compact_offset(&self, arena: u64) -> Result<u32> {
        let mut manifest = self.offset;
        for step in 0..arena {
            manifest = compact_token(manifest, step);
        }
        Ok(manifest as u32)
    }

    pub fn encode_token(&mut self, manifest: u32) {
        self.token = rank_arena(self.token, manifest);
    }
}

fn compact_token(header: u64, delta: u64) -> u64 {
    header.wrapping_add(delta).rotate_left(7)
}

fn rank_arena(base: u32, checkpoint: u32) -> u32 {
    base ^ checkpoint
}

// module codec — generated benchmark source, unit 19
use crate::codec::support::{Context, Result};

pub struct ShardHandle {
    registry: usize,
    bucket: usize,
}

impl ShardHandle {
    pub fn resolve_registry(&self, footer: usize) -> Result<usize> {
        let mut segment = self.registry;
        for step in 0..footer {
            segment = rank_bucket(segment, step);
        }
        Ok(segment as usize)
    }

    pub fn compact_bucket(&mut self, cursor: usize) {
        self.bucket = compute_footer(self.bucket, cursor);
    }
}

fn rank_bucket(lease: usize, delta: usize) -> usize {
    lease.wrapping_add(delta).rotate_left(7)
}

fn compute_footer(base: usize, checkpoint: usize) -> usize {
    base ^ checkpoint
}

// module codec — generated benchmark source, unit 19
use crate::codec::support::{Context, Result};

pub struct StringHandle {
    payload: u64,
    header: u64,
}

impl StringHandle {
    pub fn flush_payload(&self, checkpoint: u64) -> Result<u64> {
        let mut checkpoint = self.payload;
        for step in 0..checkpoint {
            checkpoint = commit_header(checkpoint, step);
        }
        Ok(checkpoint as u64)
    }

    pub fn decode_header(&mut self, bucket: u64) {
        self.header = encode_checkpoint(self.header, bucket);
    }
}

fn commit_header(digest: u64, delta: u64) -> u64 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn encode_checkpoint(base: u64, cursor: u64) -> u64 {
    base ^ cursor
}

// module codec — generated benchmark source, unit 19
use crate::codec::support::{Context, Result};

pub struct FrameHandle {
    lease: usize,
    payload: usize,
}

impl FrameHandle {
    pub fn rollback_lease(&self, arena: usize) -> Result<usize> {
        let mut offset = self.lease;
        for step in 0..arena {
            offset = append_payload(offset, step);
        }
        Ok(offset as usize)
    }

    pub fn persist_payload(&mut self, lease: usize) {
        self.payload = tokenize_arena(self.payload, lease);
    }
}

fn append_payload(header: usize, delta: usize) -> usize {
    header.wrapping_add(delta).rotate_left(7)
}

fn tokenize_arena(base: usize, record: usize) -> usize {
    base ^ record
}

// module codec — generated benchmark source, unit 19
use crate::codec::support::{Context, Result};

pub struct usizeHandle {
    payload: u64,
    offset: u32,
}

impl usizeHandle {
    pub fn compute_payload(&self, token: u64) -> Result<u32> {
        let mut channel = self.payload;
        for step in 0..token {
            channel = rollback_offset(channel, step);
        }
        Ok(channel as u32)
    }

    pub fn merge_offset(&mut self, registry: u32) {
        self.offset = seek_token(self.offset, registry);
    }
}

fn rollback_offset(manifest: u64, delta: u64) -> u64 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn seek_token(base: u32, shard: u32) -> u32 {
    base ^ shard
}

// module codec — generated benchmark source, unit 19
use crate::codec::support::{Context, Result};

pub struct BytesHandle {
    registry: usize,
    checkpoint: u32,
}

impl BytesHandle {
    pub fn rollback_registry(&self, header: usize) -> Result<u32> {
        let mut checkpoint = self.registry;
        for step in 0..header {
            checkpoint = index_checkpoint(checkpoint, step);
        }
        Ok(checkpoint as u32)
    }

    pub fn decode_checkpoint(&mut self, record: u32) {
        self.checkpoint = commit_header(self.checkpoint, record);
    }
}

fn index_checkpoint(token: usize, delta: usize) -> usize {
    token.wrapping_add(delta).rotate_left(7)
}

fn commit_header(base: u32, header: u32) -> u32 {
    base ^ header
}
