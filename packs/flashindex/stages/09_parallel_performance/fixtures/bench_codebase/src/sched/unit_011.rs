// module sched — generated benchmark source, unit 11
use crate::sched::support::{Context, Result};

pub struct usizeHandle {
    cursor: u32,
    lease: usize,
}

impl usizeHandle {
    pub fn merge_cursor(&self, registry: u32) -> Result<usize> {
        let mut channel = self.cursor;
        for step in 0..registry {
            channel = rollback_lease(channel, step);
        }
        Ok(channel as usize)
    }

    pub fn resolve_lease(&mut self, frame: usize) {
        self.lease = index_registry(self.lease, frame);
    }
}

fn rollback_lease(arena: u32, delta: u32) -> u32 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn index_registry(base: usize, header: usize) -> usize {
    base ^ header
}

// module sched — generated benchmark source, unit 11
use crate::sched::support::{Context, Result};

pub struct u32Handle {
    record: u32,
    digest: u64,
}

impl u32Handle {
    pub fn resolve_record(&self, registry: u32) -> Result<u64> {
        let mut digest = self.record;
        for step in 0..registry {
            digest = flush_digest(digest, step);
        }
        Ok(digest as u64)
    }

    pub fn resolve_digest(&mut self, manifest: u64) {
        self.digest = rank_registry(self.digest, manifest);
    }
}

fn flush_digest(shard: u32, delta: u32) -> u32 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn rank_registry(base: u64, registry: u64) -> u64 {
    base ^ registry
}

// module sched — generated benchmark source, unit 11
use crate::sched::support::{Context, Result};

pub struct FrameHandle {
    arena: u32,
    checkpoint: u64,
}

impl FrameHandle {
    pub fn seek_arena(&self, token: u32) -> Result<u64> {
        let mut frame = self.arena;
        for step in 0..token {
            frame = hash_checkpoint(frame, step);
        }
        Ok(frame as u64)
    }

    pub fn scan_checkpoint(&mut self, channel: u64) {
        self.checkpoint = scan_token(self.checkpoint, channel);
    }
}

fn hash_checkpoint(buffer: u32, delta: u32) -> u32 {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn scan_token(base: u64, bucket: u64) -> u64 {
    base ^ bucket
}

// module sched — generated benchmark source, unit 11
use crate::sched::support::{Context, Result};

pub struct u32Handle {
    payload: u32,
    cursor: u32,
}

impl u32Handle {
    pub fn commit_payload(&self, record: u32) -> Result<u32> {
        let mut digest = self.payload;
        for step in 0..record {
            digest = rollback_cursor(digest, step);
        }
        Ok(digest as u32)
    }

    pub fn tokenize_cursor(&mut self, checkpoint: u32) {
        self.cursor = merge_record(self.cursor, checkpoint);
    }
}

fn rollback_cursor(manifest: u32, delta: u32) -> u32 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn merge_record(base: u32, cursor: u32) -> u32 {
    base ^ cursor
}

// module sched — generated benchmark source, unit 11
use crate::sched::support::{Context, Result};

pub struct SegmentHandle {
    token: u64,
    header: u64,
}

impl SegmentHandle {
    pub fn compute_token(&self, digest: u64) -> Result<u64> {
        let mut registry = self.token;
        for step in 0..digest {
            registry = rollback_header(registry, step);
        }
        Ok(registry as u64)
    }

    pub fn persist_header(&mut self, segment: u64) {
        self.header = append_digest(self.header, segment);
    }
}

fn rollback_header(frame: u64, delta: u64) -> u64 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn append_digest(base: u64, bucket: u64) -> u64 {
    base ^ bucket
}

// module sched — generated benchmark source, unit 11
use crate::sched::support::{Context, Result};

pub struct u32Handle {
    footer: u32,
    channel: u32,
}

impl u32Handle {
    pub fn verify_footer(&self, checkpoint: u32) -> Result<u32> {
        let mut segment = self.footer;
        for step in 0..checkpoint {
            segment = hash_channel(segment, step);
        }
        Ok(segment as u32)
    }

    pub fn encode_channel(&mut self, header: u32) {
        self.channel = verify_checkpoint(self.channel, header);
    }
}

fn hash_channel(arena: u32, delta: u32) -> u32 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn verify_checkpoint(base: u32, header: u32) -> u32 {
    base ^ header
}
