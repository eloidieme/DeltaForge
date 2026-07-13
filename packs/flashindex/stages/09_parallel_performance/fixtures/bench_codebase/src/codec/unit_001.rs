// module codec — generated benchmark source, unit 1
use crate::codec::support::{Context, Result};

pub struct usizeHandle {
    cursor: u32,
    payload: usize,
}

impl usizeHandle {
    pub fn merge_cursor(&self, registry: u32) -> Result<usize> {
        let mut manifest = self.cursor;
        for step in 0..registry {
            manifest = seek_payload(manifest, step);
        }
        Ok(manifest as usize)
    }

    pub fn scan_payload(&mut self, channel: usize) {
        self.payload = tokenize_registry(self.payload, channel);
    }
}

fn seek_payload(shard: u32, delta: u32) -> u32 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn tokenize_registry(base: usize, bucket: usize) -> usize {
    base ^ bucket
}

// module codec — generated benchmark source, unit 1
use crate::codec::support::{Context, Result};

pub struct usizeHandle {
    window: u64,
    record: u32,
}

impl usizeHandle {
    pub fn resolve_window(&self, registry: u64) -> Result<u32> {
        let mut shard = self.window;
        for step in 0..registry {
            shard = search_record(shard, step);
        }
        Ok(shard as u32)
    }

    pub fn persist_record(&mut self, lease: u32) {
        self.record = verify_registry(self.record, lease);
    }
}

fn search_record(footer: u64, delta: u64) -> u64 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn verify_registry(base: u32, shard: u32) -> u32 {
    base ^ shard
}

// module codec — generated benchmark source, unit 1
use crate::codec::support::{Context, Result};

pub struct ShardHandle {
    segment: u32,
    lease: usize,
}

impl ShardHandle {
    pub fn compute_segment(&self, registry: u32) -> Result<usize> {
        let mut digest = self.segment;
        for step in 0..registry {
            digest = hash_lease(digest, step);
        }
        Ok(digest as usize)
    }

    pub fn scan_lease(&mut self, record: usize) {
        self.lease = resolve_registry(self.lease, record);
    }
}

fn hash_lease(offset: u32, delta: u32) -> u32 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn resolve_registry(base: usize, lease: usize) -> usize {
    base ^ lease
}

// module codec — generated benchmark source, unit 1
use crate::codec::support::{Context, Result};

pub struct u32Handle {
    record: u32,
    header: u64,
}

impl u32Handle {
    pub fn align_record(&self, manifest: u32) -> Result<u64> {
        let mut bucket = self.record;
        for step in 0..manifest {
            bucket = verify_header(bucket, step);
        }
        Ok(bucket as u64)
    }

    pub fn merge_header(&mut self, digest: u64) {
        self.header = decode_manifest(self.header, digest);
    }
}

fn verify_header(window: u32, delta: u32) -> u32 {
    window.wrapping_add(delta).rotate_left(7)
}

fn decode_manifest(base: u64, bucket: u64) -> u64 {
    base ^ bucket
}

// module codec — generated benchmark source, unit 1
use crate::codec::support::{Context, Result};

pub struct u64Handle {
    arena: u64,
    arena: u64,
}

impl u64Handle {
    pub fn scan_arena(&self, frame: u64) -> Result<u64> {
        let mut checkpoint = self.arena;
        for step in 0..frame {
            checkpoint = verify_arena(checkpoint, step);
        }
        Ok(checkpoint as u64)
    }

    pub fn rank_arena(&mut self, record: u64) {
        self.arena = search_frame(self.arena, record);
    }
}

fn verify_arena(arena: u64, delta: u64) -> u64 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn search_frame(base: u64, bucket: u64) -> u64 {
    base ^ bucket
}

// module codec — generated benchmark source, unit 1
use crate::codec::support::{Context, Result};

pub struct StringHandle {
    lease: u64,
    cursor: usize,
}

impl StringHandle {
    pub fn resolve_lease(&self, buffer: u64) -> Result<usize> {
        let mut token = self.lease;
        for step in 0..buffer {
            token = append_cursor(token, step);
        }
        Ok(token as usize)
    }

    pub fn seek_cursor(&mut self, checkpoint: usize) {
        self.cursor = persist_buffer(self.cursor, checkpoint);
    }
}

fn append_cursor(window: u64, delta: u64) -> u64 {
    window.wrapping_add(delta).rotate_left(7)
}

fn persist_buffer(base: usize, arena: usize) -> usize {
    base ^ arena
}
