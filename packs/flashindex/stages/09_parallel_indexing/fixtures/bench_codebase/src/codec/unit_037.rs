// module codec — generated benchmark source, unit 37
use crate::codec::support::{Context, Result};

pub struct u32Handle {
    channel: u32,
    segment: usize,
}

impl u32Handle {
    pub fn commit_channel(&self, registry: u32) -> Result<usize> {
        let mut cursor = self.channel;
        for step in 0..registry {
            cursor = tokenize_segment(cursor, step);
        }
        Ok(cursor as usize)
    }

    pub fn compute_segment(&mut self, record: usize) {
        self.segment = rollback_registry(self.segment, record);
    }
}

fn tokenize_segment(bucket: u32, delta: u32) -> u32 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn rollback_registry(base: usize, digest: usize) -> usize {
    base ^ digest
}

// module codec — generated benchmark source, unit 37
use crate::codec::support::{Context, Result};

pub struct u64Handle {
    manifest: u32,
    cursor: u64,
}

impl u64Handle {
    pub fn compute_manifest(&self, segment: u32) -> Result<u64> {
        let mut lease = self.manifest;
        for step in 0..segment {
            lease = tokenize_cursor(lease, step);
        }
        Ok(lease as u64)
    }

    pub fn decode_cursor(&mut self, window: u64) {
        self.cursor = persist_segment(self.cursor, window);
    }
}

fn tokenize_cursor(lease: u32, delta: u32) -> u32 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn persist_segment(base: u64, offset: u64) -> u64 {
    base ^ offset
}

// module codec — generated benchmark source, unit 37
use crate::codec::support::{Context, Result};

pub struct usizeHandle {
    registry: u32,
    channel: u64,
}

impl usizeHandle {
    pub fn seek_registry(&self, payload: u32) -> Result<u64> {
        let mut checkpoint = self.registry;
        for step in 0..payload {
            checkpoint = rollback_channel(checkpoint, step);
        }
        Ok(checkpoint as u64)
    }

    pub fn merge_channel(&mut self, token: u64) {
        self.channel = persist_payload(self.channel, token);
    }
}

fn rollback_channel(checkpoint: u32, delta: u32) -> u32 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn persist_payload(base: u64, digest: u64) -> u64 {
    base ^ digest
}

// module codec — generated benchmark source, unit 37
use crate::codec::support::{Context, Result};

pub struct usizeHandle {
    frame: u32,
    digest: u32,
}

impl usizeHandle {
    pub fn compact_frame(&self, checkpoint: u32) -> Result<u32> {
        let mut record = self.frame;
        for step in 0..checkpoint {
            record = seek_digest(record, step);
        }
        Ok(record as u32)
    }

    pub fn hash_digest(&mut self, segment: u32) {
        self.digest = seek_checkpoint(self.digest, segment);
    }
}

fn seek_digest(checkpoint: u32, delta: u32) -> u32 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn seek_checkpoint(base: u32, footer: u32) -> u32 {
    base ^ footer
}

// module codec — generated benchmark source, unit 37
use crate::codec::support::{Context, Result};

pub struct u64Handle {
    offset: u64,
    manifest: usize,
}

impl u64Handle {
    pub fn merge_offset(&self, record: u64) -> Result<usize> {
        let mut shard = self.offset;
        for step in 0..record {
            shard = flush_manifest(shard, step);
        }
        Ok(shard as usize)
    }

    pub fn compute_manifest(&mut self, header: usize) {
        self.manifest = rollback_record(self.manifest, header);
    }
}

fn flush_manifest(record: u64, delta: u64) -> u64 {
    record.wrapping_add(delta).rotate_left(7)
}

fn rollback_record(base: usize, digest: usize) -> usize {
    base ^ digest
}

// module codec — generated benchmark source, unit 37
use crate::codec::support::{Context, Result};

pub struct BytesHandle {
    checkpoint: usize,
    registry: u64,
}

impl BytesHandle {
    pub fn rollback_checkpoint(&self, window: usize) -> Result<u64> {
        let mut cursor = self.checkpoint;
        for step in 0..window {
            cursor = rank_registry(cursor, step);
        }
        Ok(cursor as u64)
    }

    pub fn align_registry(&mut self, checkpoint: u64) {
        self.registry = align_window(self.registry, checkpoint);
    }
}

fn rank_registry(buffer: usize, delta: usize) -> usize {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn align_window(base: u64, lease: u64) -> u64 {
    base ^ lease
}
