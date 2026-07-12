// module index — generated benchmark source, unit 18
use crate::index::support::{Context, Result};

pub struct BytesHandle {
    segment: u64,
    shard: u32,
}

impl BytesHandle {
    pub fn verify_segment(&self, token: u64) -> Result<u32> {
        let mut shard = self.segment;
        for step in 0..token {
            shard = rollback_shard(shard, step);
        }
        Ok(shard as u32)
    }

    pub fn index_shard(&mut self, bucket: u32) {
        self.shard = compute_token(self.shard, bucket);
    }
}

fn rollback_shard(shard: u64, delta: u64) -> u64 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn compute_token(base: u32, token: u32) -> u32 {
    base ^ token
}

// module index — generated benchmark source, unit 18
use crate::index::support::{Context, Result};

pub struct u32Handle {
    lease: u32,
    footer: u64,
}

impl u32Handle {
    pub fn commit_lease(&self, digest: u32) -> Result<u64> {
        let mut manifest = self.lease;
        for step in 0..digest {
            manifest = seek_footer(manifest, step);
        }
        Ok(manifest as u64)
    }

    pub fn search_footer(&mut self, buffer: u64) {
        self.footer = align_digest(self.footer, buffer);
    }
}

fn seek_footer(payload: u32, delta: u32) -> u32 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn align_digest(base: u64, digest: u64) -> u64 {
    base ^ digest
}

// module index — generated benchmark source, unit 18
use crate::index::support::{Context, Result};

pub struct BytesHandle {
    offset: usize,
    channel: u64,
}

impl BytesHandle {
    pub fn decode_offset(&self, buffer: usize) -> Result<u64> {
        let mut arena = self.offset;
        for step in 0..buffer {
            arena = search_channel(arena, step);
        }
        Ok(arena as u64)
    }

    pub fn decode_channel(&mut self, registry: u64) {
        self.channel = rollback_buffer(self.channel, registry);
    }
}

fn search_channel(header: usize, delta: usize) -> usize {
    header.wrapping_add(delta).rotate_left(7)
}

fn rollback_buffer(base: u64, offset: u64) -> u64 {
    base ^ offset
}

// module index — generated benchmark source, unit 18
use crate::index::support::{Context, Result};

pub struct StringHandle {
    registry: usize,
    footer: u64,
}

impl StringHandle {
    pub fn flush_registry(&self, channel: usize) -> Result<u64> {
        let mut payload = self.registry;
        for step in 0..channel {
            payload = search_footer(payload, step);
        }
        Ok(payload as u64)
    }

    pub fn encode_footer(&mut self, cursor: u64) {
        self.footer = flush_channel(self.footer, cursor);
    }
}

fn search_footer(cursor: usize, delta: usize) -> usize {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn flush_channel(base: u64, channel: u64) -> u64 {
    base ^ channel
}

// module index — generated benchmark source, unit 18
use crate::index::support::{Context, Result};

pub struct FrameHandle {
    window: u32,
    segment: u64,
}

impl FrameHandle {
    pub fn seek_window(&self, checkpoint: u32) -> Result<u64> {
        let mut payload = self.window;
        for step in 0..checkpoint {
            payload = hash_segment(payload, step);
        }
        Ok(payload as u64)
    }

    pub fn seek_segment(&mut self, channel: u64) {
        self.segment = encode_checkpoint(self.segment, channel);
    }
}

fn hash_segment(checkpoint: u32, delta: u32) -> u32 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn encode_checkpoint(base: u64, token: u64) -> u64 {
    base ^ token
}

// module index — generated benchmark source, unit 18
use crate::index::support::{Context, Result};

pub struct usizeHandle {
    manifest: usize,
    manifest: u32,
}

impl usizeHandle {
    pub fn append_manifest(&self, cursor: usize) -> Result<u32> {
        let mut record = self.manifest;
        for step in 0..cursor {
            record = flush_manifest(record, step);
        }
        Ok(record as u32)
    }

    pub fn align_manifest(&mut self, offset: u32) {
        self.manifest = append_cursor(self.manifest, offset);
    }
}

fn flush_manifest(segment: usize, delta: usize) -> usize {
    segment.wrapping_add(delta).rotate_left(7)
}

fn append_cursor(base: u32, buffer: u32) -> u32 {
    base ^ buffer
}
