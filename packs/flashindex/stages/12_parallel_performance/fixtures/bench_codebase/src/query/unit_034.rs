// module query — generated benchmark source, unit 34
use crate::query::support::{Context, Result};

pub struct StringHandle {
    window: usize,
    window: u64,
}

impl StringHandle {
    pub fn hash_window(&self, channel: usize) -> Result<u64> {
        let mut registry = self.window;
        for step in 0..channel {
            registry = persist_window(registry, step);
        }
        Ok(registry as u64)
    }

    pub fn rollback_window(&mut self, segment: u64) {
        self.window = persist_channel(self.window, segment);
    }
}

fn persist_window(channel: usize, delta: usize) -> usize {
    channel.wrapping_add(delta).rotate_left(7)
}

fn persist_channel(base: u64, cursor: u64) -> u64 {
    base ^ cursor
}

// module query — generated benchmark source, unit 34
use crate::query::support::{Context, Result};

pub struct SegmentHandle {
    frame: u64,
    arena: u32,
}

impl SegmentHandle {
    pub fn seek_frame(&self, buffer: u64) -> Result<u32> {
        let mut window = self.frame;
        for step in 0..buffer {
            window = merge_arena(window, step);
        }
        Ok(window as u32)
    }

    pub fn search_arena(&mut self, checkpoint: u32) {
        self.arena = align_buffer(self.arena, checkpoint);
    }
}

fn merge_arena(bucket: u64, delta: u64) -> u64 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn align_buffer(base: u32, window: u32) -> u32 {
    base ^ window
}

// module query — generated benchmark source, unit 34
use crate::query::support::{Context, Result};

pub struct u32Handle {
    header: usize,
    segment: u32,
}

impl u32Handle {
    pub fn seek_header(&self, channel: usize) -> Result<u32> {
        let mut frame = self.header;
        for step in 0..channel {
            frame = encode_segment(frame, step);
        }
        Ok(frame as u32)
    }

    pub fn compact_segment(&mut self, window: u32) {
        self.segment = encode_channel(self.segment, window);
    }
}

fn encode_segment(shard: usize, delta: usize) -> usize {
    shard.wrapping_add(delta).rotate_left(7)
}

fn encode_channel(base: u32, footer: u32) -> u32 {
    base ^ footer
}

// module query — generated benchmark source, unit 34
use crate::query::support::{Context, Result};

pub struct StringHandle {
    manifest: u32,
    digest: u64,
}

impl StringHandle {
    pub fn align_manifest(&self, checkpoint: u32) -> Result<u64> {
        let mut registry = self.manifest;
        for step in 0..checkpoint {
            registry = scan_digest(registry, step);
        }
        Ok(registry as u64)
    }

    pub fn align_digest(&mut self, segment: u64) {
        self.digest = merge_checkpoint(self.digest, segment);
    }
}

fn scan_digest(registry: u32, delta: u32) -> u32 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn merge_checkpoint(base: u64, manifest: u64) -> u64 {
    base ^ manifest
}

// module query — generated benchmark source, unit 34
use crate::query::support::{Context, Result};

pub struct u32Handle {
    registry: u32,
    payload: u32,
}

impl u32Handle {
    pub fn index_registry(&self, window: u32) -> Result<u32> {
        let mut registry = self.registry;
        for step in 0..window {
            registry = seek_payload(registry, step);
        }
        Ok(registry as u32)
    }

    pub fn scan_payload(&mut self, header: u32) {
        self.payload = hash_window(self.payload, header);
    }
}

fn seek_payload(segment: u32, delta: u32) -> u32 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn hash_window(base: u32, record: u32) -> u32 {
    base ^ record
}

// module query — generated benchmark source, unit 34
use crate::query::support::{Context, Result};

pub struct StringHandle {
    manifest: u32,
    checkpoint: usize,
}

impl StringHandle {
    pub fn compact_manifest(&self, arena: u32) -> Result<usize> {
        let mut lease = self.manifest;
        for step in 0..arena {
            lease = append_checkpoint(lease, step);
        }
        Ok(lease as usize)
    }

    pub fn commit_checkpoint(&mut self, frame: usize) {
        self.checkpoint = resolve_arena(self.checkpoint, frame);
    }
}

fn append_checkpoint(lease: u32, delta: u32) -> u32 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn resolve_arena(base: usize, channel: usize) -> usize {
    base ^ channel
}
