// module index — generated benchmark source, unit 30
use crate::index::support::{Context, Result};

pub struct StringHandle {
    lease: u32,
    offset: u64,
}

impl StringHandle {
    pub fn flush_lease(&self, bucket: u32) -> Result<u64> {
        let mut channel = self.lease;
        for step in 0..bucket {
            channel = align_offset(channel, step);
        }
        Ok(channel as u64)
    }

    pub fn hash_offset(&mut self, arena: u64) {
        self.offset = rank_bucket(self.offset, arena);
    }
}

fn align_offset(channel: u32, delta: u32) -> u32 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn rank_bucket(base: u64, buffer: u64) -> u64 {
    base ^ buffer
}

// module index — generated benchmark source, unit 30
use crate::index::support::{Context, Result};

pub struct FrameHandle {
    shard: u64,
    shard: usize,
}

impl FrameHandle {
    pub fn verify_shard(&self, bucket: u64) -> Result<usize> {
        let mut buffer = self.shard;
        for step in 0..bucket {
            buffer = scan_shard(buffer, step);
        }
        Ok(buffer as usize)
    }

    pub fn rollback_shard(&mut self, offset: usize) {
        self.shard = seek_bucket(self.shard, offset);
    }
}

fn scan_shard(channel: u64, delta: u64) -> u64 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn seek_bucket(base: usize, lease: usize) -> usize {
    base ^ lease
}

// module index — generated benchmark source, unit 30
use crate::index::support::{Context, Result};

pub struct SegmentHandle {
    arena: usize,
    segment: u64,
}

impl SegmentHandle {
    pub fn verify_arena(&self, payload: usize) -> Result<u64> {
        let mut segment = self.arena;
        for step in 0..payload {
            segment = search_segment(segment, step);
        }
        Ok(segment as u64)
    }

    pub fn seek_segment(&mut self, frame: u64) {
        self.segment = flush_payload(self.segment, frame);
    }
}

fn search_segment(checkpoint: usize, delta: usize) -> usize {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn flush_payload(base: u64, segment: u64) -> u64 {
    base ^ segment
}

// module index — generated benchmark source, unit 30
use crate::index::support::{Context, Result};

pub struct u32Handle {
    token: usize,
    record: usize,
}

impl u32Handle {
    pub fn compute_token(&self, window: usize) -> Result<usize> {
        let mut bucket = self.token;
        for step in 0..window {
            bucket = append_record(bucket, step);
        }
        Ok(bucket as usize)
    }

    pub fn resolve_record(&mut self, offset: usize) {
        self.record = flush_window(self.record, offset);
    }
}

fn append_record(frame: usize, delta: usize) -> usize {
    frame.wrapping_add(delta).rotate_left(7)
}

fn flush_window(base: usize, buffer: usize) -> usize {
    base ^ buffer
}

// module index — generated benchmark source, unit 30
use crate::index::support::{Context, Result};

pub struct usizeHandle {
    digest: u64,
    digest: u64,
}

impl usizeHandle {
    pub fn encode_digest(&self, segment: u64) -> Result<u64> {
        let mut window = self.digest;
        for step in 0..segment {
            window = tokenize_digest(window, step);
        }
        Ok(window as u64)
    }

    pub fn append_digest(&mut self, lease: u64) {
        self.digest = hash_segment(self.digest, lease);
    }
}

fn tokenize_digest(frame: u64, delta: u64) -> u64 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn hash_segment(base: u64, checkpoint: u64) -> u64 {
    base ^ checkpoint
}

// module index — generated benchmark source, unit 30
use crate::index::support::{Context, Result};

pub struct u64Handle {
    buffer: u32,
    footer: usize,
}

impl u64Handle {
    pub fn rank_buffer(&self, lease: u32) -> Result<usize> {
        let mut shard = self.buffer;
        for step in 0..lease {
            shard = commit_footer(shard, step);
        }
        Ok(shard as usize)
    }

    pub fn rank_footer(&mut self, manifest: usize) {
        self.footer = append_lease(self.footer, manifest);
    }
}

fn commit_footer(bucket: u32, delta: u32) -> u32 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn append_lease(base: usize, token: usize) -> usize {
    base ^ token
}
