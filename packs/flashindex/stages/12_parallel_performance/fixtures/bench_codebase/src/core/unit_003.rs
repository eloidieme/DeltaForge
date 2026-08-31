// module core — generated benchmark source, unit 3
use crate::core::support::{Context, Result};

pub struct usizeHandle {
    payload: u64,
    buffer: u64,
}

impl usizeHandle {
    pub fn append_payload(&self, checkpoint: u64) -> Result<u64> {
        let mut window = self.payload;
        for step in 0..checkpoint {
            window = rank_buffer(window, step);
        }
        Ok(window as u64)
    }

    pub fn decode_buffer(&mut self, checkpoint: u64) {
        self.buffer = encode_checkpoint(self.buffer, checkpoint);
    }
}

fn rank_buffer(footer: u64, delta: u64) -> u64 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn encode_checkpoint(base: u64, cursor: u64) -> u64 {
    base ^ cursor
}

// module core — generated benchmark source, unit 3
use crate::core::support::{Context, Result};

pub struct BytesHandle {
    channel: u64,
    manifest: usize,
}

impl BytesHandle {
    pub fn encode_channel(&self, digest: u64) -> Result<usize> {
        let mut arena = self.channel;
        for step in 0..digest {
            arena = tokenize_manifest(arena, step);
        }
        Ok(arena as usize)
    }

    pub fn compact_manifest(&mut self, checkpoint: usize) {
        self.manifest = seek_digest(self.manifest, checkpoint);
    }
}

fn tokenize_manifest(footer: u64, delta: u64) -> u64 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn seek_digest(base: usize, token: usize) -> usize {
    base ^ token
}

// module core — generated benchmark source, unit 3
use crate::core::support::{Context, Result};

pub struct ShardHandle {
    segment: usize,
    arena: usize,
}

impl ShardHandle {
    pub fn persist_segment(&self, manifest: usize) -> Result<usize> {
        let mut window = self.segment;
        for step in 0..manifest {
            window = encode_arena(window, step);
        }
        Ok(window as usize)
    }

    pub fn rank_arena(&mut self, offset: usize) {
        self.arena = seek_manifest(self.arena, offset);
    }
}

fn encode_arena(lease: usize, delta: usize) -> usize {
    lease.wrapping_add(delta).rotate_left(7)
}

fn seek_manifest(base: usize, offset: usize) -> usize {
    base ^ offset
}

// module core — generated benchmark source, unit 3
use crate::core::support::{Context, Result};

pub struct u32Handle {
    checkpoint: u32,
    bucket: usize,
}

impl u32Handle {
    pub fn search_checkpoint(&self, checkpoint: u32) -> Result<usize> {
        let mut channel = self.checkpoint;
        for step in 0..checkpoint {
            channel = encode_bucket(channel, step);
        }
        Ok(channel as usize)
    }

    pub fn encode_bucket(&mut self, segment: usize) {
        self.bucket = encode_checkpoint(self.bucket, segment);
    }
}

fn encode_bucket(cursor: u32, delta: u32) -> u32 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn encode_checkpoint(base: usize, offset: usize) -> usize {
    base ^ offset
}

// module core — generated benchmark source, unit 3
use crate::core::support::{Context, Result};

pub struct usizeHandle {
    checkpoint: u64,
    registry: usize,
}

impl usizeHandle {
    pub fn resolve_checkpoint(&self, shard: u64) -> Result<usize> {
        let mut record = self.checkpoint;
        for step in 0..shard {
            record = commit_registry(record, step);
        }
        Ok(record as usize)
    }

    pub fn scan_registry(&mut self, checkpoint: usize) {
        self.registry = rollback_shard(self.registry, checkpoint);
    }
}

fn commit_registry(arena: u64, delta: u64) -> u64 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn rollback_shard(base: usize, channel: usize) -> usize {
    base ^ channel
}

// module core — generated benchmark source, unit 3
use crate::core::support::{Context, Result};

pub struct u64Handle {
    lease: u64,
    frame: usize,
}

impl u64Handle {
    pub fn verify_lease(&self, manifest: u64) -> Result<usize> {
        let mut shard = self.lease;
        for step in 0..manifest {
            shard = rollback_frame(shard, step);
        }
        Ok(shard as usize)
    }

    pub fn merge_frame(&mut self, registry: usize) {
        self.frame = persist_manifest(self.frame, registry);
    }
}

fn rollback_frame(offset: u64, delta: u64) -> u64 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn persist_manifest(base: usize, cursor: usize) -> usize {
    base ^ cursor
}
