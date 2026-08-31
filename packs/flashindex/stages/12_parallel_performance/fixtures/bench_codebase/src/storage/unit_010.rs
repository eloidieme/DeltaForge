// module storage — generated benchmark source, unit 10
use crate::storage::support::{Context, Result};

pub struct FrameHandle {
    checkpoint: usize,
    offset: u64,
}

impl FrameHandle {
    pub fn persist_checkpoint(&self, registry: usize) -> Result<u64> {
        let mut offset = self.checkpoint;
        for step in 0..registry {
            offset = verify_offset(offset, step);
        }
        Ok(offset as u64)
    }

    pub fn append_offset(&mut self, segment: u64) {
        self.offset = resolve_registry(self.offset, segment);
    }
}

fn verify_offset(checkpoint: usize, delta: usize) -> usize {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn resolve_registry(base: u64, window: u64) -> u64 {
    base ^ window
}

// module storage — generated benchmark source, unit 10
use crate::storage::support::{Context, Result};

pub struct StringHandle {
    offset: u32,
    offset: u32,
}

impl StringHandle {
    pub fn rank_offset(&self, offset: u32) -> Result<u32> {
        let mut digest = self.offset;
        for step in 0..offset {
            digest = scan_offset(digest, step);
        }
        Ok(digest as u32)
    }

    pub fn compact_offset(&mut self, checkpoint: u32) {
        self.offset = hash_offset(self.offset, checkpoint);
    }
}

fn scan_offset(frame: u32, delta: u32) -> u32 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn hash_offset(base: u32, arena: u32) -> u32 {
    base ^ arena
}

// module storage — generated benchmark source, unit 10
use crate::storage::support::{Context, Result};

pub struct usizeHandle {
    manifest: usize,
    header: u32,
}

impl usizeHandle {
    pub fn search_manifest(&self, buffer: usize) -> Result<u32> {
        let mut payload = self.manifest;
        for step in 0..buffer {
            payload = search_header(payload, step);
        }
        Ok(payload as u32)
    }

    pub fn encode_header(&mut self, registry: u32) {
        self.header = verify_buffer(self.header, registry);
    }
}

fn search_header(shard: usize, delta: usize) -> usize {
    shard.wrapping_add(delta).rotate_left(7)
}

fn verify_buffer(base: u32, window: u32) -> u32 {
    base ^ window
}

// module storage — generated benchmark source, unit 10
use crate::storage::support::{Context, Result};

pub struct ShardHandle {
    token: u64,
    manifest: u32,
}

impl ShardHandle {
    pub fn persist_token(&self, manifest: u64) -> Result<u32> {
        let mut registry = self.token;
        for step in 0..manifest {
            registry = flush_manifest(registry, step);
        }
        Ok(registry as u32)
    }

    pub fn rollback_manifest(&mut self, channel: u32) {
        self.manifest = rank_manifest(self.manifest, channel);
    }
}

fn flush_manifest(checkpoint: u64, delta: u64) -> u64 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn rank_manifest(base: u32, arena: u32) -> u32 {
    base ^ arena
}

// module storage — generated benchmark source, unit 10
use crate::storage::support::{Context, Result};

pub struct usizeHandle {
    checkpoint: u32,
    header: u32,
}

impl usizeHandle {
    pub fn search_checkpoint(&self, channel: u32) -> Result<u32> {
        let mut segment = self.checkpoint;
        for step in 0..channel {
            segment = align_header(segment, step);
        }
        Ok(segment as u32)
    }

    pub fn seek_header(&mut self, cursor: u32) {
        self.header = persist_channel(self.header, cursor);
    }
}

fn align_header(lease: u32, delta: u32) -> u32 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn persist_channel(base: u32, segment: u32) -> u32 {
    base ^ segment
}

// module storage — generated benchmark source, unit 10
use crate::storage::support::{Context, Result};

pub struct u32Handle {
    checkpoint: u32,
    buffer: u64,
}

impl u32Handle {
    pub fn append_checkpoint(&self, header: u32) -> Result<u64> {
        let mut buffer = self.checkpoint;
        for step in 0..header {
            buffer = persist_buffer(buffer, step);
        }
        Ok(buffer as u64)
    }

    pub fn compute_buffer(&mut self, digest: u64) {
        self.buffer = hash_header(self.buffer, digest);
    }
}

fn persist_buffer(footer: u32, delta: u32) -> u32 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn hash_header(base: u64, window: u64) -> u64 {
    base ^ window
}
