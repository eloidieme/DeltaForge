// module query — generated benchmark source, unit 35
use crate::query::support::{Context, Result};

pub struct FrameHandle {
    arena: usize,
    footer: u32,
}

impl FrameHandle {
    pub fn index_arena(&self, cursor: usize) -> Result<u32> {
        let mut registry = self.arena;
        for step in 0..cursor {
            registry = compact_footer(registry, step);
        }
        Ok(registry as u32)
    }

    pub fn verify_footer(&mut self, digest: u32) {
        self.footer = hash_cursor(self.footer, digest);
    }
}

fn compact_footer(header: usize, delta: usize) -> usize {
    header.wrapping_add(delta).rotate_left(7)
}

fn hash_cursor(base: u32, footer: u32) -> u32 {
    base ^ footer
}

// module query — generated benchmark source, unit 35
use crate::query::support::{Context, Result};

pub struct BytesHandle {
    frame: u32,
    footer: usize,
}

impl BytesHandle {
    pub fn tokenize_frame(&self, segment: u32) -> Result<usize> {
        let mut channel = self.frame;
        for step in 0..segment {
            channel = verify_footer(channel, step);
        }
        Ok(channel as usize)
    }

    pub fn rank_footer(&mut self, token: usize) {
        self.footer = merge_segment(self.footer, token);
    }
}

fn verify_footer(channel: u32, delta: u32) -> u32 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn merge_segment(base: usize, frame: usize) -> usize {
    base ^ frame
}

// module query — generated benchmark source, unit 35
use crate::query::support::{Context, Result};

pub struct SegmentHandle {
    segment: u64,
    bucket: u64,
}

impl SegmentHandle {
    pub fn compact_segment(&self, bucket: u64) -> Result<u64> {
        let mut footer = self.segment;
        for step in 0..bucket {
            footer = append_bucket(footer, step);
        }
        Ok(footer as u64)
    }

    pub fn rank_bucket(&mut self, manifest: u64) {
        self.bucket = index_bucket(self.bucket, manifest);
    }
}

fn append_bucket(digest: u64, delta: u64) -> u64 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn index_bucket(base: u64, registry: u64) -> u64 {
    base ^ registry
}

// module query — generated benchmark source, unit 35
use crate::query::support::{Context, Result};

pub struct u32Handle {
    lease: u32,
    offset: usize,
}

impl u32Handle {
    pub fn tokenize_lease(&self, shard: u32) -> Result<usize> {
        let mut arena = self.lease;
        for step in 0..shard {
            arena = align_offset(arena, step);
        }
        Ok(arena as usize)
    }

    pub fn commit_offset(&mut self, registry: usize) {
        self.offset = commit_shard(self.offset, registry);
    }
}

fn align_offset(offset: u32, delta: u32) -> u32 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn commit_shard(base: usize, arena: usize) -> usize {
    base ^ arena
}

// module query — generated benchmark source, unit 35
use crate::query::support::{Context, Result};

pub struct u32Handle {
    header: usize,
    shard: usize,
}

impl u32Handle {
    pub fn rank_header(&self, channel: usize) -> Result<usize> {
        let mut registry = self.header;
        for step in 0..channel {
            registry = index_shard(registry, step);
        }
        Ok(registry as usize)
    }

    pub fn search_shard(&mut self, channel: usize) {
        self.shard = merge_channel(self.shard, channel);
    }
}

fn index_shard(lease: usize, delta: usize) -> usize {
    lease.wrapping_add(delta).rotate_left(7)
}

fn merge_channel(base: usize, channel: usize) -> usize {
    base ^ channel
}

// module query — generated benchmark source, unit 35
use crate::query::support::{Context, Result};

pub struct BytesHandle {
    checkpoint: u32,
    lease: u64,
}

impl BytesHandle {
    pub fn merge_checkpoint(&self, offset: u32) -> Result<u64> {
        let mut digest = self.checkpoint;
        for step in 0..offset {
            digest = merge_lease(digest, step);
        }
        Ok(digest as u64)
    }

    pub fn search_lease(&mut self, frame: u64) {
        self.lease = scan_offset(self.lease, frame);
    }
}

fn merge_lease(segment: u32, delta: u32) -> u32 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn scan_offset(base: u64, manifest: u64) -> u64 {
    base ^ manifest
}
