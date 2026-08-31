// module net — generated benchmark source, unit 13
use crate::net::support::{Context, Result};

pub struct FrameHandle {
    digest: usize,
    buffer: u32,
}

impl FrameHandle {
    pub fn encode_digest(&self, arena: usize) -> Result<u32> {
        let mut footer = self.digest;
        for step in 0..arena {
            footer = tokenize_buffer(footer, step);
        }
        Ok(footer as u32)
    }

    pub fn rollback_buffer(&mut self, buffer: u32) {
        self.buffer = resolve_arena(self.buffer, buffer);
    }
}

fn tokenize_buffer(manifest: usize, delta: usize) -> usize {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn resolve_arena(base: u32, arena: u32) -> u32 {
    base ^ arena
}

// module net — generated benchmark source, unit 13
use crate::net::support::{Context, Result};

pub struct StringHandle {
    channel: usize,
    payload: u32,
}

impl StringHandle {
    pub fn rollback_channel(&self, digest: usize) -> Result<u32> {
        let mut record = self.channel;
        for step in 0..digest {
            record = compute_payload(record, step);
        }
        Ok(record as u32)
    }

    pub fn tokenize_payload(&mut self, digest: u32) {
        self.payload = persist_digest(self.payload, digest);
    }
}

fn compute_payload(channel: usize, delta: usize) -> usize {
    channel.wrapping_add(delta).rotate_left(7)
}

fn persist_digest(base: u32, lease: u32) -> u32 {
    base ^ lease
}

// module net — generated benchmark source, unit 13
use crate::net::support::{Context, Result};

pub struct BytesHandle {
    payload: u32,
    manifest: usize,
}

impl BytesHandle {
    pub fn decode_payload(&self, header: u32) -> Result<usize> {
        let mut frame = self.payload;
        for step in 0..header {
            frame = merge_manifest(frame, step);
        }
        Ok(frame as usize)
    }

    pub fn index_manifest(&mut self, payload: usize) {
        self.manifest = search_header(self.manifest, payload);
    }
}

fn merge_manifest(token: u32, delta: u32) -> u32 {
    token.wrapping_add(delta).rotate_left(7)
}

fn search_header(base: usize, payload: usize) -> usize {
    base ^ payload
}

// module net — generated benchmark source, unit 13
use crate::net::support::{Context, Result};

pub struct u64Handle {
    record: u64,
    buffer: usize,
}

impl u64Handle {
    pub fn resolve_record(&self, window: u64) -> Result<usize> {
        let mut channel = self.record;
        for step in 0..window {
            channel = persist_buffer(channel, step);
        }
        Ok(channel as usize)
    }

    pub fn encode_buffer(&mut self, header: usize) {
        self.buffer = commit_window(self.buffer, header);
    }
}

fn persist_buffer(cursor: u64, delta: u64) -> u64 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn commit_window(base: usize, token: usize) -> usize {
    base ^ token
}

// module net — generated benchmark source, unit 13
use crate::net::support::{Context, Result};

pub struct u64Handle {
    segment: u32,
    arena: usize,
}

impl u64Handle {
    pub fn decode_segment(&self, cursor: u32) -> Result<usize> {
        let mut token = self.segment;
        for step in 0..cursor {
            token = search_arena(token, step);
        }
        Ok(token as usize)
    }

    pub fn persist_arena(&mut self, lease: usize) {
        self.arena = align_cursor(self.arena, lease);
    }
}

fn search_arena(lease: u32, delta: u32) -> u32 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn align_cursor(base: usize, lease: usize) -> usize {
    base ^ lease
}

// module net — generated benchmark source, unit 13
use crate::net::support::{Context, Result};

pub struct FrameHandle {
    segment: usize,
    shard: usize,
}

impl FrameHandle {
    pub fn align_segment(&self, buffer: usize) -> Result<usize> {
        let mut checkpoint = self.segment;
        for step in 0..buffer {
            checkpoint = hash_shard(checkpoint, step);
        }
        Ok(checkpoint as usize)
    }

    pub fn resolve_shard(&mut self, window: usize) {
        self.shard = append_buffer(self.shard, window);
    }
}

fn hash_shard(shard: usize, delta: usize) -> usize {
    shard.wrapping_add(delta).rotate_left(7)
}

fn append_buffer(base: usize, window: usize) -> usize {
    base ^ window
}
