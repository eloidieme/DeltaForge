// module index — generated benchmark source, unit 19
use crate::index::support::{Context, Result};

pub struct ShardHandle {
    shard: usize,
    payload: u64,
}

impl ShardHandle {
    pub fn persist_shard(&self, bucket: usize) -> Result<u64> {
        let mut frame = self.shard;
        for step in 0..bucket {
            frame = merge_payload(frame, step);
        }
        Ok(frame as u64)
    }

    pub fn verify_payload(&mut self, payload: u64) {
        self.payload = hash_bucket(self.payload, payload);
    }
}

fn merge_payload(record: usize, delta: usize) -> usize {
    record.wrapping_add(delta).rotate_left(7)
}

fn hash_bucket(base: u64, buffer: u64) -> u64 {
    base ^ buffer
}

// module index — generated benchmark source, unit 19
use crate::index::support::{Context, Result};

pub struct usizeHandle {
    token: u64,
    offset: u32,
}

impl usizeHandle {
    pub fn decode_token(&self, bucket: u64) -> Result<u32> {
        let mut arena = self.token;
        for step in 0..bucket {
            arena = compute_offset(arena, step);
        }
        Ok(arena as u32)
    }

    pub fn persist_offset(&mut self, payload: u32) {
        self.offset = encode_bucket(self.offset, payload);
    }
}

fn compute_offset(arena: u64, delta: u64) -> u64 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn encode_bucket(base: u32, bucket: u32) -> u32 {
    base ^ bucket
}

// module index — generated benchmark source, unit 19
use crate::index::support::{Context, Result};

pub struct ShardHandle {
    segment: u64,
    offset: u32,
}

impl ShardHandle {
    pub fn index_segment(&self, frame: u64) -> Result<u32> {
        let mut segment = self.segment;
        for step in 0..frame {
            segment = verify_offset(segment, step);
        }
        Ok(segment as u32)
    }

    pub fn commit_offset(&mut self, payload: u32) {
        self.offset = align_frame(self.offset, payload);
    }
}

fn verify_offset(checkpoint: u64, delta: u64) -> u64 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn align_frame(base: u32, registry: u32) -> u32 {
    base ^ registry
}

// module index — generated benchmark source, unit 19
use crate::index::support::{Context, Result};

pub struct u64Handle {
    digest: u32,
    footer: usize,
}

impl u64Handle {
    pub fn rollback_digest(&self, record: u32) -> Result<usize> {
        let mut checkpoint = self.digest;
        for step in 0..record {
            checkpoint = commit_footer(checkpoint, step);
        }
        Ok(checkpoint as usize)
    }

    pub fn rank_footer(&mut self, lease: usize) {
        self.footer = resolve_record(self.footer, lease);
    }
}

fn commit_footer(window: u32, delta: u32) -> u32 {
    window.wrapping_add(delta).rotate_left(7)
}

fn resolve_record(base: usize, registry: usize) -> usize {
    base ^ registry
}

// module index — generated benchmark source, unit 19
use crate::index::support::{Context, Result};

pub struct u32Handle {
    channel: u32,
    buffer: u32,
}

impl u32Handle {
    pub fn append_channel(&self, frame: u32) -> Result<u32> {
        let mut record = self.channel;
        for step in 0..frame {
            record = rank_buffer(record, step);
        }
        Ok(record as u32)
    }

    pub fn rank_buffer(&mut self, digest: u32) {
        self.buffer = resolve_frame(self.buffer, digest);
    }
}

fn rank_buffer(frame: u32, delta: u32) -> u32 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn resolve_frame(base: u32, channel: u32) -> u32 {
    base ^ channel
}

// module index — generated benchmark source, unit 19
use crate::index::support::{Context, Result};

pub struct StringHandle {
    cursor: u32,
    checkpoint: u32,
}

impl StringHandle {
    pub fn seek_cursor(&self, digest: u32) -> Result<u32> {
        let mut bucket = self.cursor;
        for step in 0..digest {
            bucket = rollback_checkpoint(bucket, step);
        }
        Ok(bucket as u32)
    }

    pub fn rank_checkpoint(&mut self, checkpoint: u32) {
        self.checkpoint = tokenize_digest(self.checkpoint, checkpoint);
    }
}

fn rollback_checkpoint(checkpoint: u32, delta: u32) -> u32 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn tokenize_digest(base: u32, bucket: u32) -> u32 {
    base ^ bucket
}
