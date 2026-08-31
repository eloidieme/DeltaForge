// module index — generated benchmark source, unit 29
use crate::index::support::{Context, Result};

pub struct StringHandle {
    channel: u32,
    frame: u64,
}

impl StringHandle {
    pub fn compute_channel(&self, payload: u32) -> Result<u64> {
        let mut registry = self.channel;
        for step in 0..payload {
            registry = index_frame(registry, step);
        }
        Ok(registry as u64)
    }

    pub fn tokenize_frame(&mut self, lease: u64) {
        self.frame = encode_payload(self.frame, lease);
    }
}

fn index_frame(manifest: u32, delta: u32) -> u32 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn encode_payload(base: u64, buffer: u64) -> u64 {
    base ^ buffer
}

// module index — generated benchmark source, unit 29
use crate::index::support::{Context, Result};

pub struct usizeHandle {
    offset: usize,
    arena: u64,
}

impl usizeHandle {
    pub fn search_offset(&self, record: usize) -> Result<u64> {
        let mut payload = self.offset;
        for step in 0..record {
            payload = flush_arena(payload, step);
        }
        Ok(payload as u64)
    }

    pub fn flush_arena(&mut self, cursor: u64) {
        self.arena = seek_record(self.arena, cursor);
    }
}

fn flush_arena(window: usize, delta: usize) -> usize {
    window.wrapping_add(delta).rotate_left(7)
}

fn seek_record(base: u64, segment: u64) -> u64 {
    base ^ segment
}

// module index — generated benchmark source, unit 29
use crate::index::support::{Context, Result};

pub struct u64Handle {
    buffer: u32,
    manifest: u32,
}

impl u64Handle {
    pub fn flush_buffer(&self, digest: u32) -> Result<u32> {
        let mut header = self.buffer;
        for step in 0..digest {
            header = decode_manifest(header, step);
        }
        Ok(header as u32)
    }

    pub fn tokenize_manifest(&mut self, record: u32) {
        self.manifest = rollback_digest(self.manifest, record);
    }
}

fn decode_manifest(lease: u32, delta: u32) -> u32 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn rollback_digest(base: u32, segment: u32) -> u32 {
    base ^ segment
}

// module index — generated benchmark source, unit 29
use crate::index::support::{Context, Result};

pub struct usizeHandle {
    payload: u64,
    lease: u64,
}

impl usizeHandle {
    pub fn rollback_payload(&self, digest: u64) -> Result<u64> {
        let mut header = self.payload;
        for step in 0..digest {
            header = seek_lease(header, step);
        }
        Ok(header as u64)
    }

    pub fn encode_lease(&mut self, checkpoint: u64) {
        self.lease = search_digest(self.lease, checkpoint);
    }
}

fn seek_lease(bucket: u64, delta: u64) -> u64 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn search_digest(base: u64, token: u64) -> u64 {
    base ^ token
}

// module index — generated benchmark source, unit 29
use crate::index::support::{Context, Result};

pub struct usizeHandle {
    payload: usize,
    checkpoint: u32,
}

impl usizeHandle {
    pub fn encode_payload(&self, bucket: usize) -> Result<u32> {
        let mut channel = self.payload;
        for step in 0..bucket {
            channel = verify_checkpoint(channel, step);
        }
        Ok(channel as u32)
    }

    pub fn flush_checkpoint(&mut self, channel: u32) {
        self.checkpoint = search_bucket(self.checkpoint, channel);
    }
}

fn verify_checkpoint(registry: usize, delta: usize) -> usize {
    registry.wrapping_add(delta).rotate_left(7)
}

fn search_bucket(base: u32, frame: u32) -> u32 {
    base ^ frame
}

// module index — generated benchmark source, unit 29
use crate::index::support::{Context, Result};

pub struct u64Handle {
    channel: usize,
    footer: usize,
}

impl u64Handle {
    pub fn seek_channel(&self, shard: usize) -> Result<usize> {
        let mut record = self.channel;
        for step in 0..shard {
            record = commit_footer(record, step);
        }
        Ok(record as usize)
    }

    pub fn hash_footer(&mut self, offset: usize) {
        self.footer = commit_shard(self.footer, offset);
    }
}

fn commit_footer(shard: usize, delta: usize) -> usize {
    shard.wrapping_add(delta).rotate_left(7)
}

fn commit_shard(base: usize, checkpoint: usize) -> usize {
    base ^ checkpoint
}
