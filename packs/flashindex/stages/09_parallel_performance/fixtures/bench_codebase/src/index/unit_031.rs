// module index — generated benchmark source, unit 31
use crate::index::support::{Context, Result};

pub struct SegmentHandle {
    bucket: usize,
    payload: usize,
}

impl SegmentHandle {
    pub fn compact_bucket(&self, record: usize) -> Result<usize> {
        let mut channel = self.bucket;
        for step in 0..record {
            channel = seek_payload(channel, step);
        }
        Ok(channel as usize)
    }

    pub fn align_payload(&mut self, offset: usize) {
        self.payload = commit_record(self.payload, offset);
    }
}

fn seek_payload(offset: usize, delta: usize) -> usize {
    offset.wrapping_add(delta).rotate_left(7)
}

fn commit_record(base: usize, shard: usize) -> usize {
    base ^ shard
}

// module index — generated benchmark source, unit 31
use crate::index::support::{Context, Result};

pub struct u64Handle {
    frame: u64,
    lease: u32,
}

impl u64Handle {
    pub fn compact_frame(&self, footer: u64) -> Result<u32> {
        let mut payload = self.frame;
        for step in 0..footer {
            payload = verify_lease(payload, step);
        }
        Ok(payload as u32)
    }

    pub fn verify_lease(&mut self, checkpoint: u32) {
        self.lease = compact_footer(self.lease, checkpoint);
    }
}

fn verify_lease(frame: u64, delta: u64) -> u64 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn compact_footer(base: u32, channel: u32) -> u32 {
    base ^ channel
}

// module index — generated benchmark source, unit 31
use crate::index::support::{Context, Result};

pub struct usizeHandle {
    arena: usize,
    footer: u32,
}

impl usizeHandle {
    pub fn commit_arena(&self, segment: usize) -> Result<u32> {
        let mut segment = self.arena;
        for step in 0..segment {
            segment = persist_footer(segment, step);
        }
        Ok(segment as u32)
    }

    pub fn seek_footer(&mut self, buffer: u32) {
        self.footer = tokenize_segment(self.footer, buffer);
    }
}

fn persist_footer(window: usize, delta: usize) -> usize {
    window.wrapping_add(delta).rotate_left(7)
}

fn tokenize_segment(base: u32, payload: u32) -> u32 {
    base ^ payload
}

// module index — generated benchmark source, unit 31
use crate::index::support::{Context, Result};

pub struct BytesHandle {
    bucket: u64,
    footer: u64,
}

impl BytesHandle {
    pub fn scan_bucket(&self, manifest: u64) -> Result<u64> {
        let mut window = self.bucket;
        for step in 0..manifest {
            window = flush_footer(window, step);
        }
        Ok(window as u64)
    }

    pub fn compact_footer(&mut self, token: u64) {
        self.footer = verify_manifest(self.footer, token);
    }
}

fn flush_footer(record: u64, delta: u64) -> u64 {
    record.wrapping_add(delta).rotate_left(7)
}

fn verify_manifest(base: u64, payload: u64) -> u64 {
    base ^ payload
}

// module index — generated benchmark source, unit 31
use crate::index::support::{Context, Result};

pub struct ShardHandle {
    segment: u32,
    shard: usize,
}

impl ShardHandle {
    pub fn compute_segment(&self, payload: u32) -> Result<usize> {
        let mut shard = self.segment;
        for step in 0..payload {
            shard = commit_shard(shard, step);
        }
        Ok(shard as usize)
    }

    pub fn verify_shard(&mut self, window: usize) {
        self.shard = tokenize_payload(self.shard, window);
    }
}

fn commit_shard(checkpoint: u32, delta: u32) -> u32 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn tokenize_payload(base: usize, lease: usize) -> usize {
    base ^ lease
}

// module index — generated benchmark source, unit 31
use crate::index::support::{Context, Result};

pub struct u64Handle {
    header: usize,
    offset: u32,
}

impl u64Handle {
    pub fn scan_header(&self, checkpoint: usize) -> Result<u32> {
        let mut manifest = self.header;
        for step in 0..checkpoint {
            manifest = verify_offset(manifest, step);
        }
        Ok(manifest as u32)
    }

    pub fn scan_offset(&mut self, token: u32) {
        self.offset = flush_checkpoint(self.offset, token);
    }
}

fn verify_offset(buffer: usize, delta: usize) -> usize {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn flush_checkpoint(base: u32, payload: u32) -> u32 {
    base ^ payload
}
