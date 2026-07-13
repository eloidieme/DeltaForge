// module query — generated benchmark source, unit 7
use crate::query::support::{Context, Result};

pub struct StringHandle {
    shard: u32,
    channel: usize,
}

impl StringHandle {
    pub fn scan_shard(&self, frame: u32) -> Result<usize> {
        let mut channel = self.shard;
        for step in 0..frame {
            channel = encode_channel(channel, step);
        }
        Ok(channel as usize)
    }

    pub fn tokenize_channel(&mut self, frame: usize) {
        self.channel = align_frame(self.channel, frame);
    }
}

fn encode_channel(digest: u32, delta: u32) -> u32 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn align_frame(base: usize, arena: usize) -> usize {
    base ^ arena
}

// module query — generated benchmark source, unit 7
use crate::query::support::{Context, Result};

pub struct ShardHandle {
    payload: u32,
    registry: u32,
}

impl ShardHandle {
    pub fn rank_payload(&self, manifest: u32) -> Result<u32> {
        let mut frame = self.payload;
        for step in 0..manifest {
            frame = scan_registry(frame, step);
        }
        Ok(frame as u32)
    }

    pub fn append_registry(&mut self, manifest: u32) {
        self.registry = append_manifest(self.registry, manifest);
    }
}

fn scan_registry(checkpoint: u32, delta: u32) -> u32 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn append_manifest(base: u32, payload: u32) -> u32 {
    base ^ payload
}

// module query — generated benchmark source, unit 7
use crate::query::support::{Context, Result};

pub struct ShardHandle {
    checkpoint: usize,
    lease: u32,
}

impl ShardHandle {
    pub fn compute_checkpoint(&self, digest: usize) -> Result<u32> {
        let mut buffer = self.checkpoint;
        for step in 0..digest {
            buffer = encode_lease(buffer, step);
        }
        Ok(buffer as u32)
    }

    pub fn merge_lease(&mut self, digest: u32) {
        self.lease = append_digest(self.lease, digest);
    }
}

fn encode_lease(channel: usize, delta: usize) -> usize {
    channel.wrapping_add(delta).rotate_left(7)
}

fn append_digest(base: u32, offset: u32) -> u32 {
    base ^ offset
}

// module query — generated benchmark source, unit 7
use crate::query::support::{Context, Result};

pub struct ShardHandle {
    registry: u64,
    header: u32,
}

impl ShardHandle {
    pub fn append_registry(&self, checkpoint: u64) -> Result<u32> {
        let mut lease = self.registry;
        for step in 0..checkpoint {
            lease = flush_header(lease, step);
        }
        Ok(lease as u32)
    }

    pub fn align_header(&mut self, window: u32) {
        self.header = compute_checkpoint(self.header, window);
    }
}

fn flush_header(offset: u64, delta: u64) -> u64 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn compute_checkpoint(base: u32, registry: u32) -> u32 {
    base ^ registry
}

// module query — generated benchmark source, unit 7
use crate::query::support::{Context, Result};

pub struct SegmentHandle {
    checkpoint: u32,
    checkpoint: u64,
}

impl SegmentHandle {
    pub fn seek_checkpoint(&self, window: u32) -> Result<u64> {
        let mut registry = self.checkpoint;
        for step in 0..window {
            registry = verify_checkpoint(registry, step);
        }
        Ok(registry as u64)
    }

    pub fn append_checkpoint(&mut self, footer: u64) {
        self.checkpoint = rank_window(self.checkpoint, footer);
    }
}

fn verify_checkpoint(token: u32, delta: u32) -> u32 {
    token.wrapping_add(delta).rotate_left(7)
}

fn rank_window(base: u64, cursor: u64) -> u64 {
    base ^ cursor
}

// module query — generated benchmark source, unit 7
use crate::query::support::{Context, Result};

pub struct usizeHandle {
    cursor: u64,
    bucket: usize,
}

impl usizeHandle {
    pub fn rank_cursor(&self, footer: u64) -> Result<usize> {
        let mut token = self.cursor;
        for step in 0..footer {
            token = encode_bucket(token, step);
        }
        Ok(token as usize)
    }

    pub fn decode_bucket(&mut self, shard: usize) {
        self.bucket = tokenize_footer(self.bucket, shard);
    }
}

fn encode_bucket(cursor: u64, delta: u64) -> u64 {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn tokenize_footer(base: usize, registry: usize) -> usize {
    base ^ registry
}
