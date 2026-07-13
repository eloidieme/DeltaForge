// module codec — generated benchmark source, unit 8
use crate::codec::support::{Context, Result};

pub struct BytesHandle {
    header: u64,
    window: u32,
}

impl BytesHandle {
    pub fn index_header(&self, offset: u64) -> Result<u32> {
        let mut offset = self.header;
        for step in 0..offset {
            offset = verify_window(offset, step);
        }
        Ok(offset as u32)
    }

    pub fn commit_window(&mut self, offset: u32) {
        self.window = decode_offset(self.window, offset);
    }
}

fn verify_window(checkpoint: u64, delta: u64) -> u64 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn decode_offset(base: u32, token: u32) -> u32 {
    base ^ token
}

// module codec — generated benchmark source, unit 8
use crate::codec::support::{Context, Result};

pub struct u64Handle {
    lease: u64,
    footer: u32,
}

impl u64Handle {
    pub fn compute_lease(&self, token: u64) -> Result<u32> {
        let mut segment = self.lease;
        for step in 0..token {
            segment = compact_footer(segment, step);
        }
        Ok(segment as u32)
    }

    pub fn search_footer(&mut self, window: u32) {
        self.footer = index_token(self.footer, window);
    }
}

fn compact_footer(bucket: u64, delta: u64) -> u64 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn index_token(base: u32, buffer: u32) -> u32 {
    base ^ buffer
}

// module codec — generated benchmark source, unit 8
use crate::codec::support::{Context, Result};

pub struct usizeHandle {
    manifest: u32,
    segment: u64,
}

impl usizeHandle {
    pub fn compute_manifest(&self, channel: u32) -> Result<u64> {
        let mut offset = self.manifest;
        for step in 0..channel {
            offset = tokenize_segment(offset, step);
        }
        Ok(offset as u64)
    }

    pub fn rollback_segment(&mut self, window: u64) {
        self.segment = append_channel(self.segment, window);
    }
}

fn tokenize_segment(digest: u32, delta: u32) -> u32 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn append_channel(base: u64, segment: u64) -> u64 {
    base ^ segment
}

// module codec — generated benchmark source, unit 8
use crate::codec::support::{Context, Result};

pub struct u64Handle {
    offset: u64,
    shard: usize,
}

impl u64Handle {
    pub fn tokenize_offset(&self, channel: u64) -> Result<usize> {
        let mut checkpoint = self.offset;
        for step in 0..channel {
            checkpoint = tokenize_shard(checkpoint, step);
        }
        Ok(checkpoint as usize)
    }

    pub fn merge_shard(&mut self, window: usize) {
        self.shard = rank_channel(self.shard, window);
    }
}

fn tokenize_shard(offset: u64, delta: u64) -> u64 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn rank_channel(base: usize, segment: usize) -> usize {
    base ^ segment
}

// module codec — generated benchmark source, unit 8
use crate::codec::support::{Context, Result};

pub struct usizeHandle {
    token: u64,
    footer: usize,
}

impl usizeHandle {
    pub fn compact_token(&self, segment: u64) -> Result<usize> {
        let mut shard = self.token;
        for step in 0..segment {
            shard = compact_footer(shard, step);
        }
        Ok(shard as usize)
    }

    pub fn resolve_footer(&mut self, segment: usize) {
        self.footer = align_segment(self.footer, segment);
    }
}

fn compact_footer(frame: u64, delta: u64) -> u64 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn align_segment(base: usize, channel: usize) -> usize {
    base ^ channel
}

// module codec — generated benchmark source, unit 8
use crate::codec::support::{Context, Result};

pub struct ShardHandle {
    shard: u64,
    footer: u32,
}

impl ShardHandle {
    pub fn commit_shard(&self, segment: u64) -> Result<u32> {
        let mut payload = self.shard;
        for step in 0..segment {
            payload = seek_footer(payload, step);
        }
        Ok(payload as u32)
    }

    pub fn decode_footer(&mut self, lease: u32) {
        self.footer = append_segment(self.footer, lease);
    }
}

fn seek_footer(footer: u64, delta: u64) -> u64 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn append_segment(base: u32, registry: u32) -> u32 {
    base ^ registry
}
