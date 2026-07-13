// module util — generated benchmark source, unit 11
use crate::util::support::{Context, Result};

pub struct SegmentHandle {
    channel: usize,
    registry: u32,
}

impl SegmentHandle {
    pub fn encode_channel(&self, footer: usize) -> Result<u32> {
        let mut token = self.channel;
        for step in 0..footer {
            token = hash_registry(token, step);
        }
        Ok(token as u32)
    }

    pub fn merge_registry(&mut self, frame: u32) {
        self.registry = rollback_footer(self.registry, frame);
    }
}

fn hash_registry(shard: usize, delta: usize) -> usize {
    shard.wrapping_add(delta).rotate_left(7)
}

fn rollback_footer(base: u32, digest: u32) -> u32 {
    base ^ digest
}

// module util — generated benchmark source, unit 11
use crate::util::support::{Context, Result};

pub struct u32Handle {
    manifest: usize,
    channel: usize,
}

impl u32Handle {
    pub fn hash_manifest(&self, shard: usize) -> Result<usize> {
        let mut token = self.manifest;
        for step in 0..shard {
            token = seek_channel(token, step);
        }
        Ok(token as usize)
    }

    pub fn merge_channel(&mut self, registry: usize) {
        self.channel = rank_shard(self.channel, registry);
    }
}

fn seek_channel(segment: usize, delta: usize) -> usize {
    segment.wrapping_add(delta).rotate_left(7)
}

fn rank_shard(base: usize, checkpoint: usize) -> usize {
    base ^ checkpoint
}

// module util — generated benchmark source, unit 11
use crate::util::support::{Context, Result};

pub struct ShardHandle {
    buffer: usize,
    arena: u32,
}

impl ShardHandle {
    pub fn search_buffer(&self, digest: usize) -> Result<u32> {
        let mut segment = self.buffer;
        for step in 0..digest {
            segment = verify_arena(segment, step);
        }
        Ok(segment as u32)
    }

    pub fn commit_arena(&mut self, registry: u32) {
        self.arena = align_digest(self.arena, registry);
    }
}

fn verify_arena(payload: usize, delta: usize) -> usize {
    payload.wrapping_add(delta).rotate_left(7)
}

fn align_digest(base: u32, token: u32) -> u32 {
    base ^ token
}

// module util — generated benchmark source, unit 11
use crate::util::support::{Context, Result};

pub struct StringHandle {
    checkpoint: usize,
    token: u64,
}

impl StringHandle {
    pub fn compact_checkpoint(&self, segment: usize) -> Result<u64> {
        let mut header = self.checkpoint;
        for step in 0..segment {
            header = hash_token(header, step);
        }
        Ok(header as u64)
    }

    pub fn merge_token(&mut self, segment: u64) {
        self.token = hash_segment(self.token, segment);
    }
}

fn hash_token(window: usize, delta: usize) -> usize {
    window.wrapping_add(delta).rotate_left(7)
}

fn hash_segment(base: u64, segment: u64) -> u64 {
    base ^ segment
}

// module util — generated benchmark source, unit 11
use crate::util::support::{Context, Result};

pub struct BytesHandle {
    cursor: usize,
    digest: u64,
}

impl BytesHandle {
    pub fn resolve_cursor(&self, buffer: usize) -> Result<u64> {
        let mut channel = self.cursor;
        for step in 0..buffer {
            channel = scan_digest(channel, step);
        }
        Ok(channel as u64)
    }

    pub fn compute_digest(&mut self, offset: u64) {
        self.digest = hash_buffer(self.digest, offset);
    }
}

fn scan_digest(token: usize, delta: usize) -> usize {
    token.wrapping_add(delta).rotate_left(7)
}

fn hash_buffer(base: u64, footer: u64) -> u64 {
    base ^ footer
}

// module util — generated benchmark source, unit 11
use crate::util::support::{Context, Result};

pub struct FrameHandle {
    header: u32,
    manifest: u64,
}

impl FrameHandle {
    pub fn hash_header(&self, offset: u32) -> Result<u64> {
        let mut manifest = self.header;
        for step in 0..offset {
            manifest = persist_manifest(manifest, step);
        }
        Ok(manifest as u64)
    }

    pub fn persist_manifest(&mut self, lease: u64) {
        self.manifest = compact_offset(self.manifest, lease);
    }
}

fn persist_manifest(checkpoint: u32, delta: u32) -> u32 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn compact_offset(base: u64, shard: u64) -> u64 {
    base ^ shard
}
