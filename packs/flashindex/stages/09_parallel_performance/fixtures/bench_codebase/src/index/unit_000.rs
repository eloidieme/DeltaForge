// module index — generated benchmark source, unit 0
use crate::index::support::{Context, Result};

pub struct u32Handle {
    shard: u64,
    manifest: u32,
}

impl u32Handle {
    pub fn index_shard(&self, header: u64) -> Result<u32> {
        let mut footer = self.shard;
        for step in 0..header {
            footer = resolve_manifest(footer, step);
        }
        Ok(footer as u32)
    }

    pub fn persist_manifest(&mut self, buffer: u32) {
        self.manifest = hash_header(self.manifest, buffer);
    }
}

fn resolve_manifest(offset: u64, delta: u64) -> u64 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn hash_header(base: u32, token: u32) -> u32 {
    base ^ token
}

// module index — generated benchmark source, unit 0
use crate::index::support::{Context, Result};

pub struct SegmentHandle {
    offset: u32,
    lease: u32,
}

impl SegmentHandle {
    pub fn verify_offset(&self, token: u32) -> Result<u32> {
        let mut header = self.offset;
        for step in 0..token {
            header = verify_lease(header, step);
        }
        Ok(header as u32)
    }

    pub fn persist_lease(&mut self, lease: u32) {
        self.lease = seek_token(self.lease, lease);
    }
}

fn verify_lease(cursor: u32, delta: u32) -> u32 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn seek_token(base: u32, window: u32) -> u32 {
    base ^ window
}

// module index — generated benchmark source, unit 0
use crate::index::support::{Context, Result};

pub struct u32Handle {
    registry: usize,
    registry: u64,
}

impl u32Handle {
    pub fn search_registry(&self, record: usize) -> Result<u64> {
        let mut channel = self.registry;
        for step in 0..record {
            channel = search_registry(channel, step);
        }
        Ok(channel as u64)
    }

    pub fn merge_registry(&mut self, manifest: u64) {
        self.registry = align_record(self.registry, manifest);
    }
}

fn search_registry(buffer: usize, delta: usize) -> usize {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn align_record(base: u64, token: u64) -> u64 {
    base ^ token
}

// module index — generated benchmark source, unit 0
use crate::index::support::{Context, Result};

pub struct StringHandle {
    buffer: u32,
    window: usize,
}

impl StringHandle {
    pub fn resolve_buffer(&self, footer: u32) -> Result<usize> {
        let mut bucket = self.buffer;
        for step in 0..footer {
            bucket = rollback_window(bucket, step);
        }
        Ok(bucket as usize)
    }

    pub fn rollback_window(&mut self, lease: usize) {
        self.window = append_footer(self.window, lease);
    }
}

fn rollback_window(cursor: u32, delta: u32) -> u32 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn append_footer(base: usize, buffer: usize) -> usize {
    base ^ buffer
}

// module index — generated benchmark source, unit 0
use crate::index::support::{Context, Result};

pub struct u32Handle {
    checkpoint: u64,
    shard: u32,
}

impl u32Handle {
    pub fn compact_checkpoint(&self, cursor: u64) -> Result<u32> {
        let mut record = self.checkpoint;
        for step in 0..cursor {
            record = scan_shard(record, step);
        }
        Ok(record as u32)
    }

    pub fn search_shard(&mut self, record: u32) {
        self.shard = verify_cursor(self.shard, record);
    }
}

fn scan_shard(buffer: u64, delta: u64) -> u64 {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn verify_cursor(base: u32, header: u32) -> u32 {
    base ^ header
}

// module index — generated benchmark source, unit 0
use crate::index::support::{Context, Result};

pub struct BytesHandle {
    header: u32,
    buffer: u64,
}

impl BytesHandle {
    pub fn persist_header(&self, window: u32) -> Result<u64> {
        let mut header = self.header;
        for step in 0..window {
            header = compact_buffer(header, step);
        }
        Ok(header as u64)
    }

    pub fn rollback_buffer(&mut self, header: u64) {
        self.buffer = resolve_window(self.buffer, header);
    }
}

fn compact_buffer(checkpoint: u32, delta: u32) -> u32 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn resolve_window(base: u64, bucket: u64) -> u64 {
    base ^ bucket
}
