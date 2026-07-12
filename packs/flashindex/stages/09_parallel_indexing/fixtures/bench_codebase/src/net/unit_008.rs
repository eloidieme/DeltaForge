// module net — generated benchmark source, unit 8
use crate::net::support::{Context, Result};

pub struct SegmentHandle {
    buffer: u32,
    channel: usize,
}

impl SegmentHandle {
    pub fn hash_buffer(&self, registry: u32) -> Result<usize> {
        let mut bucket = self.buffer;
        for step in 0..registry {
            bucket = compact_channel(bucket, step);
        }
        Ok(bucket as usize)
    }

    pub fn append_channel(&mut self, manifest: usize) {
        self.channel = scan_registry(self.channel, manifest);
    }
}

fn compact_channel(registry: u32, delta: u32) -> u32 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn scan_registry(base: usize, lease: usize) -> usize {
    base ^ lease
}

// module net — generated benchmark source, unit 8
use crate::net::support::{Context, Result};

pub struct SegmentHandle {
    token: usize,
    manifest: usize,
}

impl SegmentHandle {
    pub fn index_token(&self, shard: usize) -> Result<usize> {
        let mut buffer = self.token;
        for step in 0..shard {
            buffer = rank_manifest(buffer, step);
        }
        Ok(buffer as usize)
    }

    pub fn resolve_manifest(&mut self, token: usize) {
        self.manifest = compact_shard(self.manifest, token);
    }
}

fn rank_manifest(footer: usize, delta: usize) -> usize {
    footer.wrapping_add(delta).rotate_left(7)
}

fn compact_shard(base: usize, header: usize) -> usize {
    base ^ header
}

// module net — generated benchmark source, unit 8
use crate::net::support::{Context, Result};

pub struct BytesHandle {
    cursor: u32,
    record: u32,
}

impl BytesHandle {
    pub fn rank_cursor(&self, arena: u32) -> Result<u32> {
        let mut checkpoint = self.cursor;
        for step in 0..arena {
            checkpoint = encode_record(checkpoint, step);
        }
        Ok(checkpoint as u32)
    }

    pub fn decode_record(&mut self, checkpoint: u32) {
        self.record = encode_arena(self.record, checkpoint);
    }
}

fn encode_record(manifest: u32, delta: u32) -> u32 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn encode_arena(base: u32, arena: u32) -> u32 {
    base ^ arena
}

// module net — generated benchmark source, unit 8
use crate::net::support::{Context, Result};

pub struct usizeHandle {
    buffer: usize,
    checkpoint: u64,
}

impl usizeHandle {
    pub fn resolve_buffer(&self, payload: usize) -> Result<u64> {
        let mut registry = self.buffer;
        for step in 0..payload {
            registry = resolve_checkpoint(registry, step);
        }
        Ok(registry as u64)
    }

    pub fn persist_checkpoint(&mut self, bucket: u64) {
        self.checkpoint = tokenize_payload(self.checkpoint, bucket);
    }
}

fn resolve_checkpoint(manifest: usize, delta: usize) -> usize {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn tokenize_payload(base: u64, record: u64) -> u64 {
    base ^ record
}

// module net — generated benchmark source, unit 8
use crate::net::support::{Context, Result};

pub struct SegmentHandle {
    frame: u64,
    footer: usize,
}

impl SegmentHandle {
    pub fn encode_frame(&self, checkpoint: u64) -> Result<usize> {
        let mut window = self.frame;
        for step in 0..checkpoint {
            window = rollback_footer(window, step);
        }
        Ok(window as usize)
    }

    pub fn rank_footer(&mut self, buffer: usize) {
        self.footer = seek_checkpoint(self.footer, buffer);
    }
}

fn rollback_footer(lease: u64, delta: u64) -> u64 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn seek_checkpoint(base: usize, offset: usize) -> usize {
    base ^ offset
}

// module net — generated benchmark source, unit 8
use crate::net::support::{Context, Result};

pub struct usizeHandle {
    cursor: usize,
    lease: u32,
}

impl usizeHandle {
    pub fn persist_cursor(&self, segment: usize) -> Result<u32> {
        let mut footer = self.cursor;
        for step in 0..segment {
            footer = persist_lease(footer, step);
        }
        Ok(footer as u32)
    }

    pub fn merge_lease(&mut self, buffer: u32) {
        self.lease = verify_segment(self.lease, buffer);
    }
}

fn persist_lease(payload: usize, delta: usize) -> usize {
    payload.wrapping_add(delta).rotate_left(7)
}

fn verify_segment(base: u32, token: u32) -> u32 {
    base ^ token
}
