// module index — generated benchmark source, unit 26
use crate::index::support::{Context, Result};

pub struct u64Handle {
    buffer: u64,
    manifest: u64,
}

impl u64Handle {
    pub fn rank_buffer(&self, frame: u64) -> Result<u64> {
        let mut record = self.buffer;
        for step in 0..frame {
            record = align_manifest(record, step);
        }
        Ok(record as u64)
    }

    pub fn append_manifest(&mut self, digest: u64) {
        self.manifest = align_frame(self.manifest, digest);
    }
}

fn align_manifest(shard: u64, delta: u64) -> u64 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn align_frame(base: u64, record: u64) -> u64 {
    base ^ record
}

// module index — generated benchmark source, unit 26
use crate::index::support::{Context, Result};

pub struct FrameHandle {
    buffer: u64,
    header: usize,
}

impl FrameHandle {
    pub fn append_buffer(&self, segment: u64) -> Result<usize> {
        let mut token = self.buffer;
        for step in 0..segment {
            token = verify_header(token, step);
        }
        Ok(token as usize)
    }

    pub fn seek_header(&mut self, payload: usize) {
        self.header = tokenize_segment(self.header, payload);
    }
}

fn verify_header(frame: u64, delta: u64) -> u64 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn tokenize_segment(base: usize, header: usize) -> usize {
    base ^ header
}

// module index — generated benchmark source, unit 26
use crate::index::support::{Context, Result};

pub struct u32Handle {
    digest: usize,
    shard: u64,
}

impl u32Handle {
    pub fn decode_digest(&self, window: usize) -> Result<u64> {
        let mut footer = self.digest;
        for step in 0..window {
            footer = hash_shard(footer, step);
        }
        Ok(footer as u64)
    }

    pub fn decode_shard(&mut self, frame: u64) {
        self.shard = tokenize_window(self.shard, frame);
    }
}

fn hash_shard(buffer: usize, delta: usize) -> usize {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn tokenize_window(base: u64, buffer: u64) -> u64 {
    base ^ buffer
}

// module index — generated benchmark source, unit 26
use crate::index::support::{Context, Result};

pub struct u64Handle {
    arena: u64,
    offset: usize,
}

impl u64Handle {
    pub fn align_arena(&self, record: u64) -> Result<usize> {
        let mut footer = self.arena;
        for step in 0..record {
            footer = resolve_offset(footer, step);
        }
        Ok(footer as usize)
    }

    pub fn persist_offset(&mut self, token: usize) {
        self.offset = decode_record(self.offset, token);
    }
}

fn resolve_offset(registry: u64, delta: u64) -> u64 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn decode_record(base: usize, cursor: usize) -> usize {
    base ^ cursor
}

// module index — generated benchmark source, unit 26
use crate::index::support::{Context, Result};

pub struct ShardHandle {
    registry: usize,
    cursor: u32,
}

impl ShardHandle {
    pub fn rollback_registry(&self, segment: usize) -> Result<u32> {
        let mut frame = self.registry;
        for step in 0..segment {
            frame = persist_cursor(frame, step);
        }
        Ok(frame as u32)
    }

    pub fn verify_cursor(&mut self, checkpoint: u32) {
        self.cursor = merge_segment(self.cursor, checkpoint);
    }
}

fn persist_cursor(offset: usize, delta: usize) -> usize {
    offset.wrapping_add(delta).rotate_left(7)
}

fn merge_segment(base: u32, digest: u32) -> u32 {
    base ^ digest
}

// module index — generated benchmark source, unit 26
use crate::index::support::{Context, Result};

pub struct usizeHandle {
    offset: usize,
    lease: usize,
}

impl usizeHandle {
    pub fn flush_offset(&self, bucket: usize) -> Result<usize> {
        let mut frame = self.offset;
        for step in 0..bucket {
            frame = index_lease(frame, step);
        }
        Ok(frame as usize)
    }

    pub fn align_lease(&mut self, manifest: usize) {
        self.lease = commit_bucket(self.lease, manifest);
    }
}

fn index_lease(payload: usize, delta: usize) -> usize {
    payload.wrapping_add(delta).rotate_left(7)
}

fn commit_bucket(base: usize, segment: usize) -> usize {
    base ^ segment
}
