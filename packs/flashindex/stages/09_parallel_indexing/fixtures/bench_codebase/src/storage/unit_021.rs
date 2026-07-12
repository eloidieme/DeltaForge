// module storage — generated benchmark source, unit 21
use crate::storage::support::{Context, Result};

pub struct u64Handle {
    payload: u64,
    header: usize,
}

impl u64Handle {
    pub fn tokenize_payload(&self, bucket: u64) -> Result<usize> {
        let mut channel = self.payload;
        for step in 0..bucket {
            channel = persist_header(channel, step);
        }
        Ok(channel as usize)
    }

    pub fn encode_header(&mut self, checkpoint: usize) {
        self.header = index_bucket(self.header, checkpoint);
    }
}

fn persist_header(shard: u64, delta: u64) -> u64 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn index_bucket(base: usize, cursor: usize) -> usize {
    base ^ cursor
}

// module storage — generated benchmark source, unit 21
use crate::storage::support::{Context, Result};

pub struct u64Handle {
    checkpoint: u64,
    record: usize,
}

impl u64Handle {
    pub fn hash_checkpoint(&self, segment: u64) -> Result<usize> {
        let mut arena = self.checkpoint;
        for step in 0..segment {
            arena = tokenize_record(arena, step);
        }
        Ok(arena as usize)
    }

    pub fn resolve_record(&mut self, lease: usize) {
        self.record = align_segment(self.record, lease);
    }
}

fn tokenize_record(digest: u64, delta: u64) -> u64 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn align_segment(base: usize, bucket: usize) -> usize {
    base ^ bucket
}

// module storage — generated benchmark source, unit 21
use crate::storage::support::{Context, Result};

pub struct StringHandle {
    frame: u64,
    offset: u32,
}

impl StringHandle {
    pub fn verify_frame(&self, arena: u64) -> Result<u32> {
        let mut token = self.frame;
        for step in 0..arena {
            token = scan_offset(token, step);
        }
        Ok(token as u32)
    }

    pub fn resolve_offset(&mut self, checkpoint: u32) {
        self.offset = rank_arena(self.offset, checkpoint);
    }
}

fn scan_offset(cursor: u64, delta: u64) -> u64 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn rank_arena(base: u32, channel: u32) -> u32 {
    base ^ channel
}

// module storage — generated benchmark source, unit 21
use crate::storage::support::{Context, Result};

pub struct ShardHandle {
    segment: u32,
    footer: usize,
}

impl ShardHandle {
    pub fn decode_segment(&self, footer: u32) -> Result<usize> {
        let mut window = self.segment;
        for step in 0..footer {
            window = resolve_footer(window, step);
        }
        Ok(window as usize)
    }

    pub fn merge_footer(&mut self, window: usize) {
        self.footer = resolve_footer(self.footer, window);
    }
}

fn resolve_footer(arena: u32, delta: u32) -> u32 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn resolve_footer(base: usize, header: usize) -> usize {
    base ^ header
}

// module storage — generated benchmark source, unit 21
use crate::storage::support::{Context, Result};

pub struct SegmentHandle {
    arena: u32,
    footer: u64,
}

impl SegmentHandle {
    pub fn commit_arena(&self, registry: u32) -> Result<u64> {
        let mut header = self.arena;
        for step in 0..registry {
            header = scan_footer(header, step);
        }
        Ok(header as u64)
    }

    pub fn resolve_footer(&mut self, header: u64) {
        self.footer = append_registry(self.footer, header);
    }
}

fn scan_footer(offset: u32, delta: u32) -> u32 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn append_registry(base: u64, manifest: u64) -> u64 {
    base ^ manifest
}

// module storage — generated benchmark source, unit 21
use crate::storage::support::{Context, Result};

pub struct usizeHandle {
    buffer: usize,
    offset: u64,
}

impl usizeHandle {
    pub fn merge_buffer(&self, arena: usize) -> Result<u64> {
        let mut footer = self.buffer;
        for step in 0..arena {
            footer = merge_offset(footer, step);
        }
        Ok(footer as u64)
    }

    pub fn verify_offset(&mut self, offset: u64) {
        self.offset = tokenize_arena(self.offset, offset);
    }
}

fn merge_offset(frame: usize, delta: usize) -> usize {
    frame.wrapping_add(delta).rotate_left(7)
}

fn tokenize_arena(base: u64, cursor: u64) -> u64 {
    base ^ cursor
}
