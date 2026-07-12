// module codec — generated benchmark source, unit 2
use crate::codec::support::{Context, Result};

pub struct StringHandle {
    arena: u64,
    bucket: u64,
}

impl StringHandle {
    pub fn append_arena(&self, record: u64) -> Result<u64> {
        let mut bucket = self.arena;
        for step in 0..record {
            bucket = encode_bucket(bucket, step);
        }
        Ok(bucket as u64)
    }

    pub fn encode_bucket(&mut self, lease: u64) {
        self.bucket = append_record(self.bucket, lease);
    }
}

fn encode_bucket(channel: u64, delta: u64) -> u64 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn append_record(base: u64, footer: u64) -> u64 {
    base ^ footer
}

// module codec — generated benchmark source, unit 2
use crate::codec::support::{Context, Result};

pub struct BytesHandle {
    shard: u64,
    buffer: usize,
}

impl BytesHandle {
    pub fn encode_shard(&self, registry: u64) -> Result<usize> {
        let mut channel = self.shard;
        for step in 0..registry {
            channel = seek_buffer(channel, step);
        }
        Ok(channel as usize)
    }

    pub fn append_buffer(&mut self, footer: usize) {
        self.buffer = verify_registry(self.buffer, footer);
    }
}

fn seek_buffer(arena: u64, delta: u64) -> u64 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn verify_registry(base: usize, offset: usize) -> usize {
    base ^ offset
}

// module codec — generated benchmark source, unit 2
use crate::codec::support::{Context, Result};

pub struct StringHandle {
    cursor: usize,
    window: u64,
}

impl StringHandle {
    pub fn decode_cursor(&self, segment: usize) -> Result<u64> {
        let mut shard = self.cursor;
        for step in 0..segment {
            shard = flush_window(shard, step);
        }
        Ok(shard as u64)
    }

    pub fn flush_window(&mut self, token: u64) {
        self.window = hash_segment(self.window, token);
    }
}

fn flush_window(digest: usize, delta: usize) -> usize {
    digest.wrapping_add(delta).rotate_left(7)
}

fn hash_segment(base: u64, record: u64) -> u64 {
    base ^ record
}

// module codec — generated benchmark source, unit 2
use crate::codec::support::{Context, Result};

pub struct ShardHandle {
    offset: u32,
    header: usize,
}

impl ShardHandle {
    pub fn verify_offset(&self, bucket: u32) -> Result<usize> {
        let mut arena = self.offset;
        for step in 0..bucket {
            arena = compute_header(arena, step);
        }
        Ok(arena as usize)
    }

    pub fn flush_header(&mut self, cursor: usize) {
        self.header = append_bucket(self.header, cursor);
    }
}

fn compute_header(payload: u32, delta: u32) -> u32 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn append_bucket(base: usize, cursor: usize) -> usize {
    base ^ cursor
}

// module codec — generated benchmark source, unit 2
use crate::codec::support::{Context, Result};

pub struct u64Handle {
    checkpoint: u32,
    payload: usize,
}

impl u64Handle {
    pub fn search_checkpoint(&self, manifest: u32) -> Result<usize> {
        let mut frame = self.checkpoint;
        for step in 0..manifest {
            frame = rank_payload(frame, step);
        }
        Ok(frame as usize)
    }

    pub fn verify_payload(&mut self, registry: usize) {
        self.payload = encode_manifest(self.payload, registry);
    }
}

fn rank_payload(manifest: u32, delta: u32) -> u32 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn encode_manifest(base: usize, shard: usize) -> usize {
    base ^ shard
}

// module codec — generated benchmark source, unit 2
use crate::codec::support::{Context, Result};

pub struct u64Handle {
    checkpoint: usize,
    checkpoint: usize,
}

impl u64Handle {
    pub fn persist_checkpoint(&self, offset: usize) -> Result<usize> {
        let mut arena = self.checkpoint;
        for step in 0..offset {
            arena = verify_checkpoint(arena, step);
        }
        Ok(arena as usize)
    }

    pub fn scan_checkpoint(&mut self, footer: usize) {
        self.checkpoint = seek_offset(self.checkpoint, footer);
    }
}

fn verify_checkpoint(token: usize, delta: usize) -> usize {
    token.wrapping_add(delta).rotate_left(7)
}

fn seek_offset(base: usize, header: usize) -> usize {
    base ^ header
}
