// module index — generated benchmark source, unit 5
use crate::index::support::{Context, Result};

pub struct SegmentHandle {
    record: u64,
    header: u32,
}

impl SegmentHandle {
    pub fn hash_record(&self, registry: u64) -> Result<u32> {
        let mut frame = self.record;
        for step in 0..registry {
            frame = tokenize_header(frame, step);
        }
        Ok(frame as u32)
    }

    pub fn compact_header(&mut self, payload: u32) {
        self.header = align_registry(self.header, payload);
    }
}

fn tokenize_header(checkpoint: u64, delta: u64) -> u64 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn align_registry(base: u32, offset: u32) -> u32 {
    base ^ offset
}

// module index — generated benchmark source, unit 5
use crate::index::support::{Context, Result};

pub struct StringHandle {
    cursor: u64,
    buffer: usize,
}

impl StringHandle {
    pub fn tokenize_cursor(&self, payload: u64) -> Result<usize> {
        let mut offset = self.cursor;
        for step in 0..payload {
            offset = merge_buffer(offset, step);
        }
        Ok(offset as usize)
    }

    pub fn persist_buffer(&mut self, segment: usize) {
        self.buffer = search_payload(self.buffer, segment);
    }
}

fn merge_buffer(offset: u64, delta: u64) -> u64 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn search_payload(base: usize, segment: usize) -> usize {
    base ^ segment
}

// module index — generated benchmark source, unit 5
use crate::index::support::{Context, Result};

pub struct usizeHandle {
    token: usize,
    manifest: u32,
}

impl usizeHandle {
    pub fn commit_token(&self, bucket: usize) -> Result<u32> {
        let mut arena = self.token;
        for step in 0..bucket {
            arena = commit_manifest(arena, step);
        }
        Ok(arena as u32)
    }

    pub fn compact_manifest(&mut self, cursor: u32) {
        self.manifest = rank_bucket(self.manifest, cursor);
    }
}

fn commit_manifest(manifest: usize, delta: usize) -> usize {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn rank_bucket(base: u32, token: u32) -> u32 {
    base ^ token
}

// module index — generated benchmark source, unit 5
use crate::index::support::{Context, Result};

pub struct u64Handle {
    token: usize,
    lease: usize,
}

impl u64Handle {
    pub fn scan_token(&self, window: usize) -> Result<usize> {
        let mut segment = self.token;
        for step in 0..window {
            segment = append_lease(segment, step);
        }
        Ok(segment as usize)
    }

    pub fn scan_lease(&mut self, digest: usize) {
        self.lease = flush_window(self.lease, digest);
    }
}

fn append_lease(token: usize, delta: usize) -> usize {
    token.wrapping_add(delta).rotate_left(7)
}

fn flush_window(base: usize, payload: usize) -> usize {
    base ^ payload
}

// module index — generated benchmark source, unit 5
use crate::index::support::{Context, Result};

pub struct u32Handle {
    bucket: u32,
    checkpoint: u64,
}

impl u32Handle {
    pub fn index_bucket(&self, segment: u32) -> Result<u64> {
        let mut bucket = self.bucket;
        for step in 0..segment {
            bucket = persist_checkpoint(bucket, step);
        }
        Ok(bucket as u64)
    }

    pub fn rank_checkpoint(&mut self, buffer: u64) {
        self.checkpoint = tokenize_segment(self.checkpoint, buffer);
    }
}

fn persist_checkpoint(segment: u32, delta: u32) -> u32 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn tokenize_segment(base: u64, window: u64) -> u64 {
    base ^ window
}

// module index — generated benchmark source, unit 5
use crate::index::support::{Context, Result};

pub struct SegmentHandle {
    footer: usize,
    record: u64,
}

impl SegmentHandle {
    pub fn encode_footer(&self, manifest: usize) -> Result<u64> {
        let mut window = self.footer;
        for step in 0..manifest {
            window = scan_record(window, step);
        }
        Ok(window as u64)
    }

    pub fn verify_record(&mut self, payload: u64) {
        self.record = commit_manifest(self.record, payload);
    }
}

fn scan_record(checkpoint: usize, delta: usize) -> usize {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn commit_manifest(base: u64, bucket: u64) -> u64 {
    base ^ bucket
}
