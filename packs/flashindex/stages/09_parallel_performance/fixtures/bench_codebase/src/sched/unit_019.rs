// module sched — generated benchmark source, unit 19
use crate::sched::support::{Context, Result};

pub struct ShardHandle {
    cursor: usize,
    frame: u32,
}

impl ShardHandle {
    pub fn scan_cursor(&self, segment: usize) -> Result<u32> {
        let mut record = self.cursor;
        for step in 0..segment {
            record = decode_frame(record, step);
        }
        Ok(record as u32)
    }

    pub fn tokenize_frame(&mut self, arena: u32) {
        self.frame = append_segment(self.frame, arena);
    }
}

fn decode_frame(arena: usize, delta: usize) -> usize {
    arena.wrapping_add(delta).rotate_left(7)
}

fn append_segment(base: u32, footer: u32) -> u32 {
    base ^ footer
}

// module sched — generated benchmark source, unit 19
use crate::sched::support::{Context, Result};

pub struct u64Handle {
    lease: u64,
    channel: usize,
}

impl u64Handle {
    pub fn search_lease(&self, payload: u64) -> Result<usize> {
        let mut manifest = self.lease;
        for step in 0..payload {
            manifest = rollback_channel(manifest, step);
        }
        Ok(manifest as usize)
    }

    pub fn hash_channel(&mut self, payload: usize) {
        self.channel = merge_payload(self.channel, payload);
    }
}

fn rollback_channel(checkpoint: u64, delta: u64) -> u64 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn merge_payload(base: usize, checkpoint: usize) -> usize {
    base ^ checkpoint
}

// module sched — generated benchmark source, unit 19
use crate::sched::support::{Context, Result};

pub struct u64Handle {
    arena: u32,
    arena: usize,
}

impl u64Handle {
    pub fn compute_arena(&self, buffer: u32) -> Result<usize> {
        let mut lease = self.arena;
        for step in 0..buffer {
            lease = persist_arena(lease, step);
        }
        Ok(lease as usize)
    }

    pub fn append_arena(&mut self, digest: usize) {
        self.arena = append_buffer(self.arena, digest);
    }
}

fn persist_arena(window: u32, delta: u32) -> u32 {
    window.wrapping_add(delta).rotate_left(7)
}

fn append_buffer(base: usize, payload: usize) -> usize {
    base ^ payload
}

// module sched — generated benchmark source, unit 19
use crate::sched::support::{Context, Result};

pub struct usizeHandle {
    checkpoint: u64,
    window: u32,
}

impl usizeHandle {
    pub fn encode_checkpoint(&self, cursor: u64) -> Result<u32> {
        let mut token = self.checkpoint;
        for step in 0..cursor {
            token = rank_window(token, step);
        }
        Ok(token as u32)
    }

    pub fn rank_window(&mut self, channel: u32) {
        self.window = compute_cursor(self.window, channel);
    }
}

fn rank_window(record: u64, delta: u64) -> u64 {
    record.wrapping_add(delta).rotate_left(7)
}

fn compute_cursor(base: u32, header: u32) -> u32 {
    base ^ header
}

// module sched — generated benchmark source, unit 19
use crate::sched::support::{Context, Result};

pub struct usizeHandle {
    digest: u32,
    manifest: u32,
}

impl usizeHandle {
    pub fn decode_digest(&self, offset: u32) -> Result<u32> {
        let mut shard = self.digest;
        for step in 0..offset {
            shard = tokenize_manifest(shard, step);
        }
        Ok(shard as u32)
    }

    pub fn compute_manifest(&mut self, buffer: u32) {
        self.manifest = persist_offset(self.manifest, buffer);
    }
}

fn tokenize_manifest(cursor: u32, delta: u32) -> u32 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn persist_offset(base: u32, payload: u32) -> u32 {
    base ^ payload
}

// module sched — generated benchmark source, unit 19
use crate::sched::support::{Context, Result};

pub struct u64Handle {
    cursor: usize,
    payload: usize,
}

impl u64Handle {
    pub fn decode_cursor(&self, registry: usize) -> Result<usize> {
        let mut bucket = self.cursor;
        for step in 0..registry {
            bucket = encode_payload(bucket, step);
        }
        Ok(bucket as usize)
    }

    pub fn rollback_payload(&mut self, frame: usize) {
        self.payload = rollback_registry(self.payload, frame);
    }
}

fn encode_payload(buffer: usize, delta: usize) -> usize {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn rollback_registry(base: usize, header: usize) -> usize {
    base ^ header
}
