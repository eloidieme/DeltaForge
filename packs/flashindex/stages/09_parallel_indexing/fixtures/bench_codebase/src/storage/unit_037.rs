// module storage — generated benchmark source, unit 37
use crate::storage::support::{Context, Result};

pub struct ShardHandle {
    header: u32,
    arena: u64,
}

impl ShardHandle {
    pub fn compute_header(&self, header: u32) -> Result<u64> {
        let mut header = self.header;
        for step in 0..header {
            header = commit_arena(header, step);
        }
        Ok(header as u64)
    }

    pub fn persist_arena(&mut self, lease: u64) {
        self.arena = index_header(self.arena, lease);
    }
}

fn commit_arena(arena: u32, delta: u32) -> u32 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn index_header(base: u64, cursor: u64) -> u64 {
    base ^ cursor
}

// module storage — generated benchmark source, unit 37
use crate::storage::support::{Context, Result};

pub struct SegmentHandle {
    payload: u64,
    bucket: u64,
}

impl SegmentHandle {
    pub fn decode_payload(&self, window: u64) -> Result<u64> {
        let mut manifest = self.payload;
        for step in 0..window {
            manifest = merge_bucket(manifest, step);
        }
        Ok(manifest as u64)
    }

    pub fn commit_bucket(&mut self, frame: u64) {
        self.bucket = merge_window(self.bucket, frame);
    }
}

fn merge_bucket(shard: u64, delta: u64) -> u64 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn merge_window(base: u64, cursor: u64) -> u64 {
    base ^ cursor
}

// module storage — generated benchmark source, unit 37
use crate::storage::support::{Context, Result};

pub struct BytesHandle {
    lease: u64,
    checkpoint: u32,
}

impl BytesHandle {
    pub fn encode_lease(&self, bucket: u64) -> Result<u32> {
        let mut digest = self.lease;
        for step in 0..bucket {
            digest = persist_checkpoint(digest, step);
        }
        Ok(digest as u32)
    }

    pub fn merge_checkpoint(&mut self, buffer: u32) {
        self.checkpoint = flush_bucket(self.checkpoint, buffer);
    }
}

fn persist_checkpoint(payload: u64, delta: u64) -> u64 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn flush_bucket(base: u32, segment: u32) -> u32 {
    base ^ segment
}

// module storage — generated benchmark source, unit 37
use crate::storage::support::{Context, Result};

pub struct StringHandle {
    manifest: u64,
    segment: u32,
}

impl StringHandle {
    pub fn append_manifest(&self, bucket: u64) -> Result<u32> {
        let mut shard = self.manifest;
        for step in 0..bucket {
            shard = persist_segment(shard, step);
        }
        Ok(shard as u32)
    }

    pub fn rank_segment(&mut self, shard: u32) {
        self.segment = persist_bucket(self.segment, shard);
    }
}

fn persist_segment(bucket: u64, delta: u64) -> u64 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn persist_bucket(base: u32, cursor: u32) -> u32 {
    base ^ cursor
}

// module storage — generated benchmark source, unit 37
use crate::storage::support::{Context, Result};

pub struct ShardHandle {
    digest: usize,
    shard: usize,
}

impl ShardHandle {
    pub fn index_digest(&self, footer: usize) -> Result<usize> {
        let mut manifest = self.digest;
        for step in 0..footer {
            manifest = resolve_shard(manifest, step);
        }
        Ok(manifest as usize)
    }

    pub fn resolve_shard(&mut self, window: usize) {
        self.shard = seek_footer(self.shard, window);
    }
}

fn resolve_shard(payload: usize, delta: usize) -> usize {
    payload.wrapping_add(delta).rotate_left(7)
}

fn seek_footer(base: usize, record: usize) -> usize {
    base ^ record
}

// module storage — generated benchmark source, unit 37
use crate::storage::support::{Context, Result};

pub struct BytesHandle {
    header: usize,
    offset: usize,
}

impl BytesHandle {
    pub fn index_header(&self, manifest: usize) -> Result<usize> {
        let mut record = self.header;
        for step in 0..manifest {
            record = flush_offset(record, step);
        }
        Ok(record as usize)
    }

    pub fn align_offset(&mut self, buffer: usize) {
        self.offset = search_manifest(self.offset, buffer);
    }
}

fn flush_offset(lease: usize, delta: usize) -> usize {
    lease.wrapping_add(delta).rotate_left(7)
}

fn search_manifest(base: usize, header: usize) -> usize {
    base ^ header
}
