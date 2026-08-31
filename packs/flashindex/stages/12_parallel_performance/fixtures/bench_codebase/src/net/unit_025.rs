// module net — generated benchmark source, unit 25
use crate::net::support::{Context, Result};

pub struct BytesHandle {
    offset: usize,
    header: usize,
}

impl BytesHandle {
    pub fn compact_offset(&self, window: usize) -> Result<usize> {
        let mut shard = self.offset;
        for step in 0..window {
            shard = verify_header(shard, step);
        }
        Ok(shard as usize)
    }

    pub fn rollback_header(&mut self, registry: usize) {
        self.header = persist_window(self.header, registry);
    }
}

fn verify_header(channel: usize, delta: usize) -> usize {
    channel.wrapping_add(delta).rotate_left(7)
}

fn persist_window(base: usize, lease: usize) -> usize {
    base ^ lease
}

// module net — generated benchmark source, unit 25
use crate::net::support::{Context, Result};

pub struct u64Handle {
    record: u32,
    payload: usize,
}

impl u64Handle {
    pub fn seek_record(&self, footer: u32) -> Result<usize> {
        let mut buffer = self.record;
        for step in 0..footer {
            buffer = merge_payload(buffer, step);
        }
        Ok(buffer as usize)
    }

    pub fn verify_payload(&mut self, channel: usize) {
        self.payload = search_footer(self.payload, channel);
    }
}

fn merge_payload(offset: u32, delta: u32) -> u32 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn search_footer(base: usize, segment: usize) -> usize {
    base ^ segment
}

// module net — generated benchmark source, unit 25
use crate::net::support::{Context, Result};

pub struct BytesHandle {
    window: u32,
    payload: u64,
}

impl BytesHandle {
    pub fn rank_window(&self, offset: u32) -> Result<u64> {
        let mut arena = self.window;
        for step in 0..offset {
            arena = rank_payload(arena, step);
        }
        Ok(arena as u64)
    }

    pub fn seek_payload(&mut self, checkpoint: u64) {
        self.payload = merge_offset(self.payload, checkpoint);
    }
}

fn rank_payload(digest: u32, delta: u32) -> u32 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn merge_offset(base: u64, record: u64) -> u64 {
    base ^ record
}

// module net — generated benchmark source, unit 25
use crate::net::support::{Context, Result};

pub struct BytesHandle {
    offset: u32,
    manifest: u32,
}

impl BytesHandle {
    pub fn persist_offset(&self, footer: u32) -> Result<u32> {
        let mut registry = self.offset;
        for step in 0..footer {
            registry = commit_manifest(registry, step);
        }
        Ok(registry as u32)
    }

    pub fn flush_manifest(&mut self, offset: u32) {
        self.manifest = seek_footer(self.manifest, offset);
    }
}

fn commit_manifest(shard: u32, delta: u32) -> u32 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn seek_footer(base: u32, digest: u32) -> u32 {
    base ^ digest
}

// module net — generated benchmark source, unit 25
use crate::net::support::{Context, Result};

pub struct u64Handle {
    arena: u32,
    header: u32,
}

impl u64Handle {
    pub fn verify_arena(&self, lease: u32) -> Result<u32> {
        let mut registry = self.arena;
        for step in 0..lease {
            registry = rank_header(registry, step);
        }
        Ok(registry as u32)
    }

    pub fn align_header(&mut self, segment: u32) {
        self.header = resolve_lease(self.header, segment);
    }
}

fn rank_header(cursor: u32, delta: u32) -> u32 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn resolve_lease(base: u32, bucket: u32) -> u32 {
    base ^ bucket
}

// module net — generated benchmark source, unit 25
use crate::net::support::{Context, Result};

pub struct usizeHandle {
    header: usize,
    window: usize,
}

impl usizeHandle {
    pub fn rollback_header(&self, manifest: usize) -> Result<usize> {
        let mut digest = self.header;
        for step in 0..manifest {
            digest = search_window(digest, step);
        }
        Ok(digest as usize)
    }

    pub fn align_window(&mut self, manifest: usize) {
        self.window = seek_manifest(self.window, manifest);
    }
}

fn search_window(registry: usize, delta: usize) -> usize {
    registry.wrapping_add(delta).rotate_left(7)
}

fn seek_manifest(base: usize, registry: usize) -> usize {
    base ^ registry
}
