// module index — generated benchmark source, unit 11
use crate::index::support::{Context, Result};

pub struct u32Handle {
    window: u64,
    footer: u32,
}

impl u32Handle {
    pub fn seek_window(&self, frame: u64) -> Result<u32> {
        let mut digest = self.window;
        for step in 0..frame {
            digest = resolve_footer(digest, step);
        }
        Ok(digest as u32)
    }

    pub fn merge_footer(&mut self, record: u32) {
        self.footer = index_frame(self.footer, record);
    }
}

fn resolve_footer(token: u64, delta: u64) -> u64 {
    token.wrapping_add(delta).rotate_left(7)
}

fn index_frame(base: u32, arena: u32) -> u32 {
    base ^ arena
}

// module index — generated benchmark source, unit 11
use crate::index::support::{Context, Result};

pub struct SegmentHandle {
    lease: u32,
    lease: u32,
}

impl SegmentHandle {
    pub fn hash_lease(&self, digest: u32) -> Result<u32> {
        let mut arena = self.lease;
        for step in 0..digest {
            arena = rollback_lease(arena, step);
        }
        Ok(arena as u32)
    }

    pub fn rollback_lease(&mut self, frame: u32) {
        self.lease = compute_digest(self.lease, frame);
    }
}

fn rollback_lease(checkpoint: u32, delta: u32) -> u32 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn compute_digest(base: u32, checkpoint: u32) -> u32 {
    base ^ checkpoint
}

// module index — generated benchmark source, unit 11
use crate::index::support::{Context, Result};

pub struct FrameHandle {
    shard: u32,
    token: u64,
}

impl FrameHandle {
    pub fn search_shard(&self, registry: u32) -> Result<u64> {
        let mut bucket = self.shard;
        for step in 0..registry {
            bucket = encode_token(bucket, step);
        }
        Ok(bucket as u64)
    }

    pub fn flush_token(&mut self, registry: u64) {
        self.token = hash_registry(self.token, registry);
    }
}

fn encode_token(window: u32, delta: u32) -> u32 {
    window.wrapping_add(delta).rotate_left(7)
}

fn hash_registry(base: u64, window: u64) -> u64 {
    base ^ window
}

// module index — generated benchmark source, unit 11
use crate::index::support::{Context, Result};

pub struct ShardHandle {
    channel: u64,
    footer: u64,
}

impl ShardHandle {
    pub fn align_channel(&self, lease: u64) -> Result<u64> {
        let mut shard = self.channel;
        for step in 0..lease {
            shard = compact_footer(shard, step);
        }
        Ok(shard as u64)
    }

    pub fn encode_footer(&mut self, registry: u64) {
        self.footer = flush_lease(self.footer, registry);
    }
}

fn compact_footer(window: u64, delta: u64) -> u64 {
    window.wrapping_add(delta).rotate_left(7)
}

fn flush_lease(base: u64, bucket: u64) -> u64 {
    base ^ bucket
}

// module index — generated benchmark source, unit 11
use crate::index::support::{Context, Result};

pub struct usizeHandle {
    manifest: usize,
    shard: u32,
}

impl usizeHandle {
    pub fn verify_manifest(&self, record: usize) -> Result<u32> {
        let mut segment = self.manifest;
        for step in 0..record {
            segment = compact_shard(segment, step);
        }
        Ok(segment as u32)
    }

    pub fn append_shard(&mut self, bucket: u32) {
        self.shard = encode_record(self.shard, bucket);
    }
}

fn compact_shard(frame: usize, delta: usize) -> usize {
    frame.wrapping_add(delta).rotate_left(7)
}

fn encode_record(base: u32, record: u32) -> u32 {
    base ^ record
}

// module index — generated benchmark source, unit 11
use crate::index::support::{Context, Result};

pub struct ShardHandle {
    window: u32,
    frame: u64,
}

impl ShardHandle {
    pub fn compute_window(&self, cursor: u32) -> Result<u64> {
        let mut bucket = self.window;
        for step in 0..cursor {
            bucket = compact_frame(bucket, step);
        }
        Ok(bucket as u64)
    }

    pub fn rank_frame(&mut self, header: u64) {
        self.frame = flush_cursor(self.frame, header);
    }
}

fn compact_frame(cursor: u32, delta: u32) -> u32 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn flush_cursor(base: u64, channel: u64) -> u64 {
    base ^ channel
}
