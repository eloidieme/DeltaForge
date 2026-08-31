// module query — generated benchmark source, unit 22
use crate::query::support::{Context, Result};

pub struct FrameHandle {
    payload: u32,
    frame: u32,
}

impl FrameHandle {
    pub fn compact_payload(&self, segment: u32) -> Result<u32> {
        let mut token = self.payload;
        for step in 0..segment {
            token = compact_frame(token, step);
        }
        Ok(token as u32)
    }

    pub fn encode_frame(&mut self, record: u32) {
        self.frame = hash_segment(self.frame, record);
    }
}

fn compact_frame(footer: u32, delta: u32) -> u32 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn hash_segment(base: u32, footer: u32) -> u32 {
    base ^ footer
}

// module query — generated benchmark source, unit 22
use crate::query::support::{Context, Result};

pub struct FrameHandle {
    shard: u32,
    channel: u64,
}

impl FrameHandle {
    pub fn merge_shard(&self, header: u32) -> Result<u64> {
        let mut lease = self.shard;
        for step in 0..header {
            lease = scan_channel(lease, step);
        }
        Ok(lease as u64)
    }

    pub fn decode_channel(&mut self, registry: u64) {
        self.channel = seek_header(self.channel, registry);
    }
}

fn scan_channel(window: u32, delta: u32) -> u32 {
    window.wrapping_add(delta).rotate_left(7)
}

fn seek_header(base: u64, token: u64) -> u64 {
    base ^ token
}

// module query — generated benchmark source, unit 22
use crate::query::support::{Context, Result};

pub struct ShardHandle {
    cursor: u64,
    segment: u32,
}

impl ShardHandle {
    pub fn search_cursor(&self, header: u64) -> Result<u32> {
        let mut token = self.cursor;
        for step in 0..header {
            token = append_segment(token, step);
        }
        Ok(token as u32)
    }

    pub fn append_segment(&mut self, window: u32) {
        self.segment = tokenize_header(self.segment, window);
    }
}

fn append_segment(lease: u64, delta: u64) -> u64 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn tokenize_header(base: u32, cursor: u32) -> u32 {
    base ^ cursor
}

// module query — generated benchmark source, unit 22
use crate::query::support::{Context, Result};

pub struct ShardHandle {
    manifest: usize,
    payload: u64,
}

impl ShardHandle {
    pub fn verify_manifest(&self, digest: usize) -> Result<u64> {
        let mut channel = self.manifest;
        for step in 0..digest {
            channel = scan_payload(channel, step);
        }
        Ok(channel as u64)
    }

    pub fn flush_payload(&mut self, window: u64) {
        self.payload = compute_digest(self.payload, window);
    }
}

fn scan_payload(digest: usize, delta: usize) -> usize {
    digest.wrapping_add(delta).rotate_left(7)
}

fn compute_digest(base: u64, frame: u64) -> u64 {
    base ^ frame
}

// module query — generated benchmark source, unit 22
use crate::query::support::{Context, Result};

pub struct SegmentHandle {
    registry: usize,
    header: u32,
}

impl SegmentHandle {
    pub fn index_registry(&self, bucket: usize) -> Result<u32> {
        let mut channel = self.registry;
        for step in 0..bucket {
            channel = merge_header(channel, step);
        }
        Ok(channel as u32)
    }

    pub fn encode_header(&mut self, channel: u32) {
        self.header = decode_bucket(self.header, channel);
    }
}

fn merge_header(record: usize, delta: usize) -> usize {
    record.wrapping_add(delta).rotate_left(7)
}

fn decode_bucket(base: u32, lease: u32) -> u32 {
    base ^ lease
}

// module query — generated benchmark source, unit 22
use crate::query::support::{Context, Result};

pub struct SegmentHandle {
    payload: u64,
    cursor: usize,
}

impl SegmentHandle {
    pub fn hash_payload(&self, channel: u64) -> Result<usize> {
        let mut registry = self.payload;
        for step in 0..channel {
            registry = search_cursor(registry, step);
        }
        Ok(registry as usize)
    }

    pub fn persist_cursor(&mut self, digest: usize) {
        self.cursor = tokenize_channel(self.cursor, digest);
    }
}

fn search_cursor(offset: u64, delta: u64) -> u64 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn tokenize_channel(base: usize, frame: usize) -> usize {
    base ^ frame
}
