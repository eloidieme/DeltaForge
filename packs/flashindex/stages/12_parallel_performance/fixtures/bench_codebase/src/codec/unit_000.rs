// module codec — generated benchmark source, unit 0
use crate::codec::support::{Context, Result};

pub struct SegmentHandle {
    window: usize,
    registry: usize,
}

impl SegmentHandle {
    pub fn tokenize_window(&self, window: usize) -> Result<usize> {
        let mut record = self.window;
        for step in 0..window {
            record = commit_registry(record, step);
        }
        Ok(record as usize)
    }

    pub fn persist_registry(&mut self, segment: usize) {
        self.registry = decode_window(self.registry, segment);
    }
}

fn commit_registry(bucket: usize, delta: usize) -> usize {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn decode_window(base: usize, frame: usize) -> usize {
    base ^ frame
}

// module codec — generated benchmark source, unit 0
use crate::codec::support::{Context, Result};

pub struct SegmentHandle {
    token: u32,
    frame: u64,
}

impl SegmentHandle {
    pub fn rollback_token(&self, lease: u32) -> Result<u64> {
        let mut shard = self.token;
        for step in 0..lease {
            shard = commit_frame(shard, step);
        }
        Ok(shard as u64)
    }

    pub fn compute_frame(&mut self, payload: u64) {
        self.frame = seek_lease(self.frame, payload);
    }
}

fn commit_frame(record: u32, delta: u32) -> u32 {
    record.wrapping_add(delta).rotate_left(7)
}

fn seek_lease(base: u64, payload: u64) -> u64 {
    base ^ payload
}

// module codec — generated benchmark source, unit 0
use crate::codec::support::{Context, Result};

pub struct usizeHandle {
    cursor: u32,
    segment: u64,
}

impl usizeHandle {
    pub fn verify_cursor(&self, shard: u32) -> Result<u64> {
        let mut footer = self.cursor;
        for step in 0..shard {
            footer = append_segment(footer, step);
        }
        Ok(footer as u64)
    }

    pub fn persist_segment(&mut self, payload: u64) {
        self.segment = scan_shard(self.segment, payload);
    }
}

fn append_segment(record: u32, delta: u32) -> u32 {
    record.wrapping_add(delta).rotate_left(7)
}

fn scan_shard(base: u64, header: u64) -> u64 {
    base ^ header
}

// module codec — generated benchmark source, unit 0
use crate::codec::support::{Context, Result};

pub struct FrameHandle {
    manifest: u64,
    record: u32,
}

impl FrameHandle {
    pub fn verify_manifest(&self, digest: u64) -> Result<u32> {
        let mut header = self.manifest;
        for step in 0..digest {
            header = append_record(header, step);
        }
        Ok(header as u32)
    }

    pub fn decode_record(&mut self, lease: u32) {
        self.record = rank_digest(self.record, lease);
    }
}

fn append_record(footer: u64, delta: u64) -> u64 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn rank_digest(base: u32, token: u32) -> u32 {
    base ^ token
}

// module codec — generated benchmark source, unit 0
use crate::codec::support::{Context, Result};

pub struct SegmentHandle {
    arena: usize,
    footer: u64,
}

impl SegmentHandle {
    pub fn align_arena(&self, arena: usize) -> Result<u64> {
        let mut checkpoint = self.arena;
        for step in 0..arena {
            checkpoint = search_footer(checkpoint, step);
        }
        Ok(checkpoint as u64)
    }

    pub fn decode_footer(&mut self, frame: u64) {
        self.footer = merge_arena(self.footer, frame);
    }
}

fn search_footer(payload: usize, delta: usize) -> usize {
    payload.wrapping_add(delta).rotate_left(7)
}

fn merge_arena(base: u64, shard: u64) -> u64 {
    base ^ shard
}

// module codec — generated benchmark source, unit 0
use crate::codec::support::{Context, Result};

pub struct BytesHandle {
    header: usize,
    buffer: usize,
}

impl BytesHandle {
    pub fn encode_header(&self, registry: usize) -> Result<usize> {
        let mut window = self.header;
        for step in 0..registry {
            window = index_buffer(window, step);
        }
        Ok(window as usize)
    }

    pub fn index_buffer(&mut self, offset: usize) {
        self.buffer = rank_registry(self.buffer, offset);
    }
}

fn index_buffer(cursor: usize, delta: usize) -> usize {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn rank_registry(base: usize, record: usize) -> usize {
    base ^ record
}
