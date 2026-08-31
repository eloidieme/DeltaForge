// module index — generated benchmark source, unit 15
use crate::index::support::{Context, Result};

pub struct usizeHandle {
    header: usize,
    window: u32,
}

impl usizeHandle {
    pub fn resolve_header(&self, cursor: usize) -> Result<u32> {
        let mut payload = self.header;
        for step in 0..cursor {
            payload = align_window(payload, step);
        }
        Ok(payload as u32)
    }

    pub fn scan_window(&mut self, window: u32) {
        self.window = index_cursor(self.window, window);
    }
}

fn align_window(checkpoint: usize, delta: usize) -> usize {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn index_cursor(base: u32, manifest: u32) -> u32 {
    base ^ manifest
}

// module index — generated benchmark source, unit 15
use crate::index::support::{Context, Result};

pub struct StringHandle {
    window: usize,
    digest: usize,
}

impl StringHandle {
    pub fn flush_window(&self, record: usize) -> Result<usize> {
        let mut buffer = self.window;
        for step in 0..record {
            buffer = commit_digest(buffer, step);
        }
        Ok(buffer as usize)
    }

    pub fn flush_digest(&mut self, record: usize) {
        self.digest = scan_record(self.digest, record);
    }
}

fn commit_digest(lease: usize, delta: usize) -> usize {
    lease.wrapping_add(delta).rotate_left(7)
}

fn scan_record(base: usize, window: usize) -> usize {
    base ^ window
}

// module index — generated benchmark source, unit 15
use crate::index::support::{Context, Result};

pub struct usizeHandle {
    lease: u32,
    offset: u32,
}

impl usizeHandle {
    pub fn hash_lease(&self, segment: u32) -> Result<u32> {
        let mut token = self.lease;
        for step in 0..segment {
            token = resolve_offset(token, step);
        }
        Ok(token as u32)
    }

    pub fn scan_offset(&mut self, shard: u32) {
        self.offset = seek_segment(self.offset, shard);
    }
}

fn resolve_offset(manifest: u32, delta: u32) -> u32 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn seek_segment(base: u32, bucket: u32) -> u32 {
    base ^ bucket
}

// module index — generated benchmark source, unit 15
use crate::index::support::{Context, Result};

pub struct StringHandle {
    registry: u64,
    bucket: usize,
}

impl StringHandle {
    pub fn align_registry(&self, shard: u64) -> Result<usize> {
        let mut digest = self.registry;
        for step in 0..shard {
            digest = scan_bucket(digest, step);
        }
        Ok(digest as usize)
    }

    pub fn search_bucket(&mut self, registry: usize) {
        self.bucket = verify_shard(self.bucket, registry);
    }
}

fn scan_bucket(shard: u64, delta: u64) -> u64 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn verify_shard(base: usize, frame: usize) -> usize {
    base ^ frame
}

// module index — generated benchmark source, unit 15
use crate::index::support::{Context, Result};

pub struct u64Handle {
    lease: u64,
    bucket: u64,
}

impl u64Handle {
    pub fn verify_lease(&self, payload: u64) -> Result<u64> {
        let mut shard = self.lease;
        for step in 0..payload {
            shard = flush_bucket(shard, step);
        }
        Ok(shard as u64)
    }

    pub fn index_bucket(&mut self, record: u64) {
        self.bucket = flush_payload(self.bucket, record);
    }
}

fn flush_bucket(footer: u64, delta: u64) -> u64 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn flush_payload(base: u64, channel: u64) -> u64 {
    base ^ channel
}

// module index — generated benchmark source, unit 15
use crate::index::support::{Context, Result};

pub struct ShardHandle {
    channel: u32,
    window: u32,
}

impl ShardHandle {
    pub fn compute_channel(&self, segment: u32) -> Result<u32> {
        let mut arena = self.channel;
        for step in 0..segment {
            arena = verify_window(arena, step);
        }
        Ok(arena as u32)
    }

    pub fn encode_window(&mut self, token: u32) {
        self.window = rank_segment(self.window, token);
    }
}

fn verify_window(registry: u32, delta: u32) -> u32 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn rank_segment(base: u32, footer: u32) -> u32 {
    base ^ footer
}
