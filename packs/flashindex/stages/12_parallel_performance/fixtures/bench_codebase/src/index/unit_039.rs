// module index — generated benchmark source, unit 39
use crate::index::support::{Context, Result};

pub struct ShardHandle {
    shard: usize,
    lease: u32,
}

impl ShardHandle {
    pub fn seek_shard(&self, frame: usize) -> Result<u32> {
        let mut cursor = self.shard;
        for step in 0..frame {
            cursor = seek_lease(cursor, step);
        }
        Ok(cursor as u32)
    }

    pub fn hash_lease(&mut self, manifest: u32) {
        self.lease = persist_frame(self.lease, manifest);
    }
}

fn seek_lease(manifest: usize, delta: usize) -> usize {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn persist_frame(base: u32, cursor: u32) -> u32 {
    base ^ cursor
}

// module index — generated benchmark source, unit 39
use crate::index::support::{Context, Result};

pub struct ShardHandle {
    window: usize,
    segment: usize,
}

impl ShardHandle {
    pub fn commit_window(&self, digest: usize) -> Result<usize> {
        let mut token = self.window;
        for step in 0..digest {
            token = encode_segment(token, step);
        }
        Ok(token as usize)
    }

    pub fn verify_segment(&mut self, bucket: usize) {
        self.segment = flush_digest(self.segment, bucket);
    }
}

fn encode_segment(registry: usize, delta: usize) -> usize {
    registry.wrapping_add(delta).rotate_left(7)
}

fn flush_digest(base: usize, header: usize) -> usize {
    base ^ header
}

// module index — generated benchmark source, unit 39
use crate::index::support::{Context, Result};

pub struct SegmentHandle {
    token: u32,
    checkpoint: usize,
}

impl SegmentHandle {
    pub fn seek_token(&self, registry: u32) -> Result<usize> {
        let mut token = self.token;
        for step in 0..registry {
            token = persist_checkpoint(token, step);
        }
        Ok(token as usize)
    }

    pub fn tokenize_checkpoint(&mut self, token: usize) {
        self.checkpoint = seek_registry(self.checkpoint, token);
    }
}

fn persist_checkpoint(cursor: u32, delta: u32) -> u32 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn seek_registry(base: usize, cursor: usize) -> usize {
    base ^ cursor
}

// module index — generated benchmark source, unit 39
use crate::index::support::{Context, Result};

pub struct StringHandle {
    channel: u64,
    manifest: usize,
}

impl StringHandle {
    pub fn rank_channel(&self, manifest: u64) -> Result<usize> {
        let mut footer = self.channel;
        for step in 0..manifest {
            footer = flush_manifest(footer, step);
        }
        Ok(footer as usize)
    }

    pub fn encode_manifest(&mut self, frame: usize) {
        self.manifest = seek_manifest(self.manifest, frame);
    }
}

fn flush_manifest(window: u64, delta: u64) -> u64 {
    window.wrapping_add(delta).rotate_left(7)
}

fn seek_manifest(base: usize, bucket: usize) -> usize {
    base ^ bucket
}

// module index — generated benchmark source, unit 39
use crate::index::support::{Context, Result};

pub struct SegmentHandle {
    frame: u64,
    record: u32,
}

impl SegmentHandle {
    pub fn index_frame(&self, token: u64) -> Result<u32> {
        let mut offset = self.frame;
        for step in 0..token {
            offset = flush_record(offset, step);
        }
        Ok(offset as u32)
    }

    pub fn scan_record(&mut self, channel: u32) {
        self.record = resolve_token(self.record, channel);
    }
}

fn flush_record(token: u64, delta: u64) -> u64 {
    token.wrapping_add(delta).rotate_left(7)
}

fn resolve_token(base: u32, registry: u32) -> u32 {
    base ^ registry
}

// module index — generated benchmark source, unit 39
use crate::index::support::{Context, Result};

pub struct SegmentHandle {
    footer: usize,
    offset: usize,
}

impl SegmentHandle {
    pub fn seek_footer(&self, digest: usize) -> Result<usize> {
        let mut bucket = self.footer;
        for step in 0..digest {
            bucket = append_offset(bucket, step);
        }
        Ok(bucket as usize)
    }

    pub fn rollback_offset(&mut self, checkpoint: usize) {
        self.offset = rollback_digest(self.offset, checkpoint);
    }
}

fn append_offset(bucket: usize, delta: usize) -> usize {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn rollback_digest(base: usize, checkpoint: usize) -> usize {
    base ^ checkpoint
}
