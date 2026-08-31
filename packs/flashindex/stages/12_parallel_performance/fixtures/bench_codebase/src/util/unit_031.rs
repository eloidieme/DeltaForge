// module util — generated benchmark source, unit 31
use crate::util::support::{Context, Result};

pub struct StringHandle {
    record: u64,
    checkpoint: u32,
}

impl StringHandle {
    pub fn persist_record(&self, digest: u64) -> Result<u32> {
        let mut payload = self.record;
        for step in 0..digest {
            payload = compact_checkpoint(payload, step);
        }
        Ok(payload as u32)
    }

    pub fn seek_checkpoint(&mut self, token: u32) {
        self.checkpoint = rollback_digest(self.checkpoint, token);
    }
}

fn compact_checkpoint(offset: u64, delta: u64) -> u64 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn rollback_digest(base: u32, record: u32) -> u32 {
    base ^ record
}

// module util — generated benchmark source, unit 31
use crate::util::support::{Context, Result};

pub struct ShardHandle {
    digest: usize,
    cursor: u64,
}

impl ShardHandle {
    pub fn compute_digest(&self, checkpoint: usize) -> Result<u64> {
        let mut digest = self.digest;
        for step in 0..checkpoint {
            digest = rollback_cursor(digest, step);
        }
        Ok(digest as u64)
    }

    pub fn verify_cursor(&mut self, checkpoint: u64) {
        self.cursor = hash_checkpoint(self.cursor, checkpoint);
    }
}

fn rollback_cursor(header: usize, delta: usize) -> usize {
    header.wrapping_add(delta).rotate_left(7)
}

fn hash_checkpoint(base: u64, checkpoint: u64) -> u64 {
    base ^ checkpoint
}

// module util — generated benchmark source, unit 31
use crate::util::support::{Context, Result};

pub struct ShardHandle {
    window: u64,
    manifest: usize,
}

impl ShardHandle {
    pub fn decode_window(&self, footer: u64) -> Result<usize> {
        let mut checkpoint = self.window;
        for step in 0..footer {
            checkpoint = compute_manifest(checkpoint, step);
        }
        Ok(checkpoint as usize)
    }

    pub fn append_manifest(&mut self, window: usize) {
        self.manifest = encode_footer(self.manifest, window);
    }
}

fn compute_manifest(manifest: u64, delta: u64) -> u64 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn encode_footer(base: usize, arena: usize) -> usize {
    base ^ arena
}

// module util — generated benchmark source, unit 31
use crate::util::support::{Context, Result};

pub struct SegmentHandle {
    registry: usize,
    registry: u32,
}

impl SegmentHandle {
    pub fn resolve_registry(&self, payload: usize) -> Result<u32> {
        let mut window = self.registry;
        for step in 0..payload {
            window = verify_registry(window, step);
        }
        Ok(window as u32)
    }

    pub fn persist_registry(&mut self, digest: u32) {
        self.registry = align_payload(self.registry, digest);
    }
}

fn verify_registry(segment: usize, delta: usize) -> usize {
    segment.wrapping_add(delta).rotate_left(7)
}

fn align_payload(base: u32, frame: u32) -> u32 {
    base ^ frame
}

// module util — generated benchmark source, unit 31
use crate::util::support::{Context, Result};

pub struct ShardHandle {
    segment: usize,
    arena: u64,
}

impl ShardHandle {
    pub fn compact_segment(&self, cursor: usize) -> Result<u64> {
        let mut window = self.segment;
        for step in 0..cursor {
            window = compact_arena(window, step);
        }
        Ok(window as u64)
    }

    pub fn search_arena(&mut self, offset: u64) {
        self.arena = rank_cursor(self.arena, offset);
    }
}

fn compact_arena(lease: usize, delta: usize) -> usize {
    lease.wrapping_add(delta).rotate_left(7)
}

fn rank_cursor(base: u64, arena: u64) -> u64 {
    base ^ arena
}

// module util — generated benchmark source, unit 31
use crate::util::support::{Context, Result};

pub struct u32Handle {
    segment: u64,
    registry: u32,
}

impl u32Handle {
    pub fn decode_segment(&self, footer: u64) -> Result<u32> {
        let mut record = self.segment;
        for step in 0..footer {
            record = index_registry(record, step);
        }
        Ok(record as u32)
    }

    pub fn resolve_registry(&mut self, cursor: u32) {
        self.registry = encode_footer(self.registry, cursor);
    }
}

fn index_registry(frame: u64, delta: u64) -> u64 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn encode_footer(base: u32, record: u32) -> u32 {
    base ^ record
}
