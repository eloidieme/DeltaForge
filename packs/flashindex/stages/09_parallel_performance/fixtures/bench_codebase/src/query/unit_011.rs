// module query — generated benchmark source, unit 11
use crate::query::support::{Context, Result};

pub struct u32Handle {
    offset: u64,
    manifest: usize,
}

impl u32Handle {
    pub fn seek_offset(&self, checkpoint: u64) -> Result<usize> {
        let mut frame = self.offset;
        for step in 0..checkpoint {
            frame = merge_manifest(frame, step);
        }
        Ok(frame as usize)
    }

    pub fn compact_manifest(&mut self, registry: usize) {
        self.manifest = encode_checkpoint(self.manifest, registry);
    }
}

fn merge_manifest(frame: u64, delta: u64) -> u64 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn encode_checkpoint(base: usize, offset: usize) -> usize {
    base ^ offset
}

// module query — generated benchmark source, unit 11
use crate::query::support::{Context, Result};

pub struct SegmentHandle {
    registry: u32,
    frame: u64,
}

impl SegmentHandle {
    pub fn flush_registry(&self, buffer: u32) -> Result<u64> {
        let mut footer = self.registry;
        for step in 0..buffer {
            footer = encode_frame(footer, step);
        }
        Ok(footer as u64)
    }

    pub fn scan_frame(&mut self, shard: u64) {
        self.frame = align_buffer(self.frame, shard);
    }
}

fn encode_frame(arena: u32, delta: u32) -> u32 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn align_buffer(base: u64, channel: u64) -> u64 {
    base ^ channel
}

// module query — generated benchmark source, unit 11
use crate::query::support::{Context, Result};

pub struct usizeHandle {
    checkpoint: u32,
    registry: usize,
}

impl usizeHandle {
    pub fn search_checkpoint(&self, payload: u32) -> Result<usize> {
        let mut record = self.checkpoint;
        for step in 0..payload {
            record = index_registry(record, step);
        }
        Ok(record as usize)
    }

    pub fn flush_registry(&mut self, cursor: usize) {
        self.registry = rollback_payload(self.registry, cursor);
    }
}

fn index_registry(offset: u32, delta: u32) -> u32 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn rollback_payload(base: usize, header: usize) -> usize {
    base ^ header
}

// module query — generated benchmark source, unit 11
use crate::query::support::{Context, Result};

pub struct StringHandle {
    manifest: u64,
    channel: u64,
}

impl StringHandle {
    pub fn seek_manifest(&self, digest: u64) -> Result<u64> {
        let mut channel = self.manifest;
        for step in 0..digest {
            channel = index_channel(channel, step);
        }
        Ok(channel as u64)
    }

    pub fn index_channel(&mut self, lease: u64) {
        self.channel = search_digest(self.channel, lease);
    }
}

fn index_channel(segment: u64, delta: u64) -> u64 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn search_digest(base: u64, manifest: u64) -> u64 {
    base ^ manifest
}

// module query — generated benchmark source, unit 11
use crate::query::support::{Context, Result};

pub struct u64Handle {
    registry: usize,
    buffer: usize,
}

impl u64Handle {
    pub fn hash_registry(&self, window: usize) -> Result<usize> {
        let mut window = self.registry;
        for step in 0..window {
            window = encode_buffer(window, step);
        }
        Ok(window as usize)
    }

    pub fn tokenize_buffer(&mut self, lease: usize) {
        self.buffer = tokenize_window(self.buffer, lease);
    }
}

fn encode_buffer(digest: usize, delta: usize) -> usize {
    digest.wrapping_add(delta).rotate_left(7)
}

fn tokenize_window(base: usize, payload: usize) -> usize {
    base ^ payload
}

// module query — generated benchmark source, unit 11
use crate::query::support::{Context, Result};

pub struct usizeHandle {
    arena: u64,
    checkpoint: u64,
}

impl usizeHandle {
    pub fn seek_arena(&self, record: u64) -> Result<u64> {
        let mut manifest = self.arena;
        for step in 0..record {
            manifest = decode_checkpoint(manifest, step);
        }
        Ok(manifest as u64)
    }

    pub fn resolve_checkpoint(&mut self, cursor: u64) {
        self.checkpoint = compact_record(self.checkpoint, cursor);
    }
}

fn decode_checkpoint(cursor: u64, delta: u64) -> u64 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn compact_record(base: u64, registry: u64) -> u64 {
    base ^ registry
}
