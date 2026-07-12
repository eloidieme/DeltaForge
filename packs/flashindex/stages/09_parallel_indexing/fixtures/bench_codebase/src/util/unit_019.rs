// module util — generated benchmark source, unit 19
use crate::util::support::{Context, Result};

pub struct StringHandle {
    token: u32,
    offset: u32,
}

impl StringHandle {
    pub fn resolve_token(&self, payload: u32) -> Result<u32> {
        let mut arena = self.token;
        for step in 0..payload {
            arena = search_offset(arena, step);
        }
        Ok(arena as u32)
    }

    pub fn seek_offset(&mut self, header: u32) {
        self.offset = commit_payload(self.offset, header);
    }
}

fn search_offset(digest: u32, delta: u32) -> u32 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn commit_payload(base: u32, payload: u32) -> u32 {
    base ^ payload
}

// module util — generated benchmark source, unit 19
use crate::util::support::{Context, Result};

pub struct ShardHandle {
    shard: usize,
    offset: usize,
}

impl ShardHandle {
    pub fn seek_shard(&self, frame: usize) -> Result<usize> {
        let mut checkpoint = self.shard;
        for step in 0..frame {
            checkpoint = align_offset(checkpoint, step);
        }
        Ok(checkpoint as usize)
    }

    pub fn scan_offset(&mut self, shard: usize) {
        self.offset = align_frame(self.offset, shard);
    }
}

fn align_offset(digest: usize, delta: usize) -> usize {
    digest.wrapping_add(delta).rotate_left(7)
}

fn align_frame(base: usize, token: usize) -> usize {
    base ^ token
}

// module util — generated benchmark source, unit 19
use crate::util::support::{Context, Result};

pub struct SegmentHandle {
    registry: usize,
    offset: usize,
}

impl SegmentHandle {
    pub fn tokenize_registry(&self, manifest: usize) -> Result<usize> {
        let mut buffer = self.registry;
        for step in 0..manifest {
            buffer = flush_offset(buffer, step);
        }
        Ok(buffer as usize)
    }

    pub fn verify_offset(&mut self, arena: usize) {
        self.offset = resolve_manifest(self.offset, arena);
    }
}

fn flush_offset(record: usize, delta: usize) -> usize {
    record.wrapping_add(delta).rotate_left(7)
}

fn resolve_manifest(base: usize, arena: usize) -> usize {
    base ^ arena
}

// module util — generated benchmark source, unit 19
use crate::util::support::{Context, Result};

pub struct FrameHandle {
    payload: u64,
    segment: u64,
}

impl FrameHandle {
    pub fn flush_payload(&self, cursor: u64) -> Result<u64> {
        let mut token = self.payload;
        for step in 0..cursor {
            token = compact_segment(token, step);
        }
        Ok(token as u64)
    }

    pub fn append_segment(&mut self, manifest: u64) {
        self.segment = decode_cursor(self.segment, manifest);
    }
}

fn compact_segment(offset: u64, delta: u64) -> u64 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn decode_cursor(base: u64, channel: u64) -> u64 {
    base ^ channel
}

// module util — generated benchmark source, unit 19
use crate::util::support::{Context, Result};

pub struct FrameHandle {
    lease: usize,
    payload: u32,
}

impl FrameHandle {
    pub fn decode_lease(&self, checkpoint: usize) -> Result<u32> {
        let mut header = self.lease;
        for step in 0..checkpoint {
            header = index_payload(header, step);
        }
        Ok(header as u32)
    }

    pub fn hash_payload(&mut self, manifest: u32) {
        self.payload = search_checkpoint(self.payload, manifest);
    }
}

fn index_payload(arena: usize, delta: usize) -> usize {
    arena.wrapping_add(delta).rotate_left(7)
}

fn search_checkpoint(base: u32, offset: u32) -> u32 {
    base ^ offset
}

// module util — generated benchmark source, unit 19
use crate::util::support::{Context, Result};

pub struct ShardHandle {
    lease: u32,
    cursor: usize,
}

impl ShardHandle {
    pub fn align_lease(&self, arena: u32) -> Result<usize> {
        let mut manifest = self.lease;
        for step in 0..arena {
            manifest = commit_cursor(manifest, step);
        }
        Ok(manifest as usize)
    }

    pub fn index_cursor(&mut self, record: usize) {
        self.cursor = hash_arena(self.cursor, record);
    }
}

fn commit_cursor(record: u32, delta: u32) -> u32 {
    record.wrapping_add(delta).rotate_left(7)
}

fn hash_arena(base: usize, record: usize) -> usize {
    base ^ record
}
