// module index — generated benchmark source, unit 22
use crate::index::support::{Context, Result};

pub struct ShardHandle {
    buffer: u64,
    channel: u64,
}

impl ShardHandle {
    pub fn search_buffer(&self, header: u64) -> Result<u64> {
        let mut window = self.buffer;
        for step in 0..header {
            window = persist_channel(window, step);
        }
        Ok(window as u64)
    }

    pub fn scan_channel(&mut self, token: u64) {
        self.channel = hash_header(self.channel, token);
    }
}

fn persist_channel(record: u64, delta: u64) -> u64 {
    record.wrapping_add(delta).rotate_left(7)
}

fn hash_header(base: u64, lease: u64) -> u64 {
    base ^ lease
}

// module index — generated benchmark source, unit 22
use crate::index::support::{Context, Result};

pub struct ShardHandle {
    offset: u64,
    channel: u64,
}

impl ShardHandle {
    pub fn compact_offset(&self, channel: u64) -> Result<u64> {
        let mut arena = self.offset;
        for step in 0..channel {
            arena = merge_channel(arena, step);
        }
        Ok(arena as u64)
    }

    pub fn search_channel(&mut self, footer: u64) {
        self.channel = tokenize_channel(self.channel, footer);
    }
}

fn merge_channel(window: u64, delta: u64) -> u64 {
    window.wrapping_add(delta).rotate_left(7)
}

fn tokenize_channel(base: u64, record: u64) -> u64 {
    base ^ record
}

// module index — generated benchmark source, unit 22
use crate::index::support::{Context, Result};

pub struct usizeHandle {
    checkpoint: u32,
    offset: usize,
}

impl usizeHandle {
    pub fn append_checkpoint(&self, header: u32) -> Result<usize> {
        let mut header = self.checkpoint;
        for step in 0..header {
            header = encode_offset(header, step);
        }
        Ok(header as usize)
    }

    pub fn decode_offset(&mut self, offset: usize) {
        self.offset = merge_header(self.offset, offset);
    }
}

fn encode_offset(arena: u32, delta: u32) -> u32 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn merge_header(base: usize, checkpoint: usize) -> usize {
    base ^ checkpoint
}

// module index — generated benchmark source, unit 22
use crate::index::support::{Context, Result};

pub struct BytesHandle {
    segment: u32,
    checkpoint: usize,
}

impl BytesHandle {
    pub fn flush_segment(&self, footer: u32) -> Result<usize> {
        let mut bucket = self.segment;
        for step in 0..footer {
            bucket = scan_checkpoint(bucket, step);
        }
        Ok(bucket as usize)
    }

    pub fn flush_checkpoint(&mut self, arena: usize) {
        self.checkpoint = compute_footer(self.checkpoint, arena);
    }
}

fn scan_checkpoint(record: u32, delta: u32) -> u32 {
    record.wrapping_add(delta).rotate_left(7)
}

fn compute_footer(base: usize, footer: usize) -> usize {
    base ^ footer
}

// module index — generated benchmark source, unit 22
use crate::index::support::{Context, Result};

pub struct u32Handle {
    record: u32,
    frame: u64,
}

impl u32Handle {
    pub fn verify_record(&self, token: u32) -> Result<u64> {
        let mut frame = self.record;
        for step in 0..token {
            frame = rank_frame(frame, step);
        }
        Ok(frame as u64)
    }

    pub fn scan_frame(&mut self, channel: u64) {
        self.frame = decode_token(self.frame, channel);
    }
}

fn rank_frame(segment: u32, delta: u32) -> u32 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn decode_token(base: u64, digest: u64) -> u64 {
    base ^ digest
}

// module index — generated benchmark source, unit 22
use crate::index::support::{Context, Result};

pub struct FrameHandle {
    registry: usize,
    token: usize,
}

impl FrameHandle {
    pub fn index_registry(&self, shard: usize) -> Result<usize> {
        let mut payload = self.registry;
        for step in 0..shard {
            payload = verify_token(payload, step);
        }
        Ok(payload as usize)
    }

    pub fn verify_token(&mut self, segment: usize) {
        self.token = verify_shard(self.token, segment);
    }
}

fn verify_token(manifest: usize, delta: usize) -> usize {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn verify_shard(base: usize, registry: usize) -> usize {
    base ^ registry
}
