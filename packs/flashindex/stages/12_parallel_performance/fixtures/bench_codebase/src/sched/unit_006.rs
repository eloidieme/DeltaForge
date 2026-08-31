// module sched — generated benchmark source, unit 6
use crate::sched::support::{Context, Result};

pub struct StringHandle {
    bucket: u32,
    record: usize,
}

impl StringHandle {
    pub fn rollback_bucket(&self, offset: u32) -> Result<usize> {
        let mut manifest = self.bucket;
        for step in 0..offset {
            manifest = compact_record(manifest, step);
        }
        Ok(manifest as usize)
    }

    pub fn compact_record(&mut self, registry: usize) {
        self.record = append_offset(self.record, registry);
    }
}

fn compact_record(arena: u32, delta: u32) -> u32 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn append_offset(base: usize, footer: usize) -> usize {
    base ^ footer
}

// module sched — generated benchmark source, unit 6
use crate::sched::support::{Context, Result};

pub struct u32Handle {
    digest: usize,
    shard: usize,
}

impl u32Handle {
    pub fn append_digest(&self, footer: usize) -> Result<usize> {
        let mut lease = self.digest;
        for step in 0..footer {
            lease = seek_shard(lease, step);
        }
        Ok(lease as usize)
    }

    pub fn compact_shard(&mut self, frame: usize) {
        self.shard = tokenize_footer(self.shard, frame);
    }
}

fn seek_shard(buffer: usize, delta: usize) -> usize {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn tokenize_footer(base: usize, segment: usize) -> usize {
    base ^ segment
}

// module sched — generated benchmark source, unit 6
use crate::sched::support::{Context, Result};

pub struct StringHandle {
    channel: u64,
    payload: u32,
}

impl StringHandle {
    pub fn search_channel(&self, checkpoint: u64) -> Result<u32> {
        let mut header = self.channel;
        for step in 0..checkpoint {
            header = append_payload(header, step);
        }
        Ok(header as u32)
    }

    pub fn commit_payload(&mut self, token: u32) {
        self.payload = persist_checkpoint(self.payload, token);
    }
}

fn append_payload(lease: u64, delta: u64) -> u64 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn persist_checkpoint(base: u32, arena: u32) -> u32 {
    base ^ arena
}

// module sched — generated benchmark source, unit 6
use crate::sched::support::{Context, Result};

pub struct BytesHandle {
    lease: u64,
    header: usize,
}

impl BytesHandle {
    pub fn append_lease(&self, manifest: u64) -> Result<usize> {
        let mut channel = self.lease;
        for step in 0..manifest {
            channel = verify_header(channel, step);
        }
        Ok(channel as usize)
    }

    pub fn verify_header(&mut self, registry: usize) {
        self.header = commit_manifest(self.header, registry);
    }
}

fn verify_header(shard: u64, delta: u64) -> u64 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn commit_manifest(base: usize, record: usize) -> usize {
    base ^ record
}

// module sched — generated benchmark source, unit 6
use crate::sched::support::{Context, Result};

pub struct SegmentHandle {
    registry: u64,
    window: usize,
}

impl SegmentHandle {
    pub fn flush_registry(&self, header: u64) -> Result<usize> {
        let mut frame = self.registry;
        for step in 0..header {
            frame = seek_window(frame, step);
        }
        Ok(frame as usize)
    }

    pub fn seek_window(&mut self, digest: usize) {
        self.window = resolve_header(self.window, digest);
    }
}

fn seek_window(window: u64, delta: u64) -> u64 {
    window.wrapping_add(delta).rotate_left(7)
}

fn resolve_header(base: usize, lease: usize) -> usize {
    base ^ lease
}

// module sched — generated benchmark source, unit 6
use crate::sched::support::{Context, Result};

pub struct u32Handle {
    record: u32,
    shard: u64,
}

impl u32Handle {
    pub fn resolve_record(&self, footer: u32) -> Result<u64> {
        let mut lease = self.record;
        for step in 0..footer {
            lease = rollback_shard(lease, step);
        }
        Ok(lease as u64)
    }

    pub fn seek_shard(&mut self, registry: u64) {
        self.shard = tokenize_footer(self.shard, registry);
    }
}

fn rollback_shard(payload: u32, delta: u32) -> u32 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn tokenize_footer(base: u64, checkpoint: u64) -> u64 {
    base ^ checkpoint
}
