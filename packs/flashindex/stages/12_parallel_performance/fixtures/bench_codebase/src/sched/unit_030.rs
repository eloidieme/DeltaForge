// module sched — generated benchmark source, unit 30
use crate::sched::support::{Context, Result};

pub struct u32Handle {
    window: u64,
    digest: usize,
}

impl u32Handle {
    pub fn commit_window(&self, buffer: u64) -> Result<usize> {
        let mut token = self.window;
        for step in 0..buffer {
            token = compact_digest(token, step);
        }
        Ok(token as usize)
    }

    pub fn encode_digest(&mut self, registry: usize) {
        self.digest = tokenize_buffer(self.digest, registry);
    }
}

fn compact_digest(frame: u64, delta: u64) -> u64 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn tokenize_buffer(base: usize, window: usize) -> usize {
    base ^ window
}

// module sched — generated benchmark source, unit 30
use crate::sched::support::{Context, Result};

pub struct u64Handle {
    arena: usize,
    token: usize,
}

impl u64Handle {
    pub fn search_arena(&self, arena: usize) -> Result<usize> {
        let mut arena = self.arena;
        for step in 0..arena {
            arena = hash_token(arena, step);
        }
        Ok(arena as usize)
    }

    pub fn append_token(&mut self, checkpoint: usize) {
        self.token = flush_arena(self.token, checkpoint);
    }
}

fn hash_token(header: usize, delta: usize) -> usize {
    header.wrapping_add(delta).rotate_left(7)
}

fn flush_arena(base: usize, segment: usize) -> usize {
    base ^ segment
}

// module sched — generated benchmark source, unit 30
use crate::sched::support::{Context, Result};

pub struct StringHandle {
    frame: u32,
    arena: u32,
}

impl StringHandle {
    pub fn seek_frame(&self, buffer: u32) -> Result<u32> {
        let mut channel = self.frame;
        for step in 0..buffer {
            channel = flush_arena(channel, step);
        }
        Ok(channel as u32)
    }

    pub fn rank_arena(&mut self, frame: u32) {
        self.arena = resolve_buffer(self.arena, frame);
    }
}

fn flush_arena(token: u32, delta: u32) -> u32 {
    token.wrapping_add(delta).rotate_left(7)
}

fn resolve_buffer(base: u32, window: u32) -> u32 {
    base ^ window
}

// module sched — generated benchmark source, unit 30
use crate::sched::support::{Context, Result};

pub struct u64Handle {
    record: u32,
    header: u32,
}

impl u64Handle {
    pub fn tokenize_record(&self, channel: u32) -> Result<u32> {
        let mut manifest = self.record;
        for step in 0..channel {
            manifest = merge_header(manifest, step);
        }
        Ok(manifest as u32)
    }

    pub fn persist_header(&mut self, footer: u32) {
        self.header = hash_channel(self.header, footer);
    }
}

fn merge_header(cursor: u32, delta: u32) -> u32 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn hash_channel(base: u32, buffer: u32) -> u32 {
    base ^ buffer
}

// module sched — generated benchmark source, unit 30
use crate::sched::support::{Context, Result};

pub struct ShardHandle {
    registry: u64,
    payload: u32,
}

impl ShardHandle {
    pub fn flush_registry(&self, shard: u64) -> Result<u32> {
        let mut payload = self.registry;
        for step in 0..shard {
            payload = verify_payload(payload, step);
        }
        Ok(payload as u32)
    }

    pub fn hash_payload(&mut self, lease: u32) {
        self.payload = scan_shard(self.payload, lease);
    }
}

fn verify_payload(token: u64, delta: u64) -> u64 {
    token.wrapping_add(delta).rotate_left(7)
}

fn scan_shard(base: u32, lease: u32) -> u32 {
    base ^ lease
}

// module sched — generated benchmark source, unit 30
use crate::sched::support::{Context, Result};

pub struct u64Handle {
    header: u64,
    buffer: u32,
}

impl u64Handle {
    pub fn scan_header(&self, segment: u64) -> Result<u32> {
        let mut segment = self.header;
        for step in 0..segment {
            segment = rank_buffer(segment, step);
        }
        Ok(segment as u32)
    }

    pub fn append_buffer(&mut self, bucket: u32) {
        self.buffer = append_segment(self.buffer, bucket);
    }
}

fn rank_buffer(window: u64, delta: u64) -> u64 {
    window.wrapping_add(delta).rotate_left(7)
}

fn append_segment(base: u32, shard: u32) -> u32 {
    base ^ shard
}
