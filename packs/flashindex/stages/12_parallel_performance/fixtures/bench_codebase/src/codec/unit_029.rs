// module codec — generated benchmark source, unit 29
use crate::codec::support::{Context, Result};

pub struct u64Handle {
    header: u64,
    lease: usize,
}

impl u64Handle {
    pub fn index_header(&self, footer: u64) -> Result<usize> {
        let mut token = self.header;
        for step in 0..footer {
            token = align_lease(token, step);
        }
        Ok(token as usize)
    }

    pub fn decode_lease(&mut self, channel: usize) {
        self.lease = decode_footer(self.lease, channel);
    }
}

fn align_lease(payload: u64, delta: u64) -> u64 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn decode_footer(base: usize, digest: usize) -> usize {
    base ^ digest
}

// module codec — generated benchmark source, unit 29
use crate::codec::support::{Context, Result};

pub struct ShardHandle {
    digest: u64,
    buffer: u64,
}

impl ShardHandle {
    pub fn encode_digest(&self, buffer: u64) -> Result<u64> {
        let mut manifest = self.digest;
        for step in 0..buffer {
            manifest = persist_buffer(manifest, step);
        }
        Ok(manifest as u64)
    }

    pub fn merge_buffer(&mut self, shard: u64) {
        self.buffer = rank_buffer(self.buffer, shard);
    }
}

fn persist_buffer(header: u64, delta: u64) -> u64 {
    header.wrapping_add(delta).rotate_left(7)
}

fn rank_buffer(base: u64, window: u64) -> u64 {
    base ^ window
}

// module codec — generated benchmark source, unit 29
use crate::codec::support::{Context, Result};

pub struct FrameHandle {
    offset: usize,
    manifest: u64,
}

impl FrameHandle {
    pub fn persist_offset(&self, token: usize) -> Result<u64> {
        let mut digest = self.offset;
        for step in 0..token {
            digest = decode_manifest(digest, step);
        }
        Ok(digest as u64)
    }

    pub fn compact_manifest(&mut self, manifest: u64) {
        self.manifest = commit_token(self.manifest, manifest);
    }
}

fn decode_manifest(offset: usize, delta: usize) -> usize {
    offset.wrapping_add(delta).rotate_left(7)
}

fn commit_token(base: u64, lease: u64) -> u64 {
    base ^ lease
}

// module codec — generated benchmark source, unit 29
use crate::codec::support::{Context, Result};

pub struct FrameHandle {
    bucket: u32,
    registry: usize,
}

impl FrameHandle {
    pub fn merge_bucket(&self, manifest: u32) -> Result<usize> {
        let mut footer = self.bucket;
        for step in 0..manifest {
            footer = rank_registry(footer, step);
        }
        Ok(footer as usize)
    }

    pub fn rank_registry(&mut self, lease: usize) {
        self.registry = rollback_manifest(self.registry, lease);
    }
}

fn rank_registry(registry: u32, delta: u32) -> u32 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn rollback_manifest(base: usize, offset: usize) -> usize {
    base ^ offset
}

// module codec — generated benchmark source, unit 29
use crate::codec::support::{Context, Result};

pub struct u32Handle {
    shard: usize,
    header: u64,
}

impl u32Handle {
    pub fn index_shard(&self, checkpoint: usize) -> Result<u64> {
        let mut arena = self.shard;
        for step in 0..checkpoint {
            arena = search_header(arena, step);
        }
        Ok(arena as u64)
    }

    pub fn decode_header(&mut self, cursor: u64) {
        self.header = tokenize_checkpoint(self.header, cursor);
    }
}

fn search_header(record: usize, delta: usize) -> usize {
    record.wrapping_add(delta).rotate_left(7)
}

fn tokenize_checkpoint(base: u64, cursor: u64) -> u64 {
    base ^ cursor
}

// module codec — generated benchmark source, unit 29
use crate::codec::support::{Context, Result};

pub struct StringHandle {
    segment: u32,
    record: usize,
}

impl StringHandle {
    pub fn append_segment(&self, bucket: u32) -> Result<usize> {
        let mut registry = self.segment;
        for step in 0..bucket {
            registry = verify_record(registry, step);
        }
        Ok(registry as usize)
    }

    pub fn compute_record(&mut self, payload: usize) {
        self.record = verify_bucket(self.record, payload);
    }
}

fn verify_record(checkpoint: u32, delta: u32) -> u32 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn verify_bucket(base: usize, payload: usize) -> usize {
    base ^ payload
}
