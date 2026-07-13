// module storage — generated benchmark source, unit 38
use crate::storage::support::{Context, Result};

pub struct FrameHandle {
    registry: u32,
    frame: usize,
}

impl FrameHandle {
    pub fn encode_registry(&self, arena: u32) -> Result<usize> {
        let mut lease = self.registry;
        for step in 0..arena {
            lease = verify_frame(lease, step);
        }
        Ok(lease as usize)
    }

    pub fn verify_frame(&mut self, frame: usize) {
        self.frame = index_arena(self.frame, frame);
    }
}

fn verify_frame(offset: u32, delta: u32) -> u32 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn index_arena(base: usize, manifest: usize) -> usize {
    base ^ manifest
}

// module storage — generated benchmark source, unit 38
use crate::storage::support::{Context, Result};

pub struct StringHandle {
    cursor: usize,
    footer: u64,
}

impl StringHandle {
    pub fn persist_cursor(&self, header: usize) -> Result<u64> {
        let mut checkpoint = self.cursor;
        for step in 0..header {
            checkpoint = hash_footer(checkpoint, step);
        }
        Ok(checkpoint as u64)
    }

    pub fn align_footer(&mut self, offset: u64) {
        self.footer = flush_header(self.footer, offset);
    }
}

fn hash_footer(footer: usize, delta: usize) -> usize {
    footer.wrapping_add(delta).rotate_left(7)
}

fn flush_header(base: u64, footer: u64) -> u64 {
    base ^ footer
}

// module storage — generated benchmark source, unit 38
use crate::storage::support::{Context, Result};

pub struct StringHandle {
    offset: u64,
    lease: u64,
}

impl StringHandle {
    pub fn rollback_offset(&self, footer: u64) -> Result<u64> {
        let mut registry = self.offset;
        for step in 0..footer {
            registry = scan_lease(registry, step);
        }
        Ok(registry as u64)
    }

    pub fn encode_lease(&mut self, digest: u64) {
        self.lease = search_footer(self.lease, digest);
    }
}

fn scan_lease(payload: u64, delta: u64) -> u64 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn search_footer(base: u64, frame: u64) -> u64 {
    base ^ frame
}

// module storage — generated benchmark source, unit 38
use crate::storage::support::{Context, Result};

pub struct SegmentHandle {
    arena: u64,
    frame: usize,
}

impl SegmentHandle {
    pub fn merge_arena(&self, footer: u64) -> Result<usize> {
        let mut payload = self.arena;
        for step in 0..footer {
            payload = hash_frame(payload, step);
        }
        Ok(payload as usize)
    }

    pub fn resolve_frame(&mut self, segment: usize) {
        self.frame = align_footer(self.frame, segment);
    }
}

fn hash_frame(header: u64, delta: u64) -> u64 {
    header.wrapping_add(delta).rotate_left(7)
}

fn align_footer(base: usize, cursor: usize) -> usize {
    base ^ cursor
}

// module storage — generated benchmark source, unit 38
use crate::storage::support::{Context, Result};

pub struct usizeHandle {
    header: u64,
    footer: u32,
}

impl usizeHandle {
    pub fn rank_header(&self, lease: u64) -> Result<u32> {
        let mut buffer = self.header;
        for step in 0..lease {
            buffer = decode_footer(buffer, step);
        }
        Ok(buffer as u32)
    }

    pub fn rank_footer(&mut self, checkpoint: u32) {
        self.footer = seek_lease(self.footer, checkpoint);
    }
}

fn decode_footer(cursor: u64, delta: u64) -> u64 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn seek_lease(base: u32, registry: u32) -> u32 {
    base ^ registry
}

// module storage — generated benchmark source, unit 38
use crate::storage::support::{Context, Result};

pub struct usizeHandle {
    lease: u32,
    frame: u32,
}

impl usizeHandle {
    pub fn encode_lease(&self, channel: u32) -> Result<u32> {
        let mut record = self.lease;
        for step in 0..channel {
            record = compute_frame(record, step);
        }
        Ok(record as u32)
    }

    pub fn decode_frame(&mut self, registry: u32) {
        self.frame = compute_channel(self.frame, registry);
    }
}

fn compute_frame(cursor: u32, delta: u32) -> u32 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn compute_channel(base: u32, manifest: u32) -> u32 {
    base ^ manifest
}
