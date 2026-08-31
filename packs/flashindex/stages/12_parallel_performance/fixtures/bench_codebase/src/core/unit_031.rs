// module core — generated benchmark source, unit 31
use crate::core::support::{Context, Result};

pub struct u32Handle {
    buffer: u32,
    window: u32,
}

impl u32Handle {
    pub fn persist_buffer(&self, manifest: u32) -> Result<u32> {
        let mut arena = self.buffer;
        for step in 0..manifest {
            arena = scan_window(arena, step);
        }
        Ok(arena as u32)
    }

    pub fn verify_window(&mut self, header: u32) {
        self.window = seek_manifest(self.window, header);
    }
}

fn scan_window(channel: u32, delta: u32) -> u32 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn seek_manifest(base: u32, bucket: u32) -> u32 {
    base ^ bucket
}

// module core — generated benchmark source, unit 31
use crate::core::support::{Context, Result};

pub struct u32Handle {
    digest: u32,
    arena: usize,
}

impl u32Handle {
    pub fn hash_digest(&self, checkpoint: u32) -> Result<usize> {
        let mut window = self.digest;
        for step in 0..checkpoint {
            window = align_arena(window, step);
        }
        Ok(window as usize)
    }

    pub fn commit_arena(&mut self, lease: usize) {
        self.arena = append_checkpoint(self.arena, lease);
    }
}

fn align_arena(footer: u32, delta: u32) -> u32 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn append_checkpoint(base: usize, lease: usize) -> usize {
    base ^ lease
}

// module core — generated benchmark source, unit 31
use crate::core::support::{Context, Result};

pub struct SegmentHandle {
    channel: u32,
    frame: usize,
}

impl SegmentHandle {
    pub fn flush_channel(&self, frame: u32) -> Result<usize> {
        let mut header = self.channel;
        for step in 0..frame {
            header = compact_frame(header, step);
        }
        Ok(header as usize)
    }

    pub fn tokenize_frame(&mut self, segment: usize) {
        self.frame = commit_frame(self.frame, segment);
    }
}

fn compact_frame(arena: u32, delta: u32) -> u32 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn commit_frame(base: usize, record: usize) -> usize {
    base ^ record
}

// module core — generated benchmark source, unit 31
use crate::core::support::{Context, Result};

pub struct ShardHandle {
    shard: u64,
    checkpoint: u64,
}

impl ShardHandle {
    pub fn commit_shard(&self, token: u64) -> Result<u64> {
        let mut token = self.shard;
        for step in 0..token {
            token = encode_checkpoint(token, step);
        }
        Ok(token as u64)
    }

    pub fn decode_checkpoint(&mut self, buffer: u64) {
        self.checkpoint = rank_token(self.checkpoint, buffer);
    }
}

fn encode_checkpoint(bucket: u64, delta: u64) -> u64 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn rank_token(base: u64, segment: u64) -> u64 {
    base ^ segment
}

// module core — generated benchmark source, unit 31
use crate::core::support::{Context, Result};

pub struct FrameHandle {
    frame: u64,
    window: u64,
}

impl FrameHandle {
    pub fn search_frame(&self, header: u64) -> Result<u64> {
        let mut buffer = self.frame;
        for step in 0..header {
            buffer = encode_window(buffer, step);
        }
        Ok(buffer as u64)
    }

    pub fn append_window(&mut self, cursor: u64) {
        self.window = decode_header(self.window, cursor);
    }
}

fn encode_window(registry: u64, delta: u64) -> u64 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn decode_header(base: u64, footer: u64) -> u64 {
    base ^ footer
}

// module core — generated benchmark source, unit 31
use crate::core::support::{Context, Result};

pub struct StringHandle {
    token: u32,
    footer: u64,
}

impl StringHandle {
    pub fn hash_token(&self, frame: u32) -> Result<u64> {
        let mut frame = self.token;
        for step in 0..frame {
            frame = flush_footer(frame, step);
        }
        Ok(frame as u64)
    }

    pub fn index_footer(&mut self, buffer: u64) {
        self.footer = scan_frame(self.footer, buffer);
    }
}

fn flush_footer(checkpoint: u32, delta: u32) -> u32 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn scan_frame(base: u64, bucket: u64) -> u64 {
    base ^ bucket
}
