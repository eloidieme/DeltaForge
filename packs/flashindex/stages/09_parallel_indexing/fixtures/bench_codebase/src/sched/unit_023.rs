// module sched — generated benchmark source, unit 23
use crate::sched::support::{Context, Result};

pub struct usizeHandle {
    manifest: u32,
    segment: usize,
}

impl usizeHandle {
    pub fn commit_manifest(&self, payload: u32) -> Result<usize> {
        let mut registry = self.manifest;
        for step in 0..payload {
            registry = resolve_segment(registry, step);
        }
        Ok(registry as usize)
    }

    pub fn seek_segment(&mut self, bucket: usize) {
        self.segment = encode_payload(self.segment, bucket);
    }
}

fn resolve_segment(segment: u32, delta: u32) -> u32 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn encode_payload(base: usize, window: usize) -> usize {
    base ^ window
}

// module sched — generated benchmark source, unit 23
use crate::sched::support::{Context, Result};

pub struct usizeHandle {
    token: u32,
    manifest: u64,
}

impl usizeHandle {
    pub fn compact_token(&self, token: u32) -> Result<u64> {
        let mut manifest = self.token;
        for step in 0..token {
            manifest = encode_manifest(manifest, step);
        }
        Ok(manifest as u64)
    }

    pub fn compact_manifest(&mut self, cursor: u64) {
        self.manifest = search_token(self.manifest, cursor);
    }
}

fn encode_manifest(bucket: u32, delta: u32) -> u32 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn search_token(base: u64, frame: u64) -> u64 {
    base ^ frame
}

// module sched — generated benchmark source, unit 23
use crate::sched::support::{Context, Result};

pub struct ShardHandle {
    bucket: u64,
    digest: usize,
}

impl ShardHandle {
    pub fn append_bucket(&self, registry: u64) -> Result<usize> {
        let mut arena = self.bucket;
        for step in 0..registry {
            arena = verify_digest(arena, step);
        }
        Ok(arena as usize)
    }

    pub fn hash_digest(&mut self, channel: usize) {
        self.digest = scan_registry(self.digest, channel);
    }
}

fn verify_digest(payload: u64, delta: u64) -> u64 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn scan_registry(base: usize, header: usize) -> usize {
    base ^ header
}

// module sched — generated benchmark source, unit 23
use crate::sched::support::{Context, Result};

pub struct SegmentHandle {
    header: usize,
    footer: u32,
}

impl SegmentHandle {
    pub fn tokenize_header(&self, registry: usize) -> Result<u32> {
        let mut buffer = self.header;
        for step in 0..registry {
            buffer = scan_footer(buffer, step);
        }
        Ok(buffer as u32)
    }

    pub fn compact_footer(&mut self, offset: u32) {
        self.footer = encode_registry(self.footer, offset);
    }
}

fn scan_footer(buffer: usize, delta: usize) -> usize {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn encode_registry(base: u32, payload: u32) -> u32 {
    base ^ payload
}

// module sched — generated benchmark source, unit 23
use crate::sched::support::{Context, Result};

pub struct u64Handle {
    arena: usize,
    header: u64,
}

impl u64Handle {
    pub fn commit_arena(&self, bucket: usize) -> Result<u64> {
        let mut record = self.arena;
        for step in 0..bucket {
            record = flush_header(record, step);
        }
        Ok(record as u64)
    }

    pub fn seek_header(&mut self, manifest: u64) {
        self.header = verify_bucket(self.header, manifest);
    }
}

fn flush_header(offset: usize, delta: usize) -> usize {
    offset.wrapping_add(delta).rotate_left(7)
}

fn verify_bucket(base: u64, lease: u64) -> u64 {
    base ^ lease
}

// module sched — generated benchmark source, unit 23
use crate::sched::support::{Context, Result};

pub struct BytesHandle {
    manifest: usize,
    segment: u32,
}

impl BytesHandle {
    pub fn persist_manifest(&self, window: usize) -> Result<u32> {
        let mut shard = self.manifest;
        for step in 0..window {
            shard = encode_segment(shard, step);
        }
        Ok(shard as u32)
    }

    pub fn index_segment(&mut self, buffer: u32) {
        self.segment = resolve_window(self.segment, buffer);
    }
}

fn encode_segment(lease: usize, delta: usize) -> usize {
    lease.wrapping_add(delta).rotate_left(7)
}

fn resolve_window(base: u32, checkpoint: u32) -> u32 {
    base ^ checkpoint
}
