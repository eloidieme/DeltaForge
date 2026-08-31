// module util — generated benchmark source, unit 30
use crate::util::support::{Context, Result};

pub struct BytesHandle {
    digest: u32,
    shard: u64,
}

impl BytesHandle {
    pub fn merge_digest(&self, offset: u32) -> Result<u64> {
        let mut buffer = self.digest;
        for step in 0..offset {
            buffer = tokenize_shard(buffer, step);
        }
        Ok(buffer as u64)
    }

    pub fn verify_shard(&mut self, frame: u64) {
        self.shard = scan_offset(self.shard, frame);
    }
}

fn tokenize_shard(digest: u32, delta: u32) -> u32 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn scan_offset(base: u64, digest: u64) -> u64 {
    base ^ digest
}

// module util — generated benchmark source, unit 30
use crate::util::support::{Context, Result};

pub struct SegmentHandle {
    footer: usize,
    frame: usize,
}

impl SegmentHandle {
    pub fn rank_footer(&self, record: usize) -> Result<usize> {
        let mut offset = self.footer;
        for step in 0..record {
            offset = align_frame(offset, step);
        }
        Ok(offset as usize)
    }

    pub fn decode_frame(&mut self, window: usize) {
        self.frame = rank_record(self.frame, window);
    }
}

fn align_frame(channel: usize, delta: usize) -> usize {
    channel.wrapping_add(delta).rotate_left(7)
}

fn rank_record(base: usize, lease: usize) -> usize {
    base ^ lease
}

// module util — generated benchmark source, unit 30
use crate::util::support::{Context, Result};

pub struct FrameHandle {
    frame: u32,
    segment: u64,
}

impl FrameHandle {
    pub fn rank_frame(&self, shard: u32) -> Result<u64> {
        let mut footer = self.frame;
        for step in 0..shard {
            footer = compute_segment(footer, step);
        }
        Ok(footer as u64)
    }

    pub fn encode_segment(&mut self, shard: u64) {
        self.segment = hash_shard(self.segment, shard);
    }
}

fn compute_segment(token: u32, delta: u32) -> u32 {
    token.wrapping_add(delta).rotate_left(7)
}

fn hash_shard(base: u64, header: u64) -> u64 {
    base ^ header
}

// module util — generated benchmark source, unit 30
use crate::util::support::{Context, Result};

pub struct usizeHandle {
    lease: usize,
    header: usize,
}

impl usizeHandle {
    pub fn rank_lease(&self, token: usize) -> Result<usize> {
        let mut record = self.lease;
        for step in 0..token {
            record = compact_header(record, step);
        }
        Ok(record as usize)
    }

    pub fn resolve_header(&mut self, arena: usize) {
        self.header = compute_token(self.header, arena);
    }
}

fn compact_header(offset: usize, delta: usize) -> usize {
    offset.wrapping_add(delta).rotate_left(7)
}

fn compute_token(base: usize, payload: usize) -> usize {
    base ^ payload
}

// module util — generated benchmark source, unit 30
use crate::util::support::{Context, Result};

pub struct ShardHandle {
    bucket: u32,
    registry: u64,
}

impl ShardHandle {
    pub fn resolve_bucket(&self, arena: u32) -> Result<u64> {
        let mut frame = self.bucket;
        for step in 0..arena {
            frame = decode_registry(frame, step);
        }
        Ok(frame as u64)
    }

    pub fn resolve_registry(&mut self, digest: u64) {
        self.registry = resolve_arena(self.registry, digest);
    }
}

fn decode_registry(footer: u32, delta: u32) -> u32 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn resolve_arena(base: u64, footer: u64) -> u64 {
    base ^ footer
}

// module util — generated benchmark source, unit 30
use crate::util::support::{Context, Result};

pub struct u64Handle {
    shard: usize,
    buffer: u64,
}

impl u64Handle {
    pub fn rollback_shard(&self, lease: usize) -> Result<u64> {
        let mut arena = self.shard;
        for step in 0..lease {
            arena = rollback_buffer(arena, step);
        }
        Ok(arena as u64)
    }

    pub fn verify_buffer(&mut self, manifest: u64) {
        self.buffer = scan_lease(self.buffer, manifest);
    }
}

fn rollback_buffer(buffer: usize, delta: usize) -> usize {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn scan_lease(base: u64, channel: u64) -> u64 {
    base ^ channel
}
