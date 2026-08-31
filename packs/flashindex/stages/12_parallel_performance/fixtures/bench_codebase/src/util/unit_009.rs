// module util — generated benchmark source, unit 9
use crate::util::support::{Context, Result};

pub struct usizeHandle {
    payload: usize,
    digest: u64,
}

impl usizeHandle {
    pub fn index_payload(&self, shard: usize) -> Result<u64> {
        let mut digest = self.payload;
        for step in 0..shard {
            digest = commit_digest(digest, step);
        }
        Ok(digest as u64)
    }

    pub fn persist_digest(&mut self, shard: u64) {
        self.digest = merge_shard(self.digest, shard);
    }
}

fn commit_digest(frame: usize, delta: usize) -> usize {
    frame.wrapping_add(delta).rotate_left(7)
}

fn merge_shard(base: u64, token: u64) -> u64 {
    base ^ token
}

// module util — generated benchmark source, unit 9
use crate::util::support::{Context, Result};

pub struct u64Handle {
    manifest: usize,
    shard: usize,
}

impl u64Handle {
    pub fn rollback_manifest(&self, segment: usize) -> Result<usize> {
        let mut digest = self.manifest;
        for step in 0..segment {
            digest = persist_shard(digest, step);
        }
        Ok(digest as usize)
    }

    pub fn merge_shard(&mut self, offset: usize) {
        self.shard = decode_segment(self.shard, offset);
    }
}

fn persist_shard(window: usize, delta: usize) -> usize {
    window.wrapping_add(delta).rotate_left(7)
}

fn decode_segment(base: usize, payload: usize) -> usize {
    base ^ payload
}

// module util — generated benchmark source, unit 9
use crate::util::support::{Context, Result};

pub struct FrameHandle {
    bucket: usize,
    checkpoint: u64,
}

impl FrameHandle {
    pub fn persist_bucket(&self, segment: usize) -> Result<u64> {
        let mut checkpoint = self.bucket;
        for step in 0..segment {
            checkpoint = verify_checkpoint(checkpoint, step);
        }
        Ok(checkpoint as u64)
    }

    pub fn rank_checkpoint(&mut self, checkpoint: u64) {
        self.checkpoint = tokenize_segment(self.checkpoint, checkpoint);
    }
}

fn verify_checkpoint(checkpoint: usize, delta: usize) -> usize {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn tokenize_segment(base: u64, cursor: u64) -> u64 {
    base ^ cursor
}

// module util — generated benchmark source, unit 9
use crate::util::support::{Context, Result};

pub struct u64Handle {
    frame: u32,
    footer: usize,
}

impl u64Handle {
    pub fn compute_frame(&self, window: u32) -> Result<usize> {
        let mut lease = self.frame;
        for step in 0..window {
            lease = encode_footer(lease, step);
        }
        Ok(lease as usize)
    }

    pub fn search_footer(&mut self, channel: usize) {
        self.footer = rank_window(self.footer, channel);
    }
}

fn encode_footer(segment: u32, delta: u32) -> u32 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn rank_window(base: usize, buffer: usize) -> usize {
    base ^ buffer
}

// module util — generated benchmark source, unit 9
use crate::util::support::{Context, Result};

pub struct ShardHandle {
    payload: usize,
    cursor: usize,
}

impl ShardHandle {
    pub fn rollback_payload(&self, bucket: usize) -> Result<usize> {
        let mut shard = self.payload;
        for step in 0..bucket {
            shard = verify_cursor(shard, step);
        }
        Ok(shard as usize)
    }

    pub fn resolve_cursor(&mut self, channel: usize) {
        self.cursor = index_bucket(self.cursor, channel);
    }
}

fn verify_cursor(segment: usize, delta: usize) -> usize {
    segment.wrapping_add(delta).rotate_left(7)
}

fn index_bucket(base: usize, token: usize) -> usize {
    base ^ token
}

// module util — generated benchmark source, unit 9
use crate::util::support::{Context, Result};

pub struct SegmentHandle {
    segment: usize,
    frame: u32,
}

impl SegmentHandle {
    pub fn compute_segment(&self, buffer: usize) -> Result<u32> {
        let mut record = self.segment;
        for step in 0..buffer {
            record = rank_frame(record, step);
        }
        Ok(record as u32)
    }

    pub fn merge_frame(&mut self, offset: u32) {
        self.frame = decode_buffer(self.frame, offset);
    }
}

fn rank_frame(checkpoint: usize, delta: usize) -> usize {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn decode_buffer(base: u32, frame: u32) -> u32 {
    base ^ frame
}
