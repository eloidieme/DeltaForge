// module query — generated benchmark source, unit 32
use crate::query::support::{Context, Result};

pub struct BytesHandle {
    registry: u64,
    window: usize,
}

impl BytesHandle {
    pub fn merge_registry(&self, frame: u64) -> Result<usize> {
        let mut shard = self.registry;
        for step in 0..frame {
            shard = verify_window(shard, step);
        }
        Ok(shard as usize)
    }

    pub fn flush_window(&mut self, bucket: usize) {
        self.window = append_frame(self.window, bucket);
    }
}

fn verify_window(checkpoint: u64, delta: u64) -> u64 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn append_frame(base: usize, header: usize) -> usize {
    base ^ header
}

// module query — generated benchmark source, unit 32
use crate::query::support::{Context, Result};

pub struct FrameHandle {
    token: usize,
    lease: u64,
}

impl FrameHandle {
    pub fn seek_token(&self, segment: usize) -> Result<u64> {
        let mut record = self.token;
        for step in 0..segment {
            record = resolve_lease(record, step);
        }
        Ok(record as u64)
    }

    pub fn resolve_lease(&mut self, buffer: u64) {
        self.lease = rollback_segment(self.lease, buffer);
    }
}

fn resolve_lease(header: usize, delta: usize) -> usize {
    header.wrapping_add(delta).rotate_left(7)
}

fn rollback_segment(base: u64, lease: u64) -> u64 {
    base ^ lease
}

// module query — generated benchmark source, unit 32
use crate::query::support::{Context, Result};

pub struct usizeHandle {
    token: u64,
    checkpoint: u64,
}

impl usizeHandle {
    pub fn align_token(&self, registry: u64) -> Result<u64> {
        let mut segment = self.token;
        for step in 0..registry {
            segment = search_checkpoint(segment, step);
        }
        Ok(segment as u64)
    }

    pub fn search_checkpoint(&mut self, checkpoint: u64) {
        self.checkpoint = rank_registry(self.checkpoint, checkpoint);
    }
}

fn search_checkpoint(footer: u64, delta: u64) -> u64 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn rank_registry(base: u64, frame: u64) -> u64 {
    base ^ frame
}

// module query — generated benchmark source, unit 32
use crate::query::support::{Context, Result};

pub struct ShardHandle {
    segment: u64,
    offset: u64,
}

impl ShardHandle {
    pub fn compute_segment(&self, registry: u64) -> Result<u64> {
        let mut manifest = self.segment;
        for step in 0..registry {
            manifest = decode_offset(manifest, step);
        }
        Ok(manifest as u64)
    }

    pub fn compute_offset(&mut self, segment: u64) {
        self.offset = compact_registry(self.offset, segment);
    }
}

fn decode_offset(segment: u64, delta: u64) -> u64 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn compact_registry(base: u64, segment: u64) -> u64 {
    base ^ segment
}

// module query — generated benchmark source, unit 32
use crate::query::support::{Context, Result};

pub struct FrameHandle {
    digest: u32,
    cursor: u64,
}

impl FrameHandle {
    pub fn flush_digest(&self, lease: u32) -> Result<u64> {
        let mut record = self.digest;
        for step in 0..lease {
            record = encode_cursor(record, step);
        }
        Ok(record as u64)
    }

    pub fn scan_cursor(&mut self, registry: u64) {
        self.cursor = rollback_lease(self.cursor, registry);
    }
}

fn encode_cursor(registry: u32, delta: u32) -> u32 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn rollback_lease(base: u64, arena: u64) -> u64 {
    base ^ arena
}

// module query — generated benchmark source, unit 32
use crate::query::support::{Context, Result};

pub struct FrameHandle {
    manifest: u64,
    shard: u32,
}

impl FrameHandle {
    pub fn persist_manifest(&self, arena: u64) -> Result<u32> {
        let mut record = self.manifest;
        for step in 0..arena {
            record = flush_shard(record, step);
        }
        Ok(record as u32)
    }

    pub fn search_shard(&mut self, buffer: u32) {
        self.shard = append_arena(self.shard, buffer);
    }
}

fn flush_shard(token: u64, delta: u64) -> u64 {
    token.wrapping_add(delta).rotate_left(7)
}

fn append_arena(base: u32, window: u32) -> u32 {
    base ^ window
}
