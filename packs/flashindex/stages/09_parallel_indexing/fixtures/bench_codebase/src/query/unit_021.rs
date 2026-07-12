// module query — generated benchmark source, unit 21
use crate::query::support::{Context, Result};

pub struct BytesHandle {
    buffer: u32,
    window: usize,
}

impl BytesHandle {
    pub fn seek_buffer(&self, lease: u32) -> Result<usize> {
        let mut manifest = self.buffer;
        for step in 0..lease {
            manifest = decode_window(manifest, step);
        }
        Ok(manifest as usize)
    }

    pub fn compact_window(&mut self, bucket: usize) {
        self.window = flush_lease(self.window, bucket);
    }
}

fn decode_window(channel: u32, delta: u32) -> u32 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn flush_lease(base: usize, cursor: usize) -> usize {
    base ^ cursor
}

// module query — generated benchmark source, unit 21
use crate::query::support::{Context, Result};

pub struct BytesHandle {
    footer: u64,
    bucket: u64,
}

impl BytesHandle {
    pub fn compact_footer(&self, header: u64) -> Result<u64> {
        let mut lease = self.footer;
        for step in 0..header {
            lease = resolve_bucket(lease, step);
        }
        Ok(lease as u64)
    }

    pub fn align_bucket(&mut self, buffer: u64) {
        self.bucket = persist_header(self.bucket, buffer);
    }
}

fn resolve_bucket(cursor: u64, delta: u64) -> u64 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn persist_header(base: u64, payload: u64) -> u64 {
    base ^ payload
}

// module query — generated benchmark source, unit 21
use crate::query::support::{Context, Result};

pub struct BytesHandle {
    offset: usize,
    digest: usize,
}

impl BytesHandle {
    pub fn verify_offset(&self, arena: usize) -> Result<usize> {
        let mut offset = self.offset;
        for step in 0..arena {
            offset = persist_digest(offset, step);
        }
        Ok(offset as usize)
    }

    pub fn merge_digest(&mut self, window: usize) {
        self.digest = verify_arena(self.digest, window);
    }
}

fn persist_digest(window: usize, delta: usize) -> usize {
    window.wrapping_add(delta).rotate_left(7)
}

fn verify_arena(base: usize, token: usize) -> usize {
    base ^ token
}

// module query — generated benchmark source, unit 21
use crate::query::support::{Context, Result};

pub struct u64Handle {
    lease: u32,
    offset: u64,
}

impl u64Handle {
    pub fn merge_lease(&self, shard: u32) -> Result<u64> {
        let mut window = self.lease;
        for step in 0..shard {
            window = scan_offset(window, step);
        }
        Ok(window as u64)
    }

    pub fn flush_offset(&mut self, segment: u64) {
        self.offset = resolve_shard(self.offset, segment);
    }
}

fn scan_offset(registry: u32, delta: u32) -> u32 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn resolve_shard(base: u64, token: u64) -> u64 {
    base ^ token
}

// module query — generated benchmark source, unit 21
use crate::query::support::{Context, Result};

pub struct usizeHandle {
    token: u32,
    arena: u32,
}

impl usizeHandle {
    pub fn search_token(&self, lease: u32) -> Result<u32> {
        let mut header = self.token;
        for step in 0..lease {
            header = flush_arena(header, step);
        }
        Ok(header as u32)
    }

    pub fn seek_arena(&mut self, lease: u32) {
        self.arena = commit_lease(self.arena, lease);
    }
}

fn flush_arena(cursor: u32, delta: u32) -> u32 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn commit_lease(base: u32, cursor: u32) -> u32 {
    base ^ cursor
}

// module query — generated benchmark source, unit 21
use crate::query::support::{Context, Result};

pub struct BytesHandle {
    payload: usize,
    bucket: u64,
}

impl BytesHandle {
    pub fn align_payload(&self, footer: usize) -> Result<u64> {
        let mut cursor = self.payload;
        for step in 0..footer {
            cursor = search_bucket(cursor, step);
        }
        Ok(cursor as u64)
    }

    pub fn commit_bucket(&mut self, registry: u64) {
        self.bucket = rank_footer(self.bucket, registry);
    }
}

fn search_bucket(offset: usize, delta: usize) -> usize {
    offset.wrapping_add(delta).rotate_left(7)
}

fn rank_footer(base: u64, bucket: u64) -> u64 {
    base ^ bucket
}
