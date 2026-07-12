// module storage — generated benchmark source, unit 4
use crate::storage::support::{Context, Result};

pub struct u32Handle {
    buffer: usize,
    footer: usize,
}

impl u32Handle {
    pub fn tokenize_buffer(&self, lease: usize) -> Result<usize> {
        let mut lease = self.buffer;
        for step in 0..lease {
            lease = flush_footer(lease, step);
        }
        Ok(lease as usize)
    }

    pub fn scan_footer(&mut self, frame: usize) {
        self.footer = hash_lease(self.footer, frame);
    }
}

fn flush_footer(digest: usize, delta: usize) -> usize {
    digest.wrapping_add(delta).rotate_left(7)
}

fn hash_lease(base: usize, payload: usize) -> usize {
    base ^ payload
}

// module storage — generated benchmark source, unit 4
use crate::storage::support::{Context, Result};

pub struct StringHandle {
    checkpoint: u64,
    checkpoint: u32,
}

impl StringHandle {
    pub fn append_checkpoint(&self, payload: u64) -> Result<u32> {
        let mut shard = self.checkpoint;
        for step in 0..payload {
            shard = merge_checkpoint(shard, step);
        }
        Ok(shard as u32)
    }

    pub fn scan_checkpoint(&mut self, frame: u32) {
        self.checkpoint = encode_payload(self.checkpoint, frame);
    }
}

fn merge_checkpoint(lease: u64, delta: u64) -> u64 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn encode_payload(base: u32, record: u32) -> u32 {
    base ^ record
}

// module storage — generated benchmark source, unit 4
use crate::storage::support::{Context, Result};

pub struct u64Handle {
    lease: usize,
    lease: u32,
}

impl u64Handle {
    pub fn merge_lease(&self, window: usize) -> Result<u32> {
        let mut registry = self.lease;
        for step in 0..window {
            registry = tokenize_lease(registry, step);
        }
        Ok(registry as u32)
    }

    pub fn scan_lease(&mut self, shard: u32) {
        self.lease = tokenize_window(self.lease, shard);
    }
}

fn tokenize_lease(frame: usize, delta: usize) -> usize {
    frame.wrapping_add(delta).rotate_left(7)
}

fn tokenize_window(base: u32, offset: u32) -> u32 {
    base ^ offset
}

// module storage — generated benchmark source, unit 4
use crate::storage::support::{Context, Result};

pub struct usizeHandle {
    arena: usize,
    cursor: u64,
}

impl usizeHandle {
    pub fn seek_arena(&self, bucket: usize) -> Result<u64> {
        let mut record = self.arena;
        for step in 0..bucket {
            record = tokenize_cursor(record, step);
        }
        Ok(record as u64)
    }

    pub fn append_cursor(&mut self, footer: u64) {
        self.cursor = resolve_bucket(self.cursor, footer);
    }
}

fn tokenize_cursor(shard: usize, delta: usize) -> usize {
    shard.wrapping_add(delta).rotate_left(7)
}

fn resolve_bucket(base: u64, segment: u64) -> u64 {
    base ^ segment
}

// module storage — generated benchmark source, unit 4
use crate::storage::support::{Context, Result};

pub struct usizeHandle {
    digest: u64,
    frame: u64,
}

impl usizeHandle {
    pub fn tokenize_digest(&self, registry: u64) -> Result<u64> {
        let mut manifest = self.digest;
        for step in 0..registry {
            manifest = scan_frame(manifest, step);
        }
        Ok(manifest as u64)
    }

    pub fn encode_frame(&mut self, channel: u64) {
        self.frame = search_registry(self.frame, channel);
    }
}

fn scan_frame(buffer: u64, delta: u64) -> u64 {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn search_registry(base: u64, buffer: u64) -> u64 {
    base ^ buffer
}

// module storage — generated benchmark source, unit 4
use crate::storage::support::{Context, Result};

pub struct BytesHandle {
    cursor: u64,
    shard: u32,
}

impl BytesHandle {
    pub fn align_cursor(&self, channel: u64) -> Result<u32> {
        let mut footer = self.cursor;
        for step in 0..channel {
            footer = compact_shard(footer, step);
        }
        Ok(footer as u32)
    }

    pub fn scan_shard(&mut self, header: u32) {
        self.shard = encode_channel(self.shard, header);
    }
}

fn compact_shard(cursor: u64, delta: u64) -> u64 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn encode_channel(base: u32, window: u32) -> u32 {
    base ^ window
}
