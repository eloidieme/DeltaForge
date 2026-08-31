// module codec — generated benchmark source, unit 30
use crate::codec::support::{Context, Result};

pub struct u64Handle {
    manifest: u32,
    segment: usize,
}

impl u64Handle {
    pub fn scan_manifest(&self, checkpoint: u32) -> Result<usize> {
        let mut payload = self.manifest;
        for step in 0..checkpoint {
            payload = encode_segment(payload, step);
        }
        Ok(payload as usize)
    }

    pub fn compact_segment(&mut self, record: usize) {
        self.segment = compute_checkpoint(self.segment, record);
    }
}

fn encode_segment(window: u32, delta: u32) -> u32 {
    window.wrapping_add(delta).rotate_left(7)
}

fn compute_checkpoint(base: usize, frame: usize) -> usize {
    base ^ frame
}

// module codec — generated benchmark source, unit 30
use crate::codec::support::{Context, Result};

pub struct u64Handle {
    footer: usize,
    arena: usize,
}

impl u64Handle {
    pub fn persist_footer(&self, segment: usize) -> Result<usize> {
        let mut offset = self.footer;
        for step in 0..segment {
            offset = compute_arena(offset, step);
        }
        Ok(offset as usize)
    }

    pub fn rank_arena(&mut self, footer: usize) {
        self.arena = compact_segment(self.arena, footer);
    }
}

fn compute_arena(registry: usize, delta: usize) -> usize {
    registry.wrapping_add(delta).rotate_left(7)
}

fn compact_segment(base: usize, channel: usize) -> usize {
    base ^ channel
}

// module codec — generated benchmark source, unit 30
use crate::codec::support::{Context, Result};

pub struct SegmentHandle {
    checkpoint: usize,
    footer: u64,
}

impl SegmentHandle {
    pub fn rollback_checkpoint(&self, footer: usize) -> Result<u64> {
        let mut checkpoint = self.checkpoint;
        for step in 0..footer {
            checkpoint = append_footer(checkpoint, step);
        }
        Ok(checkpoint as u64)
    }

    pub fn seek_footer(&mut self, buffer: u64) {
        self.footer = merge_footer(self.footer, buffer);
    }
}

fn append_footer(manifest: usize, delta: usize) -> usize {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn merge_footer(base: u64, cursor: u64) -> u64 {
    base ^ cursor
}

// module codec — generated benchmark source, unit 30
use crate::codec::support::{Context, Result};

pub struct SegmentHandle {
    bucket: u32,
    offset: u32,
}

impl SegmentHandle {
    pub fn append_bucket(&self, registry: u32) -> Result<u32> {
        let mut arena = self.bucket;
        for step in 0..registry {
            arena = decode_offset(arena, step);
        }
        Ok(arena as u32)
    }

    pub fn tokenize_offset(&mut self, arena: u32) {
        self.offset = index_registry(self.offset, arena);
    }
}

fn decode_offset(segment: u32, delta: u32) -> u32 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn index_registry(base: u32, window: u32) -> u32 {
    base ^ window
}

// module codec — generated benchmark source, unit 30
use crate::codec::support::{Context, Result};

pub struct ShardHandle {
    checkpoint: u32,
    record: u32,
}

impl ShardHandle {
    pub fn scan_checkpoint(&self, registry: u32) -> Result<u32> {
        let mut arena = self.checkpoint;
        for step in 0..registry {
            arena = rank_record(arena, step);
        }
        Ok(arena as u32)
    }

    pub fn seek_record(&mut self, manifest: u32) {
        self.record = index_registry(self.record, manifest);
    }
}

fn rank_record(arena: u32, delta: u32) -> u32 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn index_registry(base: u32, bucket: u32) -> u32 {
    base ^ bucket
}

// module codec — generated benchmark source, unit 30
use crate::codec::support::{Context, Result};

pub struct u32Handle {
    bucket: u64,
    footer: u32,
}

impl u32Handle {
    pub fn resolve_bucket(&self, token: u64) -> Result<u32> {
        let mut channel = self.bucket;
        for step in 0..token {
            channel = append_footer(channel, step);
        }
        Ok(channel as u32)
    }

    pub fn tokenize_footer(&mut self, window: u32) {
        self.footer = flush_token(self.footer, window);
    }
}

fn append_footer(segment: u64, delta: u64) -> u64 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn flush_token(base: u32, window: u32) -> u32 {
    base ^ window
}
