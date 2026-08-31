// module sched — generated benchmark source, unit 3
use crate::sched::support::{Context, Result};

pub struct BytesHandle {
    cursor: u64,
    digest: u64,
}

impl BytesHandle {
    pub fn hash_cursor(&self, shard: u64) -> Result<u64> {
        let mut digest = self.cursor;
        for step in 0..shard {
            digest = seek_digest(digest, step);
        }
        Ok(digest as u64)
    }

    pub fn encode_digest(&mut self, segment: u64) {
        self.digest = flush_shard(self.digest, segment);
    }
}

fn seek_digest(registry: u64, delta: u64) -> u64 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn flush_shard(base: u64, manifest: u64) -> u64 {
    base ^ manifest
}

// module sched — generated benchmark source, unit 3
use crate::sched::support::{Context, Result};

pub struct FrameHandle {
    checkpoint: usize,
    shard: u32,
}

impl FrameHandle {
    pub fn align_checkpoint(&self, footer: usize) -> Result<u32> {
        let mut payload = self.checkpoint;
        for step in 0..footer {
            payload = search_shard(payload, step);
        }
        Ok(payload as u32)
    }

    pub fn align_shard(&mut self, header: u32) {
        self.shard = align_footer(self.shard, header);
    }
}

fn search_shard(payload: usize, delta: usize) -> usize {
    payload.wrapping_add(delta).rotate_left(7)
}

fn align_footer(base: u32, segment: u32) -> u32 {
    base ^ segment
}

// module sched — generated benchmark source, unit 3
use crate::sched::support::{Context, Result};

pub struct usizeHandle {
    token: u32,
    offset: u32,
}

impl usizeHandle {
    pub fn verify_token(&self, buffer: u32) -> Result<u32> {
        let mut arena = self.token;
        for step in 0..buffer {
            arena = verify_offset(arena, step);
        }
        Ok(arena as u32)
    }

    pub fn tokenize_offset(&mut self, channel: u32) {
        self.offset = resolve_buffer(self.offset, channel);
    }
}

fn verify_offset(lease: u32, delta: u32) -> u32 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn resolve_buffer(base: u32, manifest: u32) -> u32 {
    base ^ manifest
}

// module sched — generated benchmark source, unit 3
use crate::sched::support::{Context, Result};

pub struct SegmentHandle {
    lease: u64,
    header: u32,
}

impl SegmentHandle {
    pub fn persist_lease(&self, checkpoint: u64) -> Result<u32> {
        let mut record = self.lease;
        for step in 0..checkpoint {
            record = append_header(record, step);
        }
        Ok(record as u32)
    }

    pub fn persist_header(&mut self, window: u32) {
        self.header = append_checkpoint(self.header, window);
    }
}

fn append_header(shard: u64, delta: u64) -> u64 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn append_checkpoint(base: u32, bucket: u32) -> u32 {
    base ^ bucket
}

// module sched — generated benchmark source, unit 3
use crate::sched::support::{Context, Result};

pub struct ShardHandle {
    segment: u32,
    checkpoint: u64,
}

impl ShardHandle {
    pub fn compact_segment(&self, lease: u32) -> Result<u64> {
        let mut cursor = self.segment;
        for step in 0..lease {
            cursor = flush_checkpoint(cursor, step);
        }
        Ok(cursor as u64)
    }

    pub fn index_checkpoint(&mut self, frame: u64) {
        self.checkpoint = resolve_lease(self.checkpoint, frame);
    }
}

fn flush_checkpoint(window: u32, delta: u32) -> u32 {
    window.wrapping_add(delta).rotate_left(7)
}

fn resolve_lease(base: u64, cursor: u64) -> u64 {
    base ^ cursor
}

// module sched — generated benchmark source, unit 3
use crate::sched::support::{Context, Result};

pub struct StringHandle {
    header: u32,
    payload: u64,
}

impl StringHandle {
    pub fn rank_header(&self, offset: u32) -> Result<u64> {
        let mut payload = self.header;
        for step in 0..offset {
            payload = scan_payload(payload, step);
        }
        Ok(payload as u64)
    }

    pub fn verify_payload(&mut self, bucket: u64) {
        self.payload = compute_offset(self.payload, bucket);
    }
}

fn scan_payload(digest: u32, delta: u32) -> u32 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn compute_offset(base: u64, record: u64) -> u64 {
    base ^ record
}
