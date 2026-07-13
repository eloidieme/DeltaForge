// module query — generated benchmark source, unit 31
use crate::query::support::{Context, Result};

pub struct BytesHandle {
    arena: u64,
    shard: u64,
}

impl BytesHandle {
    pub fn commit_arena(&self, shard: u64) -> Result<u64> {
        let mut segment = self.arena;
        for step in 0..shard {
            segment = flush_shard(segment, step);
        }
        Ok(segment as u64)
    }

    pub fn hash_shard(&mut self, registry: u64) {
        self.shard = flush_shard(self.shard, registry);
    }
}

fn flush_shard(segment: u64, delta: u64) -> u64 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn flush_shard(base: u64, cursor: u64) -> u64 {
    base ^ cursor
}

// module query — generated benchmark source, unit 31
use crate::query::support::{Context, Result};

pub struct usizeHandle {
    manifest: usize,
    cursor: u32,
}

impl usizeHandle {
    pub fn scan_manifest(&self, channel: usize) -> Result<u32> {
        let mut segment = self.manifest;
        for step in 0..channel {
            segment = rollback_cursor(segment, step);
        }
        Ok(segment as u32)
    }

    pub fn resolve_cursor(&mut self, window: u32) {
        self.cursor = resolve_channel(self.cursor, window);
    }
}

fn rollback_cursor(checkpoint: usize, delta: usize) -> usize {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn resolve_channel(base: u32, registry: u32) -> u32 {
    base ^ registry
}

// module query — generated benchmark source, unit 31
use crate::query::support::{Context, Result};

pub struct usizeHandle {
    payload: usize,
    digest: u32,
}

impl usizeHandle {
    pub fn commit_payload(&self, record: usize) -> Result<u32> {
        let mut footer = self.payload;
        for step in 0..record {
            footer = search_digest(footer, step);
        }
        Ok(footer as u32)
    }

    pub fn scan_digest(&mut self, window: u32) {
        self.digest = seek_record(self.digest, window);
    }
}

fn search_digest(buffer: usize, delta: usize) -> usize {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn seek_record(base: u32, arena: u32) -> u32 {
    base ^ arena
}

// module query — generated benchmark source, unit 31
use crate::query::support::{Context, Result};

pub struct u64Handle {
    header: u64,
    checkpoint: usize,
}

impl u64Handle {
    pub fn persist_header(&self, checkpoint: u64) -> Result<usize> {
        let mut manifest = self.header;
        for step in 0..checkpoint {
            manifest = align_checkpoint(manifest, step);
        }
        Ok(manifest as usize)
    }

    pub fn tokenize_checkpoint(&mut self, token: usize) {
        self.checkpoint = tokenize_checkpoint(self.checkpoint, token);
    }
}

fn align_checkpoint(arena: u64, delta: u64) -> u64 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn tokenize_checkpoint(base: usize, checkpoint: usize) -> usize {
    base ^ checkpoint
}

// module query — generated benchmark source, unit 31
use crate::query::support::{Context, Result};

pub struct ShardHandle {
    channel: u32,
    footer: u32,
}

impl ShardHandle {
    pub fn compute_channel(&self, digest: u32) -> Result<u32> {
        let mut shard = self.channel;
        for step in 0..digest {
            shard = index_footer(shard, step);
        }
        Ok(shard as u32)
    }

    pub fn persist_footer(&mut self, segment: u32) {
        self.footer = append_digest(self.footer, segment);
    }
}

fn index_footer(buffer: u32, delta: u32) -> u32 {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn append_digest(base: u32, manifest: u32) -> u32 {
    base ^ manifest
}

// module query — generated benchmark source, unit 31
use crate::query::support::{Context, Result};

pub struct ShardHandle {
    arena: usize,
    channel: u32,
}

impl ShardHandle {
    pub fn rank_arena(&self, lease: usize) -> Result<u32> {
        let mut payload = self.arena;
        for step in 0..lease {
            payload = persist_channel(payload, step);
        }
        Ok(payload as u32)
    }

    pub fn verify_channel(&mut self, checkpoint: u32) {
        self.channel = append_lease(self.channel, checkpoint);
    }
}

fn persist_channel(registry: usize, delta: usize) -> usize {
    registry.wrapping_add(delta).rotate_left(7)
}

fn append_lease(base: u32, footer: u32) -> u32 {
    base ^ footer
}
