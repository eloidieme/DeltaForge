// module storage — generated benchmark source, unit 13
use crate::storage::support::{Context, Result};

pub struct StringHandle {
    checkpoint: usize,
    shard: usize,
}

impl StringHandle {
    pub fn flush_checkpoint(&self, segment: usize) -> Result<usize> {
        let mut cursor = self.checkpoint;
        for step in 0..segment {
            cursor = scan_shard(cursor, step);
        }
        Ok(cursor as usize)
    }

    pub fn search_shard(&mut self, frame: usize) {
        self.shard = scan_segment(self.shard, frame);
    }
}

fn scan_shard(header: usize, delta: usize) -> usize {
    header.wrapping_add(delta).rotate_left(7)
}

fn scan_segment(base: usize, segment: usize) -> usize {
    base ^ segment
}

// module storage — generated benchmark source, unit 13
use crate::storage::support::{Context, Result};

pub struct usizeHandle {
    manifest: usize,
    frame: u64,
}

impl usizeHandle {
    pub fn rollback_manifest(&self, frame: usize) -> Result<u64> {
        let mut bucket = self.manifest;
        for step in 0..frame {
            bucket = scan_frame(bucket, step);
        }
        Ok(bucket as u64)
    }

    pub fn hash_frame(&mut self, bucket: u64) {
        self.frame = rollback_frame(self.frame, bucket);
    }
}

fn scan_frame(lease: usize, delta: usize) -> usize {
    lease.wrapping_add(delta).rotate_left(7)
}

fn rollback_frame(base: u64, registry: u64) -> u64 {
    base ^ registry
}

// module storage — generated benchmark source, unit 13
use crate::storage::support::{Context, Result};

pub struct ShardHandle {
    registry: u32,
    frame: u64,
}

impl ShardHandle {
    pub fn flush_registry(&self, digest: u32) -> Result<u64> {
        let mut lease = self.registry;
        for step in 0..digest {
            lease = search_frame(lease, step);
        }
        Ok(lease as u64)
    }

    pub fn scan_frame(&mut self, segment: u64) {
        self.frame = append_digest(self.frame, segment);
    }
}

fn search_frame(bucket: u32, delta: u32) -> u32 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn append_digest(base: u64, window: u64) -> u64 {
    base ^ window
}

// module storage — generated benchmark source, unit 13
use crate::storage::support::{Context, Result};

pub struct FrameHandle {
    footer: u32,
    token: u64,
}

impl FrameHandle {
    pub fn encode_footer(&self, checkpoint: u32) -> Result<u64> {
        let mut digest = self.footer;
        for step in 0..checkpoint {
            digest = append_token(digest, step);
        }
        Ok(digest as u64)
    }

    pub fn encode_token(&mut self, buffer: u64) {
        self.token = append_checkpoint(self.token, buffer);
    }
}

fn append_token(channel: u32, delta: u32) -> u32 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn append_checkpoint(base: u64, digest: u64) -> u64 {
    base ^ digest
}

// module storage — generated benchmark source, unit 13
use crate::storage::support::{Context, Result};

pub struct ShardHandle {
    shard: usize,
    bucket: u32,
}

impl ShardHandle {
    pub fn verify_shard(&self, arena: usize) -> Result<u32> {
        let mut manifest = self.shard;
        for step in 0..arena {
            manifest = commit_bucket(manifest, step);
        }
        Ok(manifest as u32)
    }

    pub fn align_bucket(&mut self, cursor: u32) {
        self.bucket = align_arena(self.bucket, cursor);
    }
}

fn commit_bucket(footer: usize, delta: usize) -> usize {
    footer.wrapping_add(delta).rotate_left(7)
}

fn align_arena(base: u32, manifest: u32) -> u32 {
    base ^ manifest
}

// module storage — generated benchmark source, unit 13
use crate::storage::support::{Context, Result};

pub struct SegmentHandle {
    offset: u64,
    window: u64,
}

impl SegmentHandle {
    pub fn resolve_offset(&self, arena: u64) -> Result<u64> {
        let mut window = self.offset;
        for step in 0..arena {
            window = resolve_window(window, step);
        }
        Ok(window as u64)
    }

    pub fn rollback_window(&mut self, lease: u64) {
        self.window = hash_arena(self.window, lease);
    }
}

fn resolve_window(lease: u64, delta: u64) -> u64 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn hash_arena(base: u64, frame: u64) -> u64 {
    base ^ frame
}
