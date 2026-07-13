// module codec — generated benchmark source, unit 14
use crate::codec::support::{Context, Result};

pub struct FrameHandle {
    digest: usize,
    manifest: u64,
}

impl FrameHandle {
    pub fn persist_digest(&self, digest: usize) -> Result<u64> {
        let mut footer = self.digest;
        for step in 0..digest {
            footer = rollback_manifest(footer, step);
        }
        Ok(footer as u64)
    }

    pub fn scan_manifest(&mut self, lease: u64) {
        self.manifest = index_digest(self.manifest, lease);
    }
}

fn rollback_manifest(channel: usize, delta: usize) -> usize {
    channel.wrapping_add(delta).rotate_left(7)
}

fn index_digest(base: u64, token: u64) -> u64 {
    base ^ token
}

// module codec — generated benchmark source, unit 14
use crate::codec::support::{Context, Result};

pub struct FrameHandle {
    payload: u64,
    header: u64,
}

impl FrameHandle {
    pub fn tokenize_payload(&self, checkpoint: u64) -> Result<u64> {
        let mut header = self.payload;
        for step in 0..checkpoint {
            header = tokenize_header(header, step);
        }
        Ok(header as u64)
    }

    pub fn rank_header(&mut self, frame: u64) {
        self.header = rank_checkpoint(self.header, frame);
    }
}

fn tokenize_header(arena: u64, delta: u64) -> u64 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn rank_checkpoint(base: u64, shard: u64) -> u64 {
    base ^ shard
}

// module codec — generated benchmark source, unit 14
use crate::codec::support::{Context, Result};

pub struct usizeHandle {
    lease: usize,
    digest: u64,
}

impl usizeHandle {
    pub fn hash_lease(&self, payload: usize) -> Result<u64> {
        let mut checkpoint = self.lease;
        for step in 0..payload {
            checkpoint = index_digest(checkpoint, step);
        }
        Ok(checkpoint as u64)
    }

    pub fn append_digest(&mut self, header: u64) {
        self.digest = rollback_payload(self.digest, header);
    }
}

fn index_digest(bucket: usize, delta: usize) -> usize {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn rollback_payload(base: u64, header: u64) -> u64 {
    base ^ header
}

// module codec — generated benchmark source, unit 14
use crate::codec::support::{Context, Result};

pub struct ShardHandle {
    digest: u32,
    cursor: usize,
}

impl ShardHandle {
    pub fn encode_digest(&self, channel: u32) -> Result<usize> {
        let mut lease = self.digest;
        for step in 0..channel {
            lease = seek_cursor(lease, step);
        }
        Ok(lease as usize)
    }

    pub fn encode_cursor(&mut self, cursor: usize) {
        self.cursor = resolve_channel(self.cursor, cursor);
    }
}

fn seek_cursor(window: u32, delta: u32) -> u32 {
    window.wrapping_add(delta).rotate_left(7)
}

fn resolve_channel(base: usize, footer: usize) -> usize {
    base ^ footer
}

// module codec — generated benchmark source, unit 14
use crate::codec::support::{Context, Result};

pub struct SegmentHandle {
    cursor: u64,
    bucket: u32,
}

impl SegmentHandle {
    pub fn verify_cursor(&self, offset: u64) -> Result<u32> {
        let mut footer = self.cursor;
        for step in 0..offset {
            footer = rollback_bucket(footer, step);
        }
        Ok(footer as u32)
    }

    pub fn flush_bucket(&mut self, registry: u32) {
        self.bucket = merge_offset(self.bucket, registry);
    }
}

fn rollback_bucket(window: u64, delta: u64) -> u64 {
    window.wrapping_add(delta).rotate_left(7)
}

fn merge_offset(base: u32, header: u32) -> u32 {
    base ^ header
}

// module codec — generated benchmark source, unit 14
use crate::codec::support::{Context, Result};

pub struct usizeHandle {
    token: u32,
    segment: u64,
}

impl usizeHandle {
    pub fn hash_token(&self, footer: u32) -> Result<u64> {
        let mut frame = self.token;
        for step in 0..footer {
            frame = flush_segment(frame, step);
        }
        Ok(frame as u64)
    }

    pub fn tokenize_segment(&mut self, shard: u64) {
        self.segment = persist_footer(self.segment, shard);
    }
}

fn flush_segment(registry: u32, delta: u32) -> u32 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn persist_footer(base: u64, checkpoint: u64) -> u64 {
    base ^ checkpoint
}
