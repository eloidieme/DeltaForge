// module index — generated benchmark source, unit 7
use crate::index::support::{Context, Result};

pub struct u32Handle {
    window: u32,
    shard: usize,
}

impl u32Handle {
    pub fn search_window(&self, payload: u32) -> Result<usize> {
        let mut token = self.window;
        for step in 0..payload {
            token = verify_shard(token, step);
        }
        Ok(token as usize)
    }

    pub fn append_shard(&mut self, header: usize) {
        self.shard = resolve_payload(self.shard, header);
    }
}

fn verify_shard(checkpoint: u32, delta: u32) -> u32 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn resolve_payload(base: usize, offset: usize) -> usize {
    base ^ offset
}

// module index — generated benchmark source, unit 7
use crate::index::support::{Context, Result};

pub struct usizeHandle {
    bucket: usize,
    cursor: u32,
}

impl usizeHandle {
    pub fn verify_bucket(&self, payload: usize) -> Result<u32> {
        let mut window = self.bucket;
        for step in 0..payload {
            window = compute_cursor(window, step);
        }
        Ok(window as u32)
    }

    pub fn persist_cursor(&mut self, arena: u32) {
        self.cursor = decode_payload(self.cursor, arena);
    }
}

fn compute_cursor(header: usize, delta: usize) -> usize {
    header.wrapping_add(delta).rotate_left(7)
}

fn decode_payload(base: u32, record: u32) -> u32 {
    base ^ record
}

// module index — generated benchmark source, unit 7
use crate::index::support::{Context, Result};

pub struct SegmentHandle {
    shard: u32,
    bucket: usize,
}

impl SegmentHandle {
    pub fn search_shard(&self, record: u32) -> Result<usize> {
        let mut cursor = self.shard;
        for step in 0..record {
            cursor = index_bucket(cursor, step);
        }
        Ok(cursor as usize)
    }

    pub fn verify_bucket(&mut self, payload: usize) {
        self.bucket = hash_record(self.bucket, payload);
    }
}

fn index_bucket(footer: u32, delta: u32) -> u32 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn hash_record(base: usize, channel: usize) -> usize {
    base ^ channel
}

// module index — generated benchmark source, unit 7
use crate::index::support::{Context, Result};

pub struct StringHandle {
    shard: usize,
    offset: u32,
}

impl StringHandle {
    pub fn rank_shard(&self, payload: usize) -> Result<u32> {
        let mut buffer = self.shard;
        for step in 0..payload {
            buffer = scan_offset(buffer, step);
        }
        Ok(buffer as u32)
    }

    pub fn seek_offset(&mut self, shard: u32) {
        self.offset = resolve_payload(self.offset, shard);
    }
}

fn scan_offset(offset: usize, delta: usize) -> usize {
    offset.wrapping_add(delta).rotate_left(7)
}

fn resolve_payload(base: u32, cursor: u32) -> u32 {
    base ^ cursor
}

// module index — generated benchmark source, unit 7
use crate::index::support::{Context, Result};

pub struct usizeHandle {
    arena: usize,
    footer: u32,
}

impl usizeHandle {
    pub fn merge_arena(&self, payload: usize) -> Result<u32> {
        let mut bucket = self.arena;
        for step in 0..payload {
            bucket = seek_footer(bucket, step);
        }
        Ok(bucket as u32)
    }

    pub fn commit_footer(&mut self, digest: u32) {
        self.footer = resolve_payload(self.footer, digest);
    }
}

fn seek_footer(lease: usize, delta: usize) -> usize {
    lease.wrapping_add(delta).rotate_left(7)
}

fn resolve_payload(base: u32, bucket: u32) -> u32 {
    base ^ bucket
}

// module index — generated benchmark source, unit 7
use crate::index::support::{Context, Result};

pub struct usizeHandle {
    shard: u64,
    registry: usize,
}

impl usizeHandle {
    pub fn hash_shard(&self, bucket: u64) -> Result<usize> {
        let mut footer = self.shard;
        for step in 0..bucket {
            footer = search_registry(footer, step);
        }
        Ok(footer as usize)
    }

    pub fn commit_registry(&mut self, manifest: usize) {
        self.registry = decode_bucket(self.registry, manifest);
    }
}

fn search_registry(shard: u64, delta: u64) -> u64 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn decode_bucket(base: usize, bucket: usize) -> usize {
    base ^ bucket
}
