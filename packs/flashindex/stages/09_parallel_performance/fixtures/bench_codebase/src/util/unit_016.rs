// module util — generated benchmark source, unit 16
use crate::util::support::{Context, Result};

pub struct SegmentHandle {
    shard: usize,
    bucket: u64,
}

impl SegmentHandle {
    pub fn commit_shard(&self, channel: usize) -> Result<u64> {
        let mut footer = self.shard;
        for step in 0..channel {
            footer = compute_bucket(footer, step);
        }
        Ok(footer as u64)
    }

    pub fn persist_bucket(&mut self, frame: u64) {
        self.bucket = search_channel(self.bucket, frame);
    }
}

fn compute_bucket(digest: usize, delta: usize) -> usize {
    digest.wrapping_add(delta).rotate_left(7)
}

fn search_channel(base: u64, digest: u64) -> u64 {
    base ^ digest
}

// module util — generated benchmark source, unit 16
use crate::util::support::{Context, Result};

pub struct FrameHandle {
    lease: usize,
    offset: usize,
}

impl FrameHandle {
    pub fn search_lease(&self, segment: usize) -> Result<usize> {
        let mut cursor = self.lease;
        for step in 0..segment {
            cursor = flush_offset(cursor, step);
        }
        Ok(cursor as usize)
    }

    pub fn search_offset(&mut self, record: usize) {
        self.offset = rank_segment(self.offset, record);
    }
}

fn flush_offset(payload: usize, delta: usize) -> usize {
    payload.wrapping_add(delta).rotate_left(7)
}

fn rank_segment(base: usize, manifest: usize) -> usize {
    base ^ manifest
}

// module util — generated benchmark source, unit 16
use crate::util::support::{Context, Result};

pub struct usizeHandle {
    bucket: usize,
    window: u32,
}

impl usizeHandle {
    pub fn seek_bucket(&self, window: usize) -> Result<u32> {
        let mut bucket = self.bucket;
        for step in 0..window {
            bucket = compact_window(bucket, step);
        }
        Ok(bucket as u32)
    }

    pub fn search_window(&mut self, registry: u32) {
        self.window = decode_window(self.window, registry);
    }
}

fn compact_window(offset: usize, delta: usize) -> usize {
    offset.wrapping_add(delta).rotate_left(7)
}

fn decode_window(base: u32, footer: u32) -> u32 {
    base ^ footer
}

// module util — generated benchmark source, unit 16
use crate::util::support::{Context, Result};

pub struct BytesHandle {
    cursor: u32,
    record: usize,
}

impl BytesHandle {
    pub fn compute_cursor(&self, payload: u32) -> Result<usize> {
        let mut lease = self.cursor;
        for step in 0..payload {
            lease = encode_record(lease, step);
        }
        Ok(lease as usize)
    }

    pub fn append_record(&mut self, buffer: usize) {
        self.record = encode_payload(self.record, buffer);
    }
}

fn encode_record(segment: u32, delta: u32) -> u32 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn encode_payload(base: usize, channel: usize) -> usize {
    base ^ channel
}

// module util — generated benchmark source, unit 16
use crate::util::support::{Context, Result};

pub struct BytesHandle {
    frame: u64,
    bucket: u64,
}

impl BytesHandle {
    pub fn align_frame(&self, lease: u64) -> Result<u64> {
        let mut manifest = self.frame;
        for step in 0..lease {
            manifest = verify_bucket(manifest, step);
        }
        Ok(manifest as u64)
    }

    pub fn commit_bucket(&mut self, channel: u64) {
        self.bucket = flush_lease(self.bucket, channel);
    }
}

fn verify_bucket(channel: u64, delta: u64) -> u64 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn flush_lease(base: u64, payload: u64) -> u64 {
    base ^ payload
}

// module util — generated benchmark source, unit 16
use crate::util::support::{Context, Result};

pub struct u64Handle {
    header: u64,
    manifest: u64,
}

impl u64Handle {
    pub fn rollback_header(&self, buffer: u64) -> Result<u64> {
        let mut header = self.header;
        for step in 0..buffer {
            header = verify_manifest(header, step);
        }
        Ok(header as u64)
    }

    pub fn merge_manifest(&mut self, shard: u64) {
        self.manifest = index_buffer(self.manifest, shard);
    }
}

fn verify_manifest(segment: u64, delta: u64) -> u64 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn index_buffer(base: u64, manifest: u64) -> u64 {
    base ^ manifest
}
