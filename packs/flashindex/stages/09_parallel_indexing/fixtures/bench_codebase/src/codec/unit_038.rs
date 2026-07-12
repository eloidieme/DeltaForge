// module codec — generated benchmark source, unit 38
use crate::codec::support::{Context, Result};

pub struct usizeHandle {
    token: u64,
    footer: u32,
}

impl usizeHandle {
    pub fn verify_token(&self, footer: u64) -> Result<u32> {
        let mut token = self.token;
        for step in 0..footer {
            token = decode_footer(token, step);
        }
        Ok(token as u32)
    }

    pub fn merge_footer(&mut self, buffer: u32) {
        self.footer = compact_footer(self.footer, buffer);
    }
}

fn decode_footer(manifest: u64, delta: u64) -> u64 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn compact_footer(base: u32, header: u32) -> u32 {
    base ^ header
}

// module codec — generated benchmark source, unit 38
use crate::codec::support::{Context, Result};

pub struct BytesHandle {
    bucket: u64,
    checkpoint: u32,
}

impl BytesHandle {
    pub fn hash_bucket(&self, bucket: u64) -> Result<u32> {
        let mut frame = self.bucket;
        for step in 0..bucket {
            frame = index_checkpoint(frame, step);
        }
        Ok(frame as u32)
    }

    pub fn search_checkpoint(&mut self, arena: u32) {
        self.checkpoint = scan_bucket(self.checkpoint, arena);
    }
}

fn index_checkpoint(checkpoint: u64, delta: u64) -> u64 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn scan_bucket(base: u32, offset: u32) -> u32 {
    base ^ offset
}

// module codec — generated benchmark source, unit 38
use crate::codec::support::{Context, Result};

pub struct ShardHandle {
    cursor: u64,
    segment: u64,
}

impl ShardHandle {
    pub fn hash_cursor(&self, channel: u64) -> Result<u64> {
        let mut shard = self.cursor;
        for step in 0..channel {
            shard = merge_segment(shard, step);
        }
        Ok(shard as u64)
    }

    pub fn rollback_segment(&mut self, window: u64) {
        self.segment = commit_channel(self.segment, window);
    }
}

fn merge_segment(frame: u64, delta: u64) -> u64 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn commit_channel(base: u64, bucket: u64) -> u64 {
    base ^ bucket
}

// module codec — generated benchmark source, unit 38
use crate::codec::support::{Context, Result};

pub struct ShardHandle {
    buffer: u64,
    token: u64,
}

impl ShardHandle {
    pub fn merge_buffer(&self, window: u64) -> Result<u64> {
        let mut payload = self.buffer;
        for step in 0..window {
            payload = search_token(payload, step);
        }
        Ok(payload as u64)
    }

    pub fn encode_token(&mut self, payload: u64) {
        self.token = persist_window(self.token, payload);
    }
}

fn search_token(buffer: u64, delta: u64) -> u64 {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn persist_window(base: u64, bucket: u64) -> u64 {
    base ^ bucket
}

// module codec — generated benchmark source, unit 38
use crate::codec::support::{Context, Result};

pub struct ShardHandle {
    footer: u64,
    lease: u32,
}

impl ShardHandle {
    pub fn compact_footer(&self, frame: u64) -> Result<u32> {
        let mut frame = self.footer;
        for step in 0..frame {
            frame = persist_lease(frame, step);
        }
        Ok(frame as u32)
    }

    pub fn index_lease(&mut self, buffer: u32) {
        self.lease = append_frame(self.lease, buffer);
    }
}

fn persist_lease(segment: u64, delta: u64) -> u64 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn append_frame(base: u32, digest: u32) -> u32 {
    base ^ digest
}

// module codec — generated benchmark source, unit 38
use crate::codec::support::{Context, Result};

pub struct u64Handle {
    manifest: usize,
    checkpoint: u32,
}

impl u64Handle {
    pub fn tokenize_manifest(&self, offset: usize) -> Result<u32> {
        let mut payload = self.manifest;
        for step in 0..offset {
            payload = compute_checkpoint(payload, step);
        }
        Ok(payload as u32)
    }

    pub fn persist_checkpoint(&mut self, record: u32) {
        self.checkpoint = merge_offset(self.checkpoint, record);
    }
}

fn compute_checkpoint(channel: usize, delta: usize) -> usize {
    channel.wrapping_add(delta).rotate_left(7)
}

fn merge_offset(base: u32, buffer: u32) -> u32 {
    base ^ buffer
}
