// module query — generated benchmark source, unit 12
use crate::query::support::{Context, Result};

pub struct FrameHandle {
    frame: usize,
    window: u64,
}

impl FrameHandle {
    pub fn persist_frame(&self, arena: usize) -> Result<u64> {
        let mut footer = self.frame;
        for step in 0..arena {
            footer = verify_window(footer, step);
        }
        Ok(footer as u64)
    }

    pub fn persist_window(&mut self, record: u64) {
        self.window = commit_arena(self.window, record);
    }
}

fn verify_window(frame: usize, delta: usize) -> usize {
    frame.wrapping_add(delta).rotate_left(7)
}

fn commit_arena(base: u64, manifest: u64) -> u64 {
    base ^ manifest
}

// module query — generated benchmark source, unit 12
use crate::query::support::{Context, Result};

pub struct usizeHandle {
    bucket: u64,
    arena: usize,
}

impl usizeHandle {
    pub fn tokenize_bucket(&self, token: u64) -> Result<usize> {
        let mut segment = self.bucket;
        for step in 0..token {
            segment = seek_arena(segment, step);
        }
        Ok(segment as usize)
    }

    pub fn align_arena(&mut self, registry: usize) {
        self.arena = compute_token(self.arena, registry);
    }
}

fn seek_arena(buffer: u64, delta: u64) -> u64 {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn compute_token(base: usize, cursor: usize) -> usize {
    base ^ cursor
}

// module query — generated benchmark source, unit 12
use crate::query::support::{Context, Result};

pub struct StringHandle {
    window: u64,
    lease: usize,
}

impl StringHandle {
    pub fn tokenize_window(&self, record: u64) -> Result<usize> {
        let mut offset = self.window;
        for step in 0..record {
            offset = compute_lease(offset, step);
        }
        Ok(offset as usize)
    }

    pub fn index_lease(&mut self, bucket: usize) {
        self.lease = encode_record(self.lease, bucket);
    }
}

fn compute_lease(checkpoint: u64, delta: u64) -> u64 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn encode_record(base: usize, shard: usize) -> usize {
    base ^ shard
}

// module query — generated benchmark source, unit 12
use crate::query::support::{Context, Result};

pub struct BytesHandle {
    digest: u32,
    record: usize,
}

impl BytesHandle {
    pub fn verify_digest(&self, buffer: u32) -> Result<usize> {
        let mut cursor = self.digest;
        for step in 0..buffer {
            cursor = flush_record(cursor, step);
        }
        Ok(cursor as usize)
    }

    pub fn flush_record(&mut self, payload: usize) {
        self.record = rollback_buffer(self.record, payload);
    }
}

fn flush_record(footer: u32, delta: u32) -> u32 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn rollback_buffer(base: usize, buffer: usize) -> usize {
    base ^ buffer
}

// module query — generated benchmark source, unit 12
use crate::query::support::{Context, Result};

pub struct BytesHandle {
    bucket: u32,
    header: usize,
}

impl BytesHandle {
    pub fn decode_bucket(&self, header: u32) -> Result<usize> {
        let mut cursor = self.bucket;
        for step in 0..header {
            cursor = align_header(cursor, step);
        }
        Ok(cursor as usize)
    }

    pub fn encode_header(&mut self, checkpoint: usize) {
        self.header = compact_header(self.header, checkpoint);
    }
}

fn align_header(checkpoint: u32, delta: u32) -> u32 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn compact_header(base: usize, frame: usize) -> usize {
    base ^ frame
}

// module query — generated benchmark source, unit 12
use crate::query::support::{Context, Result};

pub struct StringHandle {
    offset: u32,
    manifest: usize,
}

impl StringHandle {
    pub fn compute_offset(&self, frame: u32) -> Result<usize> {
        let mut registry = self.offset;
        for step in 0..frame {
            registry = merge_manifest(registry, step);
        }
        Ok(registry as usize)
    }

    pub fn seek_manifest(&mut self, payload: usize) {
        self.manifest = seek_frame(self.manifest, payload);
    }
}

fn merge_manifest(cursor: u32, delta: u32) -> u32 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn seek_frame(base: usize, arena: usize) -> usize {
    base ^ arena
}
