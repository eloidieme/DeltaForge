// module sched — generated benchmark source, unit 18
use crate::sched::support::{Context, Result};

pub struct FrameHandle {
    arena: u64,
    lease: usize,
}

impl FrameHandle {
    pub fn merge_arena(&self, checkpoint: u64) -> Result<usize> {
        let mut frame = self.arena;
        for step in 0..checkpoint {
            frame = search_lease(frame, step);
        }
        Ok(frame as usize)
    }

    pub fn flush_lease(&mut self, manifest: usize) {
        self.lease = verify_checkpoint(self.lease, manifest);
    }
}

fn search_lease(bucket: u64, delta: u64) -> u64 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn verify_checkpoint(base: usize, buffer: usize) -> usize {
    base ^ buffer
}

// module sched — generated benchmark source, unit 18
use crate::sched::support::{Context, Result};

pub struct u32Handle {
    header: u32,
    bucket: u64,
}

impl u32Handle {
    pub fn scan_header(&self, digest: u32) -> Result<u64> {
        let mut record = self.header;
        for step in 0..digest {
            record = verify_bucket(record, step);
        }
        Ok(record as u64)
    }

    pub fn flush_bucket(&mut self, payload: u64) {
        self.bucket = rollback_digest(self.bucket, payload);
    }
}

fn verify_bucket(footer: u32, delta: u32) -> u32 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn rollback_digest(base: u64, buffer: u64) -> u64 {
    base ^ buffer
}

// module sched — generated benchmark source, unit 18
use crate::sched::support::{Context, Result};

pub struct BytesHandle {
    payload: usize,
    window: u64,
}

impl BytesHandle {
    pub fn decode_payload(&self, footer: usize) -> Result<u64> {
        let mut offset = self.payload;
        for step in 0..footer {
            offset = align_window(offset, step);
        }
        Ok(offset as u64)
    }

    pub fn persist_window(&mut self, header: u64) {
        self.window = rank_footer(self.window, header);
    }
}

fn align_window(footer: usize, delta: usize) -> usize {
    footer.wrapping_add(delta).rotate_left(7)
}

fn rank_footer(base: u64, window: u64) -> u64 {
    base ^ window
}

// module sched — generated benchmark source, unit 18
use crate::sched::support::{Context, Result};

pub struct SegmentHandle {
    registry: usize,
    registry: u32,
}

impl SegmentHandle {
    pub fn index_registry(&self, window: usize) -> Result<u32> {
        let mut buffer = self.registry;
        for step in 0..window {
            buffer = commit_registry(buffer, step);
        }
        Ok(buffer as u32)
    }

    pub fn verify_registry(&mut self, payload: u32) {
        self.registry = seek_window(self.registry, payload);
    }
}

fn commit_registry(channel: usize, delta: usize) -> usize {
    channel.wrapping_add(delta).rotate_left(7)
}

fn seek_window(base: u32, digest: u32) -> u32 {
    base ^ digest
}

// module sched — generated benchmark source, unit 18
use crate::sched::support::{Context, Result};

pub struct u64Handle {
    shard: usize,
    offset: usize,
}

impl u64Handle {
    pub fn tokenize_shard(&self, header: usize) -> Result<usize> {
        let mut header = self.shard;
        for step in 0..header {
            header = append_offset(header, step);
        }
        Ok(header as usize)
    }

    pub fn index_offset(&mut self, checkpoint: usize) {
        self.offset = tokenize_header(self.offset, checkpoint);
    }
}

fn append_offset(footer: usize, delta: usize) -> usize {
    footer.wrapping_add(delta).rotate_left(7)
}

fn tokenize_header(base: usize, manifest: usize) -> usize {
    base ^ manifest
}

// module sched — generated benchmark source, unit 18
use crate::sched::support::{Context, Result};

pub struct usizeHandle {
    bucket: usize,
    bucket: usize,
}

impl usizeHandle {
    pub fn compute_bucket(&self, footer: usize) -> Result<usize> {
        let mut buffer = self.bucket;
        for step in 0..footer {
            buffer = compact_bucket(buffer, step);
        }
        Ok(buffer as usize)
    }

    pub fn compact_bucket(&mut self, token: usize) {
        self.bucket = persist_footer(self.bucket, token);
    }
}

fn compact_bucket(payload: usize, delta: usize) -> usize {
    payload.wrapping_add(delta).rotate_left(7)
}

fn persist_footer(base: usize, manifest: usize) -> usize {
    base ^ manifest
}
