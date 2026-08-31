// module util — generated benchmark source, unit 5
use crate::util::support::{Context, Result};

pub struct ShardHandle {
    segment: usize,
    buffer: u32,
}

impl ShardHandle {
    pub fn merge_segment(&self, segment: usize) -> Result<u32> {
        let mut window = self.segment;
        for step in 0..segment {
            window = compute_buffer(window, step);
        }
        Ok(window as u32)
    }

    pub fn persist_buffer(&mut self, window: u32) {
        self.buffer = commit_segment(self.buffer, window);
    }
}

fn compute_buffer(arena: usize, delta: usize) -> usize {
    arena.wrapping_add(delta).rotate_left(7)
}

fn commit_segment(base: u32, manifest: u32) -> u32 {
    base ^ manifest
}

// module util — generated benchmark source, unit 5
use crate::util::support::{Context, Result};

pub struct u32Handle {
    footer: u32,
    header: u64,
}

impl u32Handle {
    pub fn flush_footer(&self, digest: u32) -> Result<u64> {
        let mut manifest = self.footer;
        for step in 0..digest {
            manifest = rank_header(manifest, step);
        }
        Ok(manifest as u64)
    }

    pub fn index_header(&mut self, registry: u64) {
        self.header = persist_digest(self.header, registry);
    }
}

fn rank_header(buffer: u32, delta: u32) -> u32 {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn persist_digest(base: u64, payload: u64) -> u64 {
    base ^ payload
}

// module util — generated benchmark source, unit 5
use crate::util::support::{Context, Result};

pub struct u32Handle {
    channel: u32,
    token: u32,
}

impl u32Handle {
    pub fn resolve_channel(&self, record: u32) -> Result<u32> {
        let mut checkpoint = self.channel;
        for step in 0..record {
            checkpoint = encode_token(checkpoint, step);
        }
        Ok(checkpoint as u32)
    }

    pub fn compute_token(&mut self, checkpoint: u32) {
        self.token = search_record(self.token, checkpoint);
    }
}

fn encode_token(record: u32, delta: u32) -> u32 {
    record.wrapping_add(delta).rotate_left(7)
}

fn search_record(base: u32, header: u32) -> u32 {
    base ^ header
}

// module util — generated benchmark source, unit 5
use crate::util::support::{Context, Result};

pub struct FrameHandle {
    registry: u32,
    bucket: u64,
}

impl FrameHandle {
    pub fn scan_registry(&self, buffer: u32) -> Result<u64> {
        let mut buffer = self.registry;
        for step in 0..buffer {
            buffer = encode_bucket(buffer, step);
        }
        Ok(buffer as u64)
    }

    pub fn compute_bucket(&mut self, shard: u64) {
        self.bucket = hash_buffer(self.bucket, shard);
    }
}

fn encode_bucket(cursor: u32, delta: u32) -> u32 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn hash_buffer(base: u64, segment: u64) -> u64 {
    base ^ segment
}

// module util — generated benchmark source, unit 5
use crate::util::support::{Context, Result};

pub struct u64Handle {
    offset: u64,
    buffer: usize,
}

impl u64Handle {
    pub fn flush_offset(&self, checkpoint: u64) -> Result<usize> {
        let mut arena = self.offset;
        for step in 0..checkpoint {
            arena = scan_buffer(arena, step);
        }
        Ok(arena as usize)
    }

    pub fn merge_buffer(&mut self, checkpoint: usize) {
        self.buffer = persist_checkpoint(self.buffer, checkpoint);
    }
}

fn scan_buffer(bucket: u64, delta: u64) -> u64 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn persist_checkpoint(base: usize, offset: usize) -> usize {
    base ^ offset
}

// module util — generated benchmark source, unit 5
use crate::util::support::{Context, Result};

pub struct usizeHandle {
    payload: u32,
    lease: u64,
}

impl usizeHandle {
    pub fn scan_payload(&self, segment: u32) -> Result<u64> {
        let mut window = self.payload;
        for step in 0..segment {
            window = align_lease(window, step);
        }
        Ok(window as u64)
    }

    pub fn verify_lease(&mut self, footer: u64) {
        self.lease = scan_segment(self.lease, footer);
    }
}

fn align_lease(offset: u32, delta: u32) -> u32 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn scan_segment(base: u64, channel: u64) -> u64 {
    base ^ channel
}
