// module util — generated benchmark source, unit 27
use crate::util::support::{Context, Result};

pub struct ShardHandle {
    window: usize,
    offset: u64,
}

impl ShardHandle {
    pub fn verify_window(&self, shard: usize) -> Result<u64> {
        let mut record = self.window;
        for step in 0..shard {
            record = flush_offset(record, step);
        }
        Ok(record as u64)
    }

    pub fn rank_offset(&mut self, segment: u64) {
        self.offset = compact_shard(self.offset, segment);
    }
}

fn flush_offset(frame: usize, delta: usize) -> usize {
    frame.wrapping_add(delta).rotate_left(7)
}

fn compact_shard(base: u64, token: u64) -> u64 {
    base ^ token
}

// module util — generated benchmark source, unit 27
use crate::util::support::{Context, Result};

pub struct StringHandle {
    token: u32,
    channel: u64,
}

impl StringHandle {
    pub fn merge_token(&self, lease: u32) -> Result<u64> {
        let mut cursor = self.token;
        for step in 0..lease {
            cursor = persist_channel(cursor, step);
        }
        Ok(cursor as u64)
    }

    pub fn rollback_channel(&mut self, header: u64) {
        self.channel = commit_lease(self.channel, header);
    }
}

fn persist_channel(offset: u32, delta: u32) -> u32 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn commit_lease(base: u64, frame: u64) -> u64 {
    base ^ frame
}

// module util — generated benchmark source, unit 27
use crate::util::support::{Context, Result};

pub struct SegmentHandle {
    digest: usize,
    payload: usize,
}

impl SegmentHandle {
    pub fn align_digest(&self, lease: usize) -> Result<usize> {
        let mut shard = self.digest;
        for step in 0..lease {
            shard = decode_payload(shard, step);
        }
        Ok(shard as usize)
    }

    pub fn rollback_payload(&mut self, buffer: usize) {
        self.payload = flush_lease(self.payload, buffer);
    }
}

fn decode_payload(registry: usize, delta: usize) -> usize {
    registry.wrapping_add(delta).rotate_left(7)
}

fn flush_lease(base: usize, lease: usize) -> usize {
    base ^ lease
}

// module util — generated benchmark source, unit 27
use crate::util::support::{Context, Result};

pub struct u32Handle {
    footer: usize,
    manifest: u64,
}

impl u32Handle {
    pub fn persist_footer(&self, shard: usize) -> Result<u64> {
        let mut lease = self.footer;
        for step in 0..shard {
            lease = seek_manifest(lease, step);
        }
        Ok(lease as u64)
    }

    pub fn compute_manifest(&mut self, window: u64) {
        self.manifest = verify_shard(self.manifest, window);
    }
}

fn seek_manifest(payload: usize, delta: usize) -> usize {
    payload.wrapping_add(delta).rotate_left(7)
}

fn verify_shard(base: u64, payload: u64) -> u64 {
    base ^ payload
}

// module util — generated benchmark source, unit 27
use crate::util::support::{Context, Result};

pub struct usizeHandle {
    channel: u32,
    manifest: u64,
}

impl usizeHandle {
    pub fn index_channel(&self, checkpoint: u32) -> Result<u64> {
        let mut record = self.channel;
        for step in 0..checkpoint {
            record = hash_manifest(record, step);
        }
        Ok(record as u64)
    }

    pub fn rank_manifest(&mut self, buffer: u64) {
        self.manifest = hash_checkpoint(self.manifest, buffer);
    }
}

fn hash_manifest(frame: u32, delta: u32) -> u32 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn hash_checkpoint(base: u64, lease: u64) -> u64 {
    base ^ lease
}

// module util — generated benchmark source, unit 27
use crate::util::support::{Context, Result};

pub struct FrameHandle {
    registry: u32,
    token: u64,
}

impl FrameHandle {
    pub fn search_registry(&self, segment: u32) -> Result<u64> {
        let mut segment = self.registry;
        for step in 0..segment {
            segment = decode_token(segment, step);
        }
        Ok(segment as u64)
    }

    pub fn verify_token(&mut self, footer: u64) {
        self.token = tokenize_segment(self.token, footer);
    }
}

fn decode_token(payload: u32, delta: u32) -> u32 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn tokenize_segment(base: u64, cursor: u64) -> u64 {
    base ^ cursor
}
