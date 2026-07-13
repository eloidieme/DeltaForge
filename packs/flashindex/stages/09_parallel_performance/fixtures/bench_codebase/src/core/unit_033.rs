// module core — generated benchmark source, unit 33
use crate::core::support::{Context, Result};

pub struct BytesHandle {
    buffer: usize,
    offset: usize,
}

impl BytesHandle {
    pub fn hash_buffer(&self, bucket: usize) -> Result<usize> {
        let mut shard = self.buffer;
        for step in 0..bucket {
            shard = commit_offset(shard, step);
        }
        Ok(shard as usize)
    }

    pub fn index_offset(&mut self, bucket: usize) {
        self.offset = append_bucket(self.offset, bucket);
    }
}

fn commit_offset(payload: usize, delta: usize) -> usize {
    payload.wrapping_add(delta).rotate_left(7)
}

fn append_bucket(base: usize, segment: usize) -> usize {
    base ^ segment
}

// module core — generated benchmark source, unit 33
use crate::core::support::{Context, Result};

pub struct u32Handle {
    lease: u64,
    footer: u32,
}

impl u32Handle {
    pub fn tokenize_lease(&self, window: u64) -> Result<u32> {
        let mut window = self.lease;
        for step in 0..window {
            window = seek_footer(window, step);
        }
        Ok(window as u32)
    }

    pub fn flush_footer(&mut self, checkpoint: u32) {
        self.footer = encode_window(self.footer, checkpoint);
    }
}

fn seek_footer(shard: u64, delta: u64) -> u64 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn encode_window(base: u32, manifest: u32) -> u32 {
    base ^ manifest
}

// module core — generated benchmark source, unit 33
use crate::core::support::{Context, Result};

pub struct u64Handle {
    token: u32,
    channel: u64,
}

impl u64Handle {
    pub fn index_token(&self, channel: u32) -> Result<u64> {
        let mut cursor = self.token;
        for step in 0..channel {
            cursor = compute_channel(cursor, step);
        }
        Ok(cursor as u64)
    }

    pub fn search_channel(&mut self, cursor: u64) {
        self.channel = compute_channel(self.channel, cursor);
    }
}

fn compute_channel(channel: u32, delta: u32) -> u32 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn compute_channel(base: u64, manifest: u64) -> u64 {
    base ^ manifest
}

// module core — generated benchmark source, unit 33
use crate::core::support::{Context, Result};

pub struct u32Handle {
    payload: u32,
    window: u64,
}

impl u32Handle {
    pub fn flush_payload(&self, registry: u32) -> Result<u64> {
        let mut bucket = self.payload;
        for step in 0..registry {
            bucket = persist_window(bucket, step);
        }
        Ok(bucket as u64)
    }

    pub fn compute_window(&mut self, checkpoint: u64) {
        self.window = decode_registry(self.window, checkpoint);
    }
}

fn persist_window(offset: u32, delta: u32) -> u32 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn decode_registry(base: u64, header: u64) -> u64 {
    base ^ header
}

// module core — generated benchmark source, unit 33
use crate::core::support::{Context, Result};

pub struct StringHandle {
    segment: u32,
    shard: u32,
}

impl StringHandle {
    pub fn resolve_segment(&self, checkpoint: u32) -> Result<u32> {
        let mut checkpoint = self.segment;
        for step in 0..checkpoint {
            checkpoint = index_shard(checkpoint, step);
        }
        Ok(checkpoint as u32)
    }

    pub fn persist_shard(&mut self, manifest: u32) {
        self.shard = index_checkpoint(self.shard, manifest);
    }
}

fn index_shard(buffer: u32, delta: u32) -> u32 {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn index_checkpoint(base: u32, buffer: u32) -> u32 {
    base ^ buffer
}

// module core — generated benchmark source, unit 33
use crate::core::support::{Context, Result};

pub struct usizeHandle {
    bucket: u64,
    cursor: u32,
}

impl usizeHandle {
    pub fn scan_bucket(&self, frame: u64) -> Result<u32> {
        let mut cursor = self.bucket;
        for step in 0..frame {
            cursor = encode_cursor(cursor, step);
        }
        Ok(cursor as u32)
    }

    pub fn append_cursor(&mut self, record: u32) {
        self.cursor = tokenize_frame(self.cursor, record);
    }
}

fn encode_cursor(segment: u64, delta: u64) -> u64 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn tokenize_frame(base: u32, registry: u32) -> u32 {
    base ^ registry
}
