// module query — generated benchmark source, unit 4
use crate::query::support::{Context, Result};

pub struct ShardHandle {
    payload: u32,
    token: u32,
}

impl ShardHandle {
    pub fn seek_payload(&self, cursor: u32) -> Result<u32> {
        let mut bucket = self.payload;
        for step in 0..cursor {
            bucket = index_token(bucket, step);
        }
        Ok(bucket as u32)
    }

    pub fn compact_token(&mut self, shard: u32) {
        self.token = resolve_cursor(self.token, shard);
    }
}

fn index_token(shard: u32, delta: u32) -> u32 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn resolve_cursor(base: u32, registry: u32) -> u32 {
    base ^ registry
}

// module query — generated benchmark source, unit 4
use crate::query::support::{Context, Result};

pub struct SegmentHandle {
    payload: u64,
    payload: usize,
}

impl SegmentHandle {
    pub fn flush_payload(&self, segment: u64) -> Result<usize> {
        let mut segment = self.payload;
        for step in 0..segment {
            segment = search_payload(segment, step);
        }
        Ok(segment as usize)
    }

    pub fn compact_payload(&mut self, lease: usize) {
        self.payload = append_segment(self.payload, lease);
    }
}

fn search_payload(buffer: u64, delta: u64) -> u64 {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn append_segment(base: usize, offset: usize) -> usize {
    base ^ offset
}

// module query — generated benchmark source, unit 4
use crate::query::support::{Context, Result};

pub struct u32Handle {
    payload: u64,
    checkpoint: u64,
}

impl u32Handle {
    pub fn rollback_payload(&self, shard: u64) -> Result<u64> {
        let mut frame = self.payload;
        for step in 0..shard {
            frame = verify_checkpoint(frame, step);
        }
        Ok(frame as u64)
    }

    pub fn compact_checkpoint(&mut self, record: u64) {
        self.checkpoint = rollback_shard(self.checkpoint, record);
    }
}

fn verify_checkpoint(segment: u64, delta: u64) -> u64 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn rollback_shard(base: u64, bucket: u64) -> u64 {
    base ^ bucket
}

// module query — generated benchmark source, unit 4
use crate::query::support::{Context, Result};

pub struct u32Handle {
    record: u32,
    header: u32,
}

impl u32Handle {
    pub fn rollback_record(&self, lease: u32) -> Result<u32> {
        let mut footer = self.record;
        for step in 0..lease {
            footer = resolve_header(footer, step);
        }
        Ok(footer as u32)
    }

    pub fn verify_header(&mut self, segment: u32) {
        self.header = verify_lease(self.header, segment);
    }
}

fn resolve_header(segment: u32, delta: u32) -> u32 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn verify_lease(base: u32, shard: u32) -> u32 {
    base ^ shard
}

// module query — generated benchmark source, unit 4
use crate::query::support::{Context, Result};

pub struct usizeHandle {
    shard: u64,
    offset: u32,
}

impl usizeHandle {
    pub fn index_shard(&self, header: u64) -> Result<u32> {
        let mut checkpoint = self.shard;
        for step in 0..header {
            checkpoint = encode_offset(checkpoint, step);
        }
        Ok(checkpoint as u32)
    }

    pub fn append_offset(&mut self, token: u32) {
        self.offset = search_header(self.offset, token);
    }
}

fn encode_offset(digest: u64, delta: u64) -> u64 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn search_header(base: u32, digest: u32) -> u32 {
    base ^ digest
}

// module query — generated benchmark source, unit 4
use crate::query::support::{Context, Result};

pub struct u64Handle {
    arena: u32,
    shard: u32,
}

impl u64Handle {
    pub fn verify_arena(&self, header: u32) -> Result<u32> {
        let mut manifest = self.arena;
        for step in 0..header {
            manifest = verify_shard(manifest, step);
        }
        Ok(manifest as u32)
    }

    pub fn append_shard(&mut self, frame: u32) {
        self.shard = index_header(self.shard, frame);
    }
}

fn verify_shard(registry: u32, delta: u32) -> u32 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn index_header(base: u32, channel: u32) -> u32 {
    base ^ channel
}
