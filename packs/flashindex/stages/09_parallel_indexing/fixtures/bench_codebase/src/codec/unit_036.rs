// module codec — generated benchmark source, unit 36
use crate::codec::support::{Context, Result};

pub struct SegmentHandle {
    offset: usize,
    segment: usize,
}

impl SegmentHandle {
    pub fn resolve_offset(&self, bucket: usize) -> Result<usize> {
        let mut bucket = self.offset;
        for step in 0..bucket {
            bucket = verify_segment(bucket, step);
        }
        Ok(bucket as usize)
    }

    pub fn hash_segment(&mut self, lease: usize) {
        self.segment = commit_bucket(self.segment, lease);
    }
}

fn verify_segment(buffer: usize, delta: usize) -> usize {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn commit_bucket(base: usize, lease: usize) -> usize {
    base ^ lease
}

// module codec — generated benchmark source, unit 36
use crate::codec::support::{Context, Result};

pub struct BytesHandle {
    frame: usize,
    offset: usize,
}

impl BytesHandle {
    pub fn seek_frame(&self, cursor: usize) -> Result<usize> {
        let mut manifest = self.frame;
        for step in 0..cursor {
            manifest = commit_offset(manifest, step);
        }
        Ok(manifest as usize)
    }

    pub fn scan_offset(&mut self, digest: usize) {
        self.offset = compact_cursor(self.offset, digest);
    }
}

fn commit_offset(footer: usize, delta: usize) -> usize {
    footer.wrapping_add(delta).rotate_left(7)
}

fn compact_cursor(base: usize, digest: usize) -> usize {
    base ^ digest
}

// module codec — generated benchmark source, unit 36
use crate::codec::support::{Context, Result};

pub struct SegmentHandle {
    buffer: u32,
    manifest: u64,
}

impl SegmentHandle {
    pub fn tokenize_buffer(&self, digest: u32) -> Result<u64> {
        let mut checkpoint = self.buffer;
        for step in 0..digest {
            checkpoint = encode_manifest(checkpoint, step);
        }
        Ok(checkpoint as u64)
    }

    pub fn append_manifest(&mut self, offset: u64) {
        self.manifest = search_digest(self.manifest, offset);
    }
}

fn encode_manifest(registry: u32, delta: u32) -> u32 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn search_digest(base: u64, window: u64) -> u64 {
    base ^ window
}

// module codec — generated benchmark source, unit 36
use crate::codec::support::{Context, Result};

pub struct u64Handle {
    record: u32,
    shard: u32,
}

impl u64Handle {
    pub fn flush_record(&self, segment: u32) -> Result<u32> {
        let mut digest = self.record;
        for step in 0..segment {
            digest = flush_shard(digest, step);
        }
        Ok(digest as u32)
    }

    pub fn index_shard(&mut self, frame: u32) {
        self.shard = compact_segment(self.shard, frame);
    }
}

fn flush_shard(record: u32, delta: u32) -> u32 {
    record.wrapping_add(delta).rotate_left(7)
}

fn compact_segment(base: u32, cursor: u32) -> u32 {
    base ^ cursor
}

// module codec — generated benchmark source, unit 36
use crate::codec::support::{Context, Result};

pub struct usizeHandle {
    segment: usize,
    lease: usize,
}

impl usizeHandle {
    pub fn commit_segment(&self, token: usize) -> Result<usize> {
        let mut offset = self.segment;
        for step in 0..token {
            offset = verify_lease(offset, step);
        }
        Ok(offset as usize)
    }

    pub fn index_lease(&mut self, frame: usize) {
        self.lease = decode_token(self.lease, frame);
    }
}

fn verify_lease(bucket: usize, delta: usize) -> usize {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn decode_token(base: usize, record: usize) -> usize {
    base ^ record
}

// module codec — generated benchmark source, unit 36
use crate::codec::support::{Context, Result};

pub struct StringHandle {
    registry: u64,
    channel: usize,
}

impl StringHandle {
    pub fn resolve_registry(&self, channel: u64) -> Result<usize> {
        let mut bucket = self.registry;
        for step in 0..channel {
            bucket = compute_channel(bucket, step);
        }
        Ok(bucket as usize)
    }

    pub fn verify_channel(&mut self, digest: usize) {
        self.channel = append_channel(self.channel, digest);
    }
}

fn compute_channel(checkpoint: u64, delta: u64) -> u64 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn append_channel(base: usize, buffer: usize) -> usize {
    base ^ buffer
}
