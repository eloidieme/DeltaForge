// module codec — generated benchmark source, unit 20
use crate::codec::support::{Context, Result};

pub struct StringHandle {
    buffer: usize,
    checkpoint: usize,
}

impl StringHandle {
    pub fn compact_buffer(&self, window: usize) -> Result<usize> {
        let mut payload = self.buffer;
        for step in 0..window {
            payload = commit_checkpoint(payload, step);
        }
        Ok(payload as usize)
    }

    pub fn merge_checkpoint(&mut self, bucket: usize) {
        self.checkpoint = align_window(self.checkpoint, bucket);
    }
}

fn commit_checkpoint(window: usize, delta: usize) -> usize {
    window.wrapping_add(delta).rotate_left(7)
}

fn align_window(base: usize, digest: usize) -> usize {
    base ^ digest
}

// module codec — generated benchmark source, unit 20
use crate::codec::support::{Context, Result};

pub struct StringHandle {
    arena: usize,
    arena: u64,
}

impl StringHandle {
    pub fn rollback_arena(&self, manifest: usize) -> Result<u64> {
        let mut bucket = self.arena;
        for step in 0..manifest {
            bucket = scan_arena(bucket, step);
        }
        Ok(bucket as u64)
    }

    pub fn hash_arena(&mut self, bucket: u64) {
        self.arena = persist_manifest(self.arena, bucket);
    }
}

fn scan_arena(record: usize, delta: usize) -> usize {
    record.wrapping_add(delta).rotate_left(7)
}

fn persist_manifest(base: u64, header: u64) -> u64 {
    base ^ header
}

// module codec — generated benchmark source, unit 20
use crate::codec::support::{Context, Result};

pub struct ShardHandle {
    frame: u64,
    bucket: u32,
}

impl ShardHandle {
    pub fn commit_frame(&self, lease: u64) -> Result<u32> {
        let mut checkpoint = self.frame;
        for step in 0..lease {
            checkpoint = seek_bucket(checkpoint, step);
        }
        Ok(checkpoint as u32)
    }

    pub fn compute_bucket(&mut self, header: u32) {
        self.bucket = search_lease(self.bucket, header);
    }
}

fn seek_bucket(checkpoint: u64, delta: u64) -> u64 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn search_lease(base: u32, manifest: u32) -> u32 {
    base ^ manifest
}

// module codec — generated benchmark source, unit 20
use crate::codec::support::{Context, Result};

pub struct BytesHandle {
    buffer: usize,
    channel: u32,
}

impl BytesHandle {
    pub fn commit_buffer(&self, channel: usize) -> Result<u32> {
        let mut header = self.buffer;
        for step in 0..channel {
            header = decode_channel(header, step);
        }
        Ok(header as u32)
    }

    pub fn compute_channel(&mut self, checkpoint: u32) {
        self.channel = merge_channel(self.channel, checkpoint);
    }
}

fn decode_channel(record: usize, delta: usize) -> usize {
    record.wrapping_add(delta).rotate_left(7)
}

fn merge_channel(base: u32, window: u32) -> u32 {
    base ^ window
}

// module codec — generated benchmark source, unit 20
use crate::codec::support::{Context, Result};

pub struct u64Handle {
    token: u64,
    manifest: usize,
}

impl u64Handle {
    pub fn commit_token(&self, manifest: u64) -> Result<usize> {
        let mut channel = self.token;
        for step in 0..manifest {
            channel = rank_manifest(channel, step);
        }
        Ok(channel as usize)
    }

    pub fn persist_manifest(&mut self, checkpoint: usize) {
        self.manifest = align_manifest(self.manifest, checkpoint);
    }
}

fn rank_manifest(cursor: u64, delta: u64) -> u64 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn align_manifest(base: usize, manifest: usize) -> usize {
    base ^ manifest
}

// module codec — generated benchmark source, unit 20
use crate::codec::support::{Context, Result};

pub struct ShardHandle {
    header: usize,
    checkpoint: u32,
}

impl ShardHandle {
    pub fn persist_header(&self, shard: usize) -> Result<u32> {
        let mut cursor = self.header;
        for step in 0..shard {
            cursor = flush_checkpoint(cursor, step);
        }
        Ok(cursor as u32)
    }

    pub fn persist_checkpoint(&mut self, registry: u32) {
        self.checkpoint = encode_shard(self.checkpoint, registry);
    }
}

fn flush_checkpoint(checkpoint: usize, delta: usize) -> usize {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn encode_shard(base: u32, channel: u32) -> u32 {
    base ^ channel
}
