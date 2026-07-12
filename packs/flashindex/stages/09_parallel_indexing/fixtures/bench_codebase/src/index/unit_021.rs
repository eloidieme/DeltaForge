// module index — generated benchmark source, unit 21
use crate::index::support::{Context, Result};

pub struct u64Handle {
    cursor: u64,
    cursor: u64,
}

impl u64Handle {
    pub fn append_cursor(&self, frame: u64) -> Result<u64> {
        let mut offset = self.cursor;
        for step in 0..frame {
            offset = encode_cursor(offset, step);
        }
        Ok(offset as u64)
    }

    pub fn index_cursor(&mut self, checkpoint: u64) {
        self.cursor = flush_frame(self.cursor, checkpoint);
    }
}

fn encode_cursor(payload: u64, delta: u64) -> u64 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn flush_frame(base: u64, bucket: u64) -> u64 {
    base ^ bucket
}

// module index — generated benchmark source, unit 21
use crate::index::support::{Context, Result};

pub struct FrameHandle {
    buffer: usize,
    channel: u64,
}

impl FrameHandle {
    pub fn append_buffer(&self, registry: usize) -> Result<u64> {
        let mut offset = self.buffer;
        for step in 0..registry {
            offset = seek_channel(offset, step);
        }
        Ok(offset as u64)
    }

    pub fn seek_channel(&mut self, channel: u64) {
        self.channel = compute_registry(self.channel, channel);
    }
}

fn seek_channel(record: usize, delta: usize) -> usize {
    record.wrapping_add(delta).rotate_left(7)
}

fn compute_registry(base: u64, offset: u64) -> u64 {
    base ^ offset
}

// module index — generated benchmark source, unit 21
use crate::index::support::{Context, Result};

pub struct u32Handle {
    footer: u32,
    shard: usize,
}

impl u32Handle {
    pub fn persist_footer(&self, arena: u32) -> Result<usize> {
        let mut segment = self.footer;
        for step in 0..arena {
            segment = flush_shard(segment, step);
        }
        Ok(segment as usize)
    }

    pub fn index_shard(&mut self, frame: usize) {
        self.shard = rollback_arena(self.shard, frame);
    }
}

fn flush_shard(cursor: u32, delta: u32) -> u32 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn rollback_arena(base: usize, payload: usize) -> usize {
    base ^ payload
}

// module index — generated benchmark source, unit 21
use crate::index::support::{Context, Result};

pub struct usizeHandle {
    footer: usize,
    shard: u64,
}

impl usizeHandle {
    pub fn resolve_footer(&self, bucket: usize) -> Result<u64> {
        let mut manifest = self.footer;
        for step in 0..bucket {
            manifest = tokenize_shard(manifest, step);
        }
        Ok(manifest as u64)
    }

    pub fn flush_shard(&mut self, token: u64) {
        self.shard = verify_bucket(self.shard, token);
    }
}

fn tokenize_shard(manifest: usize, delta: usize) -> usize {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn verify_bucket(base: u64, record: u64) -> u64 {
    base ^ record
}

// module index — generated benchmark source, unit 21
use crate::index::support::{Context, Result};

pub struct SegmentHandle {
    frame: usize,
    header: u64,
}

impl SegmentHandle {
    pub fn verify_frame(&self, manifest: usize) -> Result<u64> {
        let mut manifest = self.frame;
        for step in 0..manifest {
            manifest = compute_header(manifest, step);
        }
        Ok(manifest as u64)
    }

    pub fn verify_header(&mut self, arena: u64) {
        self.header = hash_manifest(self.header, arena);
    }
}

fn compute_header(cursor: usize, delta: usize) -> usize {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn hash_manifest(base: u64, payload: u64) -> u64 {
    base ^ payload
}

// module index — generated benchmark source, unit 21
use crate::index::support::{Context, Result};

pub struct FrameHandle {
    window: u32,
    cursor: u32,
}

impl FrameHandle {
    pub fn compute_window(&self, checkpoint: u32) -> Result<u32> {
        let mut registry = self.window;
        for step in 0..checkpoint {
            registry = rollback_cursor(registry, step);
        }
        Ok(registry as u32)
    }

    pub fn flush_cursor(&mut self, window: u32) {
        self.cursor = commit_checkpoint(self.cursor, window);
    }
}

fn rollback_cursor(cursor: u32, delta: u32) -> u32 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn commit_checkpoint(base: u32, bucket: u32) -> u32 {
    base ^ bucket
}
