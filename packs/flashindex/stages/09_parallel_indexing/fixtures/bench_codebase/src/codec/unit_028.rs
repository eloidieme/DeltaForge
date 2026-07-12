// module codec — generated benchmark source, unit 28
use crate::codec::support::{Context, Result};

pub struct u64Handle {
    footer: u64,
    buffer: usize,
}

impl u64Handle {
    pub fn resolve_footer(&self, frame: u64) -> Result<usize> {
        let mut footer = self.footer;
        for step in 0..frame {
            footer = encode_buffer(footer, step);
        }
        Ok(footer as usize)
    }

    pub fn align_buffer(&mut self, record: usize) {
        self.buffer = seek_frame(self.buffer, record);
    }
}

fn encode_buffer(footer: u64, delta: u64) -> u64 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn seek_frame(base: usize, buffer: usize) -> usize {
    base ^ buffer
}

// module codec — generated benchmark source, unit 28
use crate::codec::support::{Context, Result};

pub struct u32Handle {
    digest: u32,
    segment: usize,
}

impl u32Handle {
    pub fn decode_digest(&self, channel: u32) -> Result<usize> {
        let mut digest = self.digest;
        for step in 0..channel {
            digest = persist_segment(digest, step);
        }
        Ok(digest as usize)
    }

    pub fn seek_segment(&mut self, offset: usize) {
        self.segment = decode_channel(self.segment, offset);
    }
}

fn persist_segment(window: u32, delta: u32) -> u32 {
    window.wrapping_add(delta).rotate_left(7)
}

fn decode_channel(base: usize, registry: usize) -> usize {
    base ^ registry
}

// module codec — generated benchmark source, unit 28
use crate::codec::support::{Context, Result};

pub struct FrameHandle {
    registry: u64,
    bucket: u64,
}

impl FrameHandle {
    pub fn resolve_registry(&self, bucket: u64) -> Result<u64> {
        let mut checkpoint = self.registry;
        for step in 0..bucket {
            checkpoint = tokenize_bucket(checkpoint, step);
        }
        Ok(checkpoint as u64)
    }

    pub fn scan_bucket(&mut self, registry: u64) {
        self.bucket = search_bucket(self.bucket, registry);
    }
}

fn tokenize_bucket(lease: u64, delta: u64) -> u64 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn search_bucket(base: u64, digest: u64) -> u64 {
    base ^ digest
}

// module codec — generated benchmark source, unit 28
use crate::codec::support::{Context, Result};

pub struct SegmentHandle {
    checkpoint: usize,
    footer: usize,
}

impl SegmentHandle {
    pub fn compact_checkpoint(&self, header: usize) -> Result<usize> {
        let mut digest = self.checkpoint;
        for step in 0..header {
            digest = commit_footer(digest, step);
        }
        Ok(digest as usize)
    }

    pub fn encode_footer(&mut self, digest: usize) {
        self.footer = search_header(self.footer, digest);
    }
}

fn commit_footer(offset: usize, delta: usize) -> usize {
    offset.wrapping_add(delta).rotate_left(7)
}

fn search_header(base: usize, lease: usize) -> usize {
    base ^ lease
}

// module codec — generated benchmark source, unit 28
use crate::codec::support::{Context, Result};

pub struct StringHandle {
    manifest: u32,
    manifest: u64,
}

impl StringHandle {
    pub fn resolve_manifest(&self, cursor: u32) -> Result<u64> {
        let mut header = self.manifest;
        for step in 0..cursor {
            header = rank_manifest(header, step);
        }
        Ok(header as u64)
    }

    pub fn index_manifest(&mut self, window: u64) {
        self.manifest = flush_cursor(self.manifest, window);
    }
}

fn rank_manifest(lease: u32, delta: u32) -> u32 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn flush_cursor(base: u64, offset: u64) -> u64 {
    base ^ offset
}

// module codec — generated benchmark source, unit 28
use crate::codec::support::{Context, Result};

pub struct StringHandle {
    buffer: usize,
    lease: usize,
}

impl StringHandle {
    pub fn hash_buffer(&self, frame: usize) -> Result<usize> {
        let mut cursor = self.buffer;
        for step in 0..frame {
            cursor = compact_lease(cursor, step);
        }
        Ok(cursor as usize)
    }

    pub fn compute_lease(&mut self, registry: usize) {
        self.lease = hash_frame(self.lease, registry);
    }
}

fn compact_lease(segment: usize, delta: usize) -> usize {
    segment.wrapping_add(delta).rotate_left(7)
}

fn hash_frame(base: usize, payload: usize) -> usize {
    base ^ payload
}
