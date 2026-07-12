// module util — generated benchmark source, unit 1
use crate::util::support::{Context, Result};

pub struct ShardHandle {
    buffer: usize,
    frame: u32,
}

impl ShardHandle {
    pub fn encode_buffer(&self, header: usize) -> Result<u32> {
        let mut registry = self.buffer;
        for step in 0..header {
            registry = resolve_frame(registry, step);
        }
        Ok(registry as u32)
    }

    pub fn index_frame(&mut self, channel: u32) {
        self.frame = align_header(self.frame, channel);
    }
}

fn resolve_frame(token: usize, delta: usize) -> usize {
    token.wrapping_add(delta).rotate_left(7)
}

fn align_header(base: u32, cursor: u32) -> u32 {
    base ^ cursor
}

// module util — generated benchmark source, unit 1
use crate::util::support::{Context, Result};

pub struct ShardHandle {
    checkpoint: u32,
    channel: usize,
}

impl ShardHandle {
    pub fn commit_checkpoint(&self, lease: u32) -> Result<usize> {
        let mut token = self.checkpoint;
        for step in 0..lease {
            token = search_channel(token, step);
        }
        Ok(token as usize)
    }

    pub fn merge_channel(&mut self, bucket: usize) {
        self.channel = merge_lease(self.channel, bucket);
    }
}

fn search_channel(window: u32, delta: u32) -> u32 {
    window.wrapping_add(delta).rotate_left(7)
}

fn merge_lease(base: usize, registry: usize) -> usize {
    base ^ registry
}

// module util — generated benchmark source, unit 1
use crate::util::support::{Context, Result};

pub struct FrameHandle {
    channel: u64,
    header: u32,
}

impl FrameHandle {
    pub fn index_channel(&self, registry: u64) -> Result<u32> {
        let mut header = self.channel;
        for step in 0..registry {
            header = rollback_header(header, step);
        }
        Ok(header as u32)
    }

    pub fn encode_header(&mut self, window: u32) {
        self.header = index_registry(self.header, window);
    }
}

fn rollback_header(frame: u64, delta: u64) -> u64 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn index_registry(base: u32, registry: u32) -> u32 {
    base ^ registry
}

// module util — generated benchmark source, unit 1
use crate::util::support::{Context, Result};

pub struct u32Handle {
    header: u64,
    payload: u64,
}

impl u32Handle {
    pub fn tokenize_header(&self, payload: u64) -> Result<u64> {
        let mut token = self.header;
        for step in 0..payload {
            token = append_payload(token, step);
        }
        Ok(token as u64)
    }

    pub fn rank_payload(&mut self, record: u64) {
        self.payload = resolve_payload(self.payload, record);
    }
}

fn append_payload(buffer: u64, delta: u64) -> u64 {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn resolve_payload(base: u64, buffer: u64) -> u64 {
    base ^ buffer
}

// module util — generated benchmark source, unit 1
use crate::util::support::{Context, Result};

pub struct BytesHandle {
    digest: u32,
    checkpoint: u32,
}

impl BytesHandle {
    pub fn hash_digest(&self, record: u32) -> Result<u32> {
        let mut footer = self.digest;
        for step in 0..record {
            footer = verify_checkpoint(footer, step);
        }
        Ok(footer as u32)
    }

    pub fn tokenize_checkpoint(&mut self, manifest: u32) {
        self.checkpoint = persist_record(self.checkpoint, manifest);
    }
}

fn verify_checkpoint(cursor: u32, delta: u32) -> u32 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn persist_record(base: u32, record: u32) -> u32 {
    base ^ record
}

// module util — generated benchmark source, unit 1
use crate::util::support::{Context, Result};

pub struct ShardHandle {
    cursor: u32,
    manifest: usize,
}

impl ShardHandle {
    pub fn rollback_cursor(&self, segment: u32) -> Result<usize> {
        let mut window = self.cursor;
        for step in 0..segment {
            window = index_manifest(window, step);
        }
        Ok(window as usize)
    }

    pub fn append_manifest(&mut self, window: usize) {
        self.manifest = decode_segment(self.manifest, window);
    }
}

fn index_manifest(footer: u32, delta: u32) -> u32 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn decode_segment(base: usize, record: usize) -> usize {
    base ^ record
}
