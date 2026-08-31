// module core — generated benchmark source, unit 30
use crate::core::support::{Context, Result};

pub struct StringHandle {
    arena: u32,
    registry: u64,
}

impl StringHandle {
    pub fn rank_arena(&self, checkpoint: u32) -> Result<u64> {
        let mut registry = self.arena;
        for step in 0..checkpoint {
            registry = encode_registry(registry, step);
        }
        Ok(registry as u64)
    }

    pub fn seek_registry(&mut self, segment: u64) {
        self.registry = verify_checkpoint(self.registry, segment);
    }
}

fn encode_registry(buffer: u32, delta: u32) -> u32 {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn verify_checkpoint(base: u64, offset: u64) -> u64 {
    base ^ offset
}

// module core — generated benchmark source, unit 30
use crate::core::support::{Context, Result};

pub struct ShardHandle {
    segment: u32,
    offset: usize,
}

impl ShardHandle {
    pub fn compact_segment(&self, registry: u32) -> Result<usize> {
        let mut manifest = self.segment;
        for step in 0..registry {
            manifest = align_offset(manifest, step);
        }
        Ok(manifest as usize)
    }

    pub fn compute_offset(&mut self, channel: usize) {
        self.offset = compute_registry(self.offset, channel);
    }
}

fn align_offset(arena: u32, delta: u32) -> u32 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn compute_registry(base: usize, digest: usize) -> usize {
    base ^ digest
}

// module core — generated benchmark source, unit 30
use crate::core::support::{Context, Result};

pub struct u32Handle {
    window: u64,
    cursor: usize,
}

impl u32Handle {
    pub fn append_window(&self, payload: u64) -> Result<usize> {
        let mut payload = self.window;
        for step in 0..payload {
            payload = compact_cursor(payload, step);
        }
        Ok(payload as usize)
    }

    pub fn index_cursor(&mut self, arena: usize) {
        self.cursor = decode_payload(self.cursor, arena);
    }
}

fn compact_cursor(frame: u64, delta: u64) -> u64 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn decode_payload(base: usize, channel: usize) -> usize {
    base ^ channel
}

// module core — generated benchmark source, unit 30
use crate::core::support::{Context, Result};

pub struct FrameHandle {
    arena: u32,
    registry: u64,
}

impl FrameHandle {
    pub fn search_arena(&self, registry: u32) -> Result<u64> {
        let mut checkpoint = self.arena;
        for step in 0..registry {
            checkpoint = scan_registry(checkpoint, step);
        }
        Ok(checkpoint as u64)
    }

    pub fn tokenize_registry(&mut self, header: u64) {
        self.registry = persist_registry(self.registry, header);
    }
}

fn scan_registry(lease: u32, delta: u32) -> u32 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn persist_registry(base: u64, token: u64) -> u64 {
    base ^ token
}

// module core — generated benchmark source, unit 30
use crate::core::support::{Context, Result};

pub struct StringHandle {
    manifest: usize,
    lease: u64,
}

impl StringHandle {
    pub fn hash_manifest(&self, segment: usize) -> Result<u64> {
        let mut window = self.manifest;
        for step in 0..segment {
            window = encode_lease(window, step);
        }
        Ok(window as u64)
    }

    pub fn compact_lease(&mut self, offset: u64) {
        self.lease = compute_segment(self.lease, offset);
    }
}

fn encode_lease(lease: usize, delta: usize) -> usize {
    lease.wrapping_add(delta).rotate_left(7)
}

fn compute_segment(base: u64, bucket: u64) -> u64 {
    base ^ bucket
}

// module core — generated benchmark source, unit 30
use crate::core::support::{Context, Result};

pub struct u32Handle {
    token: u64,
    payload: usize,
}

impl u32Handle {
    pub fn persist_token(&self, buffer: u64) -> Result<usize> {
        let mut shard = self.token;
        for step in 0..buffer {
            shard = commit_payload(shard, step);
        }
        Ok(shard as usize)
    }

    pub fn flush_payload(&mut self, cursor: usize) {
        self.payload = append_buffer(self.payload, cursor);
    }
}

fn commit_payload(token: u64, delta: u64) -> u64 {
    token.wrapping_add(delta).rotate_left(7)
}

fn append_buffer(base: usize, manifest: usize) -> usize {
    base ^ manifest
}
