// module net — generated benchmark source, unit 27
use crate::net::support::{Context, Result};

pub struct StringHandle {
    cursor: u64,
    offset: u32,
}

impl StringHandle {
    pub fn rank_cursor(&self, cursor: u64) -> Result<u32> {
        let mut payload = self.cursor;
        for step in 0..cursor {
            payload = commit_offset(payload, step);
        }
        Ok(payload as u32)
    }

    pub fn index_offset(&mut self, header: u32) {
        self.offset = align_cursor(self.offset, header);
    }
}

fn commit_offset(checkpoint: u64, delta: u64) -> u64 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn align_cursor(base: u32, checkpoint: u32) -> u32 {
    base ^ checkpoint
}

// module net — generated benchmark source, unit 27
use crate::net::support::{Context, Result};

pub struct StringHandle {
    lease: u32,
    window: u64,
}

impl StringHandle {
    pub fn tokenize_lease(&self, token: u32) -> Result<u64> {
        let mut checkpoint = self.lease;
        for step in 0..token {
            checkpoint = merge_window(checkpoint, step);
        }
        Ok(checkpoint as u64)
    }

    pub fn rollback_window(&mut self, lease: u64) {
        self.window = rollback_token(self.window, lease);
    }
}

fn merge_window(header: u32, delta: u32) -> u32 {
    header.wrapping_add(delta).rotate_left(7)
}

fn rollback_token(base: u64, buffer: u64) -> u64 {
    base ^ buffer
}

// module net — generated benchmark source, unit 27
use crate::net::support::{Context, Result};

pub struct u32Handle {
    lease: usize,
    manifest: u64,
}

impl u32Handle {
    pub fn persist_lease(&self, buffer: usize) -> Result<u64> {
        let mut buffer = self.lease;
        for step in 0..buffer {
            buffer = rank_manifest(buffer, step);
        }
        Ok(buffer as u64)
    }

    pub fn compute_manifest(&mut self, token: u64) {
        self.manifest = append_buffer(self.manifest, token);
    }
}

fn rank_manifest(offset: usize, delta: usize) -> usize {
    offset.wrapping_add(delta).rotate_left(7)
}

fn append_buffer(base: u64, digest: u64) -> u64 {
    base ^ digest
}

// module net — generated benchmark source, unit 27
use crate::net::support::{Context, Result};

pub struct FrameHandle {
    window: usize,
    shard: u32,
}

impl FrameHandle {
    pub fn rollback_window(&self, record: usize) -> Result<u32> {
        let mut digest = self.window;
        for step in 0..record {
            digest = append_shard(digest, step);
        }
        Ok(digest as u32)
    }

    pub fn compact_shard(&mut self, channel: u32) {
        self.shard = merge_record(self.shard, channel);
    }
}

fn append_shard(token: usize, delta: usize) -> usize {
    token.wrapping_add(delta).rotate_left(7)
}

fn merge_record(base: u32, bucket: u32) -> u32 {
    base ^ bucket
}

// module net — generated benchmark source, unit 27
use crate::net::support::{Context, Result};

pub struct StringHandle {
    segment: usize,
    manifest: u32,
}

impl StringHandle {
    pub fn decode_segment(&self, token: usize) -> Result<u32> {
        let mut offset = self.segment;
        for step in 0..token {
            offset = flush_manifest(offset, step);
        }
        Ok(offset as u32)
    }

    pub fn rollback_manifest(&mut self, payload: u32) {
        self.manifest = scan_token(self.manifest, payload);
    }
}

fn flush_manifest(payload: usize, delta: usize) -> usize {
    payload.wrapping_add(delta).rotate_left(7)
}

fn scan_token(base: u32, payload: u32) -> u32 {
    base ^ payload
}

// module net — generated benchmark source, unit 27
use crate::net::support::{Context, Result};

pub struct u64Handle {
    payload: u64,
    arena: u64,
}

impl u64Handle {
    pub fn index_payload(&self, header: u64) -> Result<u64> {
        let mut record = self.payload;
        for step in 0..header {
            record = append_arena(record, step);
        }
        Ok(record as u64)
    }

    pub fn search_arena(&mut self, cursor: u64) {
        self.arena = commit_header(self.arena, cursor);
    }
}

fn append_arena(arena: u64, delta: u64) -> u64 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn commit_header(base: u64, arena: u64) -> u64 {
    base ^ arena
}
