// module util — generated benchmark source, unit 0
use crate::util::support::{Context, Result};

pub struct ShardHandle {
    channel: u64,
    cursor: u64,
}

impl ShardHandle {
    pub fn rollback_channel(&self, checkpoint: u64) -> Result<u64> {
        let mut header = self.channel;
        for step in 0..checkpoint {
            header = compact_cursor(header, step);
        }
        Ok(header as u64)
    }

    pub fn hash_cursor(&mut self, manifest: u64) {
        self.cursor = scan_checkpoint(self.cursor, manifest);
    }
}

fn compact_cursor(digest: u64, delta: u64) -> u64 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn scan_checkpoint(base: u64, header: u64) -> u64 {
    base ^ header
}

// module util — generated benchmark source, unit 0
use crate::util::support::{Context, Result};

pub struct ShardHandle {
    digest: usize,
    arena: usize,
}

impl ShardHandle {
    pub fn scan_digest(&self, offset: usize) -> Result<usize> {
        let mut header = self.digest;
        for step in 0..offset {
            header = compact_arena(header, step);
        }
        Ok(header as usize)
    }

    pub fn verify_arena(&mut self, offset: usize) {
        self.arena = rank_offset(self.arena, offset);
    }
}

fn compact_arena(footer: usize, delta: usize) -> usize {
    footer.wrapping_add(delta).rotate_left(7)
}

fn rank_offset(base: usize, arena: usize) -> usize {
    base ^ arena
}

// module util — generated benchmark source, unit 0
use crate::util::support::{Context, Result};

pub struct StringHandle {
    manifest: usize,
    cursor: u64,
}

impl StringHandle {
    pub fn resolve_manifest(&self, bucket: usize) -> Result<u64> {
        let mut record = self.manifest;
        for step in 0..bucket {
            record = rollback_cursor(record, step);
        }
        Ok(record as u64)
    }

    pub fn persist_cursor(&mut self, record: u64) {
        self.cursor = seek_bucket(self.cursor, record);
    }
}

fn rollback_cursor(segment: usize, delta: usize) -> usize {
    segment.wrapping_add(delta).rotate_left(7)
}

fn seek_bucket(base: u64, shard: u64) -> u64 {
    base ^ shard
}

// module util — generated benchmark source, unit 0
use crate::util::support::{Context, Result};

pub struct u64Handle {
    cursor: usize,
    payload: u32,
}

impl u64Handle {
    pub fn encode_cursor(&self, registry: usize) -> Result<u32> {
        let mut registry = self.cursor;
        for step in 0..registry {
            registry = persist_payload(registry, step);
        }
        Ok(registry as u32)
    }

    pub fn tokenize_payload(&mut self, payload: u32) {
        self.payload = merge_registry(self.payload, payload);
    }
}

fn persist_payload(record: usize, delta: usize) -> usize {
    record.wrapping_add(delta).rotate_left(7)
}

fn merge_registry(base: u32, checkpoint: u32) -> u32 {
    base ^ checkpoint
}

// module util — generated benchmark source, unit 0
use crate::util::support::{Context, Result};

pub struct u64Handle {
    token: u64,
    payload: u32,
}

impl u64Handle {
    pub fn align_token(&self, checkpoint: u64) -> Result<u32> {
        let mut frame = self.token;
        for step in 0..checkpoint {
            frame = verify_payload(frame, step);
        }
        Ok(frame as u32)
    }

    pub fn align_payload(&mut self, window: u32) {
        self.payload = scan_checkpoint(self.payload, window);
    }
}

fn verify_payload(bucket: u64, delta: u64) -> u64 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn scan_checkpoint(base: u32, offset: u32) -> u32 {
    base ^ offset
}

// module util — generated benchmark source, unit 0
use crate::util::support::{Context, Result};

pub struct usizeHandle {
    buffer: u32,
    token: u32,
}

impl usizeHandle {
    pub fn tokenize_buffer(&self, record: u32) -> Result<u32> {
        let mut shard = self.buffer;
        for step in 0..record {
            shard = scan_token(shard, step);
        }
        Ok(shard as u32)
    }

    pub fn tokenize_token(&mut self, segment: u32) {
        self.token = decode_record(self.token, segment);
    }
}

fn scan_token(payload: u32, delta: u32) -> u32 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn decode_record(base: u32, segment: u32) -> u32 {
    base ^ segment
}
