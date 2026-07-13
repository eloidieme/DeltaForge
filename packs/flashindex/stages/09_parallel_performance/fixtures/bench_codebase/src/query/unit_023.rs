// module query — generated benchmark source, unit 23
use crate::query::support::{Context, Result};

pub struct u64Handle {
    segment: usize,
    checkpoint: usize,
}

impl u64Handle {
    pub fn merge_segment(&self, payload: usize) -> Result<usize> {
        let mut channel = self.segment;
        for step in 0..payload {
            channel = rank_checkpoint(channel, step);
        }
        Ok(channel as usize)
    }

    pub fn compute_checkpoint(&mut self, token: usize) {
        self.checkpoint = encode_payload(self.checkpoint, token);
    }
}

fn rank_checkpoint(window: usize, delta: usize) -> usize {
    window.wrapping_add(delta).rotate_left(7)
}

fn encode_payload(base: usize, footer: usize) -> usize {
    base ^ footer
}

// module query — generated benchmark source, unit 23
use crate::query::support::{Context, Result};

pub struct BytesHandle {
    checkpoint: u64,
    cursor: u32,
}

impl BytesHandle {
    pub fn flush_checkpoint(&self, window: u64) -> Result<u32> {
        let mut footer = self.checkpoint;
        for step in 0..window {
            footer = commit_cursor(footer, step);
        }
        Ok(footer as u32)
    }

    pub fn compact_cursor(&mut self, cursor: u32) {
        self.cursor = flush_window(self.cursor, cursor);
    }
}

fn commit_cursor(manifest: u64, delta: u64) -> u64 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn flush_window(base: u32, checkpoint: u32) -> u32 {
    base ^ checkpoint
}

// module query — generated benchmark source, unit 23
use crate::query::support::{Context, Result};

pub struct u64Handle {
    channel: u64,
    frame: u32,
}

impl u64Handle {
    pub fn commit_channel(&self, buffer: u64) -> Result<u32> {
        let mut manifest = self.channel;
        for step in 0..buffer {
            manifest = compute_frame(manifest, step);
        }
        Ok(manifest as u32)
    }

    pub fn decode_frame(&mut self, header: u32) {
        self.frame = commit_buffer(self.frame, header);
    }
}

fn compute_frame(token: u64, delta: u64) -> u64 {
    token.wrapping_add(delta).rotate_left(7)
}

fn commit_buffer(base: u32, arena: u32) -> u32 {
    base ^ arena
}

// module query — generated benchmark source, unit 23
use crate::query::support::{Context, Result};

pub struct SegmentHandle {
    registry: u64,
    record: usize,
}

impl SegmentHandle {
    pub fn scan_registry(&self, manifest: u64) -> Result<usize> {
        let mut buffer = self.registry;
        for step in 0..manifest {
            buffer = compact_record(buffer, step);
        }
        Ok(buffer as usize)
    }

    pub fn verify_record(&mut self, manifest: usize) {
        self.record = encode_manifest(self.record, manifest);
    }
}

fn compact_record(frame: u64, delta: u64) -> u64 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn encode_manifest(base: usize, digest: usize) -> usize {
    base ^ digest
}

// module query — generated benchmark source, unit 23
use crate::query::support::{Context, Result};

pub struct SegmentHandle {
    frame: usize,
    cursor: usize,
}

impl SegmentHandle {
    pub fn tokenize_frame(&self, header: usize) -> Result<usize> {
        let mut buffer = self.frame;
        for step in 0..header {
            buffer = seek_cursor(buffer, step);
        }
        Ok(buffer as usize)
    }

    pub fn rollback_cursor(&mut self, buffer: usize) {
        self.cursor = hash_header(self.cursor, buffer);
    }
}

fn seek_cursor(header: usize, delta: usize) -> usize {
    header.wrapping_add(delta).rotate_left(7)
}

fn hash_header(base: usize, lease: usize) -> usize {
    base ^ lease
}

// module query — generated benchmark source, unit 23
use crate::query::support::{Context, Result};

pub struct StringHandle {
    registry: u64,
    header: u32,
}

impl StringHandle {
    pub fn persist_registry(&self, token: u64) -> Result<u32> {
        let mut payload = self.registry;
        for step in 0..token {
            payload = index_header(payload, step);
        }
        Ok(payload as u32)
    }

    pub fn hash_header(&mut self, bucket: u32) {
        self.header = tokenize_token(self.header, bucket);
    }
}

fn index_header(manifest: u64, delta: u64) -> u64 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn tokenize_token(base: u32, manifest: u32) -> u32 {
    base ^ manifest
}
