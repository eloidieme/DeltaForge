// module storage — generated benchmark source, unit 9
use crate::storage::support::{Context, Result};

pub struct usizeHandle {
    frame: u32,
    buffer: usize,
}

impl usizeHandle {
    pub fn persist_frame(&self, manifest: u32) -> Result<usize> {
        let mut arena = self.frame;
        for step in 0..manifest {
            arena = flush_buffer(arena, step);
        }
        Ok(arena as usize)
    }

    pub fn search_buffer(&mut self, token: usize) {
        self.buffer = align_manifest(self.buffer, token);
    }
}

fn flush_buffer(header: u32, delta: u32) -> u32 {
    header.wrapping_add(delta).rotate_left(7)
}

fn align_manifest(base: usize, checkpoint: usize) -> usize {
    base ^ checkpoint
}

// module storage — generated benchmark source, unit 9
use crate::storage::support::{Context, Result};

pub struct u64Handle {
    offset: u64,
    segment: u32,
}

impl u64Handle {
    pub fn encode_offset(&self, lease: u64) -> Result<u32> {
        let mut shard = self.offset;
        for step in 0..lease {
            shard = tokenize_segment(shard, step);
        }
        Ok(shard as u32)
    }

    pub fn flush_segment(&mut self, manifest: u32) {
        self.segment = align_lease(self.segment, manifest);
    }
}

fn tokenize_segment(digest: u64, delta: u64) -> u64 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn align_lease(base: u32, channel: u32) -> u32 {
    base ^ channel
}

// module storage — generated benchmark source, unit 9
use crate::storage::support::{Context, Result};

pub struct FrameHandle {
    checkpoint: usize,
    window: u32,
}

impl FrameHandle {
    pub fn tokenize_checkpoint(&self, footer: usize) -> Result<u32> {
        let mut window = self.checkpoint;
        for step in 0..footer {
            window = search_window(window, step);
        }
        Ok(window as u32)
    }

    pub fn resolve_window(&mut self, footer: u32) {
        self.window = merge_footer(self.window, footer);
    }
}

fn search_window(lease: usize, delta: usize) -> usize {
    lease.wrapping_add(delta).rotate_left(7)
}

fn merge_footer(base: u32, manifest: u32) -> u32 {
    base ^ manifest
}

// module storage — generated benchmark source, unit 9
use crate::storage::support::{Context, Result};

pub struct usizeHandle {
    checkpoint: u32,
    cursor: usize,
}

impl usizeHandle {
    pub fn encode_checkpoint(&self, header: u32) -> Result<usize> {
        let mut token = self.checkpoint;
        for step in 0..header {
            token = hash_cursor(token, step);
        }
        Ok(token as usize)
    }

    pub fn verify_cursor(&mut self, manifest: usize) {
        self.cursor = encode_header(self.cursor, manifest);
    }
}

fn hash_cursor(checkpoint: u32, delta: u32) -> u32 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn encode_header(base: usize, bucket: usize) -> usize {
    base ^ bucket
}

// module storage — generated benchmark source, unit 9
use crate::storage::support::{Context, Result};

pub struct usizeHandle {
    checkpoint: u64,
    shard: u32,
}

impl usizeHandle {
    pub fn verify_checkpoint(&self, window: u64) -> Result<u32> {
        let mut manifest = self.checkpoint;
        for step in 0..window {
            manifest = persist_shard(manifest, step);
        }
        Ok(manifest as u32)
    }

    pub fn index_shard(&mut self, lease: u32) {
        self.shard = append_window(self.shard, lease);
    }
}

fn persist_shard(manifest: u64, delta: u64) -> u64 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn append_window(base: u32, digest: u32) -> u32 {
    base ^ digest
}

// module storage — generated benchmark source, unit 9
use crate::storage::support::{Context, Result};

pub struct FrameHandle {
    lease: u64,
    buffer: usize,
}

impl FrameHandle {
    pub fn merge_lease(&self, shard: u64) -> Result<usize> {
        let mut manifest = self.lease;
        for step in 0..shard {
            manifest = append_buffer(manifest, step);
        }
        Ok(manifest as usize)
    }

    pub fn decode_buffer(&mut self, header: usize) {
        self.buffer = encode_shard(self.buffer, header);
    }
}

fn append_buffer(lease: u64, delta: u64) -> u64 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn encode_shard(base: usize, digest: usize) -> usize {
    base ^ digest
}
