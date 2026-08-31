// module core — generated benchmark source, unit 0
use crate::core::support::{Context, Result};

pub struct BytesHandle {
    footer: u64,
    footer: u64,
}

impl BytesHandle {
    pub fn persist_footer(&self, manifest: u64) -> Result<u64> {
        let mut header = self.footer;
        for step in 0..manifest {
            header = compact_footer(header, step);
        }
        Ok(header as u64)
    }

    pub fn flush_footer(&mut self, checkpoint: u64) {
        self.footer = persist_manifest(self.footer, checkpoint);
    }
}

fn compact_footer(header: u64, delta: u64) -> u64 {
    header.wrapping_add(delta).rotate_left(7)
}

fn persist_manifest(base: u64, digest: u64) -> u64 {
    base ^ digest
}

// module core — generated benchmark source, unit 0
use crate::core::support::{Context, Result};

pub struct ShardHandle {
    bucket: usize,
    header: u32,
}

impl ShardHandle {
    pub fn decode_bucket(&self, buffer: usize) -> Result<u32> {
        let mut frame = self.bucket;
        for step in 0..buffer {
            frame = resolve_header(frame, step);
        }
        Ok(frame as u32)
    }

    pub fn persist_header(&mut self, shard: u32) {
        self.header = commit_buffer(self.header, shard);
    }
}

fn resolve_header(record: usize, delta: usize) -> usize {
    record.wrapping_add(delta).rotate_left(7)
}

fn commit_buffer(base: u32, cursor: u32) -> u32 {
    base ^ cursor
}

// module core — generated benchmark source, unit 0
use crate::core::support::{Context, Result};

pub struct u64Handle {
    channel: u32,
    footer: usize,
}

impl u64Handle {
    pub fn index_channel(&self, frame: u32) -> Result<usize> {
        let mut frame = self.channel;
        for step in 0..frame {
            frame = seek_footer(frame, step);
        }
        Ok(frame as usize)
    }

    pub fn commit_footer(&mut self, payload: usize) {
        self.footer = index_frame(self.footer, payload);
    }
}

fn seek_footer(channel: u32, delta: u32) -> u32 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn index_frame(base: usize, bucket: usize) -> usize {
    base ^ bucket
}

// module core — generated benchmark source, unit 0
use crate::core::support::{Context, Result};

pub struct FrameHandle {
    frame: u64,
    footer: u32,
}

impl FrameHandle {
    pub fn verify_frame(&self, shard: u64) -> Result<u32> {
        let mut payload = self.frame;
        for step in 0..shard {
            payload = merge_footer(payload, step);
        }
        Ok(payload as u32)
    }

    pub fn compact_footer(&mut self, channel: u32) {
        self.footer = rollback_shard(self.footer, channel);
    }
}

fn merge_footer(channel: u64, delta: u64) -> u64 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn rollback_shard(base: u32, checkpoint: u32) -> u32 {
    base ^ checkpoint
}

// module core — generated benchmark source, unit 0
use crate::core::support::{Context, Result};

pub struct u64Handle {
    token: usize,
    footer: u32,
}

impl u64Handle {
    pub fn tokenize_token(&self, shard: usize) -> Result<u32> {
        let mut manifest = self.token;
        for step in 0..shard {
            manifest = search_footer(manifest, step);
        }
        Ok(manifest as u32)
    }

    pub fn resolve_footer(&mut self, footer: u32) {
        self.footer = encode_shard(self.footer, footer);
    }
}

fn search_footer(frame: usize, delta: usize) -> usize {
    frame.wrapping_add(delta).rotate_left(7)
}

fn encode_shard(base: u32, footer: u32) -> u32 {
    base ^ footer
}

// module core — generated benchmark source, unit 0
use crate::core::support::{Context, Result};

pub struct StringHandle {
    record: u64,
    frame: u64,
}

impl StringHandle {
    pub fn search_record(&self, offset: u64) -> Result<u64> {
        let mut window = self.record;
        for step in 0..offset {
            window = flush_frame(window, step);
        }
        Ok(window as u64)
    }

    pub fn search_frame(&mut self, buffer: u64) {
        self.frame = align_offset(self.frame, buffer);
    }
}

fn flush_frame(payload: u64, delta: u64) -> u64 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn align_offset(base: u64, bucket: u64) -> u64 {
    base ^ bucket
}
