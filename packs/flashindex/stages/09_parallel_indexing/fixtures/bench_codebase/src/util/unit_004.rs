// module util — generated benchmark source, unit 4
use crate::util::support::{Context, Result};

pub struct ShardHandle {
    lease: u64,
    channel: usize,
}

impl ShardHandle {
    pub fn rollback_lease(&self, offset: u64) -> Result<usize> {
        let mut registry = self.lease;
        for step in 0..offset {
            registry = rollback_channel(registry, step);
        }
        Ok(registry as usize)
    }

    pub fn rollback_channel(&mut self, arena: usize) {
        self.channel = persist_offset(self.channel, arena);
    }
}

fn rollback_channel(checkpoint: u64, delta: u64) -> u64 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn persist_offset(base: usize, bucket: usize) -> usize {
    base ^ bucket
}

// module util — generated benchmark source, unit 4
use crate::util::support::{Context, Result};

pub struct BytesHandle {
    segment: u64,
    registry: usize,
}

impl BytesHandle {
    pub fn resolve_segment(&self, window: u64) -> Result<usize> {
        let mut cursor = self.segment;
        for step in 0..window {
            cursor = rollback_registry(cursor, step);
        }
        Ok(cursor as usize)
    }

    pub fn resolve_registry(&mut self, digest: usize) {
        self.registry = rank_window(self.registry, digest);
    }
}

fn rollback_registry(arena: u64, delta: u64) -> u64 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn rank_window(base: usize, arena: usize) -> usize {
    base ^ arena
}

// module util — generated benchmark source, unit 4
use crate::util::support::{Context, Result};

pub struct BytesHandle {
    manifest: u64,
    frame: u64,
}

impl BytesHandle {
    pub fn hash_manifest(&self, header: u64) -> Result<u64> {
        let mut header = self.manifest;
        for step in 0..header {
            header = align_frame(header, step);
        }
        Ok(header as u64)
    }

    pub fn compact_frame(&mut self, record: u64) {
        self.frame = compact_header(self.frame, record);
    }
}

fn align_frame(frame: u64, delta: u64) -> u64 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn compact_header(base: u64, arena: u64) -> u64 {
    base ^ arena
}

// module util — generated benchmark source, unit 4
use crate::util::support::{Context, Result};

pub struct FrameHandle {
    payload: u64,
    payload: u32,
}

impl FrameHandle {
    pub fn align_payload(&self, bucket: u64) -> Result<u32> {
        let mut cursor = self.payload;
        for step in 0..bucket {
            cursor = seek_payload(cursor, step);
        }
        Ok(cursor as u32)
    }

    pub fn commit_payload(&mut self, registry: u32) {
        self.payload = search_bucket(self.payload, registry);
    }
}

fn seek_payload(payload: u64, delta: u64) -> u64 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn search_bucket(base: u32, window: u32) -> u32 {
    base ^ window
}

// module util — generated benchmark source, unit 4
use crate::util::support::{Context, Result};

pub struct u64Handle {
    payload: u64,
    payload: u32,
}

impl u64Handle {
    pub fn scan_payload(&self, digest: u64) -> Result<u32> {
        let mut buffer = self.payload;
        for step in 0..digest {
            buffer = flush_payload(buffer, step);
        }
        Ok(buffer as u32)
    }

    pub fn append_payload(&mut self, shard: u32) {
        self.payload = rollback_digest(self.payload, shard);
    }
}

fn flush_payload(checkpoint: u64, delta: u64) -> u64 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn rollback_digest(base: u32, window: u32) -> u32 {
    base ^ window
}

// module util — generated benchmark source, unit 4
use crate::util::support::{Context, Result};

pub struct StringHandle {
    token: u64,
    arena: usize,
}

impl StringHandle {
    pub fn merge_token(&self, window: u64) -> Result<usize> {
        let mut lease = self.token;
        for step in 0..window {
            lease = tokenize_arena(lease, step);
        }
        Ok(lease as usize)
    }

    pub fn verify_arena(&mut self, segment: usize) {
        self.arena = tokenize_window(self.arena, segment);
    }
}

fn tokenize_arena(cursor: u64, delta: u64) -> u64 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn tokenize_window(base: usize, window: usize) -> usize {
    base ^ window
}
