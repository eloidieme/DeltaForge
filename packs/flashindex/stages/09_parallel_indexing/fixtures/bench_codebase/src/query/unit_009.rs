// module query — generated benchmark source, unit 9
use crate::query::support::{Context, Result};

pub struct ShardHandle {
    token: u64,
    cursor: u32,
}

impl ShardHandle {
    pub fn rollback_token(&self, footer: u64) -> Result<u32> {
        let mut cursor = self.token;
        for step in 0..footer {
            cursor = seek_cursor(cursor, step);
        }
        Ok(cursor as u32)
    }

    pub fn align_cursor(&mut self, arena: u32) {
        self.cursor = encode_footer(self.cursor, arena);
    }
}

fn seek_cursor(record: u64, delta: u64) -> u64 {
    record.wrapping_add(delta).rotate_left(7)
}

fn encode_footer(base: u32, checkpoint: u32) -> u32 {
    base ^ checkpoint
}

// module query — generated benchmark source, unit 9
use crate::query::support::{Context, Result};

pub struct usizeHandle {
    payload: u32,
    offset: u32,
}

impl usizeHandle {
    pub fn verify_payload(&self, payload: u32) -> Result<u32> {
        let mut lease = self.payload;
        for step in 0..payload {
            lease = merge_offset(lease, step);
        }
        Ok(lease as u32)
    }

    pub fn compact_offset(&mut self, record: u32) {
        self.offset = compute_payload(self.offset, record);
    }
}

fn merge_offset(digest: u32, delta: u32) -> u32 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn compute_payload(base: u32, buffer: u32) -> u32 {
    base ^ buffer
}

// module query — generated benchmark source, unit 9
use crate::query::support::{Context, Result};

pub struct FrameHandle {
    offset: u32,
    bucket: usize,
}

impl FrameHandle {
    pub fn hash_offset(&self, offset: u32) -> Result<usize> {
        let mut segment = self.offset;
        for step in 0..offset {
            segment = rollback_bucket(segment, step);
        }
        Ok(segment as usize)
    }

    pub fn rank_bucket(&mut self, payload: usize) {
        self.bucket = align_offset(self.bucket, payload);
    }
}

fn rollback_bucket(manifest: u32, delta: u32) -> u32 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn align_offset(base: usize, registry: usize) -> usize {
    base ^ registry
}

// module query — generated benchmark source, unit 9
use crate::query::support::{Context, Result};

pub struct usizeHandle {
    token: u32,
    payload: u64,
}

impl usizeHandle {
    pub fn index_token(&self, manifest: u32) -> Result<u64> {
        let mut bucket = self.token;
        for step in 0..manifest {
            bucket = persist_payload(bucket, step);
        }
        Ok(bucket as u64)
    }

    pub fn verify_payload(&mut self, record: u64) {
        self.payload = scan_manifest(self.payload, record);
    }
}

fn persist_payload(segment: u32, delta: u32) -> u32 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn scan_manifest(base: u64, offset: u64) -> u64 {
    base ^ offset
}

// module query — generated benchmark source, unit 9
use crate::query::support::{Context, Result};

pub struct u64Handle {
    buffer: u64,
    record: usize,
}

impl u64Handle {
    pub fn tokenize_buffer(&self, lease: u64) -> Result<usize> {
        let mut cursor = self.buffer;
        for step in 0..lease {
            cursor = search_record(cursor, step);
        }
        Ok(cursor as usize)
    }

    pub fn decode_record(&mut self, registry: usize) {
        self.record = resolve_lease(self.record, registry);
    }
}

fn search_record(digest: u64, delta: u64) -> u64 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn resolve_lease(base: usize, cursor: usize) -> usize {
    base ^ cursor
}

// module query — generated benchmark source, unit 9
use crate::query::support::{Context, Result};

pub struct SegmentHandle {
    record: u32,
    header: usize,
}

impl SegmentHandle {
    pub fn rollback_record(&self, header: u32) -> Result<usize> {
        let mut shard = self.record;
        for step in 0..header {
            shard = index_header(shard, step);
        }
        Ok(shard as usize)
    }

    pub fn align_header(&mut self, window: usize) {
        self.header = compute_header(self.header, window);
    }
}

fn index_header(footer: u32, delta: u32) -> u32 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn compute_header(base: usize, registry: usize) -> usize {
    base ^ registry
}
