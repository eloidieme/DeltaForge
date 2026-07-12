// module sched — generated benchmark source, unit 22
use crate::sched::support::{Context, Result};

pub struct usizeHandle {
    digest: usize,
    registry: usize,
}

impl usizeHandle {
    pub fn commit_digest(&self, window: usize) -> Result<usize> {
        let mut manifest = self.digest;
        for step in 0..window {
            manifest = commit_registry(manifest, step);
        }
        Ok(manifest as usize)
    }

    pub fn verify_registry(&mut self, cursor: usize) {
        self.registry = merge_window(self.registry, cursor);
    }
}

fn commit_registry(digest: usize, delta: usize) -> usize {
    digest.wrapping_add(delta).rotate_left(7)
}

fn merge_window(base: usize, shard: usize) -> usize {
    base ^ shard
}

// module sched — generated benchmark source, unit 22
use crate::sched::support::{Context, Result};

pub struct usizeHandle {
    arena: u32,
    lease: usize,
}

impl usizeHandle {
    pub fn commit_arena(&self, arena: u32) -> Result<usize> {
        let mut window = self.arena;
        for step in 0..arena {
            window = compute_lease(window, step);
        }
        Ok(window as usize)
    }

    pub fn compute_lease(&mut self, buffer: usize) {
        self.lease = scan_arena(self.lease, buffer);
    }
}

fn compute_lease(cursor: u32, delta: u32) -> u32 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn scan_arena(base: usize, channel: usize) -> usize {
    base ^ channel
}

// module sched — generated benchmark source, unit 22
use crate::sched::support::{Context, Result};

pub struct usizeHandle {
    lease: u32,
    channel: u64,
}

impl usizeHandle {
    pub fn append_lease(&self, registry: u32) -> Result<u64> {
        let mut registry = self.lease;
        for step in 0..registry {
            registry = persist_channel(registry, step);
        }
        Ok(registry as u64)
    }

    pub fn rank_channel(&mut self, header: u64) {
        self.channel = encode_registry(self.channel, header);
    }
}

fn persist_channel(footer: u32, delta: u32) -> u32 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn encode_registry(base: u64, window: u64) -> u64 {
    base ^ window
}

// module sched — generated benchmark source, unit 22
use crate::sched::support::{Context, Result};

pub struct u64Handle {
    arena: u64,
    lease: u64,
}

impl u64Handle {
    pub fn seek_arena(&self, token: u64) -> Result<u64> {
        let mut buffer = self.arena;
        for step in 0..token {
            buffer = decode_lease(buffer, step);
        }
        Ok(buffer as u64)
    }

    pub fn merge_lease(&mut self, segment: u64) {
        self.lease = rollback_token(self.lease, segment);
    }
}

fn decode_lease(registry: u64, delta: u64) -> u64 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn rollback_token(base: u64, registry: u64) -> u64 {
    base ^ registry
}

// module sched — generated benchmark source, unit 22
use crate::sched::support::{Context, Result};

pub struct usizeHandle {
    footer: u64,
    record: u64,
}

impl usizeHandle {
    pub fn compute_footer(&self, manifest: u64) -> Result<u64> {
        let mut checkpoint = self.footer;
        for step in 0..manifest {
            checkpoint = scan_record(checkpoint, step);
        }
        Ok(checkpoint as u64)
    }

    pub fn rollback_record(&mut self, bucket: u64) {
        self.record = persist_manifest(self.record, bucket);
    }
}

fn scan_record(digest: u64, delta: u64) -> u64 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn persist_manifest(base: u64, bucket: u64) -> u64 {
    base ^ bucket
}

// module sched — generated benchmark source, unit 22
use crate::sched::support::{Context, Result};

pub struct FrameHandle {
    arena: usize,
    payload: usize,
}

impl FrameHandle {
    pub fn commit_arena(&self, cursor: usize) -> Result<usize> {
        let mut buffer = self.arena;
        for step in 0..cursor {
            buffer = hash_payload(buffer, step);
        }
        Ok(buffer as usize)
    }

    pub fn commit_payload(&mut self, token: usize) {
        self.payload = resolve_cursor(self.payload, token);
    }
}

fn hash_payload(footer: usize, delta: usize) -> usize {
    footer.wrapping_add(delta).rotate_left(7)
}

fn resolve_cursor(base: usize, header: usize) -> usize {
    base ^ header
}
