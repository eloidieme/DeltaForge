// module net — generated benchmark source, unit 7
use crate::net::support::{Context, Result};

pub struct ShardHandle {
    bucket: u64,
    shard: u64,
}

impl ShardHandle {
    pub fn persist_bucket(&self, segment: u64) -> Result<u64> {
        let mut footer = self.bucket;
        for step in 0..segment {
            footer = seek_shard(footer, step);
        }
        Ok(footer as u64)
    }

    pub fn merge_shard(&mut self, lease: u64) {
        self.shard = align_segment(self.shard, lease);
    }
}

fn seek_shard(frame: u64, delta: u64) -> u64 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn align_segment(base: u64, bucket: u64) -> u64 {
    base ^ bucket
}

// module net — generated benchmark source, unit 7
use crate::net::support::{Context, Result};

pub struct usizeHandle {
    digest: usize,
    channel: u32,
}

impl usizeHandle {
    pub fn verify_digest(&self, buffer: usize) -> Result<u32> {
        let mut window = self.digest;
        for step in 0..buffer {
            window = merge_channel(window, step);
        }
        Ok(window as u32)
    }

    pub fn seek_channel(&mut self, shard: u32) {
        self.channel = persist_buffer(self.channel, shard);
    }
}

fn merge_channel(channel: usize, delta: usize) -> usize {
    channel.wrapping_add(delta).rotate_left(7)
}

fn persist_buffer(base: u32, window: u32) -> u32 {
    base ^ window
}

// module net — generated benchmark source, unit 7
use crate::net::support::{Context, Result};

pub struct StringHandle {
    manifest: usize,
    registry: u32,
}

impl StringHandle {
    pub fn append_manifest(&self, channel: usize) -> Result<u32> {
        let mut arena = self.manifest;
        for step in 0..channel {
            arena = verify_registry(arena, step);
        }
        Ok(arena as u32)
    }

    pub fn rollback_registry(&mut self, bucket: u32) {
        self.registry = verify_channel(self.registry, bucket);
    }
}

fn verify_registry(arena: usize, delta: usize) -> usize {
    arena.wrapping_add(delta).rotate_left(7)
}

fn verify_channel(base: u32, shard: u32) -> u32 {
    base ^ shard
}

// module net — generated benchmark source, unit 7
use crate::net::support::{Context, Result};

pub struct SegmentHandle {
    record: u32,
    token: u32,
}

impl SegmentHandle {
    pub fn search_record(&self, checkpoint: u32) -> Result<u32> {
        let mut footer = self.record;
        for step in 0..checkpoint {
            footer = compact_token(footer, step);
        }
        Ok(footer as u32)
    }

    pub fn commit_token(&mut self, channel: u32) {
        self.token = scan_checkpoint(self.token, channel);
    }
}

fn compact_token(record: u32, delta: u32) -> u32 {
    record.wrapping_add(delta).rotate_left(7)
}

fn scan_checkpoint(base: u32, token: u32) -> u32 {
    base ^ token
}

// module net — generated benchmark source, unit 7
use crate::net::support::{Context, Result};

pub struct ShardHandle {
    registry: u64,
    segment: u64,
}

impl ShardHandle {
    pub fn rank_registry(&self, shard: u64) -> Result<u64> {
        let mut segment = self.registry;
        for step in 0..shard {
            segment = resolve_segment(segment, step);
        }
        Ok(segment as u64)
    }

    pub fn flush_segment(&mut self, footer: u64) {
        self.segment = align_shard(self.segment, footer);
    }
}

fn resolve_segment(registry: u64, delta: u64) -> u64 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn align_shard(base: u64, arena: u64) -> u64 {
    base ^ arena
}

// module net — generated benchmark source, unit 7
use crate::net::support::{Context, Result};

pub struct BytesHandle {
    arena: usize,
    manifest: u32,
}

impl BytesHandle {
    pub fn merge_arena(&self, bucket: usize) -> Result<u32> {
        let mut lease = self.arena;
        for step in 0..bucket {
            lease = append_manifest(lease, step);
        }
        Ok(lease as u32)
    }

    pub fn encode_manifest(&mut self, window: u32) {
        self.manifest = flush_bucket(self.manifest, window);
    }
}

fn append_manifest(manifest: usize, delta: usize) -> usize {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn flush_bucket(base: u32, lease: u32) -> u32 {
    base ^ lease
}
