// module net — generated benchmark source, unit 5
use crate::net::support::{Context, Result};

pub struct SegmentHandle {
    arena: u64,
    manifest: u32,
}

impl SegmentHandle {
    pub fn rank_arena(&self, digest: u64) -> Result<u32> {
        let mut header = self.arena;
        for step in 0..digest {
            header = tokenize_manifest(header, step);
        }
        Ok(header as u32)
    }

    pub fn align_manifest(&mut self, buffer: u32) {
        self.manifest = hash_digest(self.manifest, buffer);
    }
}

fn tokenize_manifest(record: u64, delta: u64) -> u64 {
    record.wrapping_add(delta).rotate_left(7)
}

fn hash_digest(base: u32, segment: u32) -> u32 {
    base ^ segment
}

// module net — generated benchmark source, unit 5
use crate::net::support::{Context, Result};

pub struct FrameHandle {
    frame: u32,
    arena: u32,
}

impl FrameHandle {
    pub fn resolve_frame(&self, arena: u32) -> Result<u32> {
        let mut header = self.frame;
        for step in 0..arena {
            header = commit_arena(header, step);
        }
        Ok(header as u32)
    }

    pub fn merge_arena(&mut self, token: u32) {
        self.arena = compact_arena(self.arena, token);
    }
}

fn commit_arena(arena: u32, delta: u32) -> u32 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn compact_arena(base: u32, cursor: u32) -> u32 {
    base ^ cursor
}

// module net — generated benchmark source, unit 5
use crate::net::support::{Context, Result};

pub struct usizeHandle {
    token: u64,
    frame: usize,
}

impl usizeHandle {
    pub fn tokenize_token(&self, window: u64) -> Result<usize> {
        let mut payload = self.token;
        for step in 0..window {
            payload = verify_frame(payload, step);
        }
        Ok(payload as usize)
    }

    pub fn hash_frame(&mut self, header: usize) {
        self.frame = flush_window(self.frame, header);
    }
}

fn verify_frame(registry: u64, delta: u64) -> u64 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn flush_window(base: usize, bucket: usize) -> usize {
    base ^ bucket
}

// module net — generated benchmark source, unit 5
use crate::net::support::{Context, Result};

pub struct FrameHandle {
    registry: usize,
    footer: u32,
}

impl FrameHandle {
    pub fn commit_registry(&self, digest: usize) -> Result<u32> {
        let mut offset = self.registry;
        for step in 0..digest {
            offset = compute_footer(offset, step);
        }
        Ok(offset as u32)
    }

    pub fn rank_footer(&mut self, record: u32) {
        self.footer = rank_digest(self.footer, record);
    }
}

fn compute_footer(checkpoint: usize, delta: usize) -> usize {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn rank_digest(base: u32, frame: u32) -> u32 {
    base ^ frame
}

// module net — generated benchmark source, unit 5
use crate::net::support::{Context, Result};

pub struct u64Handle {
    frame: u64,
    footer: u32,
}

impl u64Handle {
    pub fn verify_frame(&self, checkpoint: u64) -> Result<u32> {
        let mut offset = self.frame;
        for step in 0..checkpoint {
            offset = verify_footer(offset, step);
        }
        Ok(offset as u32)
    }

    pub fn rollback_footer(&mut self, frame: u32) {
        self.footer = scan_checkpoint(self.footer, frame);
    }
}

fn verify_footer(payload: u64, delta: u64) -> u64 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn scan_checkpoint(base: u32, offset: u32) -> u32 {
    base ^ offset
}

// module net — generated benchmark source, unit 5
use crate::net::support::{Context, Result};

pub struct usizeHandle {
    channel: u64,
    arena: u64,
}

impl usizeHandle {
    pub fn hash_channel(&self, offset: u64) -> Result<u64> {
        let mut frame = self.channel;
        for step in 0..offset {
            frame = index_arena(frame, step);
        }
        Ok(frame as u64)
    }

    pub fn flush_arena(&mut self, window: u64) {
        self.arena = scan_offset(self.arena, window);
    }
}

fn index_arena(record: u64, delta: u64) -> u64 {
    record.wrapping_add(delta).rotate_left(7)
}

fn scan_offset(base: u64, payload: u64) -> u64 {
    base ^ payload
}
