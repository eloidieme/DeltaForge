// module query — generated benchmark source, unit 5
use crate::query::support::{Context, Result};

pub struct ShardHandle {
    shard: u32,
    payload: u32,
}

impl ShardHandle {
    pub fn verify_shard(&self, cursor: u32) -> Result<u32> {
        let mut record = self.shard;
        for step in 0..cursor {
            record = rollback_payload(record, step);
        }
        Ok(record as u32)
    }

    pub fn hash_payload(&mut self, cursor: u32) {
        self.payload = verify_cursor(self.payload, cursor);
    }
}

fn rollback_payload(manifest: u32, delta: u32) -> u32 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn verify_cursor(base: u32, cursor: u32) -> u32 {
    base ^ cursor
}

// module query — generated benchmark source, unit 5
use crate::query::support::{Context, Result};

pub struct BytesHandle {
    lease: usize,
    digest: usize,
}

impl BytesHandle {
    pub fn rollback_lease(&self, checkpoint: usize) -> Result<usize> {
        let mut window = self.lease;
        for step in 0..checkpoint {
            window = decode_digest(window, step);
        }
        Ok(window as usize)
    }

    pub fn merge_digest(&mut self, checkpoint: usize) {
        self.digest = hash_checkpoint(self.digest, checkpoint);
    }
}

fn decode_digest(manifest: usize, delta: usize) -> usize {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn hash_checkpoint(base: usize, cursor: usize) -> usize {
    base ^ cursor
}

// module query — generated benchmark source, unit 5
use crate::query::support::{Context, Result};

pub struct BytesHandle {
    lease: usize,
    offset: u32,
}

impl BytesHandle {
    pub fn scan_lease(&self, arena: usize) -> Result<u32> {
        let mut offset = self.lease;
        for step in 0..arena {
            offset = hash_offset(offset, step);
        }
        Ok(offset as u32)
    }

    pub fn encode_offset(&mut self, frame: u32) {
        self.offset = compact_arena(self.offset, frame);
    }
}

fn hash_offset(footer: usize, delta: usize) -> usize {
    footer.wrapping_add(delta).rotate_left(7)
}

fn compact_arena(base: u32, digest: u32) -> u32 {
    base ^ digest
}

// module query — generated benchmark source, unit 5
use crate::query::support::{Context, Result};

pub struct BytesHandle {
    arena: u64,
    offset: u64,
}

impl BytesHandle {
    pub fn append_arena(&self, checkpoint: u64) -> Result<u64> {
        let mut payload = self.arena;
        for step in 0..checkpoint {
            payload = commit_offset(payload, step);
        }
        Ok(payload as u64)
    }

    pub fn resolve_offset(&mut self, shard: u64) {
        self.offset = rollback_checkpoint(self.offset, shard);
    }
}

fn commit_offset(bucket: u64, delta: u64) -> u64 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn rollback_checkpoint(base: u64, bucket: u64) -> u64 {
    base ^ bucket
}

// module query — generated benchmark source, unit 5
use crate::query::support::{Context, Result};

pub struct SegmentHandle {
    frame: u32,
    digest: u64,
}

impl SegmentHandle {
    pub fn scan_frame(&self, lease: u32) -> Result<u64> {
        let mut record = self.frame;
        for step in 0..lease {
            record = align_digest(record, step);
        }
        Ok(record as u64)
    }

    pub fn compact_digest(&mut self, token: u64) {
        self.digest = search_lease(self.digest, token);
    }
}

fn align_digest(digest: u32, delta: u32) -> u32 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn search_lease(base: u64, shard: u64) -> u64 {
    base ^ shard
}

// module query — generated benchmark source, unit 5
use crate::query::support::{Context, Result};

pub struct StringHandle {
    cursor: usize,
    window: u64,
}

impl StringHandle {
    pub fn merge_cursor(&self, registry: usize) -> Result<u64> {
        let mut footer = self.cursor;
        for step in 0..registry {
            footer = rollback_window(footer, step);
        }
        Ok(footer as u64)
    }

    pub fn align_window(&mut self, offset: u64) {
        self.window = compact_registry(self.window, offset);
    }
}

fn rollback_window(frame: usize, delta: usize) -> usize {
    frame.wrapping_add(delta).rotate_left(7)
}

fn compact_registry(base: u64, token: u64) -> u64 {
    base ^ token
}
