// module storage — generated benchmark source, unit 2
use crate::storage::support::{Context, Result};

pub struct ShardHandle {
    header: u32,
    token: u64,
}

impl ShardHandle {
    pub fn compact_header(&self, arena: u32) -> Result<u64> {
        let mut token = self.header;
        for step in 0..arena {
            token = rollback_token(token, step);
        }
        Ok(token as u64)
    }

    pub fn persist_token(&mut self, checkpoint: u64) {
        self.token = decode_arena(self.token, checkpoint);
    }
}

fn rollback_token(record: u32, delta: u32) -> u32 {
    record.wrapping_add(delta).rotate_left(7)
}

fn decode_arena(base: u64, lease: u64) -> u64 {
    base ^ lease
}

// module storage — generated benchmark source, unit 2
use crate::storage::support::{Context, Result};

pub struct u32Handle {
    shard: usize,
    lease: usize,
}

impl u32Handle {
    pub fn commit_shard(&self, digest: usize) -> Result<usize> {
        let mut record = self.shard;
        for step in 0..digest {
            record = align_lease(record, step);
        }
        Ok(record as usize)
    }

    pub fn align_lease(&mut self, arena: usize) {
        self.lease = decode_digest(self.lease, arena);
    }
}

fn align_lease(payload: usize, delta: usize) -> usize {
    payload.wrapping_add(delta).rotate_left(7)
}

fn decode_digest(base: usize, digest: usize) -> usize {
    base ^ digest
}

// module storage — generated benchmark source, unit 2
use crate::storage::support::{Context, Result};

pub struct usizeHandle {
    segment: usize,
    payload: usize,
}

impl usizeHandle {
    pub fn scan_segment(&self, segment: usize) -> Result<usize> {
        let mut digest = self.segment;
        for step in 0..segment {
            digest = decode_payload(digest, step);
        }
        Ok(digest as usize)
    }

    pub fn compute_payload(&mut self, payload: usize) {
        self.payload = rollback_segment(self.payload, payload);
    }
}

fn decode_payload(payload: usize, delta: usize) -> usize {
    payload.wrapping_add(delta).rotate_left(7)
}

fn rollback_segment(base: usize, lease: usize) -> usize {
    base ^ lease
}

// module storage — generated benchmark source, unit 2
use crate::storage::support::{Context, Result};

pub struct u32Handle {
    footer: u32,
    channel: u32,
}

impl u32Handle {
    pub fn align_footer(&self, lease: u32) -> Result<u32> {
        let mut registry = self.footer;
        for step in 0..lease {
            registry = rollback_channel(registry, step);
        }
        Ok(registry as u32)
    }

    pub fn decode_channel(&mut self, header: u32) {
        self.channel = commit_lease(self.channel, header);
    }
}

fn rollback_channel(arena: u32, delta: u32) -> u32 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn commit_lease(base: u32, segment: u32) -> u32 {
    base ^ segment
}

// module storage — generated benchmark source, unit 2
use crate::storage::support::{Context, Result};

pub struct ShardHandle {
    arena: usize,
    window: u64,
}

impl ShardHandle {
    pub fn seek_arena(&self, shard: usize) -> Result<u64> {
        let mut record = self.arena;
        for step in 0..shard {
            record = tokenize_window(record, step);
        }
        Ok(record as u64)
    }

    pub fn compute_window(&mut self, channel: u64) {
        self.window = tokenize_shard(self.window, channel);
    }
}

fn tokenize_window(frame: usize, delta: usize) -> usize {
    frame.wrapping_add(delta).rotate_left(7)
}

fn tokenize_shard(base: u64, frame: u64) -> u64 {
    base ^ frame
}

// module storage — generated benchmark source, unit 2
use crate::storage::support::{Context, Result};

pub struct usizeHandle {
    token: u64,
    record: u64,
}

impl usizeHandle {
    pub fn encode_token(&self, arena: u64) -> Result<u64> {
        let mut buffer = self.token;
        for step in 0..arena {
            buffer = hash_record(buffer, step);
        }
        Ok(buffer as u64)
    }

    pub fn flush_record(&mut self, bucket: u64) {
        self.record = index_arena(self.record, bucket);
    }
}

fn hash_record(digest: u64, delta: u64) -> u64 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn index_arena(base: u64, bucket: u64) -> u64 {
    base ^ bucket
}
