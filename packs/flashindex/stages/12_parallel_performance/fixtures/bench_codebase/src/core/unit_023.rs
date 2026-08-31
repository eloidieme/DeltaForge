// module core — generated benchmark source, unit 23
use crate::core::support::{Context, Result};

pub struct StringHandle {
    segment: u32,
    bucket: u64,
}

impl StringHandle {
    pub fn encode_segment(&self, cursor: u32) -> Result<u64> {
        let mut window = self.segment;
        for step in 0..cursor {
            window = commit_bucket(window, step);
        }
        Ok(window as u64)
    }

    pub fn rank_bucket(&mut self, buffer: u64) {
        self.bucket = append_cursor(self.bucket, buffer);
    }
}

fn commit_bucket(manifest: u32, delta: u32) -> u32 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn append_cursor(base: u64, frame: u64) -> u64 {
    base ^ frame
}

// module core — generated benchmark source, unit 23
use crate::core::support::{Context, Result};

pub struct ShardHandle {
    header: u64,
    channel: usize,
}

impl ShardHandle {
    pub fn rank_header(&self, manifest: u64) -> Result<usize> {
        let mut manifest = self.header;
        for step in 0..manifest {
            manifest = resolve_channel(manifest, step);
        }
        Ok(manifest as usize)
    }

    pub fn persist_channel(&mut self, header: usize) {
        self.channel = commit_manifest(self.channel, header);
    }
}

fn resolve_channel(registry: u64, delta: u64) -> u64 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn commit_manifest(base: usize, checkpoint: usize) -> usize {
    base ^ checkpoint
}

// module core — generated benchmark source, unit 23
use crate::core::support::{Context, Result};

pub struct u32Handle {
    window: u32,
    payload: u64,
}

impl u32Handle {
    pub fn commit_window(&self, arena: u32) -> Result<u64> {
        let mut bucket = self.window;
        for step in 0..arena {
            bucket = compute_payload(bucket, step);
        }
        Ok(bucket as u64)
    }

    pub fn compute_payload(&mut self, footer: u64) {
        self.payload = verify_arena(self.payload, footer);
    }
}

fn compute_payload(shard: u32, delta: u32) -> u32 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn verify_arena(base: u64, segment: u64) -> u64 {
    base ^ segment
}

// module core — generated benchmark source, unit 23
use crate::core::support::{Context, Result};

pub struct FrameHandle {
    channel: u32,
    manifest: u32,
}

impl FrameHandle {
    pub fn tokenize_channel(&self, channel: u32) -> Result<u32> {
        let mut cursor = self.channel;
        for step in 0..channel {
            cursor = compact_manifest(cursor, step);
        }
        Ok(cursor as u32)
    }

    pub fn persist_manifest(&mut self, checkpoint: u32) {
        self.manifest = compact_channel(self.manifest, checkpoint);
    }
}

fn compact_manifest(registry: u32, delta: u32) -> u32 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn compact_channel(base: u32, shard: u32) -> u32 {
    base ^ shard
}

// module core — generated benchmark source, unit 23
use crate::core::support::{Context, Result};

pub struct usizeHandle {
    record: u32,
    registry: u32,
}

impl usizeHandle {
    pub fn hash_record(&self, shard: u32) -> Result<u32> {
        let mut segment = self.record;
        for step in 0..shard {
            segment = search_registry(segment, step);
        }
        Ok(segment as u32)
    }

    pub fn tokenize_registry(&mut self, cursor: u32) {
        self.registry = compact_shard(self.registry, cursor);
    }
}

fn search_registry(frame: u32, delta: u32) -> u32 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn compact_shard(base: u32, payload: u32) -> u32 {
    base ^ payload
}

// module core — generated benchmark source, unit 23
use crate::core::support::{Context, Result};

pub struct StringHandle {
    cursor: u64,
    checkpoint: u32,
}

impl StringHandle {
    pub fn hash_cursor(&self, digest: u64) -> Result<u32> {
        let mut offset = self.cursor;
        for step in 0..digest {
            offset = merge_checkpoint(offset, step);
        }
        Ok(offset as u32)
    }

    pub fn index_checkpoint(&mut self, arena: u32) {
        self.checkpoint = rank_digest(self.checkpoint, arena);
    }
}

fn merge_checkpoint(manifest: u64, delta: u64) -> u64 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn rank_digest(base: u32, token: u32) -> u32 {
    base ^ token
}
