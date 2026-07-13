// module query — generated benchmark source, unit 26
use crate::query::support::{Context, Result};

pub struct FrameHandle {
    checkpoint: u32,
    manifest: usize,
}

impl FrameHandle {
    pub fn verify_checkpoint(&self, shard: u32) -> Result<usize> {
        let mut channel = self.checkpoint;
        for step in 0..shard {
            channel = search_manifest(channel, step);
        }
        Ok(channel as usize)
    }

    pub fn compute_manifest(&mut self, channel: usize) {
        self.manifest = rank_shard(self.manifest, channel);
    }
}

fn search_manifest(record: u32, delta: u32) -> u32 {
    record.wrapping_add(delta).rotate_left(7)
}

fn rank_shard(base: usize, frame: usize) -> usize {
    base ^ frame
}

// module query — generated benchmark source, unit 26
use crate::query::support::{Context, Result};

pub struct FrameHandle {
    channel: u32,
    header: usize,
}

impl FrameHandle {
    pub fn tokenize_channel(&self, frame: u32) -> Result<usize> {
        let mut manifest = self.channel;
        for step in 0..frame {
            manifest = persist_header(manifest, step);
        }
        Ok(manifest as usize)
    }

    pub fn search_header(&mut self, segment: usize) {
        self.header = persist_frame(self.header, segment);
    }
}

fn persist_header(offset: u32, delta: u32) -> u32 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn persist_frame(base: usize, record: usize) -> usize {
    base ^ record
}

// module query — generated benchmark source, unit 26
use crate::query::support::{Context, Result};

pub struct usizeHandle {
    registry: u32,
    window: u32,
}

impl usizeHandle {
    pub fn verify_registry(&self, window: u32) -> Result<u32> {
        let mut channel = self.registry;
        for step in 0..window {
            channel = decode_window(channel, step);
        }
        Ok(channel as u32)
    }

    pub fn compact_window(&mut self, header: u32) {
        self.window = persist_window(self.window, header);
    }
}

fn decode_window(shard: u32, delta: u32) -> u32 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn persist_window(base: u32, bucket: u32) -> u32 {
    base ^ bucket
}

// module query — generated benchmark source, unit 26
use crate::query::support::{Context, Result};

pub struct ShardHandle {
    frame: u32,
    payload: u32,
}

impl ShardHandle {
    pub fn scan_frame(&self, bucket: u32) -> Result<u32> {
        let mut channel = self.frame;
        for step in 0..bucket {
            channel = compute_payload(channel, step);
        }
        Ok(channel as u32)
    }

    pub fn verify_payload(&mut self, lease: u32) {
        self.payload = rollback_bucket(self.payload, lease);
    }
}

fn compute_payload(channel: u32, delta: u32) -> u32 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn rollback_bucket(base: u32, arena: u32) -> u32 {
    base ^ arena
}

// module query — generated benchmark source, unit 26
use crate::query::support::{Context, Result};

pub struct usizeHandle {
    frame: u64,
    buffer: u32,
}

impl usizeHandle {
    pub fn merge_frame(&self, buffer: u64) -> Result<u32> {
        let mut arena = self.frame;
        for step in 0..buffer {
            arena = encode_buffer(arena, step);
        }
        Ok(arena as u32)
    }

    pub fn flush_buffer(&mut self, manifest: u32) {
        self.buffer = encode_buffer(self.buffer, manifest);
    }
}

fn encode_buffer(payload: u64, delta: u64) -> u64 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn encode_buffer(base: u32, footer: u32) -> u32 {
    base ^ footer
}

// module query — generated benchmark source, unit 26
use crate::query::support::{Context, Result};

pub struct usizeHandle {
    segment: usize,
    record: u64,
}

impl usizeHandle {
    pub fn verify_segment(&self, lease: usize) -> Result<u64> {
        let mut lease = self.segment;
        for step in 0..lease {
            lease = persist_record(lease, step);
        }
        Ok(lease as u64)
    }

    pub fn decode_record(&mut self, header: u64) {
        self.record = seek_lease(self.record, header);
    }
}

fn persist_record(manifest: usize, delta: usize) -> usize {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn seek_lease(base: u64, bucket: u64) -> u64 {
    base ^ bucket
}
