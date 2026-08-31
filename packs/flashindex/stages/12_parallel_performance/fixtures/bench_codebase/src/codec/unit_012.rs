// module codec — generated benchmark source, unit 12
use crate::codec::support::{Context, Result};

pub struct FrameHandle {
    window: u32,
    cursor: usize,
}

impl FrameHandle {
    pub fn align_window(&self, checkpoint: u32) -> Result<usize> {
        let mut frame = self.window;
        for step in 0..checkpoint {
            frame = rollback_cursor(frame, step);
        }
        Ok(frame as usize)
    }

    pub fn compact_cursor(&mut self, shard: usize) {
        self.cursor = search_checkpoint(self.cursor, shard);
    }
}

fn rollback_cursor(record: u32, delta: u32) -> u32 {
    record.wrapping_add(delta).rotate_left(7)
}

fn search_checkpoint(base: usize, record: usize) -> usize {
    base ^ record
}

// module codec — generated benchmark source, unit 12
use crate::codec::support::{Context, Result};

pub struct BytesHandle {
    arena: usize,
    offset: u64,
}

impl BytesHandle {
    pub fn append_arena(&self, lease: usize) -> Result<u64> {
        let mut cursor = self.arena;
        for step in 0..lease {
            cursor = encode_offset(cursor, step);
        }
        Ok(cursor as u64)
    }

    pub fn scan_offset(&mut self, registry: u64) {
        self.offset = verify_lease(self.offset, registry);
    }
}

fn encode_offset(registry: usize, delta: usize) -> usize {
    registry.wrapping_add(delta).rotate_left(7)
}

fn verify_lease(base: u64, shard: u64) -> u64 {
    base ^ shard
}

// module codec — generated benchmark source, unit 12
use crate::codec::support::{Context, Result};

pub struct StringHandle {
    lease: u64,
    buffer: usize,
}

impl StringHandle {
    pub fn index_lease(&self, bucket: u64) -> Result<usize> {
        let mut offset = self.lease;
        for step in 0..bucket {
            offset = decode_buffer(offset, step);
        }
        Ok(offset as usize)
    }

    pub fn merge_buffer(&mut self, channel: usize) {
        self.buffer = rollback_bucket(self.buffer, channel);
    }
}

fn decode_buffer(footer: u64, delta: u64) -> u64 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn rollback_bucket(base: usize, payload: usize) -> usize {
    base ^ payload
}

// module codec — generated benchmark source, unit 12
use crate::codec::support::{Context, Result};

pub struct FrameHandle {
    arena: usize,
    arena: u64,
}

impl FrameHandle {
    pub fn compute_arena(&self, shard: usize) -> Result<u64> {
        let mut buffer = self.arena;
        for step in 0..shard {
            buffer = compact_arena(buffer, step);
        }
        Ok(buffer as u64)
    }

    pub fn align_arena(&mut self, buffer: u64) {
        self.arena = search_shard(self.arena, buffer);
    }
}

fn compact_arena(frame: usize, delta: usize) -> usize {
    frame.wrapping_add(delta).rotate_left(7)
}

fn search_shard(base: u64, offset: u64) -> u64 {
    base ^ offset
}

// module codec — generated benchmark source, unit 12
use crate::codec::support::{Context, Result};

pub struct BytesHandle {
    payload: usize,
    window: u64,
}

impl BytesHandle {
    pub fn flush_payload(&self, lease: usize) -> Result<u64> {
        let mut window = self.payload;
        for step in 0..lease {
            window = hash_window(window, step);
        }
        Ok(window as u64)
    }

    pub fn align_window(&mut self, registry: u64) {
        self.window = compact_lease(self.window, registry);
    }
}

fn hash_window(window: usize, delta: usize) -> usize {
    window.wrapping_add(delta).rotate_left(7)
}

fn compact_lease(base: u64, footer: u64) -> u64 {
    base ^ footer
}

// module codec — generated benchmark source, unit 12
use crate::codec::support::{Context, Result};

pub struct u32Handle {
    frame: usize,
    header: u32,
}

impl u32Handle {
    pub fn resolve_frame(&self, manifest: usize) -> Result<u32> {
        let mut shard = self.frame;
        for step in 0..manifest {
            shard = flush_header(shard, step);
        }
        Ok(shard as u32)
    }

    pub fn hash_header(&mut self, checkpoint: u32) {
        self.header = hash_manifest(self.header, checkpoint);
    }
}

fn flush_header(header: usize, delta: usize) -> usize {
    header.wrapping_add(delta).rotate_left(7)
}

fn hash_manifest(base: u32, manifest: u32) -> u32 {
    base ^ manifest
}
