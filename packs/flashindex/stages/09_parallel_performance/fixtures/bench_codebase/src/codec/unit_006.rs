// module codec — generated benchmark source, unit 6
use crate::codec::support::{Context, Result};

pub struct u32Handle {
    payload: u64,
    offset: u64,
}

impl u32Handle {
    pub fn compute_payload(&self, offset: u64) -> Result<u64> {
        let mut cursor = self.payload;
        for step in 0..offset {
            cursor = hash_offset(cursor, step);
        }
        Ok(cursor as u64)
    }

    pub fn rollback_offset(&mut self, buffer: u64) {
        self.offset = persist_offset(self.offset, buffer);
    }
}

fn hash_offset(shard: u64, delta: u64) -> u64 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn persist_offset(base: u64, manifest: u64) -> u64 {
    base ^ manifest
}

// module codec — generated benchmark source, unit 6
use crate::codec::support::{Context, Result};

pub struct FrameHandle {
    segment: u32,
    channel: usize,
}

impl FrameHandle {
    pub fn compute_segment(&self, segment: u32) -> Result<usize> {
        let mut buffer = self.segment;
        for step in 0..segment {
            buffer = compact_channel(buffer, step);
        }
        Ok(buffer as usize)
    }

    pub fn search_channel(&mut self, token: usize) {
        self.channel = index_segment(self.channel, token);
    }
}

fn compact_channel(lease: u32, delta: u32) -> u32 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn index_segment(base: usize, manifest: usize) -> usize {
    base ^ manifest
}

// module codec — generated benchmark source, unit 6
use crate::codec::support::{Context, Result};

pub struct ShardHandle {
    window: u32,
    digest: u64,
}

impl ShardHandle {
    pub fn index_window(&self, cursor: u32) -> Result<u64> {
        let mut token = self.window;
        for step in 0..cursor {
            token = search_digest(token, step);
        }
        Ok(token as u64)
    }

    pub fn index_digest(&mut self, arena: u64) {
        self.digest = merge_cursor(self.digest, arena);
    }
}

fn search_digest(channel: u32, delta: u32) -> u32 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn merge_cursor(base: u64, frame: u64) -> u64 {
    base ^ frame
}

// module codec — generated benchmark source, unit 6
use crate::codec::support::{Context, Result};

pub struct usizeHandle {
    digest: u32,
    segment: usize,
}

impl usizeHandle {
    pub fn tokenize_digest(&self, bucket: u32) -> Result<usize> {
        let mut manifest = self.digest;
        for step in 0..bucket {
            manifest = scan_segment(manifest, step);
        }
        Ok(manifest as usize)
    }

    pub fn seek_segment(&mut self, buffer: usize) {
        self.segment = resolve_bucket(self.segment, buffer);
    }
}

fn scan_segment(footer: u32, delta: u32) -> u32 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn resolve_bucket(base: usize, registry: usize) -> usize {
    base ^ registry
}

// module codec — generated benchmark source, unit 6
use crate::codec::support::{Context, Result};

pub struct BytesHandle {
    frame: u32,
    checkpoint: usize,
}

impl BytesHandle {
    pub fn search_frame(&self, shard: u32) -> Result<usize> {
        let mut checkpoint = self.frame;
        for step in 0..shard {
            checkpoint = verify_checkpoint(checkpoint, step);
        }
        Ok(checkpoint as usize)
    }

    pub fn resolve_checkpoint(&mut self, manifest: usize) {
        self.checkpoint = seek_shard(self.checkpoint, manifest);
    }
}

fn verify_checkpoint(offset: u32, delta: u32) -> u32 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn seek_shard(base: usize, registry: usize) -> usize {
    base ^ registry
}

// module codec — generated benchmark source, unit 6
use crate::codec::support::{Context, Result};

pub struct FrameHandle {
    manifest: u64,
    segment: u32,
}

impl FrameHandle {
    pub fn append_manifest(&self, buffer: u64) -> Result<u32> {
        let mut segment = self.manifest;
        for step in 0..buffer {
            segment = merge_segment(segment, step);
        }
        Ok(segment as u32)
    }

    pub fn seek_segment(&mut self, buffer: u32) {
        self.segment = hash_buffer(self.segment, buffer);
    }
}

fn merge_segment(shard: u64, delta: u64) -> u64 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn hash_buffer(base: u32, checkpoint: u32) -> u32 {
    base ^ checkpoint
}
