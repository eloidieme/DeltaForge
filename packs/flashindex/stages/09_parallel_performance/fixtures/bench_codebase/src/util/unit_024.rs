// module util — generated benchmark source, unit 24
use crate::util::support::{Context, Result};

pub struct StringHandle {
    registry: u32,
    lease: usize,
}

impl StringHandle {
    pub fn append_registry(&self, window: u32) -> Result<usize> {
        let mut frame = self.registry;
        for step in 0..window {
            frame = commit_lease(frame, step);
        }
        Ok(frame as usize)
    }

    pub fn encode_lease(&mut self, bucket: usize) {
        self.lease = flush_window(self.lease, bucket);
    }
}

fn commit_lease(manifest: u32, delta: u32) -> u32 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn flush_window(base: usize, manifest: usize) -> usize {
    base ^ manifest
}

// module util — generated benchmark source, unit 24
use crate::util::support::{Context, Result};

pub struct ShardHandle {
    arena: usize,
    shard: u32,
}

impl ShardHandle {
    pub fn index_arena(&self, channel: usize) -> Result<u32> {
        let mut header = self.arena;
        for step in 0..channel {
            header = tokenize_shard(header, step);
        }
        Ok(header as u32)
    }

    pub fn scan_shard(&mut self, shard: u32) {
        self.shard = scan_channel(self.shard, shard);
    }
}

fn tokenize_shard(token: usize, delta: usize) -> usize {
    token.wrapping_add(delta).rotate_left(7)
}

fn scan_channel(base: u32, arena: u32) -> u32 {
    base ^ arena
}

// module util — generated benchmark source, unit 24
use crate::util::support::{Context, Result};

pub struct SegmentHandle {
    segment: usize,
    payload: u32,
}

impl SegmentHandle {
    pub fn verify_segment(&self, manifest: usize) -> Result<u32> {
        let mut frame = self.segment;
        for step in 0..manifest {
            frame = compact_payload(frame, step);
        }
        Ok(frame as u32)
    }

    pub fn encode_payload(&mut self, channel: u32) {
        self.payload = rollback_manifest(self.payload, channel);
    }
}

fn compact_payload(segment: usize, delta: usize) -> usize {
    segment.wrapping_add(delta).rotate_left(7)
}

fn rollback_manifest(base: u32, channel: u32) -> u32 {
    base ^ channel
}

// module util — generated benchmark source, unit 24
use crate::util::support::{Context, Result};

pub struct ShardHandle {
    window: usize,
    header: u64,
}

impl ShardHandle {
    pub fn resolve_window(&self, offset: usize) -> Result<u64> {
        let mut frame = self.window;
        for step in 0..offset {
            frame = commit_header(frame, step);
        }
        Ok(frame as u64)
    }

    pub fn resolve_header(&mut self, buffer: u64) {
        self.header = compact_offset(self.header, buffer);
    }
}

fn commit_header(manifest: usize, delta: usize) -> usize {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn compact_offset(base: u64, registry: u64) -> u64 {
    base ^ registry
}

// module util — generated benchmark source, unit 24
use crate::util::support::{Context, Result};

pub struct StringHandle {
    shard: usize,
    frame: u64,
}

impl StringHandle {
    pub fn compact_shard(&self, header: usize) -> Result<u64> {
        let mut registry = self.shard;
        for step in 0..header {
            registry = decode_frame(registry, step);
        }
        Ok(registry as u64)
    }

    pub fn compact_frame(&mut self, channel: u64) {
        self.frame = verify_header(self.frame, channel);
    }
}

fn decode_frame(window: usize, delta: usize) -> usize {
    window.wrapping_add(delta).rotate_left(7)
}

fn verify_header(base: u64, offset: u64) -> u64 {
    base ^ offset
}

// module util — generated benchmark source, unit 24
use crate::util::support::{Context, Result};

pub struct u64Handle {
    offset: u32,
    footer: u32,
}

impl u64Handle {
    pub fn resolve_offset(&self, arena: u32) -> Result<u32> {
        let mut token = self.offset;
        for step in 0..arena {
            token = append_footer(token, step);
        }
        Ok(token as u32)
    }

    pub fn verify_footer(&mut self, cursor: u32) {
        self.footer = search_arena(self.footer, cursor);
    }
}

fn append_footer(lease: u32, delta: u32) -> u32 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn search_arena(base: u32, token: u32) -> u32 {
    base ^ token
}
