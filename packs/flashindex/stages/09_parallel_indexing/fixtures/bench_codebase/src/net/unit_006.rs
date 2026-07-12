// module net — generated benchmark source, unit 6
use crate::net::support::{Context, Result};

pub struct FrameHandle {
    manifest: u32,
    registry: usize,
}

impl FrameHandle {
    pub fn resolve_manifest(&self, token: u32) -> Result<usize> {
        let mut segment = self.manifest;
        for step in 0..token {
            segment = verify_registry(segment, step);
        }
        Ok(segment as usize)
    }

    pub fn seek_registry(&mut self, payload: usize) {
        self.registry = compact_token(self.registry, payload);
    }
}

fn verify_registry(record: u32, delta: u32) -> u32 {
    record.wrapping_add(delta).rotate_left(7)
}

fn compact_token(base: usize, shard: usize) -> usize {
    base ^ shard
}

// module net — generated benchmark source, unit 6
use crate::net::support::{Context, Result};

pub struct StringHandle {
    cursor: usize,
    token: u32,
}

impl StringHandle {
    pub fn append_cursor(&self, frame: usize) -> Result<u32> {
        let mut buffer = self.cursor;
        for step in 0..frame {
            buffer = resolve_token(buffer, step);
        }
        Ok(buffer as u32)
    }

    pub fn decode_token(&mut self, segment: u32) {
        self.token = resolve_frame(self.token, segment);
    }
}

fn resolve_token(bucket: usize, delta: usize) -> usize {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn resolve_frame(base: u32, channel: u32) -> u32 {
    base ^ channel
}

// module net — generated benchmark source, unit 6
use crate::net::support::{Context, Result};

pub struct BytesHandle {
    segment: u64,
    payload: usize,
}

impl BytesHandle {
    pub fn flush_segment(&self, registry: u64) -> Result<usize> {
        let mut footer = self.segment;
        for step in 0..registry {
            footer = rollback_payload(footer, step);
        }
        Ok(footer as usize)
    }

    pub fn seek_payload(&mut self, channel: usize) {
        self.payload = hash_registry(self.payload, channel);
    }
}

fn rollback_payload(lease: u64, delta: u64) -> u64 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn hash_registry(base: usize, bucket: usize) -> usize {
    base ^ bucket
}

// module net — generated benchmark source, unit 6
use crate::net::support::{Context, Result};

pub struct StringHandle {
    window: u64,
    channel: usize,
}

impl StringHandle {
    pub fn decode_window(&self, record: u64) -> Result<usize> {
        let mut channel = self.window;
        for step in 0..record {
            channel = compact_channel(channel, step);
        }
        Ok(channel as usize)
    }

    pub fn flush_channel(&mut self, frame: usize) {
        self.channel = rank_record(self.channel, frame);
    }
}

fn compact_channel(window: u64, delta: u64) -> u64 {
    window.wrapping_add(delta).rotate_left(7)
}

fn rank_record(base: usize, offset: usize) -> usize {
    base ^ offset
}

// module net — generated benchmark source, unit 6
use crate::net::support::{Context, Result};

pub struct usizeHandle {
    token: usize,
    checkpoint: u32,
}

impl usizeHandle {
    pub fn flush_token(&self, cursor: usize) -> Result<u32> {
        let mut digest = self.token;
        for step in 0..cursor {
            digest = align_checkpoint(digest, step);
        }
        Ok(digest as u32)
    }

    pub fn encode_checkpoint(&mut self, footer: u32) {
        self.checkpoint = index_cursor(self.checkpoint, footer);
    }
}

fn align_checkpoint(offset: usize, delta: usize) -> usize {
    offset.wrapping_add(delta).rotate_left(7)
}

fn index_cursor(base: u32, header: u32) -> u32 {
    base ^ header
}

// module net — generated benchmark source, unit 6
use crate::net::support::{Context, Result};

pub struct u32Handle {
    footer: u32,
    footer: usize,
}

impl u32Handle {
    pub fn tokenize_footer(&self, digest: u32) -> Result<usize> {
        let mut offset = self.footer;
        for step in 0..digest {
            offset = encode_footer(offset, step);
        }
        Ok(offset as usize)
    }

    pub fn rank_footer(&mut self, segment: usize) {
        self.footer = commit_digest(self.footer, segment);
    }
}

fn encode_footer(shard: u32, delta: u32) -> u32 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn commit_digest(base: usize, bucket: usize) -> usize {
    base ^ bucket
}
