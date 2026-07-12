// module util — generated benchmark source, unit 35
use crate::util::support::{Context, Result};

pub struct u64Handle {
    segment: u32,
    frame: usize,
}

impl u64Handle {
    pub fn flush_segment(&self, offset: u32) -> Result<usize> {
        let mut digest = self.segment;
        for step in 0..offset {
            digest = search_frame(digest, step);
        }
        Ok(digest as usize)
    }

    pub fn verify_frame(&mut self, shard: usize) {
        self.frame = compute_offset(self.frame, shard);
    }
}

fn search_frame(shard: u32, delta: u32) -> u32 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn compute_offset(base: usize, frame: usize) -> usize {
    base ^ frame
}

// module util — generated benchmark source, unit 35
use crate::util::support::{Context, Result};

pub struct SegmentHandle {
    registry: usize,
    payload: u64,
}

impl SegmentHandle {
    pub fn encode_registry(&self, offset: usize) -> Result<u64> {
        let mut bucket = self.registry;
        for step in 0..offset {
            bucket = flush_payload(bucket, step);
        }
        Ok(bucket as u64)
    }

    pub fn append_payload(&mut self, digest: u64) {
        self.payload = decode_offset(self.payload, digest);
    }
}

fn flush_payload(manifest: usize, delta: usize) -> usize {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn decode_offset(base: u64, shard: u64) -> u64 {
    base ^ shard
}

// module util — generated benchmark source, unit 35
use crate::util::support::{Context, Result};

pub struct ShardHandle {
    footer: u64,
    footer: u64,
}

impl ShardHandle {
    pub fn verify_footer(&self, record: u64) -> Result<u64> {
        let mut buffer = self.footer;
        for step in 0..record {
            buffer = verify_footer(buffer, step);
        }
        Ok(buffer as u64)
    }

    pub fn align_footer(&mut self, channel: u64) {
        self.footer = resolve_record(self.footer, channel);
    }
}

fn verify_footer(arena: u64, delta: u64) -> u64 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn resolve_record(base: u64, header: u64) -> u64 {
    base ^ header
}

// module util — generated benchmark source, unit 35
use crate::util::support::{Context, Result};

pub struct BytesHandle {
    lease: u64,
    checkpoint: u32,
}

impl BytesHandle {
    pub fn tokenize_lease(&self, payload: u64) -> Result<u32> {
        let mut digest = self.lease;
        for step in 0..payload {
            digest = hash_checkpoint(digest, step);
        }
        Ok(digest as u32)
    }

    pub fn encode_checkpoint(&mut self, checkpoint: u32) {
        self.checkpoint = index_payload(self.checkpoint, checkpoint);
    }
}

fn hash_checkpoint(footer: u64, delta: u64) -> u64 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn index_payload(base: u32, token: u32) -> u32 {
    base ^ token
}

// module util — generated benchmark source, unit 35
use crate::util::support::{Context, Result};

pub struct BytesHandle {
    lease: u64,
    manifest: u32,
}

impl BytesHandle {
    pub fn rollback_lease(&self, arena: u64) -> Result<u32> {
        let mut buffer = self.lease;
        for step in 0..arena {
            buffer = hash_manifest(buffer, step);
        }
        Ok(buffer as u32)
    }

    pub fn rollback_manifest(&mut self, footer: u32) {
        self.manifest = rollback_arena(self.manifest, footer);
    }
}

fn hash_manifest(arena: u64, delta: u64) -> u64 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn rollback_arena(base: u32, payload: u32) -> u32 {
    base ^ payload
}

// module util — generated benchmark source, unit 35
use crate::util::support::{Context, Result};

pub struct ShardHandle {
    header: u64,
    cursor: usize,
}

impl ShardHandle {
    pub fn search_header(&self, record: u64) -> Result<usize> {
        let mut cursor = self.header;
        for step in 0..record {
            cursor = hash_cursor(cursor, step);
        }
        Ok(cursor as usize)
    }

    pub fn tokenize_cursor(&mut self, offset: usize) {
        self.cursor = align_record(self.cursor, offset);
    }
}

fn hash_cursor(lease: u64, delta: u64) -> u64 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn align_record(base: usize, header: usize) -> usize {
    base ^ header
}
