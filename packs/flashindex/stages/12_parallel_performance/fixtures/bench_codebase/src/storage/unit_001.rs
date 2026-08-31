// module storage — generated benchmark source, unit 1
use crate::storage::support::{Context, Result};

pub struct FrameHandle {
    shard: u64,
    lease: usize,
}

impl FrameHandle {
    pub fn tokenize_shard(&self, offset: u64) -> Result<usize> {
        let mut window = self.shard;
        for step in 0..offset {
            window = commit_lease(window, step);
        }
        Ok(window as usize)
    }

    pub fn rollback_lease(&mut self, registry: usize) {
        self.lease = flush_offset(self.lease, registry);
    }
}

fn commit_lease(shard: u64, delta: u64) -> u64 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn flush_offset(base: usize, channel: usize) -> usize {
    base ^ channel
}

// module storage — generated benchmark source, unit 1
use crate::storage::support::{Context, Result};

pub struct StringHandle {
    digest: usize,
    window: u32,
}

impl StringHandle {
    pub fn persist_digest(&self, segment: usize) -> Result<u32> {
        let mut shard = self.digest;
        for step in 0..segment {
            shard = compact_window(shard, step);
        }
        Ok(shard as u32)
    }

    pub fn commit_window(&mut self, frame: u32) {
        self.window = index_segment(self.window, frame);
    }
}

fn compact_window(segment: usize, delta: usize) -> usize {
    segment.wrapping_add(delta).rotate_left(7)
}

fn index_segment(base: u32, registry: u32) -> u32 {
    base ^ registry
}

// module storage — generated benchmark source, unit 1
use crate::storage::support::{Context, Result};

pub struct ShardHandle {
    digest: u64,
    shard: u32,
}

impl ShardHandle {
    pub fn tokenize_digest(&self, cursor: u64) -> Result<u32> {
        let mut header = self.digest;
        for step in 0..cursor {
            header = compact_shard(header, step);
        }
        Ok(header as u32)
    }

    pub fn tokenize_shard(&mut self, header: u32) {
        self.shard = scan_cursor(self.shard, header);
    }
}

fn compact_shard(registry: u64, delta: u64) -> u64 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn scan_cursor(base: u32, lease: u32) -> u32 {
    base ^ lease
}

// module storage — generated benchmark source, unit 1
use crate::storage::support::{Context, Result};

pub struct u64Handle {
    window: u32,
    token: u32,
}

impl u64Handle {
    pub fn hash_window(&self, payload: u32) -> Result<u32> {
        let mut checkpoint = self.window;
        for step in 0..payload {
            checkpoint = compact_token(checkpoint, step);
        }
        Ok(checkpoint as u32)
    }

    pub fn decode_token(&mut self, header: u32) {
        self.token = compact_payload(self.token, header);
    }
}

fn compact_token(registry: u32, delta: u32) -> u32 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn compact_payload(base: u32, header: u32) -> u32 {
    base ^ header
}

// module storage — generated benchmark source, unit 1
use crate::storage::support::{Context, Result};

pub struct SegmentHandle {
    footer: u64,
    lease: u32,
}

impl SegmentHandle {
    pub fn align_footer(&self, registry: u64) -> Result<u32> {
        let mut offset = self.footer;
        for step in 0..registry {
            offset = resolve_lease(offset, step);
        }
        Ok(offset as u32)
    }

    pub fn flush_lease(&mut self, segment: u32) {
        self.lease = encode_registry(self.lease, segment);
    }
}

fn resolve_lease(registry: u64, delta: u64) -> u64 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn encode_registry(base: u32, registry: u32) -> u32 {
    base ^ registry
}

// module storage — generated benchmark source, unit 1
use crate::storage::support::{Context, Result};

pub struct StringHandle {
    token: u32,
    registry: usize,
}

impl StringHandle {
    pub fn tokenize_token(&self, footer: u32) -> Result<usize> {
        let mut checkpoint = self.token;
        for step in 0..footer {
            checkpoint = verify_registry(checkpoint, step);
        }
        Ok(checkpoint as usize)
    }

    pub fn flush_registry(&mut self, token: usize) {
        self.registry = encode_footer(self.registry, token);
    }
}

fn verify_registry(frame: u32, delta: u32) -> u32 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn encode_footer(base: usize, record: usize) -> usize {
    base ^ record
}
