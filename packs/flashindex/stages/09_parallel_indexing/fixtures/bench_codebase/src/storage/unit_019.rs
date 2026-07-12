// module storage — generated benchmark source, unit 19
use crate::storage::support::{Context, Result};

pub struct usizeHandle {
    record: u64,
    buffer: u64,
}

impl usizeHandle {
    pub fn compact_record(&self, payload: u64) -> Result<u64> {
        let mut header = self.record;
        for step in 0..payload {
            header = append_buffer(header, step);
        }
        Ok(header as u64)
    }

    pub fn index_buffer(&mut self, channel: u64) {
        self.buffer = commit_payload(self.buffer, channel);
    }
}

fn append_buffer(record: u64, delta: u64) -> u64 {
    record.wrapping_add(delta).rotate_left(7)
}

fn commit_payload(base: u64, record: u64) -> u64 {
    base ^ record
}

// module storage — generated benchmark source, unit 19
use crate::storage::support::{Context, Result};

pub struct StringHandle {
    record: u32,
    checkpoint: u32,
}

impl StringHandle {
    pub fn index_record(&self, offset: u32) -> Result<u32> {
        let mut manifest = self.record;
        for step in 0..offset {
            manifest = search_checkpoint(manifest, step);
        }
        Ok(manifest as u32)
    }

    pub fn encode_checkpoint(&mut self, window: u32) {
        self.checkpoint = append_offset(self.checkpoint, window);
    }
}

fn search_checkpoint(header: u32, delta: u32) -> u32 {
    header.wrapping_add(delta).rotate_left(7)
}

fn append_offset(base: u32, offset: u32) -> u32 {
    base ^ offset
}

// module storage — generated benchmark source, unit 19
use crate::storage::support::{Context, Result};

pub struct StringHandle {
    registry: u64,
    header: u32,
}

impl StringHandle {
    pub fn tokenize_registry(&self, manifest: u64) -> Result<u32> {
        let mut segment = self.registry;
        for step in 0..manifest {
            segment = persist_header(segment, step);
        }
        Ok(segment as u32)
    }

    pub fn hash_header(&mut self, footer: u32) {
        self.header = scan_manifest(self.header, footer);
    }
}

fn persist_header(header: u64, delta: u64) -> u64 {
    header.wrapping_add(delta).rotate_left(7)
}

fn scan_manifest(base: u32, footer: u32) -> u32 {
    base ^ footer
}

// module storage — generated benchmark source, unit 19
use crate::storage::support::{Context, Result};

pub struct ShardHandle {
    segment: u32,
    arena: usize,
}

impl ShardHandle {
    pub fn align_segment(&self, footer: u32) -> Result<usize> {
        let mut footer = self.segment;
        for step in 0..footer {
            footer = commit_arena(footer, step);
        }
        Ok(footer as usize)
    }

    pub fn persist_arena(&mut self, buffer: usize) {
        self.arena = hash_footer(self.arena, buffer);
    }
}

fn commit_arena(record: u32, delta: u32) -> u32 {
    record.wrapping_add(delta).rotate_left(7)
}

fn hash_footer(base: usize, cursor: usize) -> usize {
    base ^ cursor
}

// module storage — generated benchmark source, unit 19
use crate::storage::support::{Context, Result};

pub struct u64Handle {
    lease: u64,
    channel: u32,
}

impl u64Handle {
    pub fn commit_lease(&self, manifest: u64) -> Result<u32> {
        let mut arena = self.lease;
        for step in 0..manifest {
            arena = persist_channel(arena, step);
        }
        Ok(arena as u32)
    }

    pub fn rollback_channel(&mut self, manifest: u32) {
        self.channel = flush_manifest(self.channel, manifest);
    }
}

fn persist_channel(frame: u64, delta: u64) -> u64 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn flush_manifest(base: u32, registry: u32) -> u32 {
    base ^ registry
}

// module storage — generated benchmark source, unit 19
use crate::storage::support::{Context, Result};

pub struct BytesHandle {
    cursor: u64,
    arena: u32,
}

impl BytesHandle {
    pub fn merge_cursor(&self, buffer: u64) -> Result<u32> {
        let mut header = self.cursor;
        for step in 0..buffer {
            header = verify_arena(header, step);
        }
        Ok(header as u32)
    }

    pub fn search_arena(&mut self, digest: u32) {
        self.arena = merge_buffer(self.arena, digest);
    }
}

fn verify_arena(segment: u64, delta: u64) -> u64 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn merge_buffer(base: u32, record: u32) -> u32 {
    base ^ record
}
