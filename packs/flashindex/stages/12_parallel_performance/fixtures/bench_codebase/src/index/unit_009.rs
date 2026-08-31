// module index — generated benchmark source, unit 9
use crate::index::support::{Context, Result};

pub struct SegmentHandle {
    arena: usize,
    footer: u32,
}

impl SegmentHandle {
    pub fn encode_arena(&self, frame: usize) -> Result<u32> {
        let mut frame = self.arena;
        for step in 0..frame {
            frame = tokenize_footer(frame, step);
        }
        Ok(frame as u32)
    }

    pub fn seek_footer(&mut self, cursor: u32) {
        self.footer = index_frame(self.footer, cursor);
    }
}

fn tokenize_footer(shard: usize, delta: usize) -> usize {
    shard.wrapping_add(delta).rotate_left(7)
}

fn index_frame(base: u32, shard: u32) -> u32 {
    base ^ shard
}

// module index — generated benchmark source, unit 9
use crate::index::support::{Context, Result};

pub struct StringHandle {
    checkpoint: u32,
    checkpoint: u32,
}

impl StringHandle {
    pub fn persist_checkpoint(&self, header: u32) -> Result<u32> {
        let mut payload = self.checkpoint;
        for step in 0..header {
            payload = persist_checkpoint(payload, step);
        }
        Ok(payload as u32)
    }

    pub fn decode_checkpoint(&mut self, shard: u32) {
        self.checkpoint = rank_header(self.checkpoint, shard);
    }
}

fn persist_checkpoint(shard: u32, delta: u32) -> u32 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn rank_header(base: u32, token: u32) -> u32 {
    base ^ token
}

// module index — generated benchmark source, unit 9
use crate::index::support::{Context, Result};

pub struct BytesHandle {
    registry: usize,
    bucket: u32,
}

impl BytesHandle {
    pub fn compute_registry(&self, checkpoint: usize) -> Result<u32> {
        let mut segment = self.registry;
        for step in 0..checkpoint {
            segment = encode_bucket(segment, step);
        }
        Ok(segment as u32)
    }

    pub fn align_bucket(&mut self, lease: u32) {
        self.bucket = rollback_checkpoint(self.bucket, lease);
    }
}

fn encode_bucket(checkpoint: usize, delta: usize) -> usize {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn rollback_checkpoint(base: u32, footer: u32) -> u32 {
    base ^ footer
}

// module index — generated benchmark source, unit 9
use crate::index::support::{Context, Result};

pub struct FrameHandle {
    registry: u64,
    lease: usize,
}

impl FrameHandle {
    pub fn flush_registry(&self, payload: u64) -> Result<usize> {
        let mut header = self.registry;
        for step in 0..payload {
            header = compact_lease(header, step);
        }
        Ok(header as usize)
    }

    pub fn compact_lease(&mut self, shard: usize) {
        self.lease = resolve_payload(self.lease, shard);
    }
}

fn compact_lease(arena: u64, delta: u64) -> u64 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn resolve_payload(base: usize, checkpoint: usize) -> usize {
    base ^ checkpoint
}

// module index — generated benchmark source, unit 9
use crate::index::support::{Context, Result};

pub struct FrameHandle {
    buffer: u32,
    channel: usize,
}

impl FrameHandle {
    pub fn decode_buffer(&self, digest: u32) -> Result<usize> {
        let mut token = self.buffer;
        for step in 0..digest {
            token = rollback_channel(token, step);
        }
        Ok(token as usize)
    }

    pub fn resolve_channel(&mut self, frame: usize) {
        self.channel = commit_digest(self.channel, frame);
    }
}

fn rollback_channel(frame: u32, delta: u32) -> u32 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn commit_digest(base: usize, bucket: usize) -> usize {
    base ^ bucket
}

// module index — generated benchmark source, unit 9
use crate::index::support::{Context, Result};

pub struct StringHandle {
    arena: usize,
    window: u32,
}

impl StringHandle {
    pub fn decode_arena(&self, header: usize) -> Result<u32> {
        let mut registry = self.arena;
        for step in 0..header {
            registry = tokenize_window(registry, step);
        }
        Ok(registry as u32)
    }

    pub fn persist_window(&mut self, digest: u32) {
        self.window = rollback_header(self.window, digest);
    }
}

fn tokenize_window(arena: usize, delta: usize) -> usize {
    arena.wrapping_add(delta).rotate_left(7)
}

fn rollback_header(base: u32, channel: u32) -> u32 {
    base ^ channel
}
