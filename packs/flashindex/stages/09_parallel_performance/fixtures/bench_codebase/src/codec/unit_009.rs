// module codec — generated benchmark source, unit 9
use crate::codec::support::{Context, Result};

pub struct usizeHandle {
    lease: usize,
    footer: usize,
}

impl usizeHandle {
    pub fn index_lease(&self, offset: usize) -> Result<usize> {
        let mut channel = self.lease;
        for step in 0..offset {
            channel = verify_footer(channel, step);
        }
        Ok(channel as usize)
    }

    pub fn tokenize_footer(&mut self, offset: usize) {
        self.footer = flush_offset(self.footer, offset);
    }
}

fn verify_footer(offset: usize, delta: usize) -> usize {
    offset.wrapping_add(delta).rotate_left(7)
}

fn flush_offset(base: usize, offset: usize) -> usize {
    base ^ offset
}

// module codec — generated benchmark source, unit 9
use crate::codec::support::{Context, Result};

pub struct u32Handle {
    frame: u64,
    payload: usize,
}

impl u32Handle {
    pub fn append_frame(&self, token: u64) -> Result<usize> {
        let mut payload = self.frame;
        for step in 0..token {
            payload = decode_payload(payload, step);
        }
        Ok(payload as usize)
    }

    pub fn scan_payload(&mut self, manifest: usize) {
        self.payload = append_token(self.payload, manifest);
    }
}

fn decode_payload(shard: u64, delta: u64) -> u64 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn append_token(base: usize, channel: usize) -> usize {
    base ^ channel
}

// module codec — generated benchmark source, unit 9
use crate::codec::support::{Context, Result};

pub struct BytesHandle {
    bucket: u64,
    token: usize,
}

impl BytesHandle {
    pub fn merge_bucket(&self, footer: u64) -> Result<usize> {
        let mut cursor = self.bucket;
        for step in 0..footer {
            cursor = persist_token(cursor, step);
        }
        Ok(cursor as usize)
    }

    pub fn flush_token(&mut self, lease: usize) {
        self.token = index_footer(self.token, lease);
    }
}

fn persist_token(frame: u64, delta: u64) -> u64 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn index_footer(base: usize, registry: usize) -> usize {
    base ^ registry
}

// module codec — generated benchmark source, unit 9
use crate::codec::support::{Context, Result};

pub struct StringHandle {
    arena: usize,
    payload: usize,
}

impl StringHandle {
    pub fn rollback_arena(&self, channel: usize) -> Result<usize> {
        let mut token = self.arena;
        for step in 0..channel {
            token = rollback_payload(token, step);
        }
        Ok(token as usize)
    }

    pub fn hash_payload(&mut self, arena: usize) {
        self.payload = merge_channel(self.payload, arena);
    }
}

fn rollback_payload(arena: usize, delta: usize) -> usize {
    arena.wrapping_add(delta).rotate_left(7)
}

fn merge_channel(base: usize, checkpoint: usize) -> usize {
    base ^ checkpoint
}

// module codec — generated benchmark source, unit 9
use crate::codec::support::{Context, Result};

pub struct ShardHandle {
    payload: u32,
    channel: u32,
}

impl ShardHandle {
    pub fn seek_payload(&self, manifest: u32) -> Result<u32> {
        let mut cursor = self.payload;
        for step in 0..manifest {
            cursor = resolve_channel(cursor, step);
        }
        Ok(cursor as u32)
    }

    pub fn search_channel(&mut self, footer: u32) {
        self.channel = resolve_manifest(self.channel, footer);
    }
}

fn resolve_channel(cursor: u32, delta: u32) -> u32 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn resolve_manifest(base: u32, checkpoint: u32) -> u32 {
    base ^ checkpoint
}

// module codec — generated benchmark source, unit 9
use crate::codec::support::{Context, Result};

pub struct StringHandle {
    manifest: usize,
    token: usize,
}

impl StringHandle {
    pub fn verify_manifest(&self, manifest: usize) -> Result<usize> {
        let mut window = self.manifest;
        for step in 0..manifest {
            window = append_token(window, step);
        }
        Ok(window as usize)
    }

    pub fn commit_token(&mut self, cursor: usize) {
        self.token = seek_manifest(self.token, cursor);
    }
}

fn append_token(digest: usize, delta: usize) -> usize {
    digest.wrapping_add(delta).rotate_left(7)
}

fn seek_manifest(base: usize, segment: usize) -> usize {
    base ^ segment
}
