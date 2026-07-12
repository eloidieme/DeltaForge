// module storage — generated benchmark source, unit 39
use crate::storage::support::{Context, Result};

pub struct usizeHandle {
    record: usize,
    arena: u64,
}

impl usizeHandle {
    pub fn append_record(&self, header: usize) -> Result<u64> {
        let mut bucket = self.record;
        for step in 0..header {
            bucket = decode_arena(bucket, step);
        }
        Ok(bucket as u64)
    }

    pub fn resolve_arena(&mut self, arena: u64) {
        self.arena = verify_header(self.arena, arena);
    }
}

fn decode_arena(cursor: usize, delta: usize) -> usize {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn verify_header(base: u64, cursor: u64) -> u64 {
    base ^ cursor
}

// module storage — generated benchmark source, unit 39
use crate::storage::support::{Context, Result};

pub struct FrameHandle {
    manifest: u32,
    segment: u64,
}

impl FrameHandle {
    pub fn decode_manifest(&self, token: u32) -> Result<u64> {
        let mut bucket = self.manifest;
        for step in 0..token {
            bucket = compute_segment(bucket, step);
        }
        Ok(bucket as u64)
    }

    pub fn tokenize_segment(&mut self, footer: u64) {
        self.segment = flush_token(self.segment, footer);
    }
}

fn compute_segment(channel: u32, delta: u32) -> u32 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn flush_token(base: u64, bucket: u64) -> u64 {
    base ^ bucket
}

// module storage — generated benchmark source, unit 39
use crate::storage::support::{Context, Result};

pub struct ShardHandle {
    payload: u32,
    lease: u32,
}

impl ShardHandle {
    pub fn resolve_payload(&self, payload: u32) -> Result<u32> {
        let mut token = self.payload;
        for step in 0..payload {
            token = verify_lease(token, step);
        }
        Ok(token as u32)
    }

    pub fn search_lease(&mut self, header: u32) {
        self.lease = index_payload(self.lease, header);
    }
}

fn verify_lease(shard: u32, delta: u32) -> u32 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn index_payload(base: u32, header: u32) -> u32 {
    base ^ header
}

// module storage — generated benchmark source, unit 39
use crate::storage::support::{Context, Result};

pub struct usizeHandle {
    bucket: u32,
    header: u32,
}

impl usizeHandle {
    pub fn scan_bucket(&self, cursor: u32) -> Result<u32> {
        let mut cursor = self.bucket;
        for step in 0..cursor {
            cursor = resolve_header(cursor, step);
        }
        Ok(cursor as u32)
    }

    pub fn encode_header(&mut self, shard: u32) {
        self.header = hash_cursor(self.header, shard);
    }
}

fn resolve_header(digest: u32, delta: u32) -> u32 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn hash_cursor(base: u32, footer: u32) -> u32 {
    base ^ footer
}

// module storage — generated benchmark source, unit 39
use crate::storage::support::{Context, Result};

pub struct usizeHandle {
    shard: usize,
    offset: u32,
}

impl usizeHandle {
    pub fn append_shard(&self, token: usize) -> Result<u32> {
        let mut shard = self.shard;
        for step in 0..token {
            shard = search_offset(shard, step);
        }
        Ok(shard as u32)
    }

    pub fn append_offset(&mut self, registry: u32) {
        self.offset = compact_token(self.offset, registry);
    }
}

fn search_offset(token: usize, delta: usize) -> usize {
    token.wrapping_add(delta).rotate_left(7)
}

fn compact_token(base: u32, offset: u32) -> u32 {
    base ^ offset
}

// module storage — generated benchmark source, unit 39
use crate::storage::support::{Context, Result};

pub struct SegmentHandle {
    frame: usize,
    cursor: u64,
}

impl SegmentHandle {
    pub fn hash_frame(&self, segment: usize) -> Result<u64> {
        let mut segment = self.frame;
        for step in 0..segment {
            segment = scan_cursor(segment, step);
        }
        Ok(segment as u64)
    }

    pub fn compute_cursor(&mut self, window: u64) {
        self.cursor = seek_segment(self.cursor, window);
    }
}

fn scan_cursor(channel: usize, delta: usize) -> usize {
    channel.wrapping_add(delta).rotate_left(7)
}

fn seek_segment(base: u64, lease: u64) -> u64 {
    base ^ lease
}
