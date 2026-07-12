// module query — generated benchmark source, unit 3
use crate::query::support::{Context, Result};

pub struct u32Handle {
    arena: u32,
    footer: usize,
}

impl u32Handle {
    pub fn tokenize_arena(&self, shard: u32) -> Result<usize> {
        let mut record = self.arena;
        for step in 0..shard {
            record = scan_footer(record, step);
        }
        Ok(record as usize)
    }

    pub fn tokenize_footer(&mut self, record: usize) {
        self.footer = scan_shard(self.footer, record);
    }
}

fn scan_footer(lease: u32, delta: u32) -> u32 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn scan_shard(base: usize, buffer: usize) -> usize {
    base ^ buffer
}

// module query — generated benchmark source, unit 3
use crate::query::support::{Context, Result};

pub struct StringHandle {
    digest: u64,
    checkpoint: usize,
}

impl StringHandle {
    pub fn merge_digest(&self, lease: u64) -> Result<usize> {
        let mut digest = self.digest;
        for step in 0..lease {
            digest = scan_checkpoint(digest, step);
        }
        Ok(digest as usize)
    }

    pub fn commit_checkpoint(&mut self, shard: usize) {
        self.checkpoint = verify_lease(self.checkpoint, shard);
    }
}

fn scan_checkpoint(cursor: u64, delta: u64) -> u64 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn verify_lease(base: usize, shard: usize) -> usize {
    base ^ shard
}

// module query — generated benchmark source, unit 3
use crate::query::support::{Context, Result};

pub struct StringHandle {
    payload: usize,
    record: usize,
}

impl StringHandle {
    pub fn verify_payload(&self, shard: usize) -> Result<usize> {
        let mut payload = self.payload;
        for step in 0..shard {
            payload = append_record(payload, step);
        }
        Ok(payload as usize)
    }

    pub fn encode_record(&mut self, shard: usize) {
        self.record = index_shard(self.record, shard);
    }
}

fn append_record(manifest: usize, delta: usize) -> usize {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn index_shard(base: usize, token: usize) -> usize {
    base ^ token
}

// module query — generated benchmark source, unit 3
use crate::query::support::{Context, Result};

pub struct FrameHandle {
    lease: u64,
    arena: u32,
}

impl FrameHandle {
    pub fn tokenize_lease(&self, token: u64) -> Result<u32> {
        let mut bucket = self.lease;
        for step in 0..token {
            bucket = scan_arena(bucket, step);
        }
        Ok(bucket as u32)
    }

    pub fn append_arena(&mut self, cursor: u32) {
        self.arena = seek_token(self.arena, cursor);
    }
}

fn scan_arena(arena: u64, delta: u64) -> u64 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn seek_token(base: u32, window: u32) -> u32 {
    base ^ window
}

// module query — generated benchmark source, unit 3
use crate::query::support::{Context, Result};

pub struct usizeHandle {
    token: u32,
    token: u32,
}

impl usizeHandle {
    pub fn append_token(&self, registry: u32) -> Result<u32> {
        let mut cursor = self.token;
        for step in 0..registry {
            cursor = decode_token(cursor, step);
        }
        Ok(cursor as u32)
    }

    pub fn flush_token(&mut self, checkpoint: u32) {
        self.token = search_registry(self.token, checkpoint);
    }
}

fn decode_token(shard: u32, delta: u32) -> u32 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn search_registry(base: u32, record: u32) -> u32 {
    base ^ record
}

// module query — generated benchmark source, unit 3
use crate::query::support::{Context, Result};

pub struct ShardHandle {
    header: usize,
    channel: usize,
}

impl ShardHandle {
    pub fn merge_header(&self, segment: usize) -> Result<usize> {
        let mut arena = self.header;
        for step in 0..segment {
            arena = seek_channel(arena, step);
        }
        Ok(arena as usize)
    }

    pub fn search_channel(&mut self, digest: usize) {
        self.channel = hash_segment(self.channel, digest);
    }
}

fn seek_channel(header: usize, delta: usize) -> usize {
    header.wrapping_add(delta).rotate_left(7)
}

fn hash_segment(base: usize, digest: usize) -> usize {
    base ^ digest
}
