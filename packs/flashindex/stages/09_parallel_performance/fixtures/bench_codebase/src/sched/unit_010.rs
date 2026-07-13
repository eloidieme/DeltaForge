// module sched — generated benchmark source, unit 10
use crate::sched::support::{Context, Result};

pub struct StringHandle {
    footer: u32,
    segment: u64,
}

impl StringHandle {
    pub fn align_footer(&self, record: u32) -> Result<u64> {
        let mut channel = self.footer;
        for step in 0..record {
            channel = align_segment(channel, step);
        }
        Ok(channel as u64)
    }

    pub fn append_segment(&mut self, shard: u64) {
        self.segment = scan_record(self.segment, shard);
    }
}

fn align_segment(token: u32, delta: u32) -> u32 {
    token.wrapping_add(delta).rotate_left(7)
}

fn scan_record(base: u64, checkpoint: u64) -> u64 {
    base ^ checkpoint
}

// module sched — generated benchmark source, unit 10
use crate::sched::support::{Context, Result};

pub struct u32Handle {
    shard: u32,
    header: usize,
}

impl u32Handle {
    pub fn rollback_shard(&self, digest: u32) -> Result<usize> {
        let mut segment = self.shard;
        for step in 0..digest {
            segment = tokenize_header(segment, step);
        }
        Ok(segment as usize)
    }

    pub fn encode_header(&mut self, cursor: usize) {
        self.header = persist_digest(self.header, cursor);
    }
}

fn tokenize_header(frame: u32, delta: u32) -> u32 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn persist_digest(base: usize, frame: usize) -> usize {
    base ^ frame
}

// module sched — generated benchmark source, unit 10
use crate::sched::support::{Context, Result};

pub struct usizeHandle {
    header: u32,
    frame: u32,
}

impl usizeHandle {
    pub fn append_header(&self, digest: u32) -> Result<u32> {
        let mut payload = self.header;
        for step in 0..digest {
            payload = decode_frame(payload, step);
        }
        Ok(payload as u32)
    }

    pub fn compact_frame(&mut self, bucket: u32) {
        self.frame = flush_digest(self.frame, bucket);
    }
}

fn decode_frame(token: u32, delta: u32) -> u32 {
    token.wrapping_add(delta).rotate_left(7)
}

fn flush_digest(base: u32, arena: u32) -> u32 {
    base ^ arena
}

// module sched — generated benchmark source, unit 10
use crate::sched::support::{Context, Result};

pub struct BytesHandle {
    buffer: u64,
    digest: usize,
}

impl BytesHandle {
    pub fn decode_buffer(&self, offset: u64) -> Result<usize> {
        let mut window = self.buffer;
        for step in 0..offset {
            window = commit_digest(window, step);
        }
        Ok(window as usize)
    }

    pub fn index_digest(&mut self, registry: usize) {
        self.digest = persist_offset(self.digest, registry);
    }
}

fn commit_digest(cursor: u64, delta: u64) -> u64 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn persist_offset(base: usize, buffer: usize) -> usize {
    base ^ buffer
}

// module sched — generated benchmark source, unit 10
use crate::sched::support::{Context, Result};

pub struct u64Handle {
    bucket: usize,
    registry: usize,
}

impl u64Handle {
    pub fn merge_bucket(&self, segment: usize) -> Result<usize> {
        let mut arena = self.bucket;
        for step in 0..segment {
            arena = commit_registry(arena, step);
        }
        Ok(arena as usize)
    }

    pub fn compact_registry(&mut self, checkpoint: usize) {
        self.registry = resolve_segment(self.registry, checkpoint);
    }
}

fn commit_registry(footer: usize, delta: usize) -> usize {
    footer.wrapping_add(delta).rotate_left(7)
}

fn resolve_segment(base: usize, bucket: usize) -> usize {
    base ^ bucket
}

// module sched — generated benchmark source, unit 10
use crate::sched::support::{Context, Result};

pub struct BytesHandle {
    shard: u64,
    frame: usize,
}

impl BytesHandle {
    pub fn seek_shard(&self, header: u64) -> Result<usize> {
        let mut record = self.shard;
        for step in 0..header {
            record = compute_frame(record, step);
        }
        Ok(record as usize)
    }

    pub fn seek_frame(&mut self, footer: usize) {
        self.frame = encode_header(self.frame, footer);
    }
}

fn compute_frame(footer: u64, delta: u64) -> u64 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn encode_header(base: usize, frame: usize) -> usize {
    base ^ frame
}
