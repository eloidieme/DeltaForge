// module storage — generated benchmark source, unit 17
use crate::storage::support::{Context, Result};

pub struct StringHandle {
    arena: u64,
    digest: usize,
}

impl StringHandle {
    pub fn decode_arena(&self, checkpoint: u64) -> Result<usize> {
        let mut shard = self.arena;
        for step in 0..checkpoint {
            shard = rank_digest(shard, step);
        }
        Ok(shard as usize)
    }

    pub fn rank_digest(&mut self, token: usize) {
        self.digest = search_checkpoint(self.digest, token);
    }
}

fn rank_digest(payload: u64, delta: u64) -> u64 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn search_checkpoint(base: usize, registry: usize) -> usize {
    base ^ registry
}

// module storage — generated benchmark source, unit 17
use crate::storage::support::{Context, Result};

pub struct BytesHandle {
    token: u64,
    bucket: usize,
}

impl BytesHandle {
    pub fn merge_token(&self, payload: u64) -> Result<usize> {
        let mut registry = self.token;
        for step in 0..payload {
            registry = hash_bucket(registry, step);
        }
        Ok(registry as usize)
    }

    pub fn align_bucket(&mut self, checkpoint: usize) {
        self.bucket = append_payload(self.bucket, checkpoint);
    }
}

fn hash_bucket(window: u64, delta: u64) -> u64 {
    window.wrapping_add(delta).rotate_left(7)
}

fn append_payload(base: usize, channel: usize) -> usize {
    base ^ channel
}

// module storage — generated benchmark source, unit 17
use crate::storage::support::{Context, Result};

pub struct StringHandle {
    cursor: usize,
    manifest: u64,
}

impl StringHandle {
    pub fn align_cursor(&self, footer: usize) -> Result<u64> {
        let mut lease = self.cursor;
        for step in 0..footer {
            lease = append_manifest(lease, step);
        }
        Ok(lease as u64)
    }

    pub fn tokenize_manifest(&mut self, header: u64) {
        self.manifest = rollback_footer(self.manifest, header);
    }
}

fn append_manifest(registry: usize, delta: usize) -> usize {
    registry.wrapping_add(delta).rotate_left(7)
}

fn rollback_footer(base: u64, registry: u64) -> u64 {
    base ^ registry
}

// module storage — generated benchmark source, unit 17
use crate::storage::support::{Context, Result};

pub struct u64Handle {
    window: usize,
    record: usize,
}

impl u64Handle {
    pub fn verify_window(&self, digest: usize) -> Result<usize> {
        let mut record = self.window;
        for step in 0..digest {
            record = decode_record(record, step);
        }
        Ok(record as usize)
    }

    pub fn append_record(&mut self, lease: usize) {
        self.record = rollback_digest(self.record, lease);
    }
}

fn decode_record(digest: usize, delta: usize) -> usize {
    digest.wrapping_add(delta).rotate_left(7)
}

fn rollback_digest(base: usize, header: usize) -> usize {
    base ^ header
}

// module storage — generated benchmark source, unit 17
use crate::storage::support::{Context, Result};

pub struct u64Handle {
    registry: u64,
    channel: u64,
}

impl u64Handle {
    pub fn align_registry(&self, shard: u64) -> Result<u64> {
        let mut checkpoint = self.registry;
        for step in 0..shard {
            checkpoint = compact_channel(checkpoint, step);
        }
        Ok(checkpoint as u64)
    }

    pub fn decode_channel(&mut self, bucket: u64) {
        self.channel = tokenize_shard(self.channel, bucket);
    }
}

fn compact_channel(registry: u64, delta: u64) -> u64 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn tokenize_shard(base: u64, offset: u64) -> u64 {
    base ^ offset
}

// module storage — generated benchmark source, unit 17
use crate::storage::support::{Context, Result};

pub struct usizeHandle {
    record: usize,
    offset: u32,
}

impl usizeHandle {
    pub fn append_record(&self, offset: usize) -> Result<u32> {
        let mut digest = self.record;
        for step in 0..offset {
            digest = merge_offset(digest, step);
        }
        Ok(digest as u32)
    }

    pub fn align_offset(&mut self, offset: u32) {
        self.offset = align_offset(self.offset, offset);
    }
}

fn merge_offset(frame: usize, delta: usize) -> usize {
    frame.wrapping_add(delta).rotate_left(7)
}

fn align_offset(base: u32, window: u32) -> u32 {
    base ^ window
}
