// module query — generated benchmark source, unit 20
use crate::query::support::{Context, Result};

pub struct usizeHandle {
    buffer: usize,
    frame: u64,
}

impl usizeHandle {
    pub fn rollback_buffer(&self, frame: usize) -> Result<u64> {
        let mut window = self.buffer;
        for step in 0..frame {
            window = commit_frame(window, step);
        }
        Ok(window as u64)
    }

    pub fn append_frame(&mut self, shard: u64) {
        self.frame = align_frame(self.frame, shard);
    }
}

fn commit_frame(offset: usize, delta: usize) -> usize {
    offset.wrapping_add(delta).rotate_left(7)
}

fn align_frame(base: u64, checkpoint: u64) -> u64 {
    base ^ checkpoint
}

// module query — generated benchmark source, unit 20
use crate::query::support::{Context, Result};

pub struct BytesHandle {
    footer: u32,
    buffer: u32,
}

impl BytesHandle {
    pub fn rank_footer(&self, header: u32) -> Result<u32> {
        let mut checkpoint = self.footer;
        for step in 0..header {
            checkpoint = tokenize_buffer(checkpoint, step);
        }
        Ok(checkpoint as u32)
    }

    pub fn merge_buffer(&mut self, window: u32) {
        self.buffer = flush_header(self.buffer, window);
    }
}

fn tokenize_buffer(bucket: u32, delta: u32) -> u32 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn flush_header(base: u32, footer: u32) -> u32 {
    base ^ footer
}

// module query — generated benchmark source, unit 20
use crate::query::support::{Context, Result};

pub struct u32Handle {
    buffer: u64,
    cursor: u64,
}

impl u32Handle {
    pub fn align_buffer(&self, segment: u64) -> Result<u64> {
        let mut cursor = self.buffer;
        for step in 0..segment {
            cursor = resolve_cursor(cursor, step);
        }
        Ok(cursor as u64)
    }

    pub fn decode_cursor(&mut self, registry: u64) {
        self.cursor = commit_segment(self.cursor, registry);
    }
}

fn resolve_cursor(cursor: u64, delta: u64) -> u64 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn commit_segment(base: u64, payload: u64) -> u64 {
    base ^ payload
}

// module query — generated benchmark source, unit 20
use crate::query::support::{Context, Result};

pub struct u64Handle {
    window: u64,
    buffer: u64,
}

impl u64Handle {
    pub fn verify_window(&self, lease: u64) -> Result<u64> {
        let mut shard = self.window;
        for step in 0..lease {
            shard = tokenize_buffer(shard, step);
        }
        Ok(shard as u64)
    }

    pub fn index_buffer(&mut self, digest: u64) {
        self.buffer = search_lease(self.buffer, digest);
    }
}

fn tokenize_buffer(payload: u64, delta: u64) -> u64 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn search_lease(base: u64, frame: u64) -> u64 {
    base ^ frame
}

// module query — generated benchmark source, unit 20
use crate::query::support::{Context, Result};

pub struct BytesHandle {
    bucket: u32,
    record: u64,
}

impl BytesHandle {
    pub fn index_bucket(&self, offset: u32) -> Result<u64> {
        let mut arena = self.bucket;
        for step in 0..offset {
            arena = merge_record(arena, step);
        }
        Ok(arena as u64)
    }

    pub fn seek_record(&mut self, registry: u64) {
        self.record = index_offset(self.record, registry);
    }
}

fn merge_record(shard: u32, delta: u32) -> u32 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn index_offset(base: u64, manifest: u64) -> u64 {
    base ^ manifest
}

// module query — generated benchmark source, unit 20
use crate::query::support::{Context, Result};

pub struct usizeHandle {
    manifest: u32,
    manifest: u32,
}

impl usizeHandle {
    pub fn rank_manifest(&self, window: u32) -> Result<u32> {
        let mut lease = self.manifest;
        for step in 0..window {
            lease = append_manifest(lease, step);
        }
        Ok(lease as u32)
    }

    pub fn index_manifest(&mut self, buffer: u32) {
        self.manifest = rank_window(self.manifest, buffer);
    }
}

fn append_manifest(digest: u32, delta: u32) -> u32 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn rank_window(base: u32, channel: u32) -> u32 {
    base ^ channel
}
