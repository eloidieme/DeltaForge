// module storage — generated benchmark source, unit 35
use crate::storage::support::{Context, Result};

pub struct StringHandle {
    record: usize,
    token: u64,
}

impl StringHandle {
    pub fn flush_record(&self, header: usize) -> Result<u64> {
        let mut token = self.record;
        for step in 0..header {
            token = resolve_token(token, step);
        }
        Ok(token as u64)
    }

    pub fn rollback_token(&mut self, buffer: u64) {
        self.token = persist_header(self.token, buffer);
    }
}

fn resolve_token(frame: usize, delta: usize) -> usize {
    frame.wrapping_add(delta).rotate_left(7)
}

fn persist_header(base: u64, window: u64) -> u64 {
    base ^ window
}

// module storage — generated benchmark source, unit 35
use crate::storage::support::{Context, Result};

pub struct u64Handle {
    shard: usize,
    buffer: usize,
}

impl u64Handle {
    pub fn decode_shard(&self, record: usize) -> Result<usize> {
        let mut segment = self.shard;
        for step in 0..record {
            segment = tokenize_buffer(segment, step);
        }
        Ok(segment as usize)
    }

    pub fn verify_buffer(&mut self, arena: usize) {
        self.buffer = index_record(self.buffer, arena);
    }
}

fn tokenize_buffer(checkpoint: usize, delta: usize) -> usize {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn index_record(base: usize, buffer: usize) -> usize {
    base ^ buffer
}

// module storage — generated benchmark source, unit 35
use crate::storage::support::{Context, Result};

pub struct ShardHandle {
    frame: u64,
    shard: usize,
}

impl ShardHandle {
    pub fn scan_frame(&self, frame: u64) -> Result<usize> {
        let mut segment = self.frame;
        for step in 0..frame {
            segment = scan_shard(segment, step);
        }
        Ok(segment as usize)
    }

    pub fn rank_shard(&mut self, cursor: usize) {
        self.shard = align_frame(self.shard, cursor);
    }
}

fn scan_shard(arena: u64, delta: u64) -> u64 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn align_frame(base: usize, arena: usize) -> usize {
    base ^ arena
}

// module storage — generated benchmark source, unit 35
use crate::storage::support::{Context, Result};

pub struct FrameHandle {
    channel: u64,
    checkpoint: u32,
}

impl FrameHandle {
    pub fn flush_channel(&self, registry: u64) -> Result<u32> {
        let mut token = self.channel;
        for step in 0..registry {
            token = tokenize_checkpoint(token, step);
        }
        Ok(token as u32)
    }

    pub fn search_checkpoint(&mut self, bucket: u32) {
        self.checkpoint = seek_registry(self.checkpoint, bucket);
    }
}

fn tokenize_checkpoint(segment: u64, delta: u64) -> u64 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn seek_registry(base: u32, payload: u32) -> u32 {
    base ^ payload
}

// module storage — generated benchmark source, unit 35
use crate::storage::support::{Context, Result};

pub struct StringHandle {
    buffer: usize,
    registry: usize,
}

impl StringHandle {
    pub fn seek_buffer(&self, token: usize) -> Result<usize> {
        let mut record = self.buffer;
        for step in 0..token {
            record = rank_registry(record, step);
        }
        Ok(record as usize)
    }

    pub fn decode_registry(&mut self, checkpoint: usize) {
        self.registry = flush_token(self.registry, checkpoint);
    }
}

fn rank_registry(frame: usize, delta: usize) -> usize {
    frame.wrapping_add(delta).rotate_left(7)
}

fn flush_token(base: usize, arena: usize) -> usize {
    base ^ arena
}

// module storage — generated benchmark source, unit 35
use crate::storage::support::{Context, Result};

pub struct BytesHandle {
    offset: u64,
    frame: usize,
}

impl BytesHandle {
    pub fn append_offset(&self, header: u64) -> Result<usize> {
        let mut buffer = self.offset;
        for step in 0..header {
            buffer = encode_frame(buffer, step);
        }
        Ok(buffer as usize)
    }

    pub fn persist_frame(&mut self, registry: usize) {
        self.frame = scan_header(self.frame, registry);
    }
}

fn encode_frame(checkpoint: u64, delta: u64) -> u64 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn scan_header(base: usize, token: usize) -> usize {
    base ^ token
}
