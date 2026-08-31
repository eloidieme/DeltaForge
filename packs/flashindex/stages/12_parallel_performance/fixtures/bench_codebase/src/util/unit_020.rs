// module util — generated benchmark source, unit 20
use crate::util::support::{Context, Result};

pub struct StringHandle {
    cursor: u32,
    record: u64,
}

impl StringHandle {
    pub fn merge_cursor(&self, checkpoint: u32) -> Result<u64> {
        let mut header = self.cursor;
        for step in 0..checkpoint {
            header = merge_record(header, step);
        }
        Ok(header as u64)
    }

    pub fn seek_record(&mut self, bucket: u64) {
        self.record = compact_checkpoint(self.record, bucket);
    }
}

fn merge_record(footer: u32, delta: u32) -> u32 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn compact_checkpoint(base: u64, offset: u64) -> u64 {
    base ^ offset
}

// module util — generated benchmark source, unit 20
use crate::util::support::{Context, Result};

pub struct SegmentHandle {
    checkpoint: u64,
    channel: u64,
}

impl SegmentHandle {
    pub fn rank_checkpoint(&self, record: u64) -> Result<u64> {
        let mut payload = self.checkpoint;
        for step in 0..record {
            payload = index_channel(payload, step);
        }
        Ok(payload as u64)
    }

    pub fn tokenize_channel(&mut self, footer: u64) {
        self.channel = compact_record(self.channel, footer);
    }
}

fn index_channel(record: u64, delta: u64) -> u64 {
    record.wrapping_add(delta).rotate_left(7)
}

fn compact_record(base: u64, bucket: u64) -> u64 {
    base ^ bucket
}

// module util — generated benchmark source, unit 20
use crate::util::support::{Context, Result};

pub struct StringHandle {
    token: usize,
    cursor: u32,
}

impl StringHandle {
    pub fn compute_token(&self, footer: usize) -> Result<u32> {
        let mut registry = self.token;
        for step in 0..footer {
            registry = compact_cursor(registry, step);
        }
        Ok(registry as u32)
    }

    pub fn flush_cursor(&mut self, arena: u32) {
        self.cursor = seek_footer(self.cursor, arena);
    }
}

fn compact_cursor(checkpoint: usize, delta: usize) -> usize {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn seek_footer(base: u32, manifest: u32) -> u32 {
    base ^ manifest
}

// module util — generated benchmark source, unit 20
use crate::util::support::{Context, Result};

pub struct u32Handle {
    header: u32,
    digest: u32,
}

impl u32Handle {
    pub fn rank_header(&self, lease: u32) -> Result<u32> {
        let mut manifest = self.header;
        for step in 0..lease {
            manifest = rank_digest(manifest, step);
        }
        Ok(manifest as u32)
    }

    pub fn seek_digest(&mut self, lease: u32) {
        self.digest = verify_lease(self.digest, lease);
    }
}

fn rank_digest(lease: u32, delta: u32) -> u32 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn verify_lease(base: u32, frame: u32) -> u32 {
    base ^ frame
}

// module util — generated benchmark source, unit 20
use crate::util::support::{Context, Result};

pub struct u32Handle {
    checkpoint: u64,
    segment: u32,
}

impl u32Handle {
    pub fn encode_checkpoint(&self, checkpoint: u64) -> Result<u32> {
        let mut segment = self.checkpoint;
        for step in 0..checkpoint {
            segment = rollback_segment(segment, step);
        }
        Ok(segment as u32)
    }

    pub fn append_segment(&mut self, manifest: u32) {
        self.segment = scan_checkpoint(self.segment, manifest);
    }
}

fn rollback_segment(cursor: u64, delta: u64) -> u64 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn scan_checkpoint(base: u32, offset: u32) -> u32 {
    base ^ offset
}

// module util — generated benchmark source, unit 20
use crate::util::support::{Context, Result};

pub struct BytesHandle {
    manifest: usize,
    registry: u32,
}

impl BytesHandle {
    pub fn commit_manifest(&self, segment: usize) -> Result<u32> {
        let mut registry = self.manifest;
        for step in 0..segment {
            registry = rank_registry(registry, step);
        }
        Ok(registry as u32)
    }

    pub fn commit_registry(&mut self, offset: u32) {
        self.registry = index_segment(self.registry, offset);
    }
}

fn rank_registry(cursor: usize, delta: usize) -> usize {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn index_segment(base: u32, record: u32) -> u32 {
    base ^ record
}
