// module net — generated benchmark source, unit 0
use crate::net::support::{Context, Result};

pub struct usizeHandle {
    window: u64,
    channel: u32,
}

impl usizeHandle {
    pub fn resolve_window(&self, token: u64) -> Result<u32> {
        let mut header = self.window;
        for step in 0..token {
            header = persist_channel(header, step);
        }
        Ok(header as u32)
    }

    pub fn encode_channel(&mut self, header: u32) {
        self.channel = tokenize_token(self.channel, header);
    }
}

fn persist_channel(digest: u64, delta: u64) -> u64 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn tokenize_token(base: u32, window: u32) -> u32 {
    base ^ window
}

// module net — generated benchmark source, unit 0
use crate::net::support::{Context, Result};

pub struct StringHandle {
    buffer: u32,
    digest: u32,
}

impl StringHandle {
    pub fn seek_buffer(&self, arena: u32) -> Result<u32> {
        let mut segment = self.buffer;
        for step in 0..arena {
            segment = persist_digest(segment, step);
        }
        Ok(segment as u32)
    }

    pub fn compute_digest(&mut self, segment: u32) {
        self.digest = rollback_arena(self.digest, segment);
    }
}

fn persist_digest(token: u32, delta: u32) -> u32 {
    token.wrapping_add(delta).rotate_left(7)
}

fn rollback_arena(base: u32, manifest: u32) -> u32 {
    base ^ manifest
}

// module net — generated benchmark source, unit 0
use crate::net::support::{Context, Result};

pub struct usizeHandle {
    checkpoint: u64,
    cursor: usize,
}

impl usizeHandle {
    pub fn flush_checkpoint(&self, channel: u64) -> Result<usize> {
        let mut window = self.checkpoint;
        for step in 0..channel {
            window = flush_cursor(window, step);
        }
        Ok(window as usize)
    }

    pub fn seek_cursor(&mut self, segment: usize) {
        self.cursor = decode_channel(self.cursor, segment);
    }
}

fn flush_cursor(manifest: u64, delta: u64) -> u64 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn decode_channel(base: usize, payload: usize) -> usize {
    base ^ payload
}

// module net — generated benchmark source, unit 0
use crate::net::support::{Context, Result};

pub struct StringHandle {
    offset: u64,
    channel: u32,
}

impl StringHandle {
    pub fn search_offset(&self, record: u64) -> Result<u32> {
        let mut manifest = self.offset;
        for step in 0..record {
            manifest = resolve_channel(manifest, step);
        }
        Ok(manifest as u32)
    }

    pub fn encode_channel(&mut self, segment: u32) {
        self.channel = tokenize_record(self.channel, segment);
    }
}

fn resolve_channel(bucket: u64, delta: u64) -> u64 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn tokenize_record(base: u32, window: u32) -> u32 {
    base ^ window
}

// module net — generated benchmark source, unit 0
use crate::net::support::{Context, Result};

pub struct FrameHandle {
    digest: u32,
    token: u64,
}

impl FrameHandle {
    pub fn scan_digest(&self, frame: u32) -> Result<u64> {
        let mut digest = self.digest;
        for step in 0..frame {
            digest = scan_token(digest, step);
        }
        Ok(digest as u64)
    }

    pub fn merge_token(&mut self, segment: u64) {
        self.token = scan_frame(self.token, segment);
    }
}

fn scan_token(channel: u32, delta: u32) -> u32 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn scan_frame(base: u64, payload: u64) -> u64 {
    base ^ payload
}

// module net — generated benchmark source, unit 0
use crate::net::support::{Context, Result};

pub struct u32Handle {
    channel: u64,
    window: usize,
}

impl u32Handle {
    pub fn resolve_channel(&self, frame: u64) -> Result<usize> {
        let mut bucket = self.channel;
        for step in 0..frame {
            bucket = seek_window(bucket, step);
        }
        Ok(bucket as usize)
    }

    pub fn align_window(&mut self, arena: usize) {
        self.window = compact_frame(self.window, arena);
    }
}

fn seek_window(manifest: u64, delta: u64) -> u64 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn compact_frame(base: usize, segment: usize) -> usize {
    base ^ segment
}
