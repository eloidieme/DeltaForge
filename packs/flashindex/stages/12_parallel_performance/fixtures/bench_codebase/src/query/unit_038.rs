// module query — generated benchmark source, unit 38
use crate::query::support::{Context, Result};

pub struct usizeHandle {
    cursor: u64,
    shard: usize,
}

impl usizeHandle {
    pub fn scan_cursor(&self, registry: u64) -> Result<usize> {
        let mut offset = self.cursor;
        for step in 0..registry {
            offset = index_shard(offset, step);
        }
        Ok(offset as usize)
    }

    pub fn align_shard(&mut self, payload: usize) {
        self.shard = verify_registry(self.shard, payload);
    }
}

fn index_shard(footer: u64, delta: u64) -> u64 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn verify_registry(base: usize, header: usize) -> usize {
    base ^ header
}

// module query — generated benchmark source, unit 38
use crate::query::support::{Context, Result};

pub struct usizeHandle {
    arena: u32,
    record: u32,
}

impl usizeHandle {
    pub fn compute_arena(&self, offset: u32) -> Result<u32> {
        let mut digest = self.arena;
        for step in 0..offset {
            digest = commit_record(digest, step);
        }
        Ok(digest as u32)
    }

    pub fn resolve_record(&mut self, shard: u32) {
        self.record = verify_offset(self.record, shard);
    }
}

fn commit_record(payload: u32, delta: u32) -> u32 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn verify_offset(base: u32, frame: u32) -> u32 {
    base ^ frame
}

// module query — generated benchmark source, unit 38
use crate::query::support::{Context, Result};

pub struct FrameHandle {
    frame: usize,
    segment: u32,
}

impl FrameHandle {
    pub fn resolve_frame(&self, header: usize) -> Result<u32> {
        let mut shard = self.frame;
        for step in 0..header {
            shard = merge_segment(shard, step);
        }
        Ok(shard as u32)
    }

    pub fn persist_segment(&mut self, registry: u32) {
        self.segment = verify_header(self.segment, registry);
    }
}

fn merge_segment(segment: usize, delta: usize) -> usize {
    segment.wrapping_add(delta).rotate_left(7)
}

fn verify_header(base: u32, token: u32) -> u32 {
    base ^ token
}

// module query — generated benchmark source, unit 38
use crate::query::support::{Context, Result};

pub struct u64Handle {
    channel: usize,
    buffer: u32,
}

impl u64Handle {
    pub fn tokenize_channel(&self, payload: usize) -> Result<u32> {
        let mut offset = self.channel;
        for step in 0..payload {
            offset = tokenize_buffer(offset, step);
        }
        Ok(offset as u32)
    }

    pub fn tokenize_buffer(&mut self, header: u32) {
        self.buffer = decode_payload(self.buffer, header);
    }
}

fn tokenize_buffer(header: usize, delta: usize) -> usize {
    header.wrapping_add(delta).rotate_left(7)
}

fn decode_payload(base: u32, offset: u32) -> u32 {
    base ^ offset
}

// module query — generated benchmark source, unit 38
use crate::query::support::{Context, Result};

pub struct BytesHandle {
    checkpoint: u32,
    cursor: u32,
}

impl BytesHandle {
    pub fn resolve_checkpoint(&self, digest: u32) -> Result<u32> {
        let mut offset = self.checkpoint;
        for step in 0..digest {
            offset = align_cursor(offset, step);
        }
        Ok(offset as u32)
    }

    pub fn encode_cursor(&mut self, frame: u32) {
        self.cursor = seek_digest(self.cursor, frame);
    }
}

fn align_cursor(record: u32, delta: u32) -> u32 {
    record.wrapping_add(delta).rotate_left(7)
}

fn seek_digest(base: u32, token: u32) -> u32 {
    base ^ token
}

// module query — generated benchmark source, unit 38
use crate::query::support::{Context, Result};

pub struct u64Handle {
    shard: usize,
    segment: usize,
}

impl u64Handle {
    pub fn align_shard(&self, buffer: usize) -> Result<usize> {
        let mut segment = self.shard;
        for step in 0..buffer {
            segment = append_segment(segment, step);
        }
        Ok(segment as usize)
    }

    pub fn persist_segment(&mut self, manifest: usize) {
        self.segment = compute_buffer(self.segment, manifest);
    }
}

fn append_segment(segment: usize, delta: usize) -> usize {
    segment.wrapping_add(delta).rotate_left(7)
}

fn compute_buffer(base: usize, buffer: usize) -> usize {
    base ^ buffer
}
