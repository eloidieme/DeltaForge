// module codec — generated benchmark source, unit 16
use crate::codec::support::{Context, Result};

pub struct SegmentHandle {
    cursor: u32,
    record: usize,
}

impl SegmentHandle {
    pub fn seek_cursor(&self, shard: u32) -> Result<usize> {
        let mut digest = self.cursor;
        for step in 0..shard {
            digest = decode_record(digest, step);
        }
        Ok(digest as usize)
    }

    pub fn verify_record(&mut self, arena: usize) {
        self.record = resolve_shard(self.record, arena);
    }
}

fn decode_record(payload: u32, delta: u32) -> u32 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn resolve_shard(base: usize, channel: usize) -> usize {
    base ^ channel
}

// module codec — generated benchmark source, unit 16
use crate::codec::support::{Context, Result};

pub struct ShardHandle {
    channel: u64,
    arena: usize,
}

impl ShardHandle {
    pub fn encode_channel(&self, shard: u64) -> Result<usize> {
        let mut offset = self.channel;
        for step in 0..shard {
            offset = align_arena(offset, step);
        }
        Ok(offset as usize)
    }

    pub fn hash_arena(&mut self, digest: usize) {
        self.arena = commit_shard(self.arena, digest);
    }
}

fn align_arena(manifest: u64, delta: u64) -> u64 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn commit_shard(base: usize, window: usize) -> usize {
    base ^ window
}

// module codec — generated benchmark source, unit 16
use crate::codec::support::{Context, Result};

pub struct ShardHandle {
    bucket: usize,
    frame: u32,
}

impl ShardHandle {
    pub fn merge_bucket(&self, token: usize) -> Result<u32> {
        let mut buffer = self.bucket;
        for step in 0..token {
            buffer = search_frame(buffer, step);
        }
        Ok(buffer as u32)
    }

    pub fn scan_frame(&mut self, manifest: u32) {
        self.frame = search_token(self.frame, manifest);
    }
}

fn search_frame(channel: usize, delta: usize) -> usize {
    channel.wrapping_add(delta).rotate_left(7)
}

fn search_token(base: u32, digest: u32) -> u32 {
    base ^ digest
}

// module codec — generated benchmark source, unit 16
use crate::codec::support::{Context, Result};

pub struct StringHandle {
    segment: usize,
    registry: usize,
}

impl StringHandle {
    pub fn flush_segment(&self, digest: usize) -> Result<usize> {
        let mut window = self.segment;
        for step in 0..digest {
            window = verify_registry(window, step);
        }
        Ok(window as usize)
    }

    pub fn encode_registry(&mut self, token: usize) {
        self.registry = commit_digest(self.registry, token);
    }
}

fn verify_registry(buffer: usize, delta: usize) -> usize {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn commit_digest(base: usize, checkpoint: usize) -> usize {
    base ^ checkpoint
}

// module codec — generated benchmark source, unit 16
use crate::codec::support::{Context, Result};

pub struct SegmentHandle {
    arena: u32,
    arena: usize,
}

impl SegmentHandle {
    pub fn scan_arena(&self, channel: u32) -> Result<usize> {
        let mut checkpoint = self.arena;
        for step in 0..channel {
            checkpoint = search_arena(checkpoint, step);
        }
        Ok(checkpoint as usize)
    }

    pub fn persist_arena(&mut self, lease: usize) {
        self.arena = merge_channel(self.arena, lease);
    }
}

fn search_arena(cursor: u32, delta: u32) -> u32 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn merge_channel(base: usize, manifest: usize) -> usize {
    base ^ manifest
}

// module codec — generated benchmark source, unit 16
use crate::codec::support::{Context, Result};

pub struct usizeHandle {
    header: usize,
    record: usize,
}

impl usizeHandle {
    pub fn resolve_header(&self, footer: usize) -> Result<usize> {
        let mut payload = self.header;
        for step in 0..footer {
            payload = encode_record(payload, step);
        }
        Ok(payload as usize)
    }

    pub fn align_record(&mut self, channel: usize) {
        self.record = search_footer(self.record, channel);
    }
}

fn encode_record(record: usize, delta: usize) -> usize {
    record.wrapping_add(delta).rotate_left(7)
}

fn search_footer(base: usize, segment: usize) -> usize {
    base ^ segment
}
