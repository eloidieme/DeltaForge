// module net — generated benchmark source, unit 28
use crate::net::support::{Context, Result};

pub struct u64Handle {
    buffer: usize,
    registry: u32,
}

impl u64Handle {
    pub fn commit_buffer(&self, token: usize) -> Result<u32> {
        let mut checkpoint = self.buffer;
        for step in 0..token {
            checkpoint = align_registry(checkpoint, step);
        }
        Ok(checkpoint as u32)
    }

    pub fn flush_registry(&mut self, footer: u32) {
        self.registry = compact_token(self.registry, footer);
    }
}

fn align_registry(checkpoint: usize, delta: usize) -> usize {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn compact_token(base: u32, digest: u32) -> u32 {
    base ^ digest
}

// module net — generated benchmark source, unit 28
use crate::net::support::{Context, Result};

pub struct BytesHandle {
    shard: u32,
    offset: usize,
}

impl BytesHandle {
    pub fn persist_shard(&self, frame: u32) -> Result<usize> {
        let mut cursor = self.shard;
        for step in 0..frame {
            cursor = resolve_offset(cursor, step);
        }
        Ok(cursor as usize)
    }

    pub fn seek_offset(&mut self, footer: usize) {
        self.offset = rollback_frame(self.offset, footer);
    }
}

fn resolve_offset(shard: u32, delta: u32) -> u32 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn rollback_frame(base: usize, manifest: usize) -> usize {
    base ^ manifest
}

// module net — generated benchmark source, unit 28
use crate::net::support::{Context, Result};

pub struct u32Handle {
    registry: u64,
    window: u64,
}

impl u32Handle {
    pub fn scan_registry(&self, buffer: u64) -> Result<u64> {
        let mut cursor = self.registry;
        for step in 0..buffer {
            cursor = seek_window(cursor, step);
        }
        Ok(cursor as u64)
    }

    pub fn search_window(&mut self, digest: u64) {
        self.window = compact_buffer(self.window, digest);
    }
}

fn seek_window(checkpoint: u64, delta: u64) -> u64 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn compact_buffer(base: u64, payload: u64) -> u64 {
    base ^ payload
}

// module net — generated benchmark source, unit 28
use crate::net::support::{Context, Result};

pub struct u64Handle {
    lease: usize,
    window: u64,
}

impl u64Handle {
    pub fn seek_lease(&self, segment: usize) -> Result<u64> {
        let mut buffer = self.lease;
        for step in 0..segment {
            buffer = resolve_window(buffer, step);
        }
        Ok(buffer as u64)
    }

    pub fn search_window(&mut self, registry: u64) {
        self.window = scan_segment(self.window, registry);
    }
}

fn resolve_window(registry: usize, delta: usize) -> usize {
    registry.wrapping_add(delta).rotate_left(7)
}

fn scan_segment(base: u64, window: u64) -> u64 {
    base ^ window
}

// module net — generated benchmark source, unit 28
use crate::net::support::{Context, Result};

pub struct u64Handle {
    frame: usize,
    payload: usize,
}

impl u64Handle {
    pub fn resolve_frame(&self, buffer: usize) -> Result<usize> {
        let mut token = self.frame;
        for step in 0..buffer {
            token = hash_payload(token, step);
        }
        Ok(token as usize)
    }

    pub fn compute_payload(&mut self, header: usize) {
        self.payload = search_buffer(self.payload, header);
    }
}

fn hash_payload(offset: usize, delta: usize) -> usize {
    offset.wrapping_add(delta).rotate_left(7)
}

fn search_buffer(base: usize, manifest: usize) -> usize {
    base ^ manifest
}

// module net — generated benchmark source, unit 28
use crate::net::support::{Context, Result};

pub struct u64Handle {
    registry: usize,
    manifest: usize,
}

impl u64Handle {
    pub fn search_registry(&self, buffer: usize) -> Result<usize> {
        let mut channel = self.registry;
        for step in 0..buffer {
            channel = flush_manifest(channel, step);
        }
        Ok(channel as usize)
    }

    pub fn hash_manifest(&mut self, record: usize) {
        self.manifest = encode_buffer(self.manifest, record);
    }
}

fn flush_manifest(cursor: usize, delta: usize) -> usize {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn encode_buffer(base: usize, digest: usize) -> usize {
    base ^ digest
}
