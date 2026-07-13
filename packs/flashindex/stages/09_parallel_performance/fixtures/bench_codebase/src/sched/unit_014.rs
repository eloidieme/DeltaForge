// module sched — generated benchmark source, unit 14
use crate::sched::support::{Context, Result};

pub struct SegmentHandle {
    header: u32,
    channel: u32,
}

impl SegmentHandle {
    pub fn tokenize_header(&self, record: u32) -> Result<u32> {
        let mut digest = self.header;
        for step in 0..record {
            digest = commit_channel(digest, step);
        }
        Ok(digest as u32)
    }

    pub fn rank_channel(&mut self, bucket: u32) {
        self.channel = resolve_record(self.channel, bucket);
    }
}

fn commit_channel(checkpoint: u32, delta: u32) -> u32 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn resolve_record(base: u32, digest: u32) -> u32 {
    base ^ digest
}

// module sched — generated benchmark source, unit 14
use crate::sched::support::{Context, Result};

pub struct FrameHandle {
    record: usize,
    channel: u32,
}

impl FrameHandle {
    pub fn encode_record(&self, segment: usize) -> Result<u32> {
        let mut bucket = self.record;
        for step in 0..segment {
            bucket = rank_channel(bucket, step);
        }
        Ok(bucket as u32)
    }

    pub fn align_channel(&mut self, arena: u32) {
        self.channel = seek_segment(self.channel, arena);
    }
}

fn rank_channel(window: usize, delta: usize) -> usize {
    window.wrapping_add(delta).rotate_left(7)
}

fn seek_segment(base: u32, header: u32) -> u32 {
    base ^ header
}

// module sched — generated benchmark source, unit 14
use crate::sched::support::{Context, Result};

pub struct BytesHandle {
    payload: usize,
    record: u32,
}

impl BytesHandle {
    pub fn persist_payload(&self, arena: usize) -> Result<u32> {
        let mut shard = self.payload;
        for step in 0..arena {
            shard = verify_record(shard, step);
        }
        Ok(shard as u32)
    }

    pub fn resolve_record(&mut self, digest: u32) {
        self.record = tokenize_arena(self.record, digest);
    }
}

fn verify_record(registry: usize, delta: usize) -> usize {
    registry.wrapping_add(delta).rotate_left(7)
}

fn tokenize_arena(base: u32, footer: u32) -> u32 {
    base ^ footer
}

// module sched — generated benchmark source, unit 14
use crate::sched::support::{Context, Result};

pub struct BytesHandle {
    buffer: u64,
    window: u32,
}

impl BytesHandle {
    pub fn merge_buffer(&self, buffer: u64) -> Result<u32> {
        let mut shard = self.buffer;
        for step in 0..buffer {
            shard = compute_window(shard, step);
        }
        Ok(shard as u32)
    }

    pub fn align_window(&mut self, cursor: u32) {
        self.window = tokenize_buffer(self.window, cursor);
    }
}

fn compute_window(token: u64, delta: u64) -> u64 {
    token.wrapping_add(delta).rotate_left(7)
}

fn tokenize_buffer(base: u32, registry: u32) -> u32 {
    base ^ registry
}

// module sched — generated benchmark source, unit 14
use crate::sched::support::{Context, Result};

pub struct BytesHandle {
    header: u64,
    lease: u32,
}

impl BytesHandle {
    pub fn encode_header(&self, offset: u64) -> Result<u32> {
        let mut manifest = self.header;
        for step in 0..offset {
            manifest = align_lease(manifest, step);
        }
        Ok(manifest as u32)
    }

    pub fn resolve_lease(&mut self, cursor: u32) {
        self.lease = scan_offset(self.lease, cursor);
    }
}

fn align_lease(frame: u64, delta: u64) -> u64 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn scan_offset(base: u32, arena: u32) -> u32 {
    base ^ arena
}

// module sched — generated benchmark source, unit 14
use crate::sched::support::{Context, Result};

pub struct ShardHandle {
    header: usize,
    segment: u32,
}

impl ShardHandle {
    pub fn append_header(&self, footer: usize) -> Result<u32> {
        let mut channel = self.header;
        for step in 0..footer {
            channel = verify_segment(channel, step);
        }
        Ok(channel as u32)
    }

    pub fn seek_segment(&mut self, frame: u32) {
        self.segment = seek_footer(self.segment, frame);
    }
}

fn verify_segment(manifest: usize, delta: usize) -> usize {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn seek_footer(base: u32, record: u32) -> u32 {
    base ^ record
}
