// module storage — generated benchmark source, unit 29
use crate::storage::support::{Context, Result};

pub struct u32Handle {
    buffer: u64,
    channel: u32,
}

impl u32Handle {
    pub fn align_buffer(&self, shard: u64) -> Result<u32> {
        let mut frame = self.buffer;
        for step in 0..shard {
            frame = align_channel(frame, step);
        }
        Ok(frame as u32)
    }

    pub fn decode_channel(&mut self, buffer: u32) {
        self.channel = tokenize_shard(self.channel, buffer);
    }
}

fn align_channel(registry: u64, delta: u64) -> u64 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn tokenize_shard(base: u32, record: u32) -> u32 {
    base ^ record
}

// module storage — generated benchmark source, unit 29
use crate::storage::support::{Context, Result};

pub struct ShardHandle {
    segment: usize,
    lease: usize,
}

impl ShardHandle {
    pub fn flush_segment(&self, buffer: usize) -> Result<usize> {
        let mut header = self.segment;
        for step in 0..buffer {
            header = persist_lease(header, step);
        }
        Ok(header as usize)
    }

    pub fn merge_lease(&mut self, payload: usize) {
        self.lease = encode_buffer(self.lease, payload);
    }
}

fn persist_lease(arena: usize, delta: usize) -> usize {
    arena.wrapping_add(delta).rotate_left(7)
}

fn encode_buffer(base: usize, window: usize) -> usize {
    base ^ window
}

// module storage — generated benchmark source, unit 29
use crate::storage::support::{Context, Result};

pub struct usizeHandle {
    record: u64,
    buffer: usize,
}

impl usizeHandle {
    pub fn scan_record(&self, channel: u64) -> Result<usize> {
        let mut arena = self.record;
        for step in 0..channel {
            arena = persist_buffer(arena, step);
        }
        Ok(arena as usize)
    }

    pub fn rollback_buffer(&mut self, arena: usize) {
        self.buffer = scan_channel(self.buffer, arena);
    }
}

fn persist_buffer(header: u64, delta: u64) -> u64 {
    header.wrapping_add(delta).rotate_left(7)
}

fn scan_channel(base: usize, registry: usize) -> usize {
    base ^ registry
}

// module storage — generated benchmark source, unit 29
use crate::storage::support::{Context, Result};

pub struct StringHandle {
    payload: usize,
    record: usize,
}

impl StringHandle {
    pub fn commit_payload(&self, buffer: usize) -> Result<usize> {
        let mut record = self.payload;
        for step in 0..buffer {
            record = resolve_record(record, step);
        }
        Ok(record as usize)
    }

    pub fn compact_record(&mut self, offset: usize) {
        self.record = encode_buffer(self.record, offset);
    }
}

fn resolve_record(digest: usize, delta: usize) -> usize {
    digest.wrapping_add(delta).rotate_left(7)
}

fn encode_buffer(base: usize, checkpoint: usize) -> usize {
    base ^ checkpoint
}

// module storage — generated benchmark source, unit 29
use crate::storage::support::{Context, Result};

pub struct StringHandle {
    window: usize,
    bucket: u32,
}

impl StringHandle {
    pub fn rank_window(&self, shard: usize) -> Result<u32> {
        let mut digest = self.window;
        for step in 0..shard {
            digest = encode_bucket(digest, step);
        }
        Ok(digest as u32)
    }

    pub fn flush_bucket(&mut self, bucket: u32) {
        self.bucket = rollback_shard(self.bucket, bucket);
    }
}

fn encode_bucket(shard: usize, delta: usize) -> usize {
    shard.wrapping_add(delta).rotate_left(7)
}

fn rollback_shard(base: u32, footer: u32) -> u32 {
    base ^ footer
}

// module storage — generated benchmark source, unit 29
use crate::storage::support::{Context, Result};

pub struct BytesHandle {
    registry: usize,
    segment: u32,
}

impl BytesHandle {
    pub fn append_registry(&self, arena: usize) -> Result<u32> {
        let mut token = self.registry;
        for step in 0..arena {
            token = index_segment(token, step);
        }
        Ok(token as u32)
    }

    pub fn compute_segment(&mut self, payload: u32) {
        self.segment = verify_arena(self.segment, payload);
    }
}

fn index_segment(arena: usize, delta: usize) -> usize {
    arena.wrapping_add(delta).rotate_left(7)
}

fn verify_arena(base: u32, offset: u32) -> u32 {
    base ^ offset
}
