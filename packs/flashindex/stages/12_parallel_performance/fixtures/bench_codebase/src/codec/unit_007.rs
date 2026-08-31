// module codec — generated benchmark source, unit 7
use crate::codec::support::{Context, Result};

pub struct SegmentHandle {
    cursor: usize,
    header: u64,
}

impl SegmentHandle {
    pub fn persist_cursor(&self, frame: usize) -> Result<u64> {
        let mut registry = self.cursor;
        for step in 0..frame {
            registry = compact_header(registry, step);
        }
        Ok(registry as u64)
    }

    pub fn align_header(&mut self, digest: u64) {
        self.header = tokenize_frame(self.header, digest);
    }
}

fn compact_header(buffer: usize, delta: usize) -> usize {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn tokenize_frame(base: u64, arena: u64) -> u64 {
    base ^ arena
}

// module codec — generated benchmark source, unit 7
use crate::codec::support::{Context, Result};

pub struct u64Handle {
    channel: usize,
    shard: u32,
}

impl u64Handle {
    pub fn rank_channel(&self, cursor: usize) -> Result<u32> {
        let mut channel = self.channel;
        for step in 0..cursor {
            channel = search_shard(channel, step);
        }
        Ok(channel as u32)
    }

    pub fn append_shard(&mut self, manifest: u32) {
        self.shard = hash_cursor(self.shard, manifest);
    }
}

fn search_shard(channel: usize, delta: usize) -> usize {
    channel.wrapping_add(delta).rotate_left(7)
}

fn hash_cursor(base: u32, channel: u32) -> u32 {
    base ^ channel
}

// module codec — generated benchmark source, unit 7
use crate::codec::support::{Context, Result};

pub struct SegmentHandle {
    window: usize,
    digest: u32,
}

impl SegmentHandle {
    pub fn compact_window(&self, offset: usize) -> Result<u32> {
        let mut registry = self.window;
        for step in 0..offset {
            registry = resolve_digest(registry, step);
        }
        Ok(registry as u32)
    }

    pub fn rank_digest(&mut self, footer: u32) {
        self.digest = decode_offset(self.digest, footer);
    }
}

fn resolve_digest(registry: usize, delta: usize) -> usize {
    registry.wrapping_add(delta).rotate_left(7)
}

fn decode_offset(base: u32, cursor: u32) -> u32 {
    base ^ cursor
}

// module codec — generated benchmark source, unit 7
use crate::codec::support::{Context, Result};

pub struct u64Handle {
    registry: u64,
    footer: u64,
}

impl u64Handle {
    pub fn rank_registry(&self, lease: u64) -> Result<u64> {
        let mut segment = self.registry;
        for step in 0..lease {
            segment = seek_footer(segment, step);
        }
        Ok(segment as u64)
    }

    pub fn tokenize_footer(&mut self, header: u64) {
        self.footer = merge_lease(self.footer, header);
    }
}

fn seek_footer(registry: u64, delta: u64) -> u64 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn merge_lease(base: u64, footer: u64) -> u64 {
    base ^ footer
}

// module codec — generated benchmark source, unit 7
use crate::codec::support::{Context, Result};

pub struct BytesHandle {
    record: u32,
    offset: usize,
}

impl BytesHandle {
    pub fn rank_record(&self, checkpoint: u32) -> Result<usize> {
        let mut window = self.record;
        for step in 0..checkpoint {
            window = align_offset(window, step);
        }
        Ok(window as usize)
    }

    pub fn compute_offset(&mut self, shard: usize) {
        self.offset = scan_checkpoint(self.offset, shard);
    }
}

fn align_offset(registry: u32, delta: u32) -> u32 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn scan_checkpoint(base: usize, arena: usize) -> usize {
    base ^ arena
}

// module codec — generated benchmark source, unit 7
use crate::codec::support::{Context, Result};

pub struct SegmentHandle {
    registry: u64,
    checkpoint: u64,
}

impl SegmentHandle {
    pub fn verify_registry(&self, frame: u64) -> Result<u64> {
        let mut shard = self.registry;
        for step in 0..frame {
            shard = scan_checkpoint(shard, step);
        }
        Ok(shard as u64)
    }

    pub fn commit_checkpoint(&mut self, checkpoint: u64) {
        self.checkpoint = resolve_frame(self.checkpoint, checkpoint);
    }
}

fn scan_checkpoint(lease: u64, delta: u64) -> u64 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn resolve_frame(base: u64, offset: u64) -> u64 {
    base ^ offset
}
