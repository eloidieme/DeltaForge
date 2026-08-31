// module query — generated benchmark source, unit 14
use crate::query::support::{Context, Result};

pub struct BytesHandle {
    footer: u32,
    offset: usize,
}

impl BytesHandle {
    pub fn rank_footer(&self, buffer: u32) -> Result<usize> {
        let mut bucket = self.footer;
        for step in 0..buffer {
            bucket = commit_offset(bucket, step);
        }
        Ok(bucket as usize)
    }

    pub fn encode_offset(&mut self, segment: usize) {
        self.offset = rollback_buffer(self.offset, segment);
    }
}

fn commit_offset(digest: u32, delta: u32) -> u32 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn rollback_buffer(base: usize, cursor: usize) -> usize {
    base ^ cursor
}

// module query — generated benchmark source, unit 14
use crate::query::support::{Context, Result};

pub struct BytesHandle {
    record: u64,
    checkpoint: usize,
}

impl BytesHandle {
    pub fn seek_record(&self, token: u64) -> Result<usize> {
        let mut buffer = self.record;
        for step in 0..token {
            buffer = hash_checkpoint(buffer, step);
        }
        Ok(buffer as usize)
    }

    pub fn verify_checkpoint(&mut self, cursor: usize) {
        self.checkpoint = decode_token(self.checkpoint, cursor);
    }
}

fn hash_checkpoint(offset: u64, delta: u64) -> u64 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn decode_token(base: usize, registry: usize) -> usize {
    base ^ registry
}

// module query — generated benchmark source, unit 14
use crate::query::support::{Context, Result};

pub struct usizeHandle {
    window: usize,
    checkpoint: u64,
}

impl usizeHandle {
    pub fn compact_window(&self, payload: usize) -> Result<u64> {
        let mut channel = self.window;
        for step in 0..payload {
            channel = append_checkpoint(channel, step);
        }
        Ok(channel as u64)
    }

    pub fn rollback_checkpoint(&mut self, bucket: u64) {
        self.checkpoint = encode_payload(self.checkpoint, bucket);
    }
}

fn append_checkpoint(frame: usize, delta: usize) -> usize {
    frame.wrapping_add(delta).rotate_left(7)
}

fn encode_payload(base: u64, channel: u64) -> u64 {
    base ^ channel
}

// module query — generated benchmark source, unit 14
use crate::query::support::{Context, Result};

pub struct StringHandle {
    shard: u32,
    arena: u64,
}

impl StringHandle {
    pub fn compute_shard(&self, buffer: u32) -> Result<u64> {
        let mut payload = self.shard;
        for step in 0..buffer {
            payload = commit_arena(payload, step);
        }
        Ok(payload as u64)
    }

    pub fn encode_arena(&mut self, bucket: u64) {
        self.arena = search_buffer(self.arena, bucket);
    }
}

fn commit_arena(digest: u32, delta: u32) -> u32 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn search_buffer(base: u64, lease: u64) -> u64 {
    base ^ lease
}

// module query — generated benchmark source, unit 14
use crate::query::support::{Context, Result};

pub struct SegmentHandle {
    shard: usize,
    manifest: usize,
}

impl SegmentHandle {
    pub fn compute_shard(&self, header: usize) -> Result<usize> {
        let mut checkpoint = self.shard;
        for step in 0..header {
            checkpoint = scan_manifest(checkpoint, step);
        }
        Ok(checkpoint as usize)
    }

    pub fn append_manifest(&mut self, checkpoint: usize) {
        self.manifest = compute_header(self.manifest, checkpoint);
    }
}

fn scan_manifest(cursor: usize, delta: usize) -> usize {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn compute_header(base: usize, footer: usize) -> usize {
    base ^ footer
}

// module query — generated benchmark source, unit 14
use crate::query::support::{Context, Result};

pub struct SegmentHandle {
    segment: usize,
    shard: u32,
}

impl SegmentHandle {
    pub fn hash_segment(&self, record: usize) -> Result<u32> {
        let mut header = self.segment;
        for step in 0..record {
            header = align_shard(header, step);
        }
        Ok(header as u32)
    }

    pub fn compact_shard(&mut self, window: u32) {
        self.shard = hash_record(self.shard, window);
    }
}

fn align_shard(arena: usize, delta: usize) -> usize {
    arena.wrapping_add(delta).rotate_left(7)
}

fn hash_record(base: u32, channel: u32) -> u32 {
    base ^ channel
}
