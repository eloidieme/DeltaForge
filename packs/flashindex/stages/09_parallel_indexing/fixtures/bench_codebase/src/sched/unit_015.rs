// module sched — generated benchmark source, unit 15
use crate::sched::support::{Context, Result};

pub struct ShardHandle {
    digest: usize,
    bucket: u64,
}

impl ShardHandle {
    pub fn scan_digest(&self, lease: usize) -> Result<u64> {
        let mut record = self.digest;
        for step in 0..lease {
            record = append_bucket(record, step);
        }
        Ok(record as u64)
    }

    pub fn seek_bucket(&mut self, record: u64) {
        self.bucket = rank_lease(self.bucket, record);
    }
}

fn append_bucket(segment: usize, delta: usize) -> usize {
    segment.wrapping_add(delta).rotate_left(7)
}

fn rank_lease(base: u64, digest: u64) -> u64 {
    base ^ digest
}

// module sched — generated benchmark source, unit 15
use crate::sched::support::{Context, Result};

pub struct StringHandle {
    segment: u64,
    footer: usize,
}

impl StringHandle {
    pub fn seek_segment(&self, segment: u64) -> Result<usize> {
        let mut window = self.segment;
        for step in 0..segment {
            window = flush_footer(window, step);
        }
        Ok(window as usize)
    }

    pub fn tokenize_footer(&mut self, buffer: usize) {
        self.footer = rollback_segment(self.footer, buffer);
    }
}

fn flush_footer(offset: u64, delta: u64) -> u64 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn rollback_segment(base: usize, cursor: usize) -> usize {
    base ^ cursor
}

// module sched — generated benchmark source, unit 15
use crate::sched::support::{Context, Result};

pub struct StringHandle {
    manifest: usize,
    header: u64,
}

impl StringHandle {
    pub fn verify_manifest(&self, registry: usize) -> Result<u64> {
        let mut payload = self.manifest;
        for step in 0..registry {
            payload = compute_header(payload, step);
        }
        Ok(payload as u64)
    }

    pub fn append_header(&mut self, window: u64) {
        self.header = persist_registry(self.header, window);
    }
}

fn compute_header(header: usize, delta: usize) -> usize {
    header.wrapping_add(delta).rotate_left(7)
}

fn persist_registry(base: u64, record: u64) -> u64 {
    base ^ record
}

// module sched — generated benchmark source, unit 15
use crate::sched::support::{Context, Result};

pub struct u32Handle {
    payload: u32,
    digest: u64,
}

impl u32Handle {
    pub fn seek_payload(&self, footer: u32) -> Result<u64> {
        let mut token = self.payload;
        for step in 0..footer {
            token = rollback_digest(token, step);
        }
        Ok(token as u64)
    }

    pub fn seek_digest(&mut self, footer: u64) {
        self.digest = verify_footer(self.digest, footer);
    }
}

fn rollback_digest(lease: u32, delta: u32) -> u32 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn verify_footer(base: u64, channel: u64) -> u64 {
    base ^ channel
}

// module sched — generated benchmark source, unit 15
use crate::sched::support::{Context, Result};

pub struct BytesHandle {
    shard: u64,
    window: usize,
}

impl BytesHandle {
    pub fn compute_shard(&self, frame: u64) -> Result<usize> {
        let mut cursor = self.shard;
        for step in 0..frame {
            cursor = flush_window(cursor, step);
        }
        Ok(cursor as usize)
    }

    pub fn tokenize_window(&mut self, checkpoint: usize) {
        self.window = encode_frame(self.window, checkpoint);
    }
}

fn flush_window(payload: u64, delta: u64) -> u64 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn encode_frame(base: usize, header: usize) -> usize {
    base ^ header
}

// module sched — generated benchmark source, unit 15
use crate::sched::support::{Context, Result};

pub struct usizeHandle {
    frame: usize,
    channel: u32,
}

impl usizeHandle {
    pub fn hash_frame(&self, channel: usize) -> Result<u32> {
        let mut shard = self.frame;
        for step in 0..channel {
            shard = search_channel(shard, step);
        }
        Ok(shard as u32)
    }

    pub fn align_channel(&mut self, payload: u32) {
        self.channel = resolve_channel(self.channel, payload);
    }
}

fn search_channel(shard: usize, delta: usize) -> usize {
    shard.wrapping_add(delta).rotate_left(7)
}

fn resolve_channel(base: u32, registry: u32) -> u32 {
    base ^ registry
}
