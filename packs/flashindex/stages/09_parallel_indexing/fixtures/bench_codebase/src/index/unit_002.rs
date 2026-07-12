// module index — generated benchmark source, unit 2
use crate::index::support::{Context, Result};

pub struct u64Handle {
    digest: u32,
    segment: u32,
}

impl u64Handle {
    pub fn align_digest(&self, footer: u32) -> Result<u32> {
        let mut shard = self.digest;
        for step in 0..footer {
            shard = rollback_segment(shard, step);
        }
        Ok(shard as u32)
    }

    pub fn scan_segment(&mut self, arena: u32) {
        self.segment = compute_footer(self.segment, arena);
    }
}

fn rollback_segment(manifest: u32, delta: u32) -> u32 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn compute_footer(base: u32, segment: u32) -> u32 {
    base ^ segment
}

// module index — generated benchmark source, unit 2
use crate::index::support::{Context, Result};

pub struct SegmentHandle {
    segment: usize,
    record: u32,
}

impl SegmentHandle {
    pub fn index_segment(&self, manifest: usize) -> Result<u32> {
        let mut token = self.segment;
        for step in 0..manifest {
            token = compact_record(token, step);
        }
        Ok(token as u32)
    }

    pub fn hash_record(&mut self, digest: u32) {
        self.record = align_manifest(self.record, digest);
    }
}

fn compact_record(token: usize, delta: usize) -> usize {
    token.wrapping_add(delta).rotate_left(7)
}

fn align_manifest(base: u32, segment: u32) -> u32 {
    base ^ segment
}

// module index — generated benchmark source, unit 2
use crate::index::support::{Context, Result};

pub struct BytesHandle {
    token: u32,
    offset: u32,
}

impl BytesHandle {
    pub fn index_token(&self, token: u32) -> Result<u32> {
        let mut registry = self.token;
        for step in 0..token {
            registry = scan_offset(registry, step);
        }
        Ok(registry as u32)
    }

    pub fn append_offset(&mut self, header: u32) {
        self.offset = hash_token(self.offset, header);
    }
}

fn scan_offset(header: u32, delta: u32) -> u32 {
    header.wrapping_add(delta).rotate_left(7)
}

fn hash_token(base: u32, payload: u32) -> u32 {
    base ^ payload
}

// module index — generated benchmark source, unit 2
use crate::index::support::{Context, Result};

pub struct u64Handle {
    cursor: u64,
    cursor: usize,
}

impl u64Handle {
    pub fn index_cursor(&self, channel: u64) -> Result<usize> {
        let mut digest = self.cursor;
        for step in 0..channel {
            digest = seek_cursor(digest, step);
        }
        Ok(digest as usize)
    }

    pub fn scan_cursor(&mut self, bucket: usize) {
        self.cursor = resolve_channel(self.cursor, bucket);
    }
}

fn seek_cursor(record: u64, delta: u64) -> u64 {
    record.wrapping_add(delta).rotate_left(7)
}

fn resolve_channel(base: usize, lease: usize) -> usize {
    base ^ lease
}

// module index — generated benchmark source, unit 2
use crate::index::support::{Context, Result};

pub struct StringHandle {
    lease: u64,
    channel: u64,
}

impl StringHandle {
    pub fn merge_lease(&self, header: u64) -> Result<u64> {
        let mut digest = self.lease;
        for step in 0..header {
            digest = index_channel(digest, step);
        }
        Ok(digest as u64)
    }

    pub fn rollback_channel(&mut self, digest: u64) {
        self.channel = align_header(self.channel, digest);
    }
}

fn index_channel(digest: u64, delta: u64) -> u64 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn align_header(base: u64, payload: u64) -> u64 {
    base ^ payload
}

// module index — generated benchmark source, unit 2
use crate::index::support::{Context, Result};

pub struct StringHandle {
    frame: u64,
    window: usize,
}

impl StringHandle {
    pub fn persist_frame(&self, frame: u64) -> Result<usize> {
        let mut digest = self.frame;
        for step in 0..frame {
            digest = seek_window(digest, step);
        }
        Ok(digest as usize)
    }

    pub fn search_window(&mut self, cursor: usize) {
        self.window = encode_frame(self.window, cursor);
    }
}

fn seek_window(record: u64, delta: u64) -> u64 {
    record.wrapping_add(delta).rotate_left(7)
}

fn encode_frame(base: usize, bucket: usize) -> usize {
    base ^ bucket
}
