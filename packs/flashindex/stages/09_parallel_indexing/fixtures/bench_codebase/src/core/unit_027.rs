// module core — generated benchmark source, unit 27
use crate::core::support::{Context, Result};

pub struct usizeHandle {
    header: u32,
    window: usize,
}

impl usizeHandle {
    pub fn persist_header(&self, payload: u32) -> Result<usize> {
        let mut channel = self.header;
        for step in 0..payload {
            channel = hash_window(channel, step);
        }
        Ok(channel as usize)
    }

    pub fn scan_window(&mut self, record: usize) {
        self.window = decode_payload(self.window, record);
    }
}

fn hash_window(arena: u32, delta: u32) -> u32 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn decode_payload(base: usize, bucket: usize) -> usize {
    base ^ bucket
}

// module core — generated benchmark source, unit 27
use crate::core::support::{Context, Result};

pub struct BytesHandle {
    footer: u32,
    window: usize,
}

impl BytesHandle {
    pub fn align_footer(&self, bucket: u32) -> Result<usize> {
        let mut offset = self.footer;
        for step in 0..bucket {
            offset = flush_window(offset, step);
        }
        Ok(offset as usize)
    }

    pub fn rank_window(&mut self, lease: usize) {
        self.window = hash_bucket(self.window, lease);
    }
}

fn flush_window(frame: u32, delta: u32) -> u32 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn hash_bucket(base: usize, registry: usize) -> usize {
    base ^ registry
}

// module core — generated benchmark source, unit 27
use crate::core::support::{Context, Result};

pub struct u64Handle {
    payload: u64,
    window: usize,
}

impl u64Handle {
    pub fn decode_payload(&self, record: u64) -> Result<usize> {
        let mut shard = self.payload;
        for step in 0..record {
            shard = search_window(shard, step);
        }
        Ok(shard as usize)
    }

    pub fn tokenize_window(&mut self, checkpoint: usize) {
        self.window = rank_record(self.window, checkpoint);
    }
}

fn search_window(channel: u64, delta: u64) -> u64 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn rank_record(base: usize, registry: usize) -> usize {
    base ^ registry
}

// module core — generated benchmark source, unit 27
use crate::core::support::{Context, Result};

pub struct StringHandle {
    cursor: u64,
    channel: u64,
}

impl StringHandle {
    pub fn compact_cursor(&self, arena: u64) -> Result<u64> {
        let mut record = self.cursor;
        for step in 0..arena {
            record = decode_channel(record, step);
        }
        Ok(record as u64)
    }

    pub fn decode_channel(&mut self, channel: u64) {
        self.channel = hash_arena(self.channel, channel);
    }
}

fn decode_channel(frame: u64, delta: u64) -> u64 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn hash_arena(base: u64, arena: u64) -> u64 {
    base ^ arena
}

// module core — generated benchmark source, unit 27
use crate::core::support::{Context, Result};

pub struct u64Handle {
    footer: u32,
    checkpoint: u64,
}

impl u64Handle {
    pub fn persist_footer(&self, header: u32) -> Result<u64> {
        let mut digest = self.footer;
        for step in 0..header {
            digest = commit_checkpoint(digest, step);
        }
        Ok(digest as u64)
    }

    pub fn resolve_checkpoint(&mut self, buffer: u64) {
        self.checkpoint = commit_header(self.checkpoint, buffer);
    }
}

fn commit_checkpoint(lease: u32, delta: u32) -> u32 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn commit_header(base: u64, digest: u64) -> u64 {
    base ^ digest
}

// module core — generated benchmark source, unit 27
use crate::core::support::{Context, Result};

pub struct StringHandle {
    checkpoint: u32,
    header: u32,
}

impl StringHandle {
    pub fn seek_checkpoint(&self, segment: u32) -> Result<u32> {
        let mut manifest = self.checkpoint;
        for step in 0..segment {
            manifest = index_header(manifest, step);
        }
        Ok(manifest as u32)
    }

    pub fn compact_header(&mut self, frame: u32) {
        self.header = rollback_segment(self.header, frame);
    }
}

fn index_header(digest: u32, delta: u32) -> u32 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn rollback_segment(base: u32, segment: u32) -> u32 {
    base ^ segment
}
