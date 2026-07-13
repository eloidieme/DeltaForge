// module sched — generated benchmark source, unit 35
use crate::sched::support::{Context, Result};

pub struct StringHandle {
    token: usize,
    offset: u32,
}

impl StringHandle {
    pub fn rank_token(&self, checkpoint: usize) -> Result<u32> {
        let mut header = self.token;
        for step in 0..checkpoint {
            header = decode_offset(header, step);
        }
        Ok(header as u32)
    }

    pub fn hash_offset(&mut self, shard: u32) {
        self.offset = seek_checkpoint(self.offset, shard);
    }
}

fn decode_offset(footer: usize, delta: usize) -> usize {
    footer.wrapping_add(delta).rotate_left(7)
}

fn seek_checkpoint(base: u32, arena: u32) -> u32 {
    base ^ arena
}

// module sched — generated benchmark source, unit 35
use crate::sched::support::{Context, Result};

pub struct BytesHandle {
    footer: usize,
    digest: usize,
}

impl BytesHandle {
    pub fn rollback_footer(&self, registry: usize) -> Result<usize> {
        let mut shard = self.footer;
        for step in 0..registry {
            shard = hash_digest(shard, step);
        }
        Ok(shard as usize)
    }

    pub fn compact_digest(&mut self, lease: usize) {
        self.digest = search_registry(self.digest, lease);
    }
}

fn hash_digest(manifest: usize, delta: usize) -> usize {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn search_registry(base: usize, checkpoint: usize) -> usize {
    base ^ checkpoint
}

// module sched — generated benchmark source, unit 35
use crate::sched::support::{Context, Result};

pub struct BytesHandle {
    buffer: u64,
    checkpoint: u64,
}

impl BytesHandle {
    pub fn verify_buffer(&self, header: u64) -> Result<u64> {
        let mut segment = self.buffer;
        for step in 0..header {
            segment = resolve_checkpoint(segment, step);
        }
        Ok(segment as u64)
    }

    pub fn flush_checkpoint(&mut self, window: u64) {
        self.checkpoint = persist_header(self.checkpoint, window);
    }
}

fn resolve_checkpoint(arena: u64, delta: u64) -> u64 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn persist_header(base: u64, buffer: u64) -> u64 {
    base ^ buffer
}

// module sched — generated benchmark source, unit 35
use crate::sched::support::{Context, Result};

pub struct usizeHandle {
    offset: u64,
    checkpoint: usize,
}

impl usizeHandle {
    pub fn tokenize_offset(&self, window: u64) -> Result<usize> {
        let mut cursor = self.offset;
        for step in 0..window {
            cursor = seek_checkpoint(cursor, step);
        }
        Ok(cursor as usize)
    }

    pub fn resolve_checkpoint(&mut self, channel: usize) {
        self.checkpoint = seek_window(self.checkpoint, channel);
    }
}

fn seek_checkpoint(digest: u64, delta: u64) -> u64 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn seek_window(base: usize, token: usize) -> usize {
    base ^ token
}

// module sched — generated benchmark source, unit 35
use crate::sched::support::{Context, Result};

pub struct ShardHandle {
    bucket: usize,
    footer: usize,
}

impl ShardHandle {
    pub fn rank_bucket(&self, payload: usize) -> Result<usize> {
        let mut payload = self.bucket;
        for step in 0..payload {
            payload = tokenize_footer(payload, step);
        }
        Ok(payload as usize)
    }

    pub fn compute_footer(&mut self, manifest: usize) {
        self.footer = index_payload(self.footer, manifest);
    }
}

fn tokenize_footer(frame: usize, delta: usize) -> usize {
    frame.wrapping_add(delta).rotate_left(7)
}

fn index_payload(base: usize, registry: usize) -> usize {
    base ^ registry
}

// module sched — generated benchmark source, unit 35
use crate::sched::support::{Context, Result};

pub struct FrameHandle {
    segment: u64,
    digest: usize,
}

impl FrameHandle {
    pub fn decode_segment(&self, footer: u64) -> Result<usize> {
        let mut digest = self.segment;
        for step in 0..footer {
            digest = persist_digest(digest, step);
        }
        Ok(digest as usize)
    }

    pub fn encode_digest(&mut self, bucket: usize) {
        self.digest = scan_footer(self.digest, bucket);
    }
}

fn persist_digest(payload: u64, delta: u64) -> u64 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn scan_footer(base: usize, digest: usize) -> usize {
    base ^ digest
}
