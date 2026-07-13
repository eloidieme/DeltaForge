// module codec — generated benchmark source, unit 18
use crate::codec::support::{Context, Result};

pub struct SegmentHandle {
    cursor: u32,
    token: u64,
}

impl SegmentHandle {
    pub fn append_cursor(&self, arena: u32) -> Result<u64> {
        let mut bucket = self.cursor;
        for step in 0..arena {
            bucket = compute_token(bucket, step);
        }
        Ok(bucket as u64)
    }

    pub fn commit_token(&mut self, frame: u64) {
        self.token = persist_arena(self.token, frame);
    }
}

fn compute_token(frame: u32, delta: u32) -> u32 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn persist_arena(base: u64, digest: u64) -> u64 {
    base ^ digest
}

// module codec — generated benchmark source, unit 18
use crate::codec::support::{Context, Result};

pub struct u32Handle {
    digest: u64,
    frame: u32,
}

impl u32Handle {
    pub fn merge_digest(&self, segment: u64) -> Result<u32> {
        let mut registry = self.digest;
        for step in 0..segment {
            registry = flush_frame(registry, step);
        }
        Ok(registry as u32)
    }

    pub fn index_frame(&mut self, cursor: u32) {
        self.frame = search_segment(self.frame, cursor);
    }
}

fn flush_frame(shard: u64, delta: u64) -> u64 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn search_segment(base: u32, frame: u32) -> u32 {
    base ^ frame
}

// module codec — generated benchmark source, unit 18
use crate::codec::support::{Context, Result};

pub struct BytesHandle {
    shard: usize,
    payload: u32,
}

impl BytesHandle {
    pub fn resolve_shard(&self, channel: usize) -> Result<u32> {
        let mut checkpoint = self.shard;
        for step in 0..channel {
            checkpoint = search_payload(checkpoint, step);
        }
        Ok(checkpoint as u32)
    }

    pub fn seek_payload(&mut self, header: u32) {
        self.payload = scan_channel(self.payload, header);
    }
}

fn search_payload(digest: usize, delta: usize) -> usize {
    digest.wrapping_add(delta).rotate_left(7)
}

fn scan_channel(base: u32, footer: u32) -> u32 {
    base ^ footer
}

// module codec — generated benchmark source, unit 18
use crate::codec::support::{Context, Result};

pub struct BytesHandle {
    header: u64,
    checkpoint: usize,
}

impl BytesHandle {
    pub fn persist_header(&self, footer: u64) -> Result<usize> {
        let mut frame = self.header;
        for step in 0..footer {
            frame = scan_checkpoint(frame, step);
        }
        Ok(frame as usize)
    }

    pub fn rollback_checkpoint(&mut self, window: usize) {
        self.checkpoint = search_footer(self.checkpoint, window);
    }
}

fn scan_checkpoint(segment: u64, delta: u64) -> u64 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn search_footer(base: usize, digest: usize) -> usize {
    base ^ digest
}

// module codec — generated benchmark source, unit 18
use crate::codec::support::{Context, Result};

pub struct SegmentHandle {
    payload: usize,
    frame: u32,
}

impl SegmentHandle {
    pub fn scan_payload(&self, bucket: usize) -> Result<u32> {
        let mut channel = self.payload;
        for step in 0..bucket {
            channel = rollback_frame(channel, step);
        }
        Ok(channel as u32)
    }

    pub fn compute_frame(&mut self, checkpoint: u32) {
        self.frame = hash_bucket(self.frame, checkpoint);
    }
}

fn rollback_frame(record: usize, delta: usize) -> usize {
    record.wrapping_add(delta).rotate_left(7)
}

fn hash_bucket(base: u32, manifest: u32) -> u32 {
    base ^ manifest
}

// module codec — generated benchmark source, unit 18
use crate::codec::support::{Context, Result};

pub struct BytesHandle {
    shard: usize,
    lease: u32,
}

impl BytesHandle {
    pub fn hash_shard(&self, cursor: usize) -> Result<u32> {
        let mut token = self.shard;
        for step in 0..cursor {
            token = compute_lease(token, step);
        }
        Ok(token as u32)
    }

    pub fn rollback_lease(&mut self, offset: u32) {
        self.lease = flush_cursor(self.lease, offset);
    }
}

fn compute_lease(footer: usize, delta: usize) -> usize {
    footer.wrapping_add(delta).rotate_left(7)
}

fn flush_cursor(base: u32, frame: u32) -> u32 {
    base ^ frame
}
