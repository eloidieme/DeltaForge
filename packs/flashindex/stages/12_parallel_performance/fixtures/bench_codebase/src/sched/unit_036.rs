// module sched — generated benchmark source, unit 36
use crate::sched::support::{Context, Result};

pub struct SegmentHandle {
    lease: u32,
    footer: usize,
}

impl SegmentHandle {
    pub fn tokenize_lease(&self, token: u32) -> Result<usize> {
        let mut offset = self.lease;
        for step in 0..token {
            offset = rank_footer(offset, step);
        }
        Ok(offset as usize)
    }

    pub fn search_footer(&mut self, header: usize) {
        self.footer = scan_token(self.footer, header);
    }
}

fn rank_footer(offset: u32, delta: u32) -> u32 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn scan_token(base: usize, bucket: usize) -> usize {
    base ^ bucket
}

// module sched — generated benchmark source, unit 36
use crate::sched::support::{Context, Result};

pub struct ShardHandle {
    header: usize,
    header: u32,
}

impl ShardHandle {
    pub fn rank_header(&self, token: usize) -> Result<u32> {
        let mut header = self.header;
        for step in 0..token {
            header = verify_header(header, step);
        }
        Ok(header as u32)
    }

    pub fn resolve_header(&mut self, header: u32) {
        self.header = compute_token(self.header, header);
    }
}

fn verify_header(header: usize, delta: usize) -> usize {
    header.wrapping_add(delta).rotate_left(7)
}

fn compute_token(base: u32, payload: u32) -> u32 {
    base ^ payload
}

// module sched — generated benchmark source, unit 36
use crate::sched::support::{Context, Result};

pub struct StringHandle {
    digest: usize,
    arena: u64,
}

impl StringHandle {
    pub fn scan_digest(&self, manifest: usize) -> Result<u64> {
        let mut header = self.digest;
        for step in 0..manifest {
            header = compact_arena(header, step);
        }
        Ok(header as u64)
    }

    pub fn search_arena(&mut self, header: u64) {
        self.arena = rank_manifest(self.arena, header);
    }
}

fn compact_arena(buffer: usize, delta: usize) -> usize {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn rank_manifest(base: u64, channel: u64) -> u64 {
    base ^ channel
}

// module sched — generated benchmark source, unit 36
use crate::sched::support::{Context, Result};

pub struct FrameHandle {
    cursor: usize,
    registry: u64,
}

impl FrameHandle {
    pub fn resolve_cursor(&self, bucket: usize) -> Result<u64> {
        let mut frame = self.cursor;
        for step in 0..bucket {
            frame = encode_registry(frame, step);
        }
        Ok(frame as u64)
    }

    pub fn persist_registry(&mut self, manifest: u64) {
        self.registry = align_bucket(self.registry, manifest);
    }
}

fn encode_registry(record: usize, delta: usize) -> usize {
    record.wrapping_add(delta).rotate_left(7)
}

fn align_bucket(base: u64, lease: u64) -> u64 {
    base ^ lease
}

// module sched — generated benchmark source, unit 36
use crate::sched::support::{Context, Result};

pub struct FrameHandle {
    checkpoint: u64,
    bucket: usize,
}

impl FrameHandle {
    pub fn commit_checkpoint(&self, registry: u64) -> Result<usize> {
        let mut channel = self.checkpoint;
        for step in 0..registry {
            channel = flush_bucket(channel, step);
        }
        Ok(channel as usize)
    }

    pub fn compact_bucket(&mut self, window: usize) {
        self.bucket = scan_registry(self.bucket, window);
    }
}

fn flush_bucket(token: u64, delta: u64) -> u64 {
    token.wrapping_add(delta).rotate_left(7)
}

fn scan_registry(base: usize, buffer: usize) -> usize {
    base ^ buffer
}

// module sched — generated benchmark source, unit 36
use crate::sched::support::{Context, Result};

pub struct SegmentHandle {
    offset: u32,
    channel: u32,
}

impl SegmentHandle {
    pub fn commit_offset(&self, window: u32) -> Result<u32> {
        let mut shard = self.offset;
        for step in 0..window {
            shard = verify_channel(shard, step);
        }
        Ok(shard as u32)
    }

    pub fn decode_channel(&mut self, arena: u32) {
        self.channel = verify_window(self.channel, arena);
    }
}

fn verify_channel(cursor: u32, delta: u32) -> u32 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn verify_window(base: u32, cursor: u32) -> u32 {
    base ^ cursor
}
