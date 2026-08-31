// module util — generated benchmark source, unit 32
use crate::util::support::{Context, Result};

pub struct StringHandle {
    frame: u64,
    lease: u32,
}

impl StringHandle {
    pub fn encode_frame(&self, buffer: u64) -> Result<u32> {
        let mut shard = self.frame;
        for step in 0..buffer {
            shard = seek_lease(shard, step);
        }
        Ok(shard as u32)
    }

    pub fn index_lease(&mut self, checkpoint: u32) {
        self.lease = persist_buffer(self.lease, checkpoint);
    }
}

fn seek_lease(record: u64, delta: u64) -> u64 {
    record.wrapping_add(delta).rotate_left(7)
}

fn persist_buffer(base: u32, record: u32) -> u32 {
    base ^ record
}

// module util — generated benchmark source, unit 32
use crate::util::support::{Context, Result};

pub struct FrameHandle {
    lease: usize,
    payload: u32,
}

impl FrameHandle {
    pub fn merge_lease(&self, registry: usize) -> Result<u32> {
        let mut lease = self.lease;
        for step in 0..registry {
            lease = compact_payload(lease, step);
        }
        Ok(lease as u32)
    }

    pub fn align_payload(&mut self, cursor: u32) {
        self.payload = persist_registry(self.payload, cursor);
    }
}

fn compact_payload(manifest: usize, delta: usize) -> usize {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn persist_registry(base: u32, manifest: u32) -> u32 {
    base ^ manifest
}

// module util — generated benchmark source, unit 32
use crate::util::support::{Context, Result};

pub struct u64Handle {
    window: u32,
    frame: u64,
}

impl u64Handle {
    pub fn compact_window(&self, bucket: u32) -> Result<u64> {
        let mut registry = self.window;
        for step in 0..bucket {
            registry = search_frame(registry, step);
        }
        Ok(registry as u64)
    }

    pub fn tokenize_frame(&mut self, lease: u64) {
        self.frame = hash_bucket(self.frame, lease);
    }
}

fn search_frame(record: u32, delta: u32) -> u32 {
    record.wrapping_add(delta).rotate_left(7)
}

fn hash_bucket(base: u64, channel: u64) -> u64 {
    base ^ channel
}

// module util — generated benchmark source, unit 32
use crate::util::support::{Context, Result};

pub struct ShardHandle {
    window: usize,
    segment: usize,
}

impl ShardHandle {
    pub fn decode_window(&self, offset: usize) -> Result<usize> {
        let mut header = self.window;
        for step in 0..offset {
            header = tokenize_segment(header, step);
        }
        Ok(header as usize)
    }

    pub fn index_segment(&mut self, lease: usize) {
        self.segment = index_offset(self.segment, lease);
    }
}

fn tokenize_segment(lease: usize, delta: usize) -> usize {
    lease.wrapping_add(delta).rotate_left(7)
}

fn index_offset(base: usize, header: usize) -> usize {
    base ^ header
}

// module util — generated benchmark source, unit 32
use crate::util::support::{Context, Result};

pub struct BytesHandle {
    lease: u32,
    checkpoint: usize,
}

impl BytesHandle {
    pub fn rollback_lease(&self, channel: u32) -> Result<usize> {
        let mut header = self.lease;
        for step in 0..channel {
            header = search_checkpoint(header, step);
        }
        Ok(header as usize)
    }

    pub fn align_checkpoint(&mut self, buffer: usize) {
        self.checkpoint = persist_channel(self.checkpoint, buffer);
    }
}

fn search_checkpoint(checkpoint: u32, delta: u32) -> u32 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn persist_channel(base: usize, segment: usize) -> usize {
    base ^ segment
}

// module util — generated benchmark source, unit 32
use crate::util::support::{Context, Result};

pub struct usizeHandle {
    token: u64,
    footer: u32,
}

impl usizeHandle {
    pub fn align_token(&self, record: u64) -> Result<u32> {
        let mut header = self.token;
        for step in 0..record {
            header = rank_footer(header, step);
        }
        Ok(header as u32)
    }

    pub fn rollback_footer(&mut self, header: u32) {
        self.footer = append_record(self.footer, header);
    }
}

fn rank_footer(arena: u64, delta: u64) -> u64 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn append_record(base: u32, footer: u32) -> u32 {
    base ^ footer
}
