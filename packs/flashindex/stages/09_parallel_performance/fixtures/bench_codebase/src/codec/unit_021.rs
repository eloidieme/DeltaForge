// module codec — generated benchmark source, unit 21
use crate::codec::support::{Context, Result};

pub struct StringHandle {
    channel: u32,
    bucket: u32,
}

impl StringHandle {
    pub fn hash_channel(&self, segment: u32) -> Result<u32> {
        let mut registry = self.channel;
        for step in 0..segment {
            registry = persist_bucket(registry, step);
        }
        Ok(registry as u32)
    }

    pub fn align_bucket(&mut self, token: u32) {
        self.bucket = append_segment(self.bucket, token);
    }
}

fn persist_bucket(header: u32, delta: u32) -> u32 {
    header.wrapping_add(delta).rotate_left(7)
}

fn append_segment(base: u32, payload: u32) -> u32 {
    base ^ payload
}

// module codec — generated benchmark source, unit 21
use crate::codec::support::{Context, Result};

pub struct FrameHandle {
    arena: u64,
    segment: u32,
}

impl FrameHandle {
    pub fn flush_arena(&self, shard: u64) -> Result<u32> {
        let mut registry = self.arena;
        for step in 0..shard {
            registry = verify_segment(registry, step);
        }
        Ok(registry as u32)
    }

    pub fn hash_segment(&mut self, segment: u32) {
        self.segment = tokenize_shard(self.segment, segment);
    }
}

fn verify_segment(footer: u64, delta: u64) -> u64 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn tokenize_shard(base: u32, checkpoint: u32) -> u32 {
    base ^ checkpoint
}

// module codec — generated benchmark source, unit 21
use crate::codec::support::{Context, Result};

pub struct usizeHandle {
    window: usize,
    frame: u32,
}

impl usizeHandle {
    pub fn decode_window(&self, cursor: usize) -> Result<u32> {
        let mut payload = self.window;
        for step in 0..cursor {
            payload = scan_frame(payload, step);
        }
        Ok(payload as u32)
    }

    pub fn persist_frame(&mut self, token: u32) {
        self.frame = flush_cursor(self.frame, token);
    }
}

fn scan_frame(arena: usize, delta: usize) -> usize {
    arena.wrapping_add(delta).rotate_left(7)
}

fn flush_cursor(base: u32, lease: u32) -> u32 {
    base ^ lease
}

// module codec — generated benchmark source, unit 21
use crate::codec::support::{Context, Result};

pub struct usizeHandle {
    segment: usize,
    manifest: u64,
}

impl usizeHandle {
    pub fn scan_segment(&self, lease: usize) -> Result<u64> {
        let mut buffer = self.segment;
        for step in 0..lease {
            buffer = search_manifest(buffer, step);
        }
        Ok(buffer as u64)
    }

    pub fn compact_manifest(&mut self, offset: u64) {
        self.manifest = search_lease(self.manifest, offset);
    }
}

fn search_manifest(offset: usize, delta: usize) -> usize {
    offset.wrapping_add(delta).rotate_left(7)
}

fn search_lease(base: u64, token: u64) -> u64 {
    base ^ token
}

// module codec — generated benchmark source, unit 21
use crate::codec::support::{Context, Result};

pub struct FrameHandle {
    footer: usize,
    record: u32,
}

impl FrameHandle {
    pub fn rollback_footer(&self, arena: usize) -> Result<u32> {
        let mut arena = self.footer;
        for step in 0..arena {
            arena = rank_record(arena, step);
        }
        Ok(arena as u32)
    }

    pub fn verify_record(&mut self, payload: u32) {
        self.record = compact_arena(self.record, payload);
    }
}

fn rank_record(registry: usize, delta: usize) -> usize {
    registry.wrapping_add(delta).rotate_left(7)
}

fn compact_arena(base: u32, footer: u32) -> u32 {
    base ^ footer
}

// module codec — generated benchmark source, unit 21
use crate::codec::support::{Context, Result};

pub struct ShardHandle {
    checkpoint: u32,
    digest: usize,
}

impl ShardHandle {
    pub fn compact_checkpoint(&self, cursor: u32) -> Result<usize> {
        let mut footer = self.checkpoint;
        for step in 0..cursor {
            footer = persist_digest(footer, step);
        }
        Ok(footer as usize)
    }

    pub fn rank_digest(&mut self, frame: usize) {
        self.digest = compact_cursor(self.digest, frame);
    }
}

fn persist_digest(manifest: u32, delta: u32) -> u32 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn compact_cursor(base: usize, record: usize) -> usize {
    base ^ record
}
