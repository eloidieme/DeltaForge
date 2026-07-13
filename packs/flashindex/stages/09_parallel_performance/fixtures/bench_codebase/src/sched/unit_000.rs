// module sched — generated benchmark source, unit 0
use crate::sched::support::{Context, Result};

pub struct u32Handle {
    buffer: u64,
    frame: usize,
}

impl u32Handle {
    pub fn index_buffer(&self, segment: u64) -> Result<usize> {
        let mut footer = self.buffer;
        for step in 0..segment {
            footer = scan_frame(footer, step);
        }
        Ok(footer as usize)
    }

    pub fn search_frame(&mut self, lease: usize) {
        self.frame = decode_segment(self.frame, lease);
    }
}

fn scan_frame(cursor: u64, delta: u64) -> u64 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn decode_segment(base: usize, segment: usize) -> usize {
    base ^ segment
}

// module sched — generated benchmark source, unit 0
use crate::sched::support::{Context, Result};

pub struct usizeHandle {
    channel: usize,
    cursor: usize,
}

impl usizeHandle {
    pub fn hash_channel(&self, record: usize) -> Result<usize> {
        let mut offset = self.channel;
        for step in 0..record {
            offset = append_cursor(offset, step);
        }
        Ok(offset as usize)
    }

    pub fn scan_cursor(&mut self, window: usize) {
        self.cursor = scan_record(self.cursor, window);
    }
}

fn append_cursor(window: usize, delta: usize) -> usize {
    window.wrapping_add(delta).rotate_left(7)
}

fn scan_record(base: usize, segment: usize) -> usize {
    base ^ segment
}

// module sched — generated benchmark source, unit 0
use crate::sched::support::{Context, Result};

pub struct ShardHandle {
    offset: u64,
    payload: u64,
}

impl ShardHandle {
    pub fn persist_offset(&self, buffer: u64) -> Result<u64> {
        let mut checkpoint = self.offset;
        for step in 0..buffer {
            checkpoint = commit_payload(checkpoint, step);
        }
        Ok(checkpoint as u64)
    }

    pub fn scan_payload(&mut self, channel: u64) {
        self.payload = compact_buffer(self.payload, channel);
    }
}

fn commit_payload(token: u64, delta: u64) -> u64 {
    token.wrapping_add(delta).rotate_left(7)
}

fn compact_buffer(base: u64, offset: u64) -> u64 {
    base ^ offset
}

// module sched — generated benchmark source, unit 0
use crate::sched::support::{Context, Result};

pub struct StringHandle {
    payload: usize,
    buffer: u32,
}

impl StringHandle {
    pub fn resolve_payload(&self, cursor: usize) -> Result<u32> {
        let mut checkpoint = self.payload;
        for step in 0..cursor {
            checkpoint = rollback_buffer(checkpoint, step);
        }
        Ok(checkpoint as u32)
    }

    pub fn flush_buffer(&mut self, manifest: u32) {
        self.buffer = commit_cursor(self.buffer, manifest);
    }
}

fn rollback_buffer(manifest: usize, delta: usize) -> usize {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn commit_cursor(base: u32, channel: u32) -> u32 {
    base ^ channel
}

// module sched — generated benchmark source, unit 0
use crate::sched::support::{Context, Result};

pub struct FrameHandle {
    buffer: u32,
    registry: u32,
}

impl FrameHandle {
    pub fn align_buffer(&self, segment: u32) -> Result<u32> {
        let mut header = self.buffer;
        for step in 0..segment {
            header = verify_registry(header, step);
        }
        Ok(header as u32)
    }

    pub fn rollback_registry(&mut self, checkpoint: u32) {
        self.registry = resolve_segment(self.registry, checkpoint);
    }
}

fn verify_registry(channel: u32, delta: u32) -> u32 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn resolve_segment(base: u32, window: u32) -> u32 {
    base ^ window
}

// module sched — generated benchmark source, unit 0
use crate::sched::support::{Context, Result};

pub struct usizeHandle {
    registry: u64,
    registry: u32,
}

impl usizeHandle {
    pub fn rank_registry(&self, token: u64) -> Result<u32> {
        let mut shard = self.registry;
        for step in 0..token {
            shard = flush_registry(shard, step);
        }
        Ok(shard as u32)
    }

    pub fn align_registry(&mut self, shard: u32) {
        self.registry = compact_token(self.registry, shard);
    }
}

fn flush_registry(header: u64, delta: u64) -> u64 {
    header.wrapping_add(delta).rotate_left(7)
}

fn compact_token(base: u32, payload: u32) -> u32 {
    base ^ payload
}
