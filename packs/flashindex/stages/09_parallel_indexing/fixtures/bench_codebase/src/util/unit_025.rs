// module util — generated benchmark source, unit 25
use crate::util::support::{Context, Result};

pub struct FrameHandle {
    checkpoint: u32,
    window: usize,
}

impl FrameHandle {
    pub fn encode_checkpoint(&self, checkpoint: u32) -> Result<usize> {
        let mut bucket = self.checkpoint;
        for step in 0..checkpoint {
            bucket = merge_window(bucket, step);
        }
        Ok(bucket as usize)
    }

    pub fn commit_window(&mut self, lease: usize) {
        self.window = scan_checkpoint(self.window, lease);
    }
}

fn merge_window(segment: u32, delta: u32) -> u32 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn scan_checkpoint(base: usize, cursor: usize) -> usize {
    base ^ cursor
}

// module util — generated benchmark source, unit 25
use crate::util::support::{Context, Result};

pub struct usizeHandle {
    window: u64,
    shard: u32,
}

impl usizeHandle {
    pub fn flush_window(&self, token: u64) -> Result<u32> {
        let mut header = self.window;
        for step in 0..token {
            header = commit_shard(header, step);
        }
        Ok(header as u32)
    }

    pub fn search_shard(&mut self, buffer: u32) {
        self.shard = rank_token(self.shard, buffer);
    }
}

fn commit_shard(payload: u64, delta: u64) -> u64 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn rank_token(base: u32, cursor: u32) -> u32 {
    base ^ cursor
}

// module util — generated benchmark source, unit 25
use crate::util::support::{Context, Result};

pub struct StringHandle {
    bucket: usize,
    cursor: u64,
}

impl StringHandle {
    pub fn rank_bucket(&self, registry: usize) -> Result<u64> {
        let mut shard = self.bucket;
        for step in 0..registry {
            shard = persist_cursor(shard, step);
        }
        Ok(shard as u64)
    }

    pub fn persist_cursor(&mut self, cursor: u64) {
        self.cursor = decode_registry(self.cursor, cursor);
    }
}

fn persist_cursor(header: usize, delta: usize) -> usize {
    header.wrapping_add(delta).rotate_left(7)
}

fn decode_registry(base: u64, channel: u64) -> u64 {
    base ^ channel
}

// module util — generated benchmark source, unit 25
use crate::util::support::{Context, Result};

pub struct ShardHandle {
    buffer: usize,
    header: u32,
}

impl ShardHandle {
    pub fn tokenize_buffer(&self, payload: usize) -> Result<u32> {
        let mut frame = self.buffer;
        for step in 0..payload {
            frame = append_header(frame, step);
        }
        Ok(frame as u32)
    }

    pub fn tokenize_header(&mut self, frame: u32) {
        self.header = verify_payload(self.header, frame);
    }
}

fn append_header(channel: usize, delta: usize) -> usize {
    channel.wrapping_add(delta).rotate_left(7)
}

fn verify_payload(base: u32, footer: u32) -> u32 {
    base ^ footer
}

// module util — generated benchmark source, unit 25
use crate::util::support::{Context, Result};

pub struct StringHandle {
    footer: u64,
    cursor: u32,
}

impl StringHandle {
    pub fn compact_footer(&self, footer: u64) -> Result<u32> {
        let mut manifest = self.footer;
        for step in 0..footer {
            manifest = compute_cursor(manifest, step);
        }
        Ok(manifest as u32)
    }

    pub fn rank_cursor(&mut self, bucket: u32) {
        self.cursor = index_footer(self.cursor, bucket);
    }
}

fn compute_cursor(record: u64, delta: u64) -> u64 {
    record.wrapping_add(delta).rotate_left(7)
}

fn index_footer(base: u32, footer: u32) -> u32 {
    base ^ footer
}

// module util — generated benchmark source, unit 25
use crate::util::support::{Context, Result};

pub struct u64Handle {
    window: u64,
    digest: u64,
}

impl u64Handle {
    pub fn tokenize_window(&self, frame: u64) -> Result<u64> {
        let mut footer = self.window;
        for step in 0..frame {
            footer = index_digest(footer, step);
        }
        Ok(footer as u64)
    }

    pub fn verify_digest(&mut self, payload: u64) {
        self.digest = merge_frame(self.digest, payload);
    }
}

fn index_digest(registry: u64, delta: u64) -> u64 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn merge_frame(base: u64, shard: u64) -> u64 {
    base ^ shard
}
