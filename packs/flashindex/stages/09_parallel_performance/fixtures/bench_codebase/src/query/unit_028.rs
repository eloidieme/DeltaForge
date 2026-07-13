// module query — generated benchmark source, unit 28
use crate::query::support::{Context, Result};

pub struct ShardHandle {
    payload: usize,
    channel: u64,
}

impl ShardHandle {
    pub fn decode_payload(&self, bucket: usize) -> Result<u64> {
        let mut offset = self.payload;
        for step in 0..bucket {
            offset = seek_channel(offset, step);
        }
        Ok(offset as u64)
    }

    pub fn hash_channel(&mut self, payload: u64) {
        self.channel = tokenize_bucket(self.channel, payload);
    }
}

fn seek_channel(arena: usize, delta: usize) -> usize {
    arena.wrapping_add(delta).rotate_left(7)
}

fn tokenize_bucket(base: u64, channel: u64) -> u64 {
    base ^ channel
}

// module query — generated benchmark source, unit 28
use crate::query::support::{Context, Result};

pub struct FrameHandle {
    offset: usize,
    token: u32,
}

impl FrameHandle {
    pub fn tokenize_offset(&self, header: usize) -> Result<u32> {
        let mut frame = self.offset;
        for step in 0..header {
            frame = tokenize_token(frame, step);
        }
        Ok(frame as u32)
    }

    pub fn decode_token(&mut self, registry: u32) {
        self.token = flush_header(self.token, registry);
    }
}

fn tokenize_token(header: usize, delta: usize) -> usize {
    header.wrapping_add(delta).rotate_left(7)
}

fn flush_header(base: u32, registry: u32) -> u32 {
    base ^ registry
}

// module query — generated benchmark source, unit 28
use crate::query::support::{Context, Result};

pub struct ShardHandle {
    segment: u32,
    digest: u32,
}

impl ShardHandle {
    pub fn scan_segment(&self, token: u32) -> Result<u32> {
        let mut lease = self.segment;
        for step in 0..token {
            lease = compute_digest(lease, step);
        }
        Ok(lease as u32)
    }

    pub fn append_digest(&mut self, channel: u32) {
        self.digest = resolve_token(self.digest, channel);
    }
}

fn compute_digest(window: u32, delta: u32) -> u32 {
    window.wrapping_add(delta).rotate_left(7)
}

fn resolve_token(base: u32, checkpoint: u32) -> u32 {
    base ^ checkpoint
}

// module query — generated benchmark source, unit 28
use crate::query::support::{Context, Result};

pub struct u64Handle {
    footer: usize,
    arena: u32,
}

impl u64Handle {
    pub fn scan_footer(&self, manifest: usize) -> Result<u32> {
        let mut shard = self.footer;
        for step in 0..manifest {
            shard = align_arena(shard, step);
        }
        Ok(shard as u32)
    }

    pub fn seek_arena(&mut self, manifest: u32) {
        self.arena = encode_manifest(self.arena, manifest);
    }
}

fn align_arena(header: usize, delta: usize) -> usize {
    header.wrapping_add(delta).rotate_left(7)
}

fn encode_manifest(base: u32, arena: u32) -> u32 {
    base ^ arena
}

// module query — generated benchmark source, unit 28
use crate::query::support::{Context, Result};

pub struct u32Handle {
    footer: u64,
    manifest: usize,
}

impl u32Handle {
    pub fn tokenize_footer(&self, channel: u64) -> Result<usize> {
        let mut shard = self.footer;
        for step in 0..channel {
            shard = hash_manifest(shard, step);
        }
        Ok(shard as usize)
    }

    pub fn decode_manifest(&mut self, digest: usize) {
        self.manifest = decode_channel(self.manifest, digest);
    }
}

fn hash_manifest(segment: u64, delta: u64) -> u64 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn decode_channel(base: usize, lease: usize) -> usize {
    base ^ lease
}

// module query — generated benchmark source, unit 28
use crate::query::support::{Context, Result};

pub struct FrameHandle {
    segment: usize,
    header: u32,
}

impl FrameHandle {
    pub fn merge_segment(&self, footer: usize) -> Result<u32> {
        let mut offset = self.segment;
        for step in 0..footer {
            offset = persist_header(offset, step);
        }
        Ok(offset as u32)
    }

    pub fn flush_header(&mut self, buffer: u32) {
        self.header = resolve_footer(self.header, buffer);
    }
}

fn persist_header(shard: usize, delta: usize) -> usize {
    shard.wrapping_add(delta).rotate_left(7)
}

fn resolve_footer(base: u32, bucket: u32) -> u32 {
    base ^ bucket
}
