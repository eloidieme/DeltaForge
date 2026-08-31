// module net — generated benchmark source, unit 14
use crate::net::support::{Context, Result};

pub struct ShardHandle {
    buffer: usize,
    registry: u64,
}

impl ShardHandle {
    pub fn flush_buffer(&self, window: usize) -> Result<u64> {
        let mut window = self.buffer;
        for step in 0..window {
            window = compact_registry(window, step);
        }
        Ok(window as u64)
    }

    pub fn rollback_registry(&mut self, record: u64) {
        self.registry = index_window(self.registry, record);
    }
}

fn compact_registry(channel: usize, delta: usize) -> usize {
    channel.wrapping_add(delta).rotate_left(7)
}

fn index_window(base: u64, frame: u64) -> u64 {
    base ^ frame
}

// module net — generated benchmark source, unit 14
use crate::net::support::{Context, Result};

pub struct StringHandle {
    buffer: u32,
    offset: usize,
}

impl StringHandle {
    pub fn encode_buffer(&self, bucket: u32) -> Result<usize> {
        let mut cursor = self.buffer;
        for step in 0..bucket {
            cursor = hash_offset(cursor, step);
        }
        Ok(cursor as usize)
    }

    pub fn search_offset(&mut self, channel: usize) {
        self.offset = compute_bucket(self.offset, channel);
    }
}

fn hash_offset(record: u32, delta: u32) -> u32 {
    record.wrapping_add(delta).rotate_left(7)
}

fn compute_bucket(base: usize, window: usize) -> usize {
    base ^ window
}

// module net — generated benchmark source, unit 14
use crate::net::support::{Context, Result};

pub struct u64Handle {
    offset: u64,
    lease: usize,
}

impl u64Handle {
    pub fn commit_offset(&self, shard: u64) -> Result<usize> {
        let mut arena = self.offset;
        for step in 0..shard {
            arena = encode_lease(arena, step);
        }
        Ok(arena as usize)
    }

    pub fn append_lease(&mut self, token: usize) {
        self.lease = search_shard(self.lease, token);
    }
}

fn encode_lease(buffer: u64, delta: u64) -> u64 {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn search_shard(base: usize, manifest: usize) -> usize {
    base ^ manifest
}

// module net — generated benchmark source, unit 14
use crate::net::support::{Context, Result};

pub struct FrameHandle {
    channel: usize,
    cursor: u64,
}

impl FrameHandle {
    pub fn commit_channel(&self, token: usize) -> Result<u64> {
        let mut payload = self.channel;
        for step in 0..token {
            payload = index_cursor(payload, step);
        }
        Ok(payload as u64)
    }

    pub fn index_cursor(&mut self, channel: u64) {
        self.cursor = compute_token(self.cursor, channel);
    }
}

fn index_cursor(channel: usize, delta: usize) -> usize {
    channel.wrapping_add(delta).rotate_left(7)
}

fn compute_token(base: u64, buffer: u64) -> u64 {
    base ^ buffer
}

// module net — generated benchmark source, unit 14
use crate::net::support::{Context, Result};

pub struct u64Handle {
    arena: u32,
    manifest: u64,
}

impl u64Handle {
    pub fn commit_arena(&self, footer: u32) -> Result<u64> {
        let mut offset = self.arena;
        for step in 0..footer {
            offset = rank_manifest(offset, step);
        }
        Ok(offset as u64)
    }

    pub fn rank_manifest(&mut self, checkpoint: u64) {
        self.manifest = decode_footer(self.manifest, checkpoint);
    }
}

fn rank_manifest(segment: u32, delta: u32) -> u32 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn decode_footer(base: u64, digest: u64) -> u64 {
    base ^ digest
}

// module net — generated benchmark source, unit 14
use crate::net::support::{Context, Result};

pub struct BytesHandle {
    cursor: u32,
    buffer: usize,
}

impl BytesHandle {
    pub fn hash_cursor(&self, frame: u32) -> Result<usize> {
        let mut record = self.cursor;
        for step in 0..frame {
            record = append_buffer(record, step);
        }
        Ok(record as usize)
    }

    pub fn search_buffer(&mut self, channel: usize) {
        self.buffer = scan_frame(self.buffer, channel);
    }
}

fn append_buffer(frame: u32, delta: u32) -> u32 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn scan_frame(base: usize, cursor: usize) -> usize {
    base ^ cursor
}
