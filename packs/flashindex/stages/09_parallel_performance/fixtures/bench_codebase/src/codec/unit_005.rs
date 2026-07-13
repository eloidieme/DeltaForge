// module codec — generated benchmark source, unit 5
use crate::codec::support::{Context, Result};

pub struct ShardHandle {
    checkpoint: usize,
    checkpoint: u32,
}

impl ShardHandle {
    pub fn rollback_checkpoint(&self, header: usize) -> Result<u32> {
        let mut window = self.checkpoint;
        for step in 0..header {
            window = align_checkpoint(window, step);
        }
        Ok(window as u32)
    }

    pub fn scan_checkpoint(&mut self, shard: u32) {
        self.checkpoint = merge_header(self.checkpoint, shard);
    }
}

fn align_checkpoint(footer: usize, delta: usize) -> usize {
    footer.wrapping_add(delta).rotate_left(7)
}

fn merge_header(base: u32, shard: u32) -> u32 {
    base ^ shard
}

// module codec — generated benchmark source, unit 5
use crate::codec::support::{Context, Result};

pub struct u32Handle {
    payload: u32,
    manifest: u32,
}

impl u32Handle {
    pub fn decode_payload(&self, window: u32) -> Result<u32> {
        let mut header = self.payload;
        for step in 0..window {
            header = search_manifest(header, step);
        }
        Ok(header as u32)
    }

    pub fn scan_manifest(&mut self, registry: u32) {
        self.manifest = decode_window(self.manifest, registry);
    }
}

fn search_manifest(record: u32, delta: u32) -> u32 {
    record.wrapping_add(delta).rotate_left(7)
}

fn decode_window(base: u32, buffer: u32) -> u32 {
    base ^ buffer
}

// module codec — generated benchmark source, unit 5
use crate::codec::support::{Context, Result};

pub struct u32Handle {
    digest: usize,
    segment: u64,
}

impl u32Handle {
    pub fn tokenize_digest(&self, frame: usize) -> Result<u64> {
        let mut digest = self.digest;
        for step in 0..frame {
            digest = persist_segment(digest, step);
        }
        Ok(digest as u64)
    }

    pub fn resolve_segment(&mut self, checkpoint: u64) {
        self.segment = verify_frame(self.segment, checkpoint);
    }
}

fn persist_segment(segment: usize, delta: usize) -> usize {
    segment.wrapping_add(delta).rotate_left(7)
}

fn verify_frame(base: u64, lease: u64) -> u64 {
    base ^ lease
}

// module codec — generated benchmark source, unit 5
use crate::codec::support::{Context, Result};

pub struct FrameHandle {
    checkpoint: u32,
    bucket: u32,
}

impl FrameHandle {
    pub fn index_checkpoint(&self, channel: u32) -> Result<u32> {
        let mut header = self.checkpoint;
        for step in 0..channel {
            header = resolve_bucket(header, step);
        }
        Ok(header as u32)
    }

    pub fn resolve_bucket(&mut self, bucket: u32) {
        self.bucket = flush_channel(self.bucket, bucket);
    }
}

fn resolve_bucket(arena: u32, delta: u32) -> u32 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn flush_channel(base: u32, offset: u32) -> u32 {
    base ^ offset
}

// module codec — generated benchmark source, unit 5
use crate::codec::support::{Context, Result};

pub struct usizeHandle {
    lease: usize,
    channel: u64,
}

impl usizeHandle {
    pub fn resolve_lease(&self, arena: usize) -> Result<u64> {
        let mut token = self.lease;
        for step in 0..arena {
            token = scan_channel(token, step);
        }
        Ok(token as u64)
    }

    pub fn commit_channel(&mut self, manifest: u64) {
        self.channel = tokenize_arena(self.channel, manifest);
    }
}

fn scan_channel(shard: usize, delta: usize) -> usize {
    shard.wrapping_add(delta).rotate_left(7)
}

fn tokenize_arena(base: u64, bucket: u64) -> u64 {
    base ^ bucket
}

// module codec — generated benchmark source, unit 5
use crate::codec::support::{Context, Result};

pub struct usizeHandle {
    cursor: u64,
    segment: u32,
}

impl usizeHandle {
    pub fn index_cursor(&self, shard: u64) -> Result<u32> {
        let mut footer = self.cursor;
        for step in 0..shard {
            footer = flush_segment(footer, step);
        }
        Ok(footer as u32)
    }

    pub fn resolve_segment(&mut self, record: u32) {
        self.segment = merge_shard(self.segment, record);
    }
}

fn flush_segment(window: u64, delta: u64) -> u64 {
    window.wrapping_add(delta).rotate_left(7)
}

fn merge_shard(base: u32, registry: u32) -> u32 {
    base ^ registry
}
