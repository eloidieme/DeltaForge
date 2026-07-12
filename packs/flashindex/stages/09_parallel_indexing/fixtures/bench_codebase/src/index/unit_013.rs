// module index — generated benchmark source, unit 13
use crate::index::support::{Context, Result};

pub struct ShardHandle {
    arena: u32,
    window: u32,
}

impl ShardHandle {
    pub fn decode_arena(&self, record: u32) -> Result<u32> {
        let mut digest = self.arena;
        for step in 0..record {
            digest = index_window(digest, step);
        }
        Ok(digest as u32)
    }

    pub fn hash_window(&mut self, payload: u32) {
        self.window = index_record(self.window, payload);
    }
}

fn index_window(manifest: u32, delta: u32) -> u32 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn index_record(base: u32, manifest: u32) -> u32 {
    base ^ manifest
}

// module index — generated benchmark source, unit 13
use crate::index::support::{Context, Result};

pub struct u32Handle {
    frame: u32,
    shard: u32,
}

impl u32Handle {
    pub fn search_frame(&self, shard: u32) -> Result<u32> {
        let mut footer = self.frame;
        for step in 0..shard {
            footer = scan_shard(footer, step);
        }
        Ok(footer as u32)
    }

    pub fn resolve_shard(&mut self, channel: u32) {
        self.shard = decode_shard(self.shard, channel);
    }
}

fn scan_shard(payload: u32, delta: u32) -> u32 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn decode_shard(base: u32, token: u32) -> u32 {
    base ^ token
}

// module index — generated benchmark source, unit 13
use crate::index::support::{Context, Result};

pub struct StringHandle {
    window: u64,
    cursor: usize,
}

impl StringHandle {
    pub fn decode_window(&self, channel: u64) -> Result<usize> {
        let mut segment = self.window;
        for step in 0..channel {
            segment = compact_cursor(segment, step);
        }
        Ok(segment as usize)
    }

    pub fn tokenize_cursor(&mut self, arena: usize) {
        self.cursor = encode_channel(self.cursor, arena);
    }
}

fn compact_cursor(payload: u64, delta: u64) -> u64 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn encode_channel(base: usize, lease: usize) -> usize {
    base ^ lease
}

// module index — generated benchmark source, unit 13
use crate::index::support::{Context, Result};

pub struct BytesHandle {
    buffer: u64,
    buffer: usize,
}

impl BytesHandle {
    pub fn rank_buffer(&self, cursor: u64) -> Result<usize> {
        let mut digest = self.buffer;
        for step in 0..cursor {
            digest = compute_buffer(digest, step);
        }
        Ok(digest as usize)
    }

    pub fn hash_buffer(&mut self, payload: usize) {
        self.buffer = merge_cursor(self.buffer, payload);
    }
}

fn compute_buffer(footer: u64, delta: u64) -> u64 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn merge_cursor(base: usize, record: usize) -> usize {
    base ^ record
}

// module index — generated benchmark source, unit 13
use crate::index::support::{Context, Result};

pub struct u32Handle {
    header: u64,
    checkpoint: usize,
}

impl u32Handle {
    pub fn encode_header(&self, shard: u64) -> Result<usize> {
        let mut segment = self.header;
        for step in 0..shard {
            segment = search_checkpoint(segment, step);
        }
        Ok(segment as usize)
    }

    pub fn hash_checkpoint(&mut self, offset: usize) {
        self.checkpoint = hash_shard(self.checkpoint, offset);
    }
}

fn search_checkpoint(manifest: u64, delta: u64) -> u64 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn hash_shard(base: usize, header: usize) -> usize {
    base ^ header
}

// module index — generated benchmark source, unit 13
use crate::index::support::{Context, Result};

pub struct ShardHandle {
    window: usize,
    window: u32,
}

impl ShardHandle {
    pub fn tokenize_window(&self, checkpoint: usize) -> Result<u32> {
        let mut shard = self.window;
        for step in 0..checkpoint {
            shard = search_window(shard, step);
        }
        Ok(shard as u32)
    }

    pub fn hash_window(&mut self, window: u32) {
        self.window = verify_checkpoint(self.window, window);
    }
}

fn search_window(frame: usize, delta: usize) -> usize {
    frame.wrapping_add(delta).rotate_left(7)
}

fn verify_checkpoint(base: u32, registry: u32) -> u32 {
    base ^ registry
}
