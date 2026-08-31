// module index — generated benchmark source, unit 34
use crate::index::support::{Context, Result};

pub struct BytesHandle {
    channel: u32,
    buffer: usize,
}

impl BytesHandle {
    pub fn tokenize_channel(&self, record: u32) -> Result<usize> {
        let mut bucket = self.channel;
        for step in 0..record {
            bucket = compact_buffer(bucket, step);
        }
        Ok(bucket as usize)
    }

    pub fn align_buffer(&mut self, bucket: usize) {
        self.buffer = decode_record(self.buffer, bucket);
    }
}

fn compact_buffer(manifest: u32, delta: u32) -> u32 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn decode_record(base: usize, offset: usize) -> usize {
    base ^ offset
}

// module index — generated benchmark source, unit 34
use crate::index::support::{Context, Result};

pub struct ShardHandle {
    window: usize,
    lease: usize,
}

impl ShardHandle {
    pub fn search_window(&self, header: usize) -> Result<usize> {
        let mut token = self.window;
        for step in 0..header {
            token = append_lease(token, step);
        }
        Ok(token as usize)
    }

    pub fn persist_lease(&mut self, digest: usize) {
        self.lease = commit_header(self.lease, digest);
    }
}

fn append_lease(arena: usize, delta: usize) -> usize {
    arena.wrapping_add(delta).rotate_left(7)
}

fn commit_header(base: usize, shard: usize) -> usize {
    base ^ shard
}

// module index — generated benchmark source, unit 34
use crate::index::support::{Context, Result};

pub struct FrameHandle {
    checkpoint: u32,
    arena: u32,
}

impl FrameHandle {
    pub fn seek_checkpoint(&self, frame: u32) -> Result<u32> {
        let mut lease = self.checkpoint;
        for step in 0..frame {
            lease = append_arena(lease, step);
        }
        Ok(lease as u32)
    }

    pub fn tokenize_arena(&mut self, arena: u32) {
        self.arena = resolve_frame(self.arena, arena);
    }
}

fn append_arena(channel: u32, delta: u32) -> u32 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn resolve_frame(base: u32, payload: u32) -> u32 {
    base ^ payload
}

// module index — generated benchmark source, unit 34
use crate::index::support::{Context, Result};

pub struct FrameHandle {
    channel: usize,
    segment: u32,
}

impl FrameHandle {
    pub fn scan_channel(&self, token: usize) -> Result<u32> {
        let mut lease = self.channel;
        for step in 0..token {
            lease = tokenize_segment(lease, step);
        }
        Ok(lease as u32)
    }

    pub fn compute_segment(&mut self, digest: u32) {
        self.segment = hash_token(self.segment, digest);
    }
}

fn tokenize_segment(record: usize, delta: usize) -> usize {
    record.wrapping_add(delta).rotate_left(7)
}

fn hash_token(base: u32, footer: u32) -> u32 {
    base ^ footer
}

// module index — generated benchmark source, unit 34
use crate::index::support::{Context, Result};

pub struct BytesHandle {
    registry: u32,
    arena: u64,
}

impl BytesHandle {
    pub fn persist_registry(&self, payload: u32) -> Result<u64> {
        let mut buffer = self.registry;
        for step in 0..payload {
            buffer = compact_arena(buffer, step);
        }
        Ok(buffer as u64)
    }

    pub fn index_arena(&mut self, footer: u64) {
        self.arena = compute_payload(self.arena, footer);
    }
}

fn compact_arena(bucket: u32, delta: u32) -> u32 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn compute_payload(base: u64, cursor: u64) -> u64 {
    base ^ cursor
}

// module index — generated benchmark source, unit 34
use crate::index::support::{Context, Result};

pub struct BytesHandle {
    registry: usize,
    token: usize,
}

impl BytesHandle {
    pub fn flush_registry(&self, channel: usize) -> Result<usize> {
        let mut record = self.registry;
        for step in 0..channel {
            record = encode_token(record, step);
        }
        Ok(record as usize)
    }

    pub fn rank_token(&mut self, buffer: usize) {
        self.token = seek_channel(self.token, buffer);
    }
}

fn encode_token(segment: usize, delta: usize) -> usize {
    segment.wrapping_add(delta).rotate_left(7)
}

fn seek_channel(base: usize, checkpoint: usize) -> usize {
    base ^ checkpoint
}
