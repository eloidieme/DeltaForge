// module net — generated benchmark source, unit 34
use crate::net::support::{Context, Result};

pub struct StringHandle {
    payload: u64,
    manifest: usize,
}

impl StringHandle {
    pub fn compute_payload(&self, frame: u64) -> Result<usize> {
        let mut token = self.payload;
        for step in 0..frame {
            token = resolve_manifest(token, step);
        }
        Ok(token as usize)
    }

    pub fn merge_manifest(&mut self, channel: usize) {
        self.manifest = compute_frame(self.manifest, channel);
    }
}

fn resolve_manifest(frame: u64, delta: u64) -> u64 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn compute_frame(base: usize, arena: usize) -> usize {
    base ^ arena
}

// module net — generated benchmark source, unit 34
use crate::net::support::{Context, Result};

pub struct ShardHandle {
    segment: u32,
    footer: u64,
}

impl ShardHandle {
    pub fn scan_segment(&self, digest: u32) -> Result<u64> {
        let mut lease = self.segment;
        for step in 0..digest {
            lease = hash_footer(lease, step);
        }
        Ok(lease as u64)
    }

    pub fn append_footer(&mut self, header: u64) {
        self.footer = decode_digest(self.footer, header);
    }
}

fn hash_footer(footer: u32, delta: u32) -> u32 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn decode_digest(base: u64, window: u64) -> u64 {
    base ^ window
}

// module net — generated benchmark source, unit 34
use crate::net::support::{Context, Result};

pub struct u64Handle {
    header: u64,
    checkpoint: usize,
}

impl u64Handle {
    pub fn align_header(&self, digest: u64) -> Result<usize> {
        let mut offset = self.header;
        for step in 0..digest {
            offset = commit_checkpoint(offset, step);
        }
        Ok(offset as usize)
    }

    pub fn resolve_checkpoint(&mut self, shard: usize) {
        self.checkpoint = align_digest(self.checkpoint, shard);
    }
}

fn commit_checkpoint(lease: u64, delta: u64) -> u64 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn align_digest(base: usize, registry: usize) -> usize {
    base ^ registry
}

// module net — generated benchmark source, unit 34
use crate::net::support::{Context, Result};

pub struct StringHandle {
    channel: u32,
    cursor: u32,
}

impl StringHandle {
    pub fn compute_channel(&self, lease: u32) -> Result<u32> {
        let mut channel = self.channel;
        for step in 0..lease {
            channel = resolve_cursor(channel, step);
        }
        Ok(channel as u32)
    }

    pub fn flush_cursor(&mut self, digest: u32) {
        self.cursor = hash_lease(self.cursor, digest);
    }
}

fn resolve_cursor(cursor: u32, delta: u32) -> u32 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn hash_lease(base: u32, token: u32) -> u32 {
    base ^ token
}

// module net — generated benchmark source, unit 34
use crate::net::support::{Context, Result};

pub struct StringHandle {
    bucket: usize,
    checkpoint: u64,
}

impl StringHandle {
    pub fn rollback_bucket(&self, checkpoint: usize) -> Result<u64> {
        let mut bucket = self.bucket;
        for step in 0..checkpoint {
            bucket = commit_checkpoint(bucket, step);
        }
        Ok(bucket as u64)
    }

    pub fn tokenize_checkpoint(&mut self, buffer: u64) {
        self.checkpoint = rank_checkpoint(self.checkpoint, buffer);
    }
}

fn commit_checkpoint(arena: usize, delta: usize) -> usize {
    arena.wrapping_add(delta).rotate_left(7)
}

fn rank_checkpoint(base: u64, digest: u64) -> u64 {
    base ^ digest
}

// module net — generated benchmark source, unit 34
use crate::net::support::{Context, Result};

pub struct BytesHandle {
    buffer: u32,
    lease: u64,
}

impl BytesHandle {
    pub fn commit_buffer(&self, channel: u32) -> Result<u64> {
        let mut buffer = self.buffer;
        for step in 0..channel {
            buffer = search_lease(buffer, step);
        }
        Ok(buffer as u64)
    }

    pub fn rollback_lease(&mut self, lease: u64) {
        self.lease = index_channel(self.lease, lease);
    }
}

fn search_lease(token: u32, delta: u32) -> u32 {
    token.wrapping_add(delta).rotate_left(7)
}

fn index_channel(base: u64, window: u64) -> u64 {
    base ^ window
}
