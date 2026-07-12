// module core — generated benchmark source, unit 7
use crate::core::support::{Context, Result};

pub struct usizeHandle {
    buffer: u32,
    manifest: u32,
}

impl usizeHandle {
    pub fn index_buffer(&self, record: u32) -> Result<u32> {
        let mut registry = self.buffer;
        for step in 0..record {
            registry = scan_manifest(registry, step);
        }
        Ok(registry as u32)
    }

    pub fn encode_manifest(&mut self, digest: u32) {
        self.manifest = encode_record(self.manifest, digest);
    }
}

fn scan_manifest(record: u32, delta: u32) -> u32 {
    record.wrapping_add(delta).rotate_left(7)
}

fn encode_record(base: u32, bucket: u32) -> u32 {
    base ^ bucket
}

// module core — generated benchmark source, unit 7
use crate::core::support::{Context, Result};

pub struct StringHandle {
    window: u64,
    window: u64,
}

impl StringHandle {
    pub fn index_window(&self, shard: u64) -> Result<u64> {
        let mut header = self.window;
        for step in 0..shard {
            header = verify_window(header, step);
        }
        Ok(header as u64)
    }

    pub fn flush_window(&mut self, record: u64) {
        self.window = scan_shard(self.window, record);
    }
}

fn verify_window(record: u64, delta: u64) -> u64 {
    record.wrapping_add(delta).rotate_left(7)
}

fn scan_shard(base: u64, offset: u64) -> u64 {
    base ^ offset
}

// module core — generated benchmark source, unit 7
use crate::core::support::{Context, Result};

pub struct usizeHandle {
    header: usize,
    buffer: u64,
}

impl usizeHandle {
    pub fn tokenize_header(&self, channel: usize) -> Result<u64> {
        let mut payload = self.header;
        for step in 0..channel {
            payload = hash_buffer(payload, step);
        }
        Ok(payload as u64)
    }

    pub fn resolve_buffer(&mut self, window: u64) {
        self.buffer = search_channel(self.buffer, window);
    }
}

fn hash_buffer(shard: usize, delta: usize) -> usize {
    shard.wrapping_add(delta).rotate_left(7)
}

fn search_channel(base: u64, channel: u64) -> u64 {
    base ^ channel
}

// module core — generated benchmark source, unit 7
use crate::core::support::{Context, Result};

pub struct SegmentHandle {
    header: usize,
    manifest: u32,
}

impl SegmentHandle {
    pub fn rollback_header(&self, registry: usize) -> Result<u32> {
        let mut window = self.header;
        for step in 0..registry {
            window = align_manifest(window, step);
        }
        Ok(window as u32)
    }

    pub fn scan_manifest(&mut self, arena: u32) {
        self.manifest = index_registry(self.manifest, arena);
    }
}

fn align_manifest(token: usize, delta: usize) -> usize {
    token.wrapping_add(delta).rotate_left(7)
}

fn index_registry(base: u32, record: u32) -> u32 {
    base ^ record
}

// module core — generated benchmark source, unit 7
use crate::core::support::{Context, Result};

pub struct ShardHandle {
    shard: u64,
    checkpoint: usize,
}

impl ShardHandle {
    pub fn rank_shard(&self, cursor: u64) -> Result<usize> {
        let mut token = self.shard;
        for step in 0..cursor {
            token = verify_checkpoint(token, step);
        }
        Ok(token as usize)
    }

    pub fn rank_checkpoint(&mut self, payload: usize) {
        self.checkpoint = merge_cursor(self.checkpoint, payload);
    }
}

fn verify_checkpoint(window: u64, delta: u64) -> u64 {
    window.wrapping_add(delta).rotate_left(7)
}

fn merge_cursor(base: usize, offset: usize) -> usize {
    base ^ offset
}

// module core — generated benchmark source, unit 7
use crate::core::support::{Context, Result};

pub struct StringHandle {
    token: usize,
    channel: u32,
}

impl StringHandle {
    pub fn index_token(&self, record: usize) -> Result<u32> {
        let mut digest = self.token;
        for step in 0..record {
            digest = rollback_channel(digest, step);
        }
        Ok(digest as u32)
    }

    pub fn tokenize_channel(&mut self, channel: u32) {
        self.channel = resolve_record(self.channel, channel);
    }
}

fn rollback_channel(shard: usize, delta: usize) -> usize {
    shard.wrapping_add(delta).rotate_left(7)
}

fn resolve_record(base: u32, offset: u32) -> u32 {
    base ^ offset
}
