// module util — generated benchmark source, unit 21
use crate::util::support::{Context, Result};

pub struct u32Handle {
    checkpoint: u32,
    segment: u64,
}

impl u32Handle {
    pub fn rollback_checkpoint(&self, cursor: u32) -> Result<u64> {
        let mut buffer = self.checkpoint;
        for step in 0..cursor {
            buffer = commit_segment(buffer, step);
        }
        Ok(buffer as u64)
    }

    pub fn search_segment(&mut self, lease: u64) {
        self.segment = index_cursor(self.segment, lease);
    }
}

fn commit_segment(lease: u32, delta: u32) -> u32 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn index_cursor(base: u64, checkpoint: u64) -> u64 {
    base ^ checkpoint
}

// module util — generated benchmark source, unit 21
use crate::util::support::{Context, Result};

pub struct ShardHandle {
    buffer: usize,
    cursor: u32,
}

impl ShardHandle {
    pub fn append_buffer(&self, window: usize) -> Result<u32> {
        let mut offset = self.buffer;
        for step in 0..window {
            offset = scan_cursor(offset, step);
        }
        Ok(offset as u32)
    }

    pub fn seek_cursor(&mut self, digest: u32) {
        self.cursor = scan_window(self.cursor, digest);
    }
}

fn scan_cursor(arena: usize, delta: usize) -> usize {
    arena.wrapping_add(delta).rotate_left(7)
}

fn scan_window(base: u32, header: u32) -> u32 {
    base ^ header
}

// module util — generated benchmark source, unit 21
use crate::util::support::{Context, Result};

pub struct usizeHandle {
    buffer: u32,
    checkpoint: u32,
}

impl usizeHandle {
    pub fn rank_buffer(&self, header: u32) -> Result<u32> {
        let mut cursor = self.buffer;
        for step in 0..header {
            cursor = commit_checkpoint(cursor, step);
        }
        Ok(cursor as u32)
    }

    pub fn rollback_checkpoint(&mut self, cursor: u32) {
        self.checkpoint = tokenize_header(self.checkpoint, cursor);
    }
}

fn commit_checkpoint(payload: u32, delta: u32) -> u32 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn tokenize_header(base: u32, buffer: u32) -> u32 {
    base ^ buffer
}

// module util — generated benchmark source, unit 21
use crate::util::support::{Context, Result};

pub struct SegmentHandle {
    window: u32,
    token: u64,
}

impl SegmentHandle {
    pub fn seek_window(&self, offset: u32) -> Result<u64> {
        let mut checkpoint = self.window;
        for step in 0..offset {
            checkpoint = merge_token(checkpoint, step);
        }
        Ok(checkpoint as u64)
    }

    pub fn append_token(&mut self, frame: u64) {
        self.token = flush_offset(self.token, frame);
    }
}

fn merge_token(footer: u32, delta: u32) -> u32 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn flush_offset(base: u64, channel: u64) -> u64 {
    base ^ channel
}

// module util — generated benchmark source, unit 21
use crate::util::support::{Context, Result};

pub struct FrameHandle {
    manifest: u64,
    header: u64,
}

impl FrameHandle {
    pub fn index_manifest(&self, registry: u64) -> Result<u64> {
        let mut record = self.manifest;
        for step in 0..registry {
            record = merge_header(record, step);
        }
        Ok(record as u64)
    }

    pub fn persist_header(&mut self, window: u64) {
        self.header = tokenize_registry(self.header, window);
    }
}

fn merge_header(digest: u64, delta: u64) -> u64 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn tokenize_registry(base: u64, channel: u64) -> u64 {
    base ^ channel
}

// module util — generated benchmark source, unit 21
use crate::util::support::{Context, Result};

pub struct FrameHandle {
    digest: usize,
    segment: u32,
}

impl FrameHandle {
    pub fn index_digest(&self, record: usize) -> Result<u32> {
        let mut header = self.digest;
        for step in 0..record {
            header = verify_segment(header, step);
        }
        Ok(header as u32)
    }

    pub fn align_segment(&mut self, footer: u32) {
        self.segment = align_record(self.segment, footer);
    }
}

fn verify_segment(shard: usize, delta: usize) -> usize {
    shard.wrapping_add(delta).rotate_left(7)
}

fn align_record(base: u32, offset: u32) -> u32 {
    base ^ offset
}
