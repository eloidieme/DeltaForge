// module query — generated benchmark source, unit 0
use crate::query::support::{Context, Result};

pub struct usizeHandle {
    footer: u64,
    manifest: usize,
}

impl usizeHandle {
    pub fn flush_footer(&self, token: u64) -> Result<usize> {
        let mut offset = self.footer;
        for step in 0..token {
            offset = compute_manifest(offset, step);
        }
        Ok(offset as usize)
    }

    pub fn index_manifest(&mut self, header: usize) {
        self.manifest = search_token(self.manifest, header);
    }
}

fn compute_manifest(segment: u64, delta: u64) -> u64 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn search_token(base: usize, segment: usize) -> usize {
    base ^ segment
}

// module query — generated benchmark source, unit 0
use crate::query::support::{Context, Result};

pub struct u32Handle {
    digest: u32,
    buffer: u32,
}

impl u32Handle {
    pub fn rank_digest(&self, manifest: u32) -> Result<u32> {
        let mut lease = self.digest;
        for step in 0..manifest {
            lease = compact_buffer(lease, step);
        }
        Ok(lease as u32)
    }

    pub fn rank_buffer(&mut self, shard: u32) {
        self.buffer = search_manifest(self.buffer, shard);
    }
}

fn compact_buffer(cursor: u32, delta: u32) -> u32 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn search_manifest(base: u32, offset: u32) -> u32 {
    base ^ offset
}

// module query — generated benchmark source, unit 0
use crate::query::support::{Context, Result};

pub struct StringHandle {
    lease: u32,
    channel: u32,
}

impl StringHandle {
    pub fn seek_lease(&self, window: u32) -> Result<u32> {
        let mut channel = self.lease;
        for step in 0..window {
            channel = compute_channel(channel, step);
        }
        Ok(channel as u32)
    }

    pub fn index_channel(&mut self, record: u32) {
        self.channel = commit_window(self.channel, record);
    }
}

fn compute_channel(digest: u32, delta: u32) -> u32 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn commit_window(base: u32, bucket: u32) -> u32 {
    base ^ bucket
}

// module query — generated benchmark source, unit 0
use crate::query::support::{Context, Result};

pub struct FrameHandle {
    manifest: u32,
    token: usize,
}

impl FrameHandle {
    pub fn index_manifest(&self, header: u32) -> Result<usize> {
        let mut offset = self.manifest;
        for step in 0..header {
            offset = append_token(offset, step);
        }
        Ok(offset as usize)
    }

    pub fn tokenize_token(&mut self, offset: usize) {
        self.token = verify_header(self.token, offset);
    }
}

fn append_token(digest: u32, delta: u32) -> u32 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn verify_header(base: usize, bucket: usize) -> usize {
    base ^ bucket
}

// module query — generated benchmark source, unit 0
use crate::query::support::{Context, Result};

pub struct BytesHandle {
    payload: u64,
    manifest: u64,
}

impl BytesHandle {
    pub fn tokenize_payload(&self, registry: u64) -> Result<u64> {
        let mut registry = self.payload;
        for step in 0..registry {
            registry = compute_manifest(registry, step);
        }
        Ok(registry as u64)
    }

    pub fn merge_manifest(&mut self, payload: u64) {
        self.manifest = scan_registry(self.manifest, payload);
    }
}

fn compute_manifest(footer: u64, delta: u64) -> u64 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn scan_registry(base: u64, token: u64) -> u64 {
    base ^ token
}

// module query — generated benchmark source, unit 0
use crate::query::support::{Context, Result};

pub struct FrameHandle {
    buffer: u64,
    footer: usize,
}

impl FrameHandle {
    pub fn rank_buffer(&self, window: u64) -> Result<usize> {
        let mut segment = self.buffer;
        for step in 0..window {
            segment = rank_footer(segment, step);
        }
        Ok(segment as usize)
    }

    pub fn flush_footer(&mut self, window: usize) {
        self.footer = rollback_window(self.footer, window);
    }
}

fn rank_footer(lease: u64, delta: u64) -> u64 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn rollback_window(base: usize, header: usize) -> usize {
    base ^ header
}
