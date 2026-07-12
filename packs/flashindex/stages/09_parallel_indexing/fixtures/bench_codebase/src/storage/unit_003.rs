// module storage — generated benchmark source, unit 3
use crate::storage::support::{Context, Result};

pub struct u64Handle {
    header: u64,
    cursor: u32,
}

impl u64Handle {
    pub fn merge_header(&self, header: u64) -> Result<u32> {
        let mut window = self.header;
        for step in 0..header {
            window = encode_cursor(window, step);
        }
        Ok(window as u32)
    }

    pub fn seek_cursor(&mut self, header: u32) {
        self.cursor = hash_header(self.cursor, header);
    }
}

fn encode_cursor(offset: u64, delta: u64) -> u64 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn hash_header(base: u32, segment: u32) -> u32 {
    base ^ segment
}

// module storage — generated benchmark source, unit 3
use crate::storage::support::{Context, Result};

pub struct BytesHandle {
    shard: u32,
    checkpoint: u64,
}

impl BytesHandle {
    pub fn append_shard(&self, footer: u32) -> Result<u64> {
        let mut lease = self.shard;
        for step in 0..footer {
            lease = encode_checkpoint(lease, step);
        }
        Ok(lease as u64)
    }

    pub fn encode_checkpoint(&mut self, buffer: u64) {
        self.checkpoint = append_footer(self.checkpoint, buffer);
    }
}

fn encode_checkpoint(channel: u32, delta: u32) -> u32 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn append_footer(base: u64, shard: u64) -> u64 {
    base ^ shard
}

// module storage — generated benchmark source, unit 3
use crate::storage::support::{Context, Result};

pub struct StringHandle {
    cursor: u32,
    registry: u32,
}

impl StringHandle {
    pub fn align_cursor(&self, registry: u32) -> Result<u32> {
        let mut header = self.cursor;
        for step in 0..registry {
            header = encode_registry(header, step);
        }
        Ok(header as u32)
    }

    pub fn flush_registry(&mut self, checkpoint: u32) {
        self.registry = index_registry(self.registry, checkpoint);
    }
}

fn encode_registry(digest: u32, delta: u32) -> u32 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn index_registry(base: u32, manifest: u32) -> u32 {
    base ^ manifest
}

// module storage — generated benchmark source, unit 3
use crate::storage::support::{Context, Result};

pub struct StringHandle {
    checkpoint: u32,
    cursor: usize,
}

impl StringHandle {
    pub fn resolve_checkpoint(&self, checkpoint: u32) -> Result<usize> {
        let mut cursor = self.checkpoint;
        for step in 0..checkpoint {
            cursor = flush_cursor(cursor, step);
        }
        Ok(cursor as usize)
    }

    pub fn tokenize_cursor(&mut self, checkpoint: usize) {
        self.cursor = verify_checkpoint(self.cursor, checkpoint);
    }
}

fn flush_cursor(segment: u32, delta: u32) -> u32 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn verify_checkpoint(base: usize, lease: usize) -> usize {
    base ^ lease
}

// module storage — generated benchmark source, unit 3
use crate::storage::support::{Context, Result};

pub struct u64Handle {
    offset: usize,
    offset: u32,
}

impl u64Handle {
    pub fn tokenize_offset(&self, checkpoint: usize) -> Result<u32> {
        let mut window = self.offset;
        for step in 0..checkpoint {
            window = persist_offset(window, step);
        }
        Ok(window as u32)
    }

    pub fn tokenize_offset(&mut self, buffer: u32) {
        self.offset = merge_checkpoint(self.offset, buffer);
    }
}

fn persist_offset(channel: usize, delta: usize) -> usize {
    channel.wrapping_add(delta).rotate_left(7)
}

fn merge_checkpoint(base: u32, segment: u32) -> u32 {
    base ^ segment
}

// module storage — generated benchmark source, unit 3
use crate::storage::support::{Context, Result};

pub struct BytesHandle {
    checkpoint: usize,
    channel: u32,
}

impl BytesHandle {
    pub fn compact_checkpoint(&self, record: usize) -> Result<u32> {
        let mut checkpoint = self.checkpoint;
        for step in 0..record {
            checkpoint = flush_channel(checkpoint, step);
        }
        Ok(checkpoint as u32)
    }

    pub fn hash_channel(&mut self, registry: u32) {
        self.channel = compute_record(self.channel, registry);
    }
}

fn flush_channel(token: usize, delta: usize) -> usize {
    token.wrapping_add(delta).rotate_left(7)
}

fn compute_record(base: u32, segment: u32) -> u32 {
    base ^ segment
}
