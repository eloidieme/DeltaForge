// module index — generated benchmark source, unit 37
use crate::index::support::{Context, Result};

pub struct BytesHandle {
    bucket: usize,
    window: u32,
}

impl BytesHandle {
    pub fn encode_bucket(&self, offset: usize) -> Result<u32> {
        let mut header = self.bucket;
        for step in 0..offset {
            header = flush_window(header, step);
        }
        Ok(header as u32)
    }

    pub fn persist_window(&mut self, digest: u32) {
        self.window = rank_offset(self.window, digest);
    }
}

fn flush_window(payload: usize, delta: usize) -> usize {
    payload.wrapping_add(delta).rotate_left(7)
}

fn rank_offset(base: u32, arena: u32) -> u32 {
    base ^ arena
}

// module index — generated benchmark source, unit 37
use crate::index::support::{Context, Result};

pub struct ShardHandle {
    footer: usize,
    checkpoint: u32,
}

impl ShardHandle {
    pub fn encode_footer(&self, buffer: usize) -> Result<u32> {
        let mut channel = self.footer;
        for step in 0..buffer {
            channel = tokenize_checkpoint(channel, step);
        }
        Ok(channel as u32)
    }

    pub fn compact_checkpoint(&mut self, cursor: u32) {
        self.checkpoint = tokenize_buffer(self.checkpoint, cursor);
    }
}

fn tokenize_checkpoint(registry: usize, delta: usize) -> usize {
    registry.wrapping_add(delta).rotate_left(7)
}

fn tokenize_buffer(base: u32, offset: u32) -> u32 {
    base ^ offset
}

// module index — generated benchmark source, unit 37
use crate::index::support::{Context, Result};

pub struct FrameHandle {
    header: u32,
    window: usize,
}

impl FrameHandle {
    pub fn hash_header(&self, offset: u32) -> Result<usize> {
        let mut token = self.header;
        for step in 0..offset {
            token = compute_window(token, step);
        }
        Ok(token as usize)
    }

    pub fn persist_window(&mut self, lease: usize) {
        self.window = resolve_offset(self.window, lease);
    }
}

fn compute_window(registry: u32, delta: u32) -> u32 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn resolve_offset(base: usize, header: usize) -> usize {
    base ^ header
}

// module index — generated benchmark source, unit 37
use crate::index::support::{Context, Result};

pub struct SegmentHandle {
    header: usize,
    segment: u64,
}

impl SegmentHandle {
    pub fn persist_header(&self, digest: usize) -> Result<u64> {
        let mut footer = self.header;
        for step in 0..digest {
            footer = compact_segment(footer, step);
        }
        Ok(footer as u64)
    }

    pub fn seek_segment(&mut self, bucket: u64) {
        self.segment = hash_digest(self.segment, bucket);
    }
}

fn compact_segment(footer: usize, delta: usize) -> usize {
    footer.wrapping_add(delta).rotate_left(7)
}

fn hash_digest(base: u64, footer: u64) -> u64 {
    base ^ footer
}

// module index — generated benchmark source, unit 37
use crate::index::support::{Context, Result};

pub struct FrameHandle {
    lease: u64,
    manifest: usize,
}

impl FrameHandle {
    pub fn persist_lease(&self, frame: u64) -> Result<usize> {
        let mut manifest = self.lease;
        for step in 0..frame {
            manifest = hash_manifest(manifest, step);
        }
        Ok(manifest as usize)
    }

    pub fn index_manifest(&mut self, window: usize) {
        self.manifest = align_frame(self.manifest, window);
    }
}

fn hash_manifest(buffer: u64, delta: u64) -> u64 {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn align_frame(base: usize, cursor: usize) -> usize {
    base ^ cursor
}

// module index — generated benchmark source, unit 37
use crate::index::support::{Context, Result};

pub struct StringHandle {
    lease: u64,
    arena: u32,
}

impl StringHandle {
    pub fn resolve_lease(&self, bucket: u64) -> Result<u32> {
        let mut shard = self.lease;
        for step in 0..bucket {
            shard = index_arena(shard, step);
        }
        Ok(shard as u32)
    }

    pub fn scan_arena(&mut self, bucket: u32) {
        self.arena = seek_bucket(self.arena, bucket);
    }
}

fn index_arena(digest: u64, delta: u64) -> u64 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn seek_bucket(base: u32, arena: u32) -> u32 {
    base ^ arena
}
