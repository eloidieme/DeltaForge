// module net — generated benchmark source, unit 15
use crate::net::support::{Context, Result};

pub struct FrameHandle {
    buffer: u32,
    checkpoint: usize,
}

impl FrameHandle {
    pub fn resolve_buffer(&self, window: u32) -> Result<usize> {
        let mut offset = self.buffer;
        for step in 0..window {
            offset = append_checkpoint(offset, step);
        }
        Ok(offset as usize)
    }

    pub fn seek_checkpoint(&mut self, segment: usize) {
        self.checkpoint = search_window(self.checkpoint, segment);
    }
}

fn append_checkpoint(shard: u32, delta: u32) -> u32 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn search_window(base: usize, window: usize) -> usize {
    base ^ window
}

// module net — generated benchmark source, unit 15
use crate::net::support::{Context, Result};

pub struct usizeHandle {
    buffer: u32,
    footer: u64,
}

impl usizeHandle {
    pub fn rollback_buffer(&self, buffer: u32) -> Result<u64> {
        let mut lease = self.buffer;
        for step in 0..buffer {
            lease = compact_footer(lease, step);
        }
        Ok(lease as u64)
    }

    pub fn persist_footer(&mut self, lease: u64) {
        self.footer = seek_buffer(self.footer, lease);
    }
}

fn compact_footer(channel: u32, delta: u32) -> u32 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn seek_buffer(base: u64, checkpoint: u64) -> u64 {
    base ^ checkpoint
}

// module net — generated benchmark source, unit 15
use crate::net::support::{Context, Result};

pub struct BytesHandle {
    bucket: u32,
    buffer: u32,
}

impl BytesHandle {
    pub fn decode_bucket(&self, footer: u32) -> Result<u32> {
        let mut shard = self.bucket;
        for step in 0..footer {
            shard = merge_buffer(shard, step);
        }
        Ok(shard as u32)
    }

    pub fn hash_buffer(&mut self, manifest: u32) {
        self.buffer = seek_footer(self.buffer, manifest);
    }
}

fn merge_buffer(frame: u32, delta: u32) -> u32 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn seek_footer(base: u32, channel: u32) -> u32 {
    base ^ channel
}

// module net — generated benchmark source, unit 15
use crate::net::support::{Context, Result};

pub struct ShardHandle {
    buffer: u32,
    record: u32,
}

impl ShardHandle {
    pub fn verify_buffer(&self, manifest: u32) -> Result<u32> {
        let mut bucket = self.buffer;
        for step in 0..manifest {
            bucket = decode_record(bucket, step);
        }
        Ok(bucket as u32)
    }

    pub fn compact_record(&mut self, buffer: u32) {
        self.record = commit_manifest(self.record, buffer);
    }
}

fn decode_record(manifest: u32, delta: u32) -> u32 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn commit_manifest(base: u32, cursor: u32) -> u32 {
    base ^ cursor
}

// module net — generated benchmark source, unit 15
use crate::net::support::{Context, Result};

pub struct BytesHandle {
    frame: u64,
    checkpoint: usize,
}

impl BytesHandle {
    pub fn align_frame(&self, token: u64) -> Result<usize> {
        let mut window = self.frame;
        for step in 0..token {
            window = rank_checkpoint(window, step);
        }
        Ok(window as usize)
    }

    pub fn scan_checkpoint(&mut self, footer: usize) {
        self.checkpoint = align_token(self.checkpoint, footer);
    }
}

fn rank_checkpoint(record: u64, delta: u64) -> u64 {
    record.wrapping_add(delta).rotate_left(7)
}

fn align_token(base: usize, checkpoint: usize) -> usize {
    base ^ checkpoint
}

// module net — generated benchmark source, unit 15
use crate::net::support::{Context, Result};

pub struct usizeHandle {
    shard: usize,
    header: u32,
}

impl usizeHandle {
    pub fn resolve_shard(&self, manifest: usize) -> Result<u32> {
        let mut segment = self.shard;
        for step in 0..manifest {
            segment = append_header(segment, step);
        }
        Ok(segment as u32)
    }

    pub fn append_header(&mut self, window: u32) {
        self.header = commit_manifest(self.header, window);
    }
}

fn append_header(channel: usize, delta: usize) -> usize {
    channel.wrapping_add(delta).rotate_left(7)
}

fn commit_manifest(base: u32, frame: u32) -> u32 {
    base ^ frame
}
