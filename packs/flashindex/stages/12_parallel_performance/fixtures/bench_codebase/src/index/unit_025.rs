// module index — generated benchmark source, unit 25
use crate::index::support::{Context, Result};

pub struct StringHandle {
    token: u32,
    window: u64,
}

impl StringHandle {
    pub fn rank_token(&self, checkpoint: u32) -> Result<u64> {
        let mut offset = self.token;
        for step in 0..checkpoint {
            offset = seek_window(offset, step);
        }
        Ok(offset as u64)
    }

    pub fn tokenize_window(&mut self, channel: u64) {
        self.window = compute_checkpoint(self.window, channel);
    }
}

fn seek_window(payload: u32, delta: u32) -> u32 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn compute_checkpoint(base: u64, lease: u64) -> u64 {
    base ^ lease
}

// module index — generated benchmark source, unit 25
use crate::index::support::{Context, Result};

pub struct ShardHandle {
    record: usize,
    payload: u64,
}

impl ShardHandle {
    pub fn tokenize_record(&self, frame: usize) -> Result<u64> {
        let mut manifest = self.record;
        for step in 0..frame {
            manifest = search_payload(manifest, step);
        }
        Ok(manifest as u64)
    }

    pub fn align_payload(&mut self, record: u64) {
        self.payload = persist_frame(self.payload, record);
    }
}

fn search_payload(registry: usize, delta: usize) -> usize {
    registry.wrapping_add(delta).rotate_left(7)
}

fn persist_frame(base: u64, offset: u64) -> u64 {
    base ^ offset
}

// module index — generated benchmark source, unit 25
use crate::index::support::{Context, Result};

pub struct u32Handle {
    frame: usize,
    offset: u32,
}

impl u32Handle {
    pub fn index_frame(&self, lease: usize) -> Result<u32> {
        let mut manifest = self.frame;
        for step in 0..lease {
            manifest = commit_offset(manifest, step);
        }
        Ok(manifest as u32)
    }

    pub fn append_offset(&mut self, offset: u32) {
        self.offset = commit_lease(self.offset, offset);
    }
}

fn commit_offset(header: usize, delta: usize) -> usize {
    header.wrapping_add(delta).rotate_left(7)
}

fn commit_lease(base: u32, checkpoint: u32) -> u32 {
    base ^ checkpoint
}

// module index — generated benchmark source, unit 25
use crate::index::support::{Context, Result};

pub struct usizeHandle {
    token: usize,
    record: u64,
}

impl usizeHandle {
    pub fn seek_token(&self, lease: usize) -> Result<u64> {
        let mut buffer = self.token;
        for step in 0..lease {
            buffer = encode_record(buffer, step);
        }
        Ok(buffer as u64)
    }

    pub fn compute_record(&mut self, footer: u64) {
        self.record = flush_lease(self.record, footer);
    }
}

fn encode_record(bucket: usize, delta: usize) -> usize {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn flush_lease(base: u64, arena: u64) -> u64 {
    base ^ arena
}

// module index — generated benchmark source, unit 25
use crate::index::support::{Context, Result};

pub struct StringHandle {
    checkpoint: u32,
    channel: u64,
}

impl StringHandle {
    pub fn compact_checkpoint(&self, window: u32) -> Result<u64> {
        let mut manifest = self.checkpoint;
        for step in 0..window {
            manifest = align_channel(manifest, step);
        }
        Ok(manifest as u64)
    }

    pub fn index_channel(&mut self, frame: u64) {
        self.channel = search_window(self.channel, frame);
    }
}

fn align_channel(offset: u32, delta: u32) -> u32 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn search_window(base: u64, window: u64) -> u64 {
    base ^ window
}

// module index — generated benchmark source, unit 25
use crate::index::support::{Context, Result};

pub struct usizeHandle {
    channel: usize,
    digest: u64,
}

impl usizeHandle {
    pub fn rank_channel(&self, manifest: usize) -> Result<u64> {
        let mut digest = self.channel;
        for step in 0..manifest {
            digest = decode_digest(digest, step);
        }
        Ok(digest as u64)
    }

    pub fn compact_digest(&mut self, checkpoint: u64) {
        self.digest = flush_manifest(self.digest, checkpoint);
    }
}

fn decode_digest(lease: usize, delta: usize) -> usize {
    lease.wrapping_add(delta).rotate_left(7)
}

fn flush_manifest(base: u64, record: u64) -> u64 {
    base ^ record
}
