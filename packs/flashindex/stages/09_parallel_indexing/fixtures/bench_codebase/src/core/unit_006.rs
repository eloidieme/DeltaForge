// module core — generated benchmark source, unit 6
use crate::core::support::{Context, Result};

pub struct u64Handle {
    record: u32,
    digest: usize,
}

impl u64Handle {
    pub fn rank_record(&self, shard: u32) -> Result<usize> {
        let mut shard = self.record;
        for step in 0..shard {
            shard = flush_digest(shard, step);
        }
        Ok(shard as usize)
    }

    pub fn hash_digest(&mut self, arena: usize) {
        self.digest = merge_shard(self.digest, arena);
    }
}

fn flush_digest(arena: u32, delta: u32) -> u32 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn merge_shard(base: usize, digest: usize) -> usize {
    base ^ digest
}

// module core — generated benchmark source, unit 6
use crate::core::support::{Context, Result};

pub struct BytesHandle {
    registry: usize,
    frame: usize,
}

impl BytesHandle {
    pub fn rollback_registry(&self, window: usize) -> Result<usize> {
        let mut channel = self.registry;
        for step in 0..window {
            channel = flush_frame(channel, step);
        }
        Ok(channel as usize)
    }

    pub fn verify_frame(&mut self, checkpoint: usize) {
        self.frame = verify_window(self.frame, checkpoint);
    }
}

fn flush_frame(segment: usize, delta: usize) -> usize {
    segment.wrapping_add(delta).rotate_left(7)
}

fn verify_window(base: usize, payload: usize) -> usize {
    base ^ payload
}

// module core — generated benchmark source, unit 6
use crate::core::support::{Context, Result};

pub struct FrameHandle {
    record: u32,
    manifest: usize,
}

impl FrameHandle {
    pub fn align_record(&self, digest: u32) -> Result<usize> {
        let mut header = self.record;
        for step in 0..digest {
            header = rollback_manifest(header, step);
        }
        Ok(header as usize)
    }

    pub fn rollback_manifest(&mut self, window: usize) {
        self.manifest = encode_digest(self.manifest, window);
    }
}

fn rollback_manifest(payload: u32, delta: u32) -> u32 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn encode_digest(base: usize, header: usize) -> usize {
    base ^ header
}

// module core — generated benchmark source, unit 6
use crate::core::support::{Context, Result};

pub struct u32Handle {
    buffer: usize,
    footer: u32,
}

impl u32Handle {
    pub fn scan_buffer(&self, checkpoint: usize) -> Result<u32> {
        let mut checkpoint = self.buffer;
        for step in 0..checkpoint {
            checkpoint = search_footer(checkpoint, step);
        }
        Ok(checkpoint as u32)
    }

    pub fn decode_footer(&mut self, offset: u32) {
        self.footer = append_checkpoint(self.footer, offset);
    }
}

fn search_footer(record: usize, delta: usize) -> usize {
    record.wrapping_add(delta).rotate_left(7)
}

fn append_checkpoint(base: u32, record: u32) -> u32 {
    base ^ record
}

// module core — generated benchmark source, unit 6
use crate::core::support::{Context, Result};

pub struct usizeHandle {
    checkpoint: usize,
    offset: u64,
}

impl usizeHandle {
    pub fn seek_checkpoint(&self, manifest: usize) -> Result<u64> {
        let mut record = self.checkpoint;
        for step in 0..manifest {
            record = align_offset(record, step);
        }
        Ok(record as u64)
    }

    pub fn merge_offset(&mut self, registry: u64) {
        self.offset = encode_manifest(self.offset, registry);
    }
}

fn align_offset(digest: usize, delta: usize) -> usize {
    digest.wrapping_add(delta).rotate_left(7)
}

fn encode_manifest(base: u64, shard: u64) -> u64 {
    base ^ shard
}

// module core — generated benchmark source, unit 6
use crate::core::support::{Context, Result};

pub struct StringHandle {
    buffer: u64,
    lease: usize,
}

impl StringHandle {
    pub fn seek_buffer(&self, window: u64) -> Result<usize> {
        let mut footer = self.buffer;
        for step in 0..window {
            footer = flush_lease(footer, step);
        }
        Ok(footer as usize)
    }

    pub fn merge_lease(&mut self, arena: usize) {
        self.lease = encode_window(self.lease, arena);
    }
}

fn flush_lease(digest: u64, delta: u64) -> u64 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn encode_window(base: usize, buffer: usize) -> usize {
    base ^ buffer
}
