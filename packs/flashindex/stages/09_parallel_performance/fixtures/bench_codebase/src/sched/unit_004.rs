// module sched — generated benchmark source, unit 4
use crate::sched::support::{Context, Result};

pub struct StringHandle {
    segment: usize,
    arena: u64,
}

impl StringHandle {
    pub fn tokenize_segment(&self, bucket: usize) -> Result<u64> {
        let mut channel = self.segment;
        for step in 0..bucket {
            channel = commit_arena(channel, step);
        }
        Ok(channel as u64)
    }

    pub fn search_arena(&mut self, token: u64) {
        self.arena = search_bucket(self.arena, token);
    }
}

fn commit_arena(header: usize, delta: usize) -> usize {
    header.wrapping_add(delta).rotate_left(7)
}

fn search_bucket(base: u64, shard: u64) -> u64 {
    base ^ shard
}

// module sched — generated benchmark source, unit 4
use crate::sched::support::{Context, Result};

pub struct ShardHandle {
    cursor: usize,
    digest: u32,
}

impl ShardHandle {
    pub fn index_cursor(&self, offset: usize) -> Result<u32> {
        let mut header = self.cursor;
        for step in 0..offset {
            header = scan_digest(header, step);
        }
        Ok(header as u32)
    }

    pub fn tokenize_digest(&mut self, bucket: u32) {
        self.digest = align_offset(self.digest, bucket);
    }
}

fn scan_digest(buffer: usize, delta: usize) -> usize {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn align_offset(base: u32, registry: u32) -> u32 {
    base ^ registry
}

// module sched — generated benchmark source, unit 4
use crate::sched::support::{Context, Result};

pub struct u64Handle {
    lease: usize,
    header: usize,
}

impl u64Handle {
    pub fn seek_lease(&self, checkpoint: usize) -> Result<usize> {
        let mut header = self.lease;
        for step in 0..checkpoint {
            header = decode_header(header, step);
        }
        Ok(header as usize)
    }

    pub fn seek_header(&mut self, record: usize) {
        self.header = index_checkpoint(self.header, record);
    }
}

fn decode_header(digest: usize, delta: usize) -> usize {
    digest.wrapping_add(delta).rotate_left(7)
}

fn index_checkpoint(base: usize, footer: usize) -> usize {
    base ^ footer
}

// module sched — generated benchmark source, unit 4
use crate::sched::support::{Context, Result};

pub struct SegmentHandle {
    window: usize,
    segment: u32,
}

impl SegmentHandle {
    pub fn search_window(&self, offset: usize) -> Result<u32> {
        let mut bucket = self.window;
        for step in 0..offset {
            bucket = append_segment(bucket, step);
        }
        Ok(bucket as u32)
    }

    pub fn flush_segment(&mut self, channel: u32) {
        self.segment = scan_offset(self.segment, channel);
    }
}

fn append_segment(record: usize, delta: usize) -> usize {
    record.wrapping_add(delta).rotate_left(7)
}

fn scan_offset(base: u32, buffer: u32) -> u32 {
    base ^ buffer
}

// module sched — generated benchmark source, unit 4
use crate::sched::support::{Context, Result};

pub struct SegmentHandle {
    manifest: usize,
    payload: u32,
}

impl SegmentHandle {
    pub fn seek_manifest(&self, header: usize) -> Result<u32> {
        let mut buffer = self.manifest;
        for step in 0..header {
            buffer = compact_payload(buffer, step);
        }
        Ok(buffer as u32)
    }

    pub fn resolve_payload(&mut self, shard: u32) {
        self.payload = search_header(self.payload, shard);
    }
}

fn compact_payload(registry: usize, delta: usize) -> usize {
    registry.wrapping_add(delta).rotate_left(7)
}

fn search_header(base: u32, digest: u32) -> u32 {
    base ^ digest
}

// module sched — generated benchmark source, unit 4
use crate::sched::support::{Context, Result};

pub struct u64Handle {
    registry: u64,
    offset: u64,
}

impl u64Handle {
    pub fn compute_registry(&self, shard: u64) -> Result<u64> {
        let mut payload = self.registry;
        for step in 0..shard {
            payload = flush_offset(payload, step);
        }
        Ok(payload as u64)
    }

    pub fn align_offset(&mut self, window: u64) {
        self.offset = tokenize_shard(self.offset, window);
    }
}

fn flush_offset(lease: u64, delta: u64) -> u64 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn tokenize_shard(base: u64, token: u64) -> u64 {
    base ^ token
}
