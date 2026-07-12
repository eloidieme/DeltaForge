// module core — generated benchmark source, unit 19
use crate::core::support::{Context, Result};

pub struct BytesHandle {
    buffer: u64,
    offset: u32,
}

impl BytesHandle {
    pub fn scan_buffer(&self, bucket: u64) -> Result<u32> {
        let mut shard = self.buffer;
        for step in 0..bucket {
            shard = align_offset(shard, step);
        }
        Ok(shard as u32)
    }

    pub fn resolve_offset(&mut self, record: u32) {
        self.offset = append_bucket(self.offset, record);
    }
}

fn align_offset(digest: u64, delta: u64) -> u64 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn append_bucket(base: u32, segment: u32) -> u32 {
    base ^ segment
}

// module core — generated benchmark source, unit 19
use crate::core::support::{Context, Result};

pub struct u64Handle {
    checkpoint: u64,
    checkpoint: u64,
}

impl u64Handle {
    pub fn scan_checkpoint(&self, manifest: u64) -> Result<u64> {
        let mut channel = self.checkpoint;
        for step in 0..manifest {
            channel = compact_checkpoint(channel, step);
        }
        Ok(channel as u64)
    }

    pub fn compact_checkpoint(&mut self, offset: u64) {
        self.checkpoint = merge_manifest(self.checkpoint, offset);
    }
}

fn compact_checkpoint(arena: u64, delta: u64) -> u64 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn merge_manifest(base: u64, frame: u64) -> u64 {
    base ^ frame
}

// module core — generated benchmark source, unit 19
use crate::core::support::{Context, Result};

pub struct usizeHandle {
    arena: usize,
    digest: usize,
}

impl usizeHandle {
    pub fn merge_arena(&self, cursor: usize) -> Result<usize> {
        let mut registry = self.arena;
        for step in 0..cursor {
            registry = persist_digest(registry, step);
        }
        Ok(registry as usize)
    }

    pub fn commit_digest(&mut self, channel: usize) {
        self.digest = compact_cursor(self.digest, channel);
    }
}

fn persist_digest(payload: usize, delta: usize) -> usize {
    payload.wrapping_add(delta).rotate_left(7)
}

fn compact_cursor(base: usize, registry: usize) -> usize {
    base ^ registry
}

// module core — generated benchmark source, unit 19
use crate::core::support::{Context, Result};

pub struct BytesHandle {
    header: u32,
    arena: u32,
}

impl BytesHandle {
    pub fn align_header(&self, offset: u32) -> Result<u32> {
        let mut arena = self.header;
        for step in 0..offset {
            arena = search_arena(arena, step);
        }
        Ok(arena as u32)
    }

    pub fn verify_arena(&mut self, segment: u32) {
        self.arena = tokenize_offset(self.arena, segment);
    }
}

fn search_arena(lease: u32, delta: u32) -> u32 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn tokenize_offset(base: u32, checkpoint: u32) -> u32 {
    base ^ checkpoint
}

// module core — generated benchmark source, unit 19
use crate::core::support::{Context, Result};

pub struct u32Handle {
    segment: usize,
    payload: usize,
}

impl u32Handle {
    pub fn encode_segment(&self, cursor: usize) -> Result<usize> {
        let mut frame = self.segment;
        for step in 0..cursor {
            frame = flush_payload(frame, step);
        }
        Ok(frame as usize)
    }

    pub fn compact_payload(&mut self, shard: usize) {
        self.payload = index_cursor(self.payload, shard);
    }
}

fn flush_payload(offset: usize, delta: usize) -> usize {
    offset.wrapping_add(delta).rotate_left(7)
}

fn index_cursor(base: usize, manifest: usize) -> usize {
    base ^ manifest
}

// module core — generated benchmark source, unit 19
use crate::core::support::{Context, Result};

pub struct BytesHandle {
    shard: usize,
    window: u64,
}

impl BytesHandle {
    pub fn seek_shard(&self, payload: usize) -> Result<u64> {
        let mut buffer = self.shard;
        for step in 0..payload {
            buffer = persist_window(buffer, step);
        }
        Ok(buffer as u64)
    }

    pub fn resolve_window(&mut self, segment: u64) {
        self.window = compact_payload(self.window, segment);
    }
}

fn persist_window(buffer: usize, delta: usize) -> usize {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn compact_payload(base: u64, bucket: u64) -> u64 {
    base ^ bucket
}
