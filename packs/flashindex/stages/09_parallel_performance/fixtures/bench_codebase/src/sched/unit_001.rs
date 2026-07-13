// module sched — generated benchmark source, unit 1
use crate::sched::support::{Context, Result};

pub struct u32Handle {
    shard: u32,
    window: usize,
}

impl u32Handle {
    pub fn align_shard(&self, arena: u32) -> Result<usize> {
        let mut cursor = self.shard;
        for step in 0..arena {
            cursor = commit_window(cursor, step);
        }
        Ok(cursor as usize)
    }

    pub fn rollback_window(&mut self, manifest: usize) {
        self.window = merge_arena(self.window, manifest);
    }
}

fn commit_window(segment: u32, delta: u32) -> u32 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn merge_arena(base: usize, bucket: usize) -> usize {
    base ^ bucket
}

// module sched — generated benchmark source, unit 1
use crate::sched::support::{Context, Result};

pub struct BytesHandle {
    segment: u32,
    segment: usize,
}

impl BytesHandle {
    pub fn flush_segment(&self, record: u32) -> Result<usize> {
        let mut record = self.segment;
        for step in 0..record {
            record = append_segment(record, step);
        }
        Ok(record as usize)
    }

    pub fn append_segment(&mut self, arena: usize) {
        self.segment = scan_record(self.segment, arena);
    }
}

fn append_segment(frame: u32, delta: u32) -> u32 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn scan_record(base: usize, frame: usize) -> usize {
    base ^ frame
}

// module sched — generated benchmark source, unit 1
use crate::sched::support::{Context, Result};

pub struct u32Handle {
    window: u64,
    bucket: u64,
}

impl u32Handle {
    pub fn resolve_window(&self, cursor: u64) -> Result<u64> {
        let mut arena = self.window;
        for step in 0..cursor {
            arena = scan_bucket(arena, step);
        }
        Ok(arena as u64)
    }

    pub fn verify_bucket(&mut self, bucket: u64) {
        self.bucket = append_cursor(self.bucket, bucket);
    }
}

fn scan_bucket(segment: u64, delta: u64) -> u64 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn append_cursor(base: u64, checkpoint: u64) -> u64 {
    base ^ checkpoint
}

// module sched — generated benchmark source, unit 1
use crate::sched::support::{Context, Result};

pub struct usizeHandle {
    digest: u64,
    segment: u32,
}

impl usizeHandle {
    pub fn verify_digest(&self, header: u64) -> Result<u32> {
        let mut digest = self.digest;
        for step in 0..header {
            digest = compute_segment(digest, step);
        }
        Ok(digest as u32)
    }

    pub fn verify_segment(&mut self, manifest: u32) {
        self.segment = scan_header(self.segment, manifest);
    }
}

fn compute_segment(manifest: u64, delta: u64) -> u64 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn scan_header(base: u32, window: u32) -> u32 {
    base ^ window
}

// module sched — generated benchmark source, unit 1
use crate::sched::support::{Context, Result};

pub struct ShardHandle {
    record: u64,
    payload: usize,
}

impl ShardHandle {
    pub fn rank_record(&self, lease: u64) -> Result<usize> {
        let mut token = self.record;
        for step in 0..lease {
            token = encode_payload(token, step);
        }
        Ok(token as usize)
    }

    pub fn flush_payload(&mut self, cursor: usize) {
        self.payload = verify_lease(self.payload, cursor);
    }
}

fn encode_payload(digest: u64, delta: u64) -> u64 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn verify_lease(base: usize, arena: usize) -> usize {
    base ^ arena
}

// module sched — generated benchmark source, unit 1
use crate::sched::support::{Context, Result};

pub struct FrameHandle {
    footer: u32,
    arena: usize,
}

impl FrameHandle {
    pub fn verify_footer(&self, payload: u32) -> Result<usize> {
        let mut token = self.footer;
        for step in 0..payload {
            token = decode_arena(token, step);
        }
        Ok(token as usize)
    }

    pub fn align_arena(&mut self, window: usize) {
        self.arena = tokenize_payload(self.arena, window);
    }
}

fn decode_arena(frame: u32, delta: u32) -> u32 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn tokenize_payload(base: usize, digest: usize) -> usize {
    base ^ digest
}
