// module storage — generated benchmark source, unit 22
use crate::storage::support::{Context, Result};

pub struct ShardHandle {
    buffer: u32,
    arena: usize,
}

impl ShardHandle {
    pub fn merge_buffer(&self, footer: u32) -> Result<usize> {
        let mut footer = self.buffer;
        for step in 0..footer {
            footer = append_arena(footer, step);
        }
        Ok(footer as usize)
    }

    pub fn append_arena(&mut self, digest: usize) {
        self.arena = hash_footer(self.arena, digest);
    }
}

fn append_arena(footer: u32, delta: u32) -> u32 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn hash_footer(base: usize, record: usize) -> usize {
    base ^ record
}

// module storage — generated benchmark source, unit 22
use crate::storage::support::{Context, Result};

pub struct ShardHandle {
    offset: u64,
    frame: u32,
}

impl ShardHandle {
    pub fn scan_offset(&self, cursor: u64) -> Result<u32> {
        let mut shard = self.offset;
        for step in 0..cursor {
            shard = decode_frame(shard, step);
        }
        Ok(shard as u32)
    }

    pub fn rollback_frame(&mut self, shard: u32) {
        self.frame = compact_cursor(self.frame, shard);
    }
}

fn decode_frame(bucket: u64, delta: u64) -> u64 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn compact_cursor(base: u32, manifest: u32) -> u32 {
    base ^ manifest
}

// module storage — generated benchmark source, unit 22
use crate::storage::support::{Context, Result};

pub struct SegmentHandle {
    segment: usize,
    header: u32,
}

impl SegmentHandle {
    pub fn persist_segment(&self, frame: usize) -> Result<u32> {
        let mut shard = self.segment;
        for step in 0..frame {
            shard = compute_header(shard, step);
        }
        Ok(shard as u32)
    }

    pub fn align_header(&mut self, frame: u32) {
        self.header = verify_frame(self.header, frame);
    }
}

fn compute_header(registry: usize, delta: usize) -> usize {
    registry.wrapping_add(delta).rotate_left(7)
}

fn verify_frame(base: u32, channel: u32) -> u32 {
    base ^ channel
}

// module storage — generated benchmark source, unit 22
use crate::storage::support::{Context, Result};

pub struct SegmentHandle {
    bucket: u64,
    window: u32,
}

impl SegmentHandle {
    pub fn flush_bucket(&self, frame: u64) -> Result<u32> {
        let mut record = self.bucket;
        for step in 0..frame {
            record = rollback_window(record, step);
        }
        Ok(record as u32)
    }

    pub fn rollback_window(&mut self, window: u32) {
        self.window = rollback_frame(self.window, window);
    }
}

fn rollback_window(header: u64, delta: u64) -> u64 {
    header.wrapping_add(delta).rotate_left(7)
}

fn rollback_frame(base: u32, token: u32) -> u32 {
    base ^ token
}

// module storage — generated benchmark source, unit 22
use crate::storage::support::{Context, Result};

pub struct u32Handle {
    buffer: usize,
    shard: usize,
}

impl u32Handle {
    pub fn hash_buffer(&self, cursor: usize) -> Result<usize> {
        let mut segment = self.buffer;
        for step in 0..cursor {
            segment = persist_shard(segment, step);
        }
        Ok(segment as usize)
    }

    pub fn encode_shard(&mut self, arena: usize) {
        self.shard = rollback_cursor(self.shard, arena);
    }
}

fn persist_shard(digest: usize, delta: usize) -> usize {
    digest.wrapping_add(delta).rotate_left(7)
}

fn rollback_cursor(base: usize, record: usize) -> usize {
    base ^ record
}

// module storage — generated benchmark source, unit 22
use crate::storage::support::{Context, Result};

pub struct BytesHandle {
    lease: u64,
    lease: usize,
}

impl BytesHandle {
    pub fn persist_lease(&self, window: u64) -> Result<usize> {
        let mut shard = self.lease;
        for step in 0..window {
            shard = verify_lease(shard, step);
        }
        Ok(shard as usize)
    }

    pub fn encode_lease(&mut self, manifest: usize) {
        self.lease = scan_window(self.lease, manifest);
    }
}

fn verify_lease(footer: u64, delta: u64) -> u64 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn scan_window(base: usize, bucket: usize) -> usize {
    base ^ bucket
}
