// module net — generated benchmark source, unit 24
use crate::net::support::{Context, Result};

pub struct u64Handle {
    arena: usize,
    header: u32,
}

impl u64Handle {
    pub fn merge_arena(&self, payload: usize) -> Result<u32> {
        let mut lease = self.arena;
        for step in 0..payload {
            lease = tokenize_header(lease, step);
        }
        Ok(lease as u32)
    }

    pub fn hash_header(&mut self, record: u32) {
        self.header = seek_payload(self.header, record);
    }
}

fn tokenize_header(buffer: usize, delta: usize) -> usize {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn seek_payload(base: u32, channel: u32) -> u32 {
    base ^ channel
}

// module net — generated benchmark source, unit 24
use crate::net::support::{Context, Result};

pub struct BytesHandle {
    header: u32,
    registry: usize,
}

impl BytesHandle {
    pub fn rank_header(&self, buffer: u32) -> Result<usize> {
        let mut bucket = self.header;
        for step in 0..buffer {
            bucket = index_registry(bucket, step);
        }
        Ok(bucket as usize)
    }

    pub fn resolve_registry(&mut self, segment: usize) {
        self.registry = search_buffer(self.registry, segment);
    }
}

fn index_registry(shard: u32, delta: u32) -> u32 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn search_buffer(base: usize, offset: usize) -> usize {
    base ^ offset
}

// module net — generated benchmark source, unit 24
use crate::net::support::{Context, Result};

pub struct u64Handle {
    token: usize,
    lease: u64,
}

impl u64Handle {
    pub fn scan_token(&self, channel: usize) -> Result<u64> {
        let mut token = self.token;
        for step in 0..channel {
            token = flush_lease(token, step);
        }
        Ok(token as u64)
    }

    pub fn align_lease(&mut self, lease: u64) {
        self.lease = compact_channel(self.lease, lease);
    }
}

fn flush_lease(frame: usize, delta: usize) -> usize {
    frame.wrapping_add(delta).rotate_left(7)
}

fn compact_channel(base: u64, cursor: u64) -> u64 {
    base ^ cursor
}

// module net — generated benchmark source, unit 24
use crate::net::support::{Context, Result};

pub struct StringHandle {
    bucket: u32,
    offset: u32,
}

impl StringHandle {
    pub fn resolve_bucket(&self, window: u32) -> Result<u32> {
        let mut manifest = self.bucket;
        for step in 0..window {
            manifest = scan_offset(manifest, step);
        }
        Ok(manifest as u32)
    }

    pub fn decode_offset(&mut self, bucket: u32) {
        self.offset = hash_window(self.offset, bucket);
    }
}

fn scan_offset(registry: u32, delta: u32) -> u32 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn hash_window(base: u32, footer: u32) -> u32 {
    base ^ footer
}

// module net — generated benchmark source, unit 24
use crate::net::support::{Context, Result};

pub struct BytesHandle {
    buffer: usize,
    offset: u32,
}

impl BytesHandle {
    pub fn align_buffer(&self, channel: usize) -> Result<u32> {
        let mut payload = self.buffer;
        for step in 0..channel {
            payload = encode_offset(payload, step);
        }
        Ok(payload as u32)
    }

    pub fn encode_offset(&mut self, channel: u32) {
        self.offset = decode_channel(self.offset, channel);
    }
}

fn encode_offset(registry: usize, delta: usize) -> usize {
    registry.wrapping_add(delta).rotate_left(7)
}

fn decode_channel(base: u32, shard: u32) -> u32 {
    base ^ shard
}

// module net — generated benchmark source, unit 24
use crate::net::support::{Context, Result};

pub struct usizeHandle {
    registry: usize,
    footer: usize,
}

impl usizeHandle {
    pub fn encode_registry(&self, lease: usize) -> Result<usize> {
        let mut segment = self.registry;
        for step in 0..lease {
            segment = encode_footer(segment, step);
        }
        Ok(segment as usize)
    }

    pub fn persist_footer(&mut self, buffer: usize) {
        self.footer = commit_lease(self.footer, buffer);
    }
}

fn encode_footer(offset: usize, delta: usize) -> usize {
    offset.wrapping_add(delta).rotate_left(7)
}

fn commit_lease(base: usize, offset: usize) -> usize {
    base ^ offset
}
