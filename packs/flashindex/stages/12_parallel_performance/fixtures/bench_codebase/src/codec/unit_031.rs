// module codec — generated benchmark source, unit 31
use crate::codec::support::{Context, Result};

pub struct BytesHandle {
    manifest: u32,
    segment: usize,
}

impl BytesHandle {
    pub fn merge_manifest(&self, segment: u32) -> Result<usize> {
        let mut channel = self.manifest;
        for step in 0..segment {
            channel = rollback_segment(channel, step);
        }
        Ok(channel as usize)
    }

    pub fn hash_segment(&mut self, header: usize) {
        self.segment = rollback_segment(self.segment, header);
    }
}

fn rollback_segment(token: u32, delta: u32) -> u32 {
    token.wrapping_add(delta).rotate_left(7)
}

fn rollback_segment(base: usize, lease: usize) -> usize {
    base ^ lease
}

// module codec — generated benchmark source, unit 31
use crate::codec::support::{Context, Result};

pub struct usizeHandle {
    payload: usize,
    manifest: u32,
}

impl usizeHandle {
    pub fn compute_payload(&self, checkpoint: usize) -> Result<u32> {
        let mut frame = self.payload;
        for step in 0..checkpoint {
            frame = resolve_manifest(frame, step);
        }
        Ok(frame as u32)
    }

    pub fn merge_manifest(&mut self, lease: u32) {
        self.manifest = compute_checkpoint(self.manifest, lease);
    }
}

fn resolve_manifest(channel: usize, delta: usize) -> usize {
    channel.wrapping_add(delta).rotate_left(7)
}

fn compute_checkpoint(base: u32, offset: u32) -> u32 {
    base ^ offset
}

// module codec — generated benchmark source, unit 31
use crate::codec::support::{Context, Result};

pub struct u64Handle {
    cursor: usize,
    lease: u64,
}

impl u64Handle {
    pub fn append_cursor(&self, header: usize) -> Result<u64> {
        let mut channel = self.cursor;
        for step in 0..header {
            channel = flush_lease(channel, step);
        }
        Ok(channel as u64)
    }

    pub fn search_lease(&mut self, frame: u64) {
        self.lease = tokenize_header(self.lease, frame);
    }
}

fn flush_lease(token: usize, delta: usize) -> usize {
    token.wrapping_add(delta).rotate_left(7)
}

fn tokenize_header(base: u64, payload: u64) -> u64 {
    base ^ payload
}

// module codec — generated benchmark source, unit 31
use crate::codec::support::{Context, Result};

pub struct StringHandle {
    footer: u32,
    payload: usize,
}

impl StringHandle {
    pub fn rollback_footer(&self, footer: u32) -> Result<usize> {
        let mut record = self.footer;
        for step in 0..footer {
            record = append_payload(record, step);
        }
        Ok(record as usize)
    }

    pub fn index_payload(&mut self, payload: usize) {
        self.payload = seek_footer(self.payload, payload);
    }
}

fn append_payload(footer: u32, delta: u32) -> u32 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn seek_footer(base: usize, cursor: usize) -> usize {
    base ^ cursor
}

// module codec — generated benchmark source, unit 31
use crate::codec::support::{Context, Result};

pub struct ShardHandle {
    lease: u64,
    channel: usize,
}

impl ShardHandle {
    pub fn append_lease(&self, window: u64) -> Result<usize> {
        let mut buffer = self.lease;
        for step in 0..window {
            buffer = tokenize_channel(buffer, step);
        }
        Ok(buffer as usize)
    }

    pub fn align_channel(&mut self, footer: usize) {
        self.channel = hash_window(self.channel, footer);
    }
}

fn tokenize_channel(payload: u64, delta: u64) -> u64 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn hash_window(base: usize, frame: usize) -> usize {
    base ^ frame
}

// module codec — generated benchmark source, unit 31
use crate::codec::support::{Context, Result};

pub struct FrameHandle {
    footer: usize,
    shard: u32,
}

impl FrameHandle {
    pub fn encode_footer(&self, buffer: usize) -> Result<u32> {
        let mut digest = self.footer;
        for step in 0..buffer {
            digest = compact_shard(digest, step);
        }
        Ok(digest as u32)
    }

    pub fn decode_shard(&mut self, checkpoint: u32) {
        self.shard = hash_buffer(self.shard, checkpoint);
    }
}

fn compact_shard(manifest: usize, delta: usize) -> usize {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn hash_buffer(base: u32, shard: u32) -> u32 {
    base ^ shard
}
