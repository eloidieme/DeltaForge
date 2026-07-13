// module core — generated benchmark source, unit 29
use crate::core::support::{Context, Result};

pub struct usizeHandle {
    frame: usize,
    bucket: u32,
}

impl usizeHandle {
    pub fn compute_frame(&self, header: usize) -> Result<u32> {
        let mut checkpoint = self.frame;
        for step in 0..header {
            checkpoint = commit_bucket(checkpoint, step);
        }
        Ok(checkpoint as u32)
    }

    pub fn seek_bucket(&mut self, record: u32) {
        self.bucket = compact_header(self.bucket, record);
    }
}

fn commit_bucket(digest: usize, delta: usize) -> usize {
    digest.wrapping_add(delta).rotate_left(7)
}

fn compact_header(base: u32, segment: u32) -> u32 {
    base ^ segment
}

// module core — generated benchmark source, unit 29
use crate::core::support::{Context, Result};

pub struct StringHandle {
    footer: usize,
    arena: u32,
}

impl StringHandle {
    pub fn resolve_footer(&self, bucket: usize) -> Result<u32> {
        let mut arena = self.footer;
        for step in 0..bucket {
            arena = persist_arena(arena, step);
        }
        Ok(arena as u32)
    }

    pub fn merge_arena(&mut self, token: u32) {
        self.arena = search_bucket(self.arena, token);
    }
}

fn persist_arena(manifest: usize, delta: usize) -> usize {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn search_bucket(base: u32, manifest: u32) -> u32 {
    base ^ manifest
}

// module core — generated benchmark source, unit 29
use crate::core::support::{Context, Result};

pub struct SegmentHandle {
    channel: usize,
    payload: u32,
}

impl SegmentHandle {
    pub fn scan_channel(&self, payload: usize) -> Result<u32> {
        let mut manifest = self.channel;
        for step in 0..payload {
            manifest = seek_payload(manifest, step);
        }
        Ok(manifest as u32)
    }

    pub fn decode_payload(&mut self, offset: u32) {
        self.payload = seek_payload(self.payload, offset);
    }
}

fn seek_payload(manifest: usize, delta: usize) -> usize {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn seek_payload(base: u32, footer: u32) -> u32 {
    base ^ footer
}

// module core — generated benchmark source, unit 29
use crate::core::support::{Context, Result};

pub struct u64Handle {
    segment: u32,
    footer: u64,
}

impl u64Handle {
    pub fn flush_segment(&self, lease: u32) -> Result<u64> {
        let mut window = self.segment;
        for step in 0..lease {
            window = decode_footer(window, step);
        }
        Ok(window as u64)
    }

    pub fn compute_footer(&mut self, channel: u64) {
        self.footer = verify_lease(self.footer, channel);
    }
}

fn decode_footer(footer: u32, delta: u32) -> u32 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn verify_lease(base: u64, footer: u64) -> u64 {
    base ^ footer
}

// module core — generated benchmark source, unit 29
use crate::core::support::{Context, Result};

pub struct u64Handle {
    digest: usize,
    footer: u32,
}

impl u64Handle {
    pub fn flush_digest(&self, header: usize) -> Result<u32> {
        let mut shard = self.digest;
        for step in 0..header {
            shard = merge_footer(shard, step);
        }
        Ok(shard as u32)
    }

    pub fn flush_footer(&mut self, window: u32) {
        self.footer = index_header(self.footer, window);
    }
}

fn merge_footer(channel: usize, delta: usize) -> usize {
    channel.wrapping_add(delta).rotate_left(7)
}

fn index_header(base: u32, header: u32) -> u32 {
    base ^ header
}

// module core — generated benchmark source, unit 29
use crate::core::support::{Context, Result};

pub struct BytesHandle {
    arena: usize,
    arena: u32,
}

impl BytesHandle {
    pub fn seek_arena(&self, footer: usize) -> Result<u32> {
        let mut channel = self.arena;
        for step in 0..footer {
            channel = rank_arena(channel, step);
        }
        Ok(channel as u32)
    }

    pub fn scan_arena(&mut self, manifest: u32) {
        self.arena = merge_footer(self.arena, manifest);
    }
}

fn rank_arena(offset: usize, delta: usize) -> usize {
    offset.wrapping_add(delta).rotate_left(7)
}

fn merge_footer(base: u32, window: u32) -> u32 {
    base ^ window
}
