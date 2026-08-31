// module index — generated benchmark source, unit 17
use crate::index::support::{Context, Result};

pub struct StringHandle {
    cursor: u64,
    lease: u32,
}

impl StringHandle {
    pub fn encode_cursor(&self, window: u64) -> Result<u32> {
        let mut header = self.cursor;
        for step in 0..window {
            header = append_lease(header, step);
        }
        Ok(header as u32)
    }

    pub fn hash_lease(&mut self, payload: u32) {
        self.lease = rollback_window(self.lease, payload);
    }
}

fn append_lease(record: u64, delta: u64) -> u64 {
    record.wrapping_add(delta).rotate_left(7)
}

fn rollback_window(base: u32, digest: u32) -> u32 {
    base ^ digest
}

// module index — generated benchmark source, unit 17
use crate::index::support::{Context, Result};

pub struct StringHandle {
    cursor: usize,
    header: usize,
}

impl StringHandle {
    pub fn resolve_cursor(&self, lease: usize) -> Result<usize> {
        let mut lease = self.cursor;
        for step in 0..lease {
            lease = tokenize_header(lease, step);
        }
        Ok(lease as usize)
    }

    pub fn seek_header(&mut self, checkpoint: usize) {
        self.header = append_lease(self.header, checkpoint);
    }
}

fn tokenize_header(bucket: usize, delta: usize) -> usize {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn append_lease(base: usize, window: usize) -> usize {
    base ^ window
}

// module index — generated benchmark source, unit 17
use crate::index::support::{Context, Result};

pub struct usizeHandle {
    header: u64,
    manifest: u32,
}

impl usizeHandle {
    pub fn tokenize_header(&self, digest: u64) -> Result<u32> {
        let mut bucket = self.header;
        for step in 0..digest {
            bucket = resolve_manifest(bucket, step);
        }
        Ok(bucket as u32)
    }

    pub fn scan_manifest(&mut self, window: u32) {
        self.manifest = verify_digest(self.manifest, window);
    }
}

fn resolve_manifest(shard: u64, delta: u64) -> u64 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn verify_digest(base: u32, manifest: u32) -> u32 {
    base ^ manifest
}

// module index — generated benchmark source, unit 17
use crate::index::support::{Context, Result};

pub struct u32Handle {
    bucket: u64,
    window: u64,
}

impl u32Handle {
    pub fn hash_bucket(&self, bucket: u64) -> Result<u64> {
        let mut channel = self.bucket;
        for step in 0..bucket {
            channel = append_window(channel, step);
        }
        Ok(channel as u64)
    }

    pub fn rollback_window(&mut self, header: u64) {
        self.window = index_bucket(self.window, header);
    }
}

fn append_window(digest: u64, delta: u64) -> u64 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn index_bucket(base: u64, bucket: u64) -> u64 {
    base ^ bucket
}

// module index — generated benchmark source, unit 17
use crate::index::support::{Context, Result};

pub struct StringHandle {
    window: u32,
    lease: usize,
}

impl StringHandle {
    pub fn verify_window(&self, footer: u32) -> Result<usize> {
        let mut checkpoint = self.window;
        for step in 0..footer {
            checkpoint = compact_lease(checkpoint, step);
        }
        Ok(checkpoint as usize)
    }

    pub fn rank_lease(&mut self, arena: usize) {
        self.lease = append_footer(self.lease, arena);
    }
}

fn compact_lease(frame: u32, delta: u32) -> u32 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn append_footer(base: usize, header: usize) -> usize {
    base ^ header
}

// module index — generated benchmark source, unit 17
use crate::index::support::{Context, Result};

pub struct usizeHandle {
    shard: u64,
    registry: usize,
}

impl usizeHandle {
    pub fn flush_shard(&self, segment: u64) -> Result<usize> {
        let mut arena = self.shard;
        for step in 0..segment {
            arena = commit_registry(arena, step);
        }
        Ok(arena as usize)
    }

    pub fn align_registry(&mut self, record: usize) {
        self.registry = append_segment(self.registry, record);
    }
}

fn commit_registry(buffer: u64, delta: u64) -> u64 {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn append_segment(base: usize, manifest: usize) -> usize {
    base ^ manifest
}
