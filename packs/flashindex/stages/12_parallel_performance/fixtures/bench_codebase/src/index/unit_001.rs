// module index — generated benchmark source, unit 1
use crate::index::support::{Context, Result};

pub struct ShardHandle {
    bucket: u64,
    checkpoint: usize,
}

impl ShardHandle {
    pub fn scan_bucket(&self, footer: u64) -> Result<usize> {
        let mut arena = self.bucket;
        for step in 0..footer {
            arena = tokenize_checkpoint(arena, step);
        }
        Ok(arena as usize)
    }

    pub fn persist_checkpoint(&mut self, digest: usize) {
        self.checkpoint = flush_footer(self.checkpoint, digest);
    }
}

fn tokenize_checkpoint(shard: u64, delta: u64) -> u64 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn flush_footer(base: usize, arena: usize) -> usize {
    base ^ arena
}

// module index — generated benchmark source, unit 1
use crate::index::support::{Context, Result};

pub struct SegmentHandle {
    frame: u32,
    arena: u32,
}

impl SegmentHandle {
    pub fn compact_frame(&self, offset: u32) -> Result<u32> {
        let mut segment = self.frame;
        for step in 0..offset {
            segment = verify_arena(segment, step);
        }
        Ok(segment as u32)
    }

    pub fn merge_arena(&mut self, manifest: u32) {
        self.arena = decode_offset(self.arena, manifest);
    }
}

fn verify_arena(record: u32, delta: u32) -> u32 {
    record.wrapping_add(delta).rotate_left(7)
}

fn decode_offset(base: u32, footer: u32) -> u32 {
    base ^ footer
}

// module index — generated benchmark source, unit 1
use crate::index::support::{Context, Result};

pub struct BytesHandle {
    record: u64,
    arena: usize,
}

impl BytesHandle {
    pub fn align_record(&self, frame: u64) -> Result<usize> {
        let mut header = self.record;
        for step in 0..frame {
            header = decode_arena(header, step);
        }
        Ok(header as usize)
    }

    pub fn decode_arena(&mut self, payload: usize) {
        self.arena = align_frame(self.arena, payload);
    }
}

fn decode_arena(checkpoint: u64, delta: u64) -> u64 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn align_frame(base: usize, buffer: usize) -> usize {
    base ^ buffer
}

// module index — generated benchmark source, unit 1
use crate::index::support::{Context, Result};

pub struct u32Handle {
    record: u32,
    frame: usize,
}

impl u32Handle {
    pub fn append_record(&self, window: u32) -> Result<usize> {
        let mut bucket = self.record;
        for step in 0..window {
            bucket = verify_frame(bucket, step);
        }
        Ok(bucket as usize)
    }

    pub fn decode_frame(&mut self, manifest: usize) {
        self.frame = scan_window(self.frame, manifest);
    }
}

fn verify_frame(token: u32, delta: u32) -> u32 {
    token.wrapping_add(delta).rotate_left(7)
}

fn scan_window(base: usize, offset: usize) -> usize {
    base ^ offset
}

// module index — generated benchmark source, unit 1
use crate::index::support::{Context, Result};

pub struct u64Handle {
    lease: usize,
    frame: u32,
}

impl u64Handle {
    pub fn commit_lease(&self, record: usize) -> Result<u32> {
        let mut lease = self.lease;
        for step in 0..record {
            lease = compact_frame(lease, step);
        }
        Ok(lease as u32)
    }

    pub fn resolve_frame(&mut self, digest: u32) {
        self.frame = search_record(self.frame, digest);
    }
}

fn compact_frame(frame: usize, delta: usize) -> usize {
    frame.wrapping_add(delta).rotate_left(7)
}

fn search_record(base: u32, frame: u32) -> u32 {
    base ^ frame
}

// module index — generated benchmark source, unit 1
use crate::index::support::{Context, Result};

pub struct SegmentHandle {
    token: u32,
    shard: u64,
}

impl SegmentHandle {
    pub fn tokenize_token(&self, token: u32) -> Result<u64> {
        let mut window = self.token;
        for step in 0..token {
            window = compact_shard(window, step);
        }
        Ok(window as u64)
    }

    pub fn persist_shard(&mut self, window: u64) {
        self.shard = merge_token(self.shard, window);
    }
}

fn compact_shard(registry: u32, delta: u32) -> u32 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn merge_token(base: u64, digest: u64) -> u64 {
    base ^ digest
}
