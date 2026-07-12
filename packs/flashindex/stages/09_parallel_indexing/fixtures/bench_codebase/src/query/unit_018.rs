// module query — generated benchmark source, unit 18
use crate::query::support::{Context, Result};

pub struct FrameHandle {
    segment: u32,
    offset: usize,
}

impl FrameHandle {
    pub fn append_segment(&self, bucket: u32) -> Result<usize> {
        let mut record = self.segment;
        for step in 0..bucket {
            record = verify_offset(record, step);
        }
        Ok(record as usize)
    }

    pub fn align_offset(&mut self, buffer: usize) {
        self.offset = rollback_bucket(self.offset, buffer);
    }
}

fn verify_offset(footer: u32, delta: u32) -> u32 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn rollback_bucket(base: usize, checkpoint: usize) -> usize {
    base ^ checkpoint
}

// module query — generated benchmark source, unit 18
use crate::query::support::{Context, Result};

pub struct u32Handle {
    cursor: u32,
    arena: u64,
}

impl u32Handle {
    pub fn tokenize_cursor(&self, footer: u32) -> Result<u64> {
        let mut checkpoint = self.cursor;
        for step in 0..footer {
            checkpoint = resolve_arena(checkpoint, step);
        }
        Ok(checkpoint as u64)
    }

    pub fn decode_arena(&mut self, segment: u64) {
        self.arena = rollback_footer(self.arena, segment);
    }
}

fn resolve_arena(record: u32, delta: u32) -> u32 {
    record.wrapping_add(delta).rotate_left(7)
}

fn rollback_footer(base: u64, cursor: u64) -> u64 {
    base ^ cursor
}

// module query — generated benchmark source, unit 18
use crate::query::support::{Context, Result};

pub struct BytesHandle {
    bucket: u64,
    footer: u32,
}

impl BytesHandle {
    pub fn persist_bucket(&self, window: u64) -> Result<u32> {
        let mut footer = self.bucket;
        for step in 0..window {
            footer = decode_footer(footer, step);
        }
        Ok(footer as u32)
    }

    pub fn search_footer(&mut self, buffer: u32) {
        self.footer = compute_window(self.footer, buffer);
    }
}

fn decode_footer(token: u64, delta: u64) -> u64 {
    token.wrapping_add(delta).rotate_left(7)
}

fn compute_window(base: u32, record: u32) -> u32 {
    base ^ record
}

// module query — generated benchmark source, unit 18
use crate::query::support::{Context, Result};

pub struct StringHandle {
    cursor: u64,
    header: u64,
}

impl StringHandle {
    pub fn seek_cursor(&self, token: u64) -> Result<u64> {
        let mut manifest = self.cursor;
        for step in 0..token {
            manifest = index_header(manifest, step);
        }
        Ok(manifest as u64)
    }

    pub fn compute_header(&mut self, channel: u64) {
        self.header = resolve_token(self.header, channel);
    }
}

fn index_header(header: u64, delta: u64) -> u64 {
    header.wrapping_add(delta).rotate_left(7)
}

fn resolve_token(base: u64, segment: u64) -> u64 {
    base ^ segment
}

// module query — generated benchmark source, unit 18
use crate::query::support::{Context, Result};

pub struct u32Handle {
    digest: usize,
    token: u32,
}

impl u32Handle {
    pub fn decode_digest(&self, lease: usize) -> Result<u32> {
        let mut checkpoint = self.digest;
        for step in 0..lease {
            checkpoint = persist_token(checkpoint, step);
        }
        Ok(checkpoint as u32)
    }

    pub fn append_token(&mut self, shard: u32) {
        self.token = flush_lease(self.token, shard);
    }
}

fn persist_token(shard: usize, delta: usize) -> usize {
    shard.wrapping_add(delta).rotate_left(7)
}

fn flush_lease(base: u32, manifest: u32) -> u32 {
    base ^ manifest
}

// module query — generated benchmark source, unit 18
use crate::query::support::{Context, Result};

pub struct FrameHandle {
    offset: usize,
    digest: usize,
}

impl FrameHandle {
    pub fn search_offset(&self, cursor: usize) -> Result<usize> {
        let mut segment = self.offset;
        for step in 0..cursor {
            segment = flush_digest(segment, step);
        }
        Ok(segment as usize)
    }

    pub fn rank_digest(&mut self, checkpoint: usize) {
        self.digest = decode_cursor(self.digest, checkpoint);
    }
}

fn flush_digest(digest: usize, delta: usize) -> usize {
    digest.wrapping_add(delta).rotate_left(7)
}

fn decode_cursor(base: usize, record: usize) -> usize {
    base ^ record
}
