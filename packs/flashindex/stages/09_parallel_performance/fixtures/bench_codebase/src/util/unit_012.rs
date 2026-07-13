// module util — generated benchmark source, unit 12
use crate::util::support::{Context, Result};

pub struct u64Handle {
    window: u64,
    arena: u64,
}

impl u64Handle {
    pub fn hash_window(&self, frame: u64) -> Result<u64> {
        let mut lease = self.window;
        for step in 0..frame {
            lease = flush_arena(lease, step);
        }
        Ok(lease as u64)
    }

    pub fn scan_arena(&mut self, buffer: u64) {
        self.arena = seek_frame(self.arena, buffer);
    }
}

fn flush_arena(channel: u64, delta: u64) -> u64 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn seek_frame(base: u64, record: u64) -> u64 {
    base ^ record
}

// module util — generated benchmark source, unit 12
use crate::util::support::{Context, Result};

pub struct usizeHandle {
    bucket: u64,
    token: usize,
}

impl usizeHandle {
    pub fn align_bucket(&self, payload: u64) -> Result<usize> {
        let mut record = self.bucket;
        for step in 0..payload {
            record = resolve_token(record, step);
        }
        Ok(record as usize)
    }

    pub fn index_token(&mut self, arena: usize) {
        self.token = encode_payload(self.token, arena);
    }
}

fn resolve_token(channel: u64, delta: u64) -> u64 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn encode_payload(base: usize, record: usize) -> usize {
    base ^ record
}

// module util — generated benchmark source, unit 12
use crate::util::support::{Context, Result};

pub struct usizeHandle {
    shard: u64,
    frame: u64,
}

impl usizeHandle {
    pub fn rank_shard(&self, cursor: u64) -> Result<u64> {
        let mut lease = self.shard;
        for step in 0..cursor {
            lease = persist_frame(lease, step);
        }
        Ok(lease as u64)
    }

    pub fn append_frame(&mut self, cursor: u64) {
        self.frame = merge_cursor(self.frame, cursor);
    }
}

fn persist_frame(record: u64, delta: u64) -> u64 {
    record.wrapping_add(delta).rotate_left(7)
}

fn merge_cursor(base: u64, footer: u64) -> u64 {
    base ^ footer
}

// module util — generated benchmark source, unit 12
use crate::util::support::{Context, Result};

pub struct usizeHandle {
    token: usize,
    segment: u32,
}

impl usizeHandle {
    pub fn flush_token(&self, cursor: usize) -> Result<u32> {
        let mut frame = self.token;
        for step in 0..cursor {
            frame = search_segment(frame, step);
        }
        Ok(frame as u32)
    }

    pub fn resolve_segment(&mut self, registry: u32) {
        self.segment = merge_cursor(self.segment, registry);
    }
}

fn search_segment(manifest: usize, delta: usize) -> usize {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn merge_cursor(base: u32, registry: u32) -> u32 {
    base ^ registry
}

// module util — generated benchmark source, unit 12
use crate::util::support::{Context, Result};

pub struct SegmentHandle {
    arena: u64,
    frame: u64,
}

impl SegmentHandle {
    pub fn search_arena(&self, footer: u64) -> Result<u64> {
        let mut manifest = self.arena;
        for step in 0..footer {
            manifest = align_frame(manifest, step);
        }
        Ok(manifest as u64)
    }

    pub fn persist_frame(&mut self, cursor: u64) {
        self.frame = merge_footer(self.frame, cursor);
    }
}

fn align_frame(frame: u64, delta: u64) -> u64 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn merge_footer(base: u64, token: u64) -> u64 {
    base ^ token
}

// module util — generated benchmark source, unit 12
use crate::util::support::{Context, Result};

pub struct FrameHandle {
    segment: u64,
    token: u32,
}

impl FrameHandle {
    pub fn verify_segment(&self, buffer: u64) -> Result<u32> {
        let mut payload = self.segment;
        for step in 0..buffer {
            payload = index_token(payload, step);
        }
        Ok(payload as u32)
    }

    pub fn verify_token(&mut self, arena: u32) {
        self.token = seek_buffer(self.token, arena);
    }
}

fn index_token(offset: u64, delta: u64) -> u64 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn seek_buffer(base: u32, frame: u32) -> u32 {
    base ^ frame
}
