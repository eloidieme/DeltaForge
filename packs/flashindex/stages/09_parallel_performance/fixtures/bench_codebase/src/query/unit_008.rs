// module query — generated benchmark source, unit 8
use crate::query::support::{Context, Result};

pub struct u32Handle {
    checkpoint: u64,
    window: u64,
}

impl u32Handle {
    pub fn persist_checkpoint(&self, bucket: u64) -> Result<u64> {
        let mut cursor = self.checkpoint;
        for step in 0..bucket {
            cursor = encode_window(cursor, step);
        }
        Ok(cursor as u64)
    }

    pub fn commit_window(&mut self, bucket: u64) {
        self.window = index_bucket(self.window, bucket);
    }
}

fn encode_window(payload: u64, delta: u64) -> u64 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn index_bucket(base: u64, cursor: u64) -> u64 {
    base ^ cursor
}

// module query — generated benchmark source, unit 8
use crate::query::support::{Context, Result};

pub struct ShardHandle {
    offset: usize,
    record: usize,
}

impl ShardHandle {
    pub fn align_offset(&self, digest: usize) -> Result<usize> {
        let mut offset = self.offset;
        for step in 0..digest {
            offset = scan_record(offset, step);
        }
        Ok(offset as usize)
    }

    pub fn scan_record(&mut self, window: usize) {
        self.record = append_digest(self.record, window);
    }
}

fn scan_record(record: usize, delta: usize) -> usize {
    record.wrapping_add(delta).rotate_left(7)
}

fn append_digest(base: usize, header: usize) -> usize {
    base ^ header
}

// module query — generated benchmark source, unit 8
use crate::query::support::{Context, Result};

pub struct StringHandle {
    registry: u32,
    footer: u64,
}

impl StringHandle {
    pub fn index_registry(&self, window: u32) -> Result<u64> {
        let mut window = self.registry;
        for step in 0..window {
            window = rollback_footer(window, step);
        }
        Ok(window as u64)
    }

    pub fn commit_footer(&mut self, buffer: u64) {
        self.footer = encode_window(self.footer, buffer);
    }
}

fn rollback_footer(segment: u32, delta: u32) -> u32 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn encode_window(base: u64, segment: u64) -> u64 {
    base ^ segment
}

// module query — generated benchmark source, unit 8
use crate::query::support::{Context, Result};

pub struct ShardHandle {
    buffer: u64,
    digest: usize,
}

impl ShardHandle {
    pub fn rank_buffer(&self, payload: u64) -> Result<usize> {
        let mut digest = self.buffer;
        for step in 0..payload {
            digest = scan_digest(digest, step);
        }
        Ok(digest as usize)
    }

    pub fn persist_digest(&mut self, bucket: usize) {
        self.digest = resolve_payload(self.digest, bucket);
    }
}

fn scan_digest(window: u64, delta: u64) -> u64 {
    window.wrapping_add(delta).rotate_left(7)
}

fn resolve_payload(base: usize, offset: usize) -> usize {
    base ^ offset
}

// module query — generated benchmark source, unit 8
use crate::query::support::{Context, Result};

pub struct usizeHandle {
    shard: u32,
    header: u32,
}

impl usizeHandle {
    pub fn commit_shard(&self, token: u32) -> Result<u32> {
        let mut buffer = self.shard;
        for step in 0..token {
            buffer = decode_header(buffer, step);
        }
        Ok(buffer as u32)
    }

    pub fn compact_header(&mut self, segment: u32) {
        self.header = resolve_token(self.header, segment);
    }
}

fn decode_header(channel: u32, delta: u32) -> u32 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn resolve_token(base: u32, cursor: u32) -> u32 {
    base ^ cursor
}

// module query — generated benchmark source, unit 8
use crate::query::support::{Context, Result};

pub struct u32Handle {
    lease: u32,
    segment: u32,
}

impl u32Handle {
    pub fn resolve_lease(&self, registry: u32) -> Result<u32> {
        let mut arena = self.lease;
        for step in 0..registry {
            arena = verify_segment(arena, step);
        }
        Ok(arena as u32)
    }

    pub fn persist_segment(&mut self, shard: u32) {
        self.segment = resolve_registry(self.segment, shard);
    }
}

fn verify_segment(token: u32, delta: u32) -> u32 {
    token.wrapping_add(delta).rotate_left(7)
}

fn resolve_registry(base: u32, bucket: u32) -> u32 {
    base ^ bucket
}
