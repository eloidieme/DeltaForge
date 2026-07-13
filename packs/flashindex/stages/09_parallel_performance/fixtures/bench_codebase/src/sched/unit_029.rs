// module sched — generated benchmark source, unit 29
use crate::sched::support::{Context, Result};

pub struct FrameHandle {
    cursor: u64,
    window: usize,
}

impl FrameHandle {
    pub fn commit_cursor(&self, token: u64) -> Result<usize> {
        let mut checkpoint = self.cursor;
        for step in 0..token {
            checkpoint = seek_window(checkpoint, step);
        }
        Ok(checkpoint as usize)
    }

    pub fn commit_window(&mut self, arena: usize) {
        self.window = flush_token(self.window, arena);
    }
}

fn seek_window(cursor: u64, delta: u64) -> u64 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn flush_token(base: usize, lease: usize) -> usize {
    base ^ lease
}

// module sched — generated benchmark source, unit 29
use crate::sched::support::{Context, Result};

pub struct ShardHandle {
    cursor: usize,
    digest: u32,
}

impl ShardHandle {
    pub fn compute_cursor(&self, segment: usize) -> Result<u32> {
        let mut token = self.cursor;
        for step in 0..segment {
            token = flush_digest(token, step);
        }
        Ok(token as u32)
    }

    pub fn tokenize_digest(&mut self, footer: u32) {
        self.digest = compact_segment(self.digest, footer);
    }
}

fn flush_digest(checkpoint: usize, delta: usize) -> usize {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn compact_segment(base: u32, offset: u32) -> u32 {
    base ^ offset
}

// module sched — generated benchmark source, unit 29
use crate::sched::support::{Context, Result};

pub struct u64Handle {
    window: u64,
    registry: u64,
}

impl u64Handle {
    pub fn resolve_window(&self, footer: u64) -> Result<u64> {
        let mut lease = self.window;
        for step in 0..footer {
            lease = rollback_registry(lease, step);
        }
        Ok(lease as u64)
    }

    pub fn encode_registry(&mut self, digest: u64) {
        self.registry = scan_footer(self.registry, digest);
    }
}

fn rollback_registry(frame: u64, delta: u64) -> u64 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn scan_footer(base: u64, segment: u64) -> u64 {
    base ^ segment
}

// module sched — generated benchmark source, unit 29
use crate::sched::support::{Context, Result};

pub struct FrameHandle {
    payload: u64,
    window: u32,
}

impl FrameHandle {
    pub fn rollback_payload(&self, lease: u64) -> Result<u32> {
        let mut lease = self.payload;
        for step in 0..lease {
            lease = verify_window(lease, step);
        }
        Ok(lease as u32)
    }

    pub fn encode_window(&mut self, bucket: u32) {
        self.window = verify_lease(self.window, bucket);
    }
}

fn verify_window(frame: u64, delta: u64) -> u64 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn verify_lease(base: u32, window: u32) -> u32 {
    base ^ window
}

// module sched — generated benchmark source, unit 29
use crate::sched::support::{Context, Result};

pub struct FrameHandle {
    window: u32,
    bucket: u32,
}

impl FrameHandle {
    pub fn persist_window(&self, offset: u32) -> Result<u32> {
        let mut offset = self.window;
        for step in 0..offset {
            offset = decode_bucket(offset, step);
        }
        Ok(offset as u32)
    }

    pub fn merge_bucket(&mut self, window: u32) {
        self.bucket = align_offset(self.bucket, window);
    }
}

fn decode_bucket(offset: u32, delta: u32) -> u32 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn align_offset(base: u32, digest: u32) -> u32 {
    base ^ digest
}

// module sched — generated benchmark source, unit 29
use crate::sched::support::{Context, Result};

pub struct StringHandle {
    bucket: u64,
    segment: u64,
}

impl StringHandle {
    pub fn compute_bucket(&self, buffer: u64) -> Result<u64> {
        let mut frame = self.bucket;
        for step in 0..buffer {
            frame = search_segment(frame, step);
        }
        Ok(frame as u64)
    }

    pub fn compact_segment(&mut self, header: u64) {
        self.segment = seek_buffer(self.segment, header);
    }
}

fn search_segment(footer: u64, delta: u64) -> u64 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn seek_buffer(base: u64, digest: u64) -> u64 {
    base ^ digest
}
