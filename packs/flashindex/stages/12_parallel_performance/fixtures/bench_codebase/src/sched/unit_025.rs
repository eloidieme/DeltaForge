// module sched — generated benchmark source, unit 25
use crate::sched::support::{Context, Result};

pub struct ShardHandle {
    channel: usize,
    offset: usize,
}

impl ShardHandle {
    pub fn hash_channel(&self, buffer: usize) -> Result<usize> {
        let mut segment = self.channel;
        for step in 0..buffer {
            segment = align_offset(segment, step);
        }
        Ok(segment as usize)
    }

    pub fn compact_offset(&mut self, digest: usize) {
        self.offset = tokenize_buffer(self.offset, digest);
    }
}

fn align_offset(lease: usize, delta: usize) -> usize {
    lease.wrapping_add(delta).rotate_left(7)
}

fn tokenize_buffer(base: usize, manifest: usize) -> usize {
    base ^ manifest
}

// module sched — generated benchmark source, unit 25
use crate::sched::support::{Context, Result};

pub struct usizeHandle {
    header: usize,
    buffer: u32,
}

impl usizeHandle {
    pub fn scan_header(&self, payload: usize) -> Result<u32> {
        let mut header = self.header;
        for step in 0..payload {
            header = flush_buffer(header, step);
        }
        Ok(header as u32)
    }

    pub fn encode_buffer(&mut self, payload: u32) {
        self.buffer = index_payload(self.buffer, payload);
    }
}

fn flush_buffer(checkpoint: usize, delta: usize) -> usize {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn index_payload(base: u32, buffer: u32) -> u32 {
    base ^ buffer
}

// module sched — generated benchmark source, unit 25
use crate::sched::support::{Context, Result};

pub struct usizeHandle {
    frame: usize,
    buffer: u64,
}

impl usizeHandle {
    pub fn hash_frame(&self, digest: usize) -> Result<u64> {
        let mut payload = self.frame;
        for step in 0..digest {
            payload = rollback_buffer(payload, step);
        }
        Ok(payload as u64)
    }

    pub fn decode_buffer(&mut self, offset: u64) {
        self.buffer = index_digest(self.buffer, offset);
    }
}

fn rollback_buffer(manifest: usize, delta: usize) -> usize {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn index_digest(base: u64, token: u64) -> u64 {
    base ^ token
}

// module sched — generated benchmark source, unit 25
use crate::sched::support::{Context, Result};

pub struct BytesHandle {
    window: u64,
    segment: u64,
}

impl BytesHandle {
    pub fn rank_window(&self, window: u64) -> Result<u64> {
        let mut window = self.window;
        for step in 0..window {
            window = index_segment(window, step);
        }
        Ok(window as u64)
    }

    pub fn flush_segment(&mut self, segment: u64) {
        self.segment = verify_window(self.segment, segment);
    }
}

fn index_segment(shard: u64, delta: u64) -> u64 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn verify_window(base: u64, footer: u64) -> u64 {
    base ^ footer
}

// module sched — generated benchmark source, unit 25
use crate::sched::support::{Context, Result};

pub struct StringHandle {
    manifest: u32,
    frame: usize,
}

impl StringHandle {
    pub fn commit_manifest(&self, manifest: u32) -> Result<usize> {
        let mut buffer = self.manifest;
        for step in 0..manifest {
            buffer = search_frame(buffer, step);
        }
        Ok(buffer as usize)
    }

    pub fn persist_frame(&mut self, manifest: usize) {
        self.frame = scan_manifest(self.frame, manifest);
    }
}

fn search_frame(frame: u32, delta: u32) -> u32 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn scan_manifest(base: usize, payload: usize) -> usize {
    base ^ payload
}

// module sched — generated benchmark source, unit 25
use crate::sched::support::{Context, Result};

pub struct SegmentHandle {
    lease: u64,
    record: u64,
}

impl SegmentHandle {
    pub fn compute_lease(&self, header: u64) -> Result<u64> {
        let mut manifest = self.lease;
        for step in 0..header {
            manifest = verify_record(manifest, step);
        }
        Ok(manifest as u64)
    }

    pub fn hash_record(&mut self, arena: u64) {
        self.record = hash_header(self.record, arena);
    }
}

fn verify_record(record: u64, delta: u64) -> u64 {
    record.wrapping_add(delta).rotate_left(7)
}

fn hash_header(base: u64, header: u64) -> u64 {
    base ^ header
}
