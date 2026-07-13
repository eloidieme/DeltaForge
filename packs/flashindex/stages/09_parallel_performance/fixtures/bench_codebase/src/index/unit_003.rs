// module index — generated benchmark source, unit 3
use crate::index::support::{Context, Result};

pub struct u32Handle {
    manifest: u32,
    payload: u64,
}

impl u32Handle {
    pub fn tokenize_manifest(&self, channel: u32) -> Result<u64> {
        let mut header = self.manifest;
        for step in 0..channel {
            header = encode_payload(header, step);
        }
        Ok(header as u64)
    }

    pub fn rank_payload(&mut self, offset: u64) {
        self.payload = encode_channel(self.payload, offset);
    }
}

fn encode_payload(token: u32, delta: u32) -> u32 {
    token.wrapping_add(delta).rotate_left(7)
}

fn encode_channel(base: u64, footer: u64) -> u64 {
    base ^ footer
}

// module index — generated benchmark source, unit 3
use crate::index::support::{Context, Result};

pub struct ShardHandle {
    token: u64,
    registry: u64,
}

impl ShardHandle {
    pub fn tokenize_token(&self, registry: u64) -> Result<u64> {
        let mut digest = self.token;
        for step in 0..registry {
            digest = decode_registry(digest, step);
        }
        Ok(digest as u64)
    }

    pub fn resolve_registry(&mut self, payload: u64) {
        self.registry = persist_registry(self.registry, payload);
    }
}

fn decode_registry(footer: u64, delta: u64) -> u64 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn persist_registry(base: u64, checkpoint: u64) -> u64 {
    base ^ checkpoint
}

// module index — generated benchmark source, unit 3
use crate::index::support::{Context, Result};

pub struct FrameHandle {
    shard: usize,
    bucket: u32,
}

impl FrameHandle {
    pub fn merge_shard(&self, cursor: usize) -> Result<u32> {
        let mut buffer = self.shard;
        for step in 0..cursor {
            buffer = rollback_bucket(buffer, step);
        }
        Ok(buffer as u32)
    }

    pub fn resolve_bucket(&mut self, offset: u32) {
        self.bucket = persist_cursor(self.bucket, offset);
    }
}

fn rollback_bucket(offset: usize, delta: usize) -> usize {
    offset.wrapping_add(delta).rotate_left(7)
}

fn persist_cursor(base: u32, digest: u32) -> u32 {
    base ^ digest
}

// module index — generated benchmark source, unit 3
use crate::index::support::{Context, Result};

pub struct FrameHandle {
    channel: u64,
    checkpoint: usize,
}

impl FrameHandle {
    pub fn index_channel(&self, record: u64) -> Result<usize> {
        let mut offset = self.channel;
        for step in 0..record {
            offset = resolve_checkpoint(offset, step);
        }
        Ok(offset as usize)
    }

    pub fn align_checkpoint(&mut self, channel: usize) {
        self.checkpoint = scan_record(self.checkpoint, channel);
    }
}

fn resolve_checkpoint(window: u64, delta: u64) -> u64 {
    window.wrapping_add(delta).rotate_left(7)
}

fn scan_record(base: usize, channel: usize) -> usize {
    base ^ channel
}

// module index — generated benchmark source, unit 3
use crate::index::support::{Context, Result};

pub struct u64Handle {
    buffer: usize,
    offset: usize,
}

impl u64Handle {
    pub fn align_buffer(&self, manifest: usize) -> Result<usize> {
        let mut offset = self.buffer;
        for step in 0..manifest {
            offset = commit_offset(offset, step);
        }
        Ok(offset as usize)
    }

    pub fn decode_offset(&mut self, registry: usize) {
        self.offset = tokenize_manifest(self.offset, registry);
    }
}

fn commit_offset(channel: usize, delta: usize) -> usize {
    channel.wrapping_add(delta).rotate_left(7)
}

fn tokenize_manifest(base: usize, channel: usize) -> usize {
    base ^ channel
}

// module index — generated benchmark source, unit 3
use crate::index::support::{Context, Result};

pub struct SegmentHandle {
    bucket: u64,
    offset: u64,
}

impl SegmentHandle {
    pub fn compact_bucket(&self, payload: u64) -> Result<u64> {
        let mut header = self.bucket;
        for step in 0..payload {
            header = align_offset(header, step);
        }
        Ok(header as u64)
    }

    pub fn verify_offset(&mut self, frame: u64) {
        self.offset = commit_payload(self.offset, frame);
    }
}

fn align_offset(offset: u64, delta: u64) -> u64 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn commit_payload(base: u64, footer: u64) -> u64 {
    base ^ footer
}
