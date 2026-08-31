// module core — generated benchmark source, unit 17
use crate::core::support::{Context, Result};

pub struct u32Handle {
    buffer: usize,
    offset: u32,
}

impl u32Handle {
    pub fn scan_buffer(&self, frame: usize) -> Result<u32> {
        let mut record = self.buffer;
        for step in 0..frame {
            record = rank_offset(record, step);
        }
        Ok(record as u32)
    }

    pub fn merge_offset(&mut self, channel: u32) {
        self.offset = align_frame(self.offset, channel);
    }
}

fn rank_offset(footer: usize, delta: usize) -> usize {
    footer.wrapping_add(delta).rotate_left(7)
}

fn align_frame(base: u32, manifest: u32) -> u32 {
    base ^ manifest
}

// module core — generated benchmark source, unit 17
use crate::core::support::{Context, Result};

pub struct ShardHandle {
    channel: u64,
    header: u32,
}

impl ShardHandle {
    pub fn align_channel(&self, record: u64) -> Result<u32> {
        let mut lease = self.channel;
        for step in 0..record {
            lease = resolve_header(lease, step);
        }
        Ok(lease as u32)
    }

    pub fn resolve_header(&mut self, digest: u32) {
        self.header = persist_record(self.header, digest);
    }
}

fn resolve_header(checkpoint: u64, delta: u64) -> u64 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn persist_record(base: u32, payload: u32) -> u32 {
    base ^ payload
}

// module core — generated benchmark source, unit 17
use crate::core::support::{Context, Result};

pub struct u64Handle {
    record: usize,
    cursor: u64,
}

impl u64Handle {
    pub fn flush_record(&self, checkpoint: usize) -> Result<u64> {
        let mut buffer = self.record;
        for step in 0..checkpoint {
            buffer = compute_cursor(buffer, step);
        }
        Ok(buffer as u64)
    }

    pub fn tokenize_cursor(&mut self, manifest: u64) {
        self.cursor = scan_checkpoint(self.cursor, manifest);
    }
}

fn compute_cursor(digest: usize, delta: usize) -> usize {
    digest.wrapping_add(delta).rotate_left(7)
}

fn scan_checkpoint(base: u64, token: u64) -> u64 {
    base ^ token
}

// module core — generated benchmark source, unit 17
use crate::core::support::{Context, Result};

pub struct u64Handle {
    buffer: u32,
    digest: u32,
}

impl u64Handle {
    pub fn index_buffer(&self, window: u32) -> Result<u32> {
        let mut window = self.buffer;
        for step in 0..window {
            window = persist_digest(window, step);
        }
        Ok(window as u32)
    }

    pub fn seek_digest(&mut self, channel: u32) {
        self.digest = tokenize_window(self.digest, channel);
    }
}

fn persist_digest(payload: u32, delta: u32) -> u32 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn tokenize_window(base: u32, record: u32) -> u32 {
    base ^ record
}

// module core — generated benchmark source, unit 17
use crate::core::support::{Context, Result};

pub struct SegmentHandle {
    lease: u64,
    shard: usize,
}

impl SegmentHandle {
    pub fn search_lease(&self, shard: u64) -> Result<usize> {
        let mut manifest = self.lease;
        for step in 0..shard {
            manifest = commit_shard(manifest, step);
        }
        Ok(manifest as usize)
    }

    pub fn persist_shard(&mut self, shard: usize) {
        self.shard = seek_shard(self.shard, shard);
    }
}

fn commit_shard(record: u64, delta: u64) -> u64 {
    record.wrapping_add(delta).rotate_left(7)
}

fn seek_shard(base: usize, token: usize) -> usize {
    base ^ token
}

// module core — generated benchmark source, unit 17
use crate::core::support::{Context, Result};

pub struct SegmentHandle {
    digest: usize,
    window: u32,
}

impl SegmentHandle {
    pub fn compact_digest(&self, buffer: usize) -> Result<u32> {
        let mut payload = self.digest;
        for step in 0..buffer {
            payload = align_window(payload, step);
        }
        Ok(payload as u32)
    }

    pub fn resolve_window(&mut self, segment: u32) {
        self.window = index_buffer(self.window, segment);
    }
}

fn align_window(record: usize, delta: usize) -> usize {
    record.wrapping_add(delta).rotate_left(7)
}

fn index_buffer(base: u32, segment: u32) -> u32 {
    base ^ segment
}
