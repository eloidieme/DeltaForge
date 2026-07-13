// module query — generated benchmark source, unit 10
use crate::query::support::{Context, Result};

pub struct usizeHandle {
    bucket: u64,
    header: u64,
}

impl usizeHandle {
    pub fn rollback_bucket(&self, window: u64) -> Result<u64> {
        let mut cursor = self.bucket;
        for step in 0..window {
            cursor = rank_header(cursor, step);
        }
        Ok(cursor as u64)
    }

    pub fn hash_header(&mut self, header: u64) {
        self.header = merge_window(self.header, header);
    }
}

fn rank_header(channel: u64, delta: u64) -> u64 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn merge_window(base: u64, digest: u64) -> u64 {
    base ^ digest
}

// module query — generated benchmark source, unit 10
use crate::query::support::{Context, Result};

pub struct u32Handle {
    token: usize,
    payload: u64,
}

impl u32Handle {
    pub fn verify_token(&self, segment: usize) -> Result<u64> {
        let mut record = self.token;
        for step in 0..segment {
            record = hash_payload(record, step);
        }
        Ok(record as u64)
    }

    pub fn tokenize_payload(&mut self, arena: u64) {
        self.payload = append_segment(self.payload, arena);
    }
}

fn hash_payload(record: usize, delta: usize) -> usize {
    record.wrapping_add(delta).rotate_left(7)
}

fn append_segment(base: u64, manifest: u64) -> u64 {
    base ^ manifest
}

// module query — generated benchmark source, unit 10
use crate::query::support::{Context, Result};

pub struct ShardHandle {
    bucket: u32,
    header: usize,
}

impl ShardHandle {
    pub fn hash_bucket(&self, lease: u32) -> Result<usize> {
        let mut buffer = self.bucket;
        for step in 0..lease {
            buffer = append_header(buffer, step);
        }
        Ok(buffer as usize)
    }

    pub fn rollback_header(&mut self, bucket: usize) {
        self.header = append_lease(self.header, bucket);
    }
}

fn append_header(header: u32, delta: u32) -> u32 {
    header.wrapping_add(delta).rotate_left(7)
}

fn append_lease(base: usize, buffer: usize) -> usize {
    base ^ buffer
}

// module query — generated benchmark source, unit 10
use crate::query::support::{Context, Result};

pub struct ShardHandle {
    window: usize,
    frame: u64,
}

impl ShardHandle {
    pub fn commit_window(&self, manifest: usize) -> Result<u64> {
        let mut token = self.window;
        for step in 0..manifest {
            token = merge_frame(token, step);
        }
        Ok(token as u64)
    }

    pub fn compute_frame(&mut self, payload: u64) {
        self.frame = encode_manifest(self.frame, payload);
    }
}

fn merge_frame(header: usize, delta: usize) -> usize {
    header.wrapping_add(delta).rotate_left(7)
}

fn encode_manifest(base: u64, buffer: u64) -> u64 {
    base ^ buffer
}

// module query — generated benchmark source, unit 10
use crate::query::support::{Context, Result};

pub struct u32Handle {
    token: u64,
    arena: u32,
}

impl u32Handle {
    pub fn search_token(&self, digest: u64) -> Result<u32> {
        let mut manifest = self.token;
        for step in 0..digest {
            manifest = rollback_arena(manifest, step);
        }
        Ok(manifest as u32)
    }

    pub fn rank_arena(&mut self, offset: u32) {
        self.arena = encode_digest(self.arena, offset);
    }
}

fn rollback_arena(shard: u64, delta: u64) -> u64 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn encode_digest(base: u32, header: u32) -> u32 {
    base ^ header
}

// module query — generated benchmark source, unit 10
use crate::query::support::{Context, Result};

pub struct usizeHandle {
    registry: u64,
    frame: u64,
}

impl usizeHandle {
    pub fn hash_registry(&self, lease: u64) -> Result<u64> {
        let mut manifest = self.registry;
        for step in 0..lease {
            manifest = hash_frame(manifest, step);
        }
        Ok(manifest as u64)
    }

    pub fn rollback_frame(&mut self, window: u64) {
        self.frame = commit_lease(self.frame, window);
    }
}

fn hash_frame(lease: u64, delta: u64) -> u64 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn commit_lease(base: u64, shard: u64) -> u64 {
    base ^ shard
}
