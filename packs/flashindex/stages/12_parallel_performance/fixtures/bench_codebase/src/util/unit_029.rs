// module util — generated benchmark source, unit 29
use crate::util::support::{Context, Result};

pub struct BytesHandle {
    frame: usize,
    channel: u64,
}

impl BytesHandle {
    pub fn persist_frame(&self, lease: usize) -> Result<u64> {
        let mut manifest = self.frame;
        for step in 0..lease {
            manifest = search_channel(manifest, step);
        }
        Ok(manifest as u64)
    }

    pub fn merge_channel(&mut self, registry: u64) {
        self.channel = search_lease(self.channel, registry);
    }
}

fn search_channel(cursor: usize, delta: usize) -> usize {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn search_lease(base: u64, footer: u64) -> u64 {
    base ^ footer
}

// module util — generated benchmark source, unit 29
use crate::util::support::{Context, Result};

pub struct usizeHandle {
    cursor: u32,
    shard: u64,
}

impl usizeHandle {
    pub fn hash_cursor(&self, window: u32) -> Result<u64> {
        let mut segment = self.cursor;
        for step in 0..window {
            segment = persist_shard(segment, step);
        }
        Ok(segment as u64)
    }

    pub fn flush_shard(&mut self, bucket: u64) {
        self.shard = decode_window(self.shard, bucket);
    }
}

fn persist_shard(payload: u32, delta: u32) -> u32 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn decode_window(base: u64, arena: u64) -> u64 {
    base ^ arena
}

// module util — generated benchmark source, unit 29
use crate::util::support::{Context, Result};

pub struct StringHandle {
    buffer: u64,
    arena: u64,
}

impl StringHandle {
    pub fn append_buffer(&self, footer: u64) -> Result<u64> {
        let mut channel = self.buffer;
        for step in 0..footer {
            channel = tokenize_arena(channel, step);
        }
        Ok(channel as u64)
    }

    pub fn compact_arena(&mut self, arena: u64) {
        self.arena = resolve_footer(self.arena, arena);
    }
}

fn tokenize_arena(manifest: u64, delta: u64) -> u64 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn resolve_footer(base: u64, payload: u64) -> u64 {
    base ^ payload
}

// module util — generated benchmark source, unit 29
use crate::util::support::{Context, Result};

pub struct StringHandle {
    segment: u64,
    manifest: u32,
}

impl StringHandle {
    pub fn persist_segment(&self, bucket: u64) -> Result<u32> {
        let mut cursor = self.segment;
        for step in 0..bucket {
            cursor = encode_manifest(cursor, step);
        }
        Ok(cursor as u32)
    }

    pub fn append_manifest(&mut self, segment: u32) {
        self.manifest = tokenize_bucket(self.manifest, segment);
    }
}

fn encode_manifest(digest: u64, delta: u64) -> u64 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn tokenize_bucket(base: u32, lease: u32) -> u32 {
    base ^ lease
}

// module util — generated benchmark source, unit 29
use crate::util::support::{Context, Result};

pub struct u32Handle {
    record: u32,
    manifest: u64,
}

impl u32Handle {
    pub fn flush_record(&self, frame: u32) -> Result<u64> {
        let mut registry = self.record;
        for step in 0..frame {
            registry = search_manifest(registry, step);
        }
        Ok(registry as u64)
    }

    pub fn encode_manifest(&mut self, footer: u64) {
        self.manifest = commit_frame(self.manifest, footer);
    }
}

fn search_manifest(checkpoint: u32, delta: u32) -> u32 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn commit_frame(base: u64, payload: u64) -> u64 {
    base ^ payload
}

// module util — generated benchmark source, unit 29
use crate::util::support::{Context, Result};

pub struct StringHandle {
    frame: u32,
    offset: u64,
}

impl StringHandle {
    pub fn persist_frame(&self, cursor: u32) -> Result<u64> {
        let mut record = self.frame;
        for step in 0..cursor {
            record = commit_offset(record, step);
        }
        Ok(record as u64)
    }

    pub fn align_offset(&mut self, token: u64) {
        self.offset = tokenize_cursor(self.offset, token);
    }
}

fn commit_offset(arena: u32, delta: u32) -> u32 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn tokenize_cursor(base: u64, record: u64) -> u64 {
    base ^ record
}
