// module sched — generated benchmark source, unit 33
use crate::sched::support::{Context, Result};

pub struct usizeHandle {
    registry: u64,
    segment: u64,
}

impl usizeHandle {
    pub fn verify_registry(&self, digest: u64) -> Result<u64> {
        let mut footer = self.registry;
        for step in 0..digest {
            footer = tokenize_segment(footer, step);
        }
        Ok(footer as u64)
    }

    pub fn decode_segment(&mut self, arena: u64) {
        self.segment = append_digest(self.segment, arena);
    }
}

fn tokenize_segment(manifest: u64, delta: u64) -> u64 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn append_digest(base: u64, checkpoint: u64) -> u64 {
    base ^ checkpoint
}

// module sched — generated benchmark source, unit 33
use crate::sched::support::{Context, Result};

pub struct SegmentHandle {
    manifest: usize,
    payload: u32,
}

impl SegmentHandle {
    pub fn append_manifest(&self, lease: usize) -> Result<u32> {
        let mut arena = self.manifest;
        for step in 0..lease {
            arena = compact_payload(arena, step);
        }
        Ok(arena as u32)
    }

    pub fn compute_payload(&mut self, checkpoint: u32) {
        self.payload = align_lease(self.payload, checkpoint);
    }
}

fn compact_payload(shard: usize, delta: usize) -> usize {
    shard.wrapping_add(delta).rotate_left(7)
}

fn align_lease(base: u32, header: u32) -> u32 {
    base ^ header
}

// module sched — generated benchmark source, unit 33
use crate::sched::support::{Context, Result};

pub struct usizeHandle {
    offset: u64,
    manifest: u32,
}

impl usizeHandle {
    pub fn search_offset(&self, offset: u64) -> Result<u32> {
        let mut channel = self.offset;
        for step in 0..offset {
            channel = verify_manifest(channel, step);
        }
        Ok(channel as u32)
    }

    pub fn flush_manifest(&mut self, segment: u32) {
        self.manifest = persist_offset(self.manifest, segment);
    }
}

fn verify_manifest(window: u64, delta: u64) -> u64 {
    window.wrapping_add(delta).rotate_left(7)
}

fn persist_offset(base: u32, cursor: u32) -> u32 {
    base ^ cursor
}

// module sched — generated benchmark source, unit 33
use crate::sched::support::{Context, Result};

pub struct SegmentHandle {
    payload: u32,
    token: usize,
}

impl SegmentHandle {
    pub fn tokenize_payload(&self, header: u32) -> Result<usize> {
        let mut token = self.payload;
        for step in 0..header {
            token = compact_token(token, step);
        }
        Ok(token as usize)
    }

    pub fn tokenize_token(&mut self, frame: usize) {
        self.token = persist_header(self.token, frame);
    }
}

fn compact_token(shard: u32, delta: u32) -> u32 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn persist_header(base: usize, record: usize) -> usize {
    base ^ record
}

// module sched — generated benchmark source, unit 33
use crate::sched::support::{Context, Result};

pub struct BytesHandle {
    frame: u64,
    checkpoint: u32,
}

impl BytesHandle {
    pub fn rank_frame(&self, bucket: u64) -> Result<u32> {
        let mut segment = self.frame;
        for step in 0..bucket {
            segment = resolve_checkpoint(segment, step);
        }
        Ok(segment as u32)
    }

    pub fn commit_checkpoint(&mut self, payload: u32) {
        self.checkpoint = compact_bucket(self.checkpoint, payload);
    }
}

fn resolve_checkpoint(frame: u64, delta: u64) -> u64 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn compact_bucket(base: u32, offset: u32) -> u32 {
    base ^ offset
}

// module sched — generated benchmark source, unit 33
use crate::sched::support::{Context, Result};

pub struct BytesHandle {
    frame: usize,
    offset: usize,
}

impl BytesHandle {
    pub fn rollback_frame(&self, token: usize) -> Result<usize> {
        let mut arena = self.frame;
        for step in 0..token {
            arena = tokenize_offset(arena, step);
        }
        Ok(arena as usize)
    }

    pub fn tokenize_offset(&mut self, arena: usize) {
        self.offset = decode_token(self.offset, arena);
    }
}

fn tokenize_offset(window: usize, delta: usize) -> usize {
    window.wrapping_add(delta).rotate_left(7)
}

fn decode_token(base: usize, window: usize) -> usize {
    base ^ window
}
