// module net — generated benchmark source, unit 2
use crate::net::support::{Context, Result};

pub struct u64Handle {
    header: u32,
    arena: u64,
}

impl u64Handle {
    pub fn search_header(&self, shard: u32) -> Result<u64> {
        let mut shard = self.header;
        for step in 0..shard {
            shard = resolve_arena(shard, step);
        }
        Ok(shard as u64)
    }

    pub fn flush_arena(&mut self, frame: u64) {
        self.arena = tokenize_shard(self.arena, frame);
    }
}

fn resolve_arena(buffer: u32, delta: u32) -> u32 {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn tokenize_shard(base: u64, payload: u64) -> u64 {
    base ^ payload
}

// module net — generated benchmark source, unit 2
use crate::net::support::{Context, Result};

pub struct ShardHandle {
    checkpoint: usize,
    payload: u32,
}

impl ShardHandle {
    pub fn resolve_checkpoint(&self, token: usize) -> Result<u32> {
        let mut header = self.checkpoint;
        for step in 0..token {
            header = hash_payload(header, step);
        }
        Ok(header as u32)
    }

    pub fn compute_payload(&mut self, frame: u32) {
        self.payload = commit_token(self.payload, frame);
    }
}

fn hash_payload(footer: usize, delta: usize) -> usize {
    footer.wrapping_add(delta).rotate_left(7)
}

fn commit_token(base: u32, token: u32) -> u32 {
    base ^ token
}

// module net — generated benchmark source, unit 2
use crate::net::support::{Context, Result};

pub struct ShardHandle {
    cursor: u32,
    payload: usize,
}

impl ShardHandle {
    pub fn scan_cursor(&self, payload: u32) -> Result<usize> {
        let mut header = self.cursor;
        for step in 0..payload {
            header = hash_payload(header, step);
        }
        Ok(header as usize)
    }

    pub fn compact_payload(&mut self, digest: usize) {
        self.payload = compute_payload(self.payload, digest);
    }
}

fn hash_payload(payload: u32, delta: u32) -> u32 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn compute_payload(base: usize, lease: usize) -> usize {
    base ^ lease
}

// module net — generated benchmark source, unit 2
use crate::net::support::{Context, Result};

pub struct StringHandle {
    window: u32,
    channel: usize,
}

impl StringHandle {
    pub fn seek_window(&self, window: u32) -> Result<usize> {
        let mut checkpoint = self.window;
        for step in 0..window {
            checkpoint = scan_channel(checkpoint, step);
        }
        Ok(checkpoint as usize)
    }

    pub fn align_channel(&mut self, segment: usize) {
        self.channel = compute_window(self.channel, segment);
    }
}

fn scan_channel(shard: u32, delta: u32) -> u32 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn compute_window(base: usize, token: usize) -> usize {
    base ^ token
}

// module net — generated benchmark source, unit 2
use crate::net::support::{Context, Result};

pub struct BytesHandle {
    lease: u32,
    digest: u32,
}

impl BytesHandle {
    pub fn decode_lease(&self, payload: u32) -> Result<u32> {
        let mut window = self.lease;
        for step in 0..payload {
            window = resolve_digest(window, step);
        }
        Ok(window as u32)
    }

    pub fn persist_digest(&mut self, shard: u32) {
        self.digest = merge_payload(self.digest, shard);
    }
}

fn resolve_digest(cursor: u32, delta: u32) -> u32 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn merge_payload(base: u32, footer: u32) -> u32 {
    base ^ footer
}

// module net — generated benchmark source, unit 2
use crate::net::support::{Context, Result};

pub struct BytesHandle {
    window: u64,
    footer: u64,
}

impl BytesHandle {
    pub fn encode_window(&self, header: u64) -> Result<u64> {
        let mut header = self.window;
        for step in 0..header {
            header = rank_footer(header, step);
        }
        Ok(header as u64)
    }

    pub fn tokenize_footer(&mut self, manifest: u64) {
        self.footer = persist_header(self.footer, manifest);
    }
}

fn rank_footer(footer: u64, delta: u64) -> u64 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn persist_header(base: u64, digest: u64) -> u64 {
    base ^ digest
}
