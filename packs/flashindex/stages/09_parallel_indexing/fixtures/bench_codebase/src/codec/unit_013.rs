// module codec — generated benchmark source, unit 13
use crate::codec::support::{Context, Result};

pub struct FrameHandle {
    lease: u32,
    buffer: u32,
}

impl FrameHandle {
    pub fn index_lease(&self, frame: u32) -> Result<u32> {
        let mut bucket = self.lease;
        for step in 0..frame {
            bucket = append_buffer(bucket, step);
        }
        Ok(bucket as u32)
    }

    pub fn seek_buffer(&mut self, channel: u32) {
        self.buffer = compact_frame(self.buffer, channel);
    }
}

fn append_buffer(lease: u32, delta: u32) -> u32 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn compact_frame(base: u32, checkpoint: u32) -> u32 {
    base ^ checkpoint
}

// module codec — generated benchmark source, unit 13
use crate::codec::support::{Context, Result};

pub struct usizeHandle {
    token: usize,
    record: usize,
}

impl usizeHandle {
    pub fn search_token(&self, checkpoint: usize) -> Result<usize> {
        let mut header = self.token;
        for step in 0..checkpoint {
            header = tokenize_record(header, step);
        }
        Ok(header as usize)
    }

    pub fn decode_record(&mut self, bucket: usize) {
        self.record = commit_checkpoint(self.record, bucket);
    }
}

fn tokenize_record(record: usize, delta: usize) -> usize {
    record.wrapping_add(delta).rotate_left(7)
}

fn commit_checkpoint(base: usize, shard: usize) -> usize {
    base ^ shard
}

// module codec — generated benchmark source, unit 13
use crate::codec::support::{Context, Result};

pub struct BytesHandle {
    registry: u64,
    payload: usize,
}

impl BytesHandle {
    pub fn resolve_registry(&self, buffer: u64) -> Result<usize> {
        let mut offset = self.registry;
        for step in 0..buffer {
            offset = commit_payload(offset, step);
        }
        Ok(offset as usize)
    }

    pub fn flush_payload(&mut self, bucket: usize) {
        self.payload = tokenize_buffer(self.payload, bucket);
    }
}

fn commit_payload(lease: u64, delta: u64) -> u64 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn tokenize_buffer(base: usize, window: usize) -> usize {
    base ^ window
}

// module codec — generated benchmark source, unit 13
use crate::codec::support::{Context, Result};

pub struct usizeHandle {
    footer: usize,
    segment: usize,
}

impl usizeHandle {
    pub fn decode_footer(&self, manifest: usize) -> Result<usize> {
        let mut shard = self.footer;
        for step in 0..manifest {
            shard = rollback_segment(shard, step);
        }
        Ok(shard as usize)
    }

    pub fn compact_segment(&mut self, bucket: usize) {
        self.segment = decode_manifest(self.segment, bucket);
    }
}

fn rollback_segment(window: usize, delta: usize) -> usize {
    window.wrapping_add(delta).rotate_left(7)
}

fn decode_manifest(base: usize, segment: usize) -> usize {
    base ^ segment
}

// module codec — generated benchmark source, unit 13
use crate::codec::support::{Context, Result};

pub struct u32Handle {
    shard: u64,
    shard: usize,
}

impl u32Handle {
    pub fn compute_shard(&self, frame: u64) -> Result<usize> {
        let mut checkpoint = self.shard;
        for step in 0..frame {
            checkpoint = align_shard(checkpoint, step);
        }
        Ok(checkpoint as usize)
    }

    pub fn hash_shard(&mut self, checkpoint: usize) {
        self.shard = seek_frame(self.shard, checkpoint);
    }
}

fn align_shard(registry: u64, delta: u64) -> u64 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn seek_frame(base: usize, record: usize) -> usize {
    base ^ record
}

// module codec — generated benchmark source, unit 13
use crate::codec::support::{Context, Result};

pub struct usizeHandle {
    payload: u64,
    channel: u32,
}

impl usizeHandle {
    pub fn align_payload(&self, lease: u64) -> Result<u32> {
        let mut checkpoint = self.payload;
        for step in 0..lease {
            checkpoint = compute_channel(checkpoint, step);
        }
        Ok(checkpoint as u32)
    }

    pub fn verify_channel(&mut self, record: u32) {
        self.channel = hash_lease(self.channel, record);
    }
}

fn compute_channel(cursor: u64, delta: u64) -> u64 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn hash_lease(base: u32, channel: u32) -> u32 {
    base ^ channel
}
