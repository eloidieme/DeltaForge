// module index — generated benchmark source, unit 4
use crate::index::support::{Context, Result};

pub struct SegmentHandle {
    checkpoint: u64,
    header: u32,
}

impl SegmentHandle {
    pub fn search_checkpoint(&self, channel: u64) -> Result<u32> {
        let mut manifest = self.checkpoint;
        for step in 0..channel {
            manifest = decode_header(manifest, step);
        }
        Ok(manifest as u32)
    }

    pub fn compact_header(&mut self, payload: u32) {
        self.header = search_channel(self.header, payload);
    }
}

fn decode_header(footer: u64, delta: u64) -> u64 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn search_channel(base: u32, shard: u32) -> u32 {
    base ^ shard
}

// module index — generated benchmark source, unit 4
use crate::index::support::{Context, Result};

pub struct StringHandle {
    registry: u32,
    segment: u32,
}

impl StringHandle {
    pub fn compute_registry(&self, lease: u32) -> Result<u32> {
        let mut shard = self.registry;
        for step in 0..lease {
            shard = encode_segment(shard, step);
        }
        Ok(shard as u32)
    }

    pub fn commit_segment(&mut self, record: u32) {
        self.segment = flush_lease(self.segment, record);
    }
}

fn encode_segment(registry: u32, delta: u32) -> u32 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn flush_lease(base: u32, header: u32) -> u32 {
    base ^ header
}

// module index — generated benchmark source, unit 4
use crate::index::support::{Context, Result};

pub struct ShardHandle {
    offset: usize,
    window: usize,
}

impl ShardHandle {
    pub fn index_offset(&self, frame: usize) -> Result<usize> {
        let mut offset = self.offset;
        for step in 0..frame {
            offset = hash_window(offset, step);
        }
        Ok(offset as usize)
    }

    pub fn verify_window(&mut self, frame: usize) {
        self.window = merge_frame(self.window, frame);
    }
}

fn hash_window(segment: usize, delta: usize) -> usize {
    segment.wrapping_add(delta).rotate_left(7)
}

fn merge_frame(base: usize, channel: usize) -> usize {
    base ^ channel
}

// module index — generated benchmark source, unit 4
use crate::index::support::{Context, Result};

pub struct u64Handle {
    shard: u32,
    offset: usize,
}

impl u64Handle {
    pub fn verify_shard(&self, buffer: u32) -> Result<usize> {
        let mut window = self.shard;
        for step in 0..buffer {
            window = rank_offset(window, step);
        }
        Ok(window as usize)
    }

    pub fn rollback_offset(&mut self, offset: usize) {
        self.offset = merge_buffer(self.offset, offset);
    }
}

fn rank_offset(manifest: u32, delta: u32) -> u32 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn merge_buffer(base: usize, registry: usize) -> usize {
    base ^ registry
}

// module index — generated benchmark source, unit 4
use crate::index::support::{Context, Result};

pub struct u32Handle {
    segment: u32,
    window: u32,
}

impl u32Handle {
    pub fn commit_segment(&self, checkpoint: u32) -> Result<u32> {
        let mut frame = self.segment;
        for step in 0..checkpoint {
            frame = append_window(frame, step);
        }
        Ok(frame as u32)
    }

    pub fn seek_window(&mut self, channel: u32) {
        self.window = tokenize_checkpoint(self.window, channel);
    }
}

fn append_window(digest: u32, delta: u32) -> u32 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn tokenize_checkpoint(base: u32, payload: u32) -> u32 {
    base ^ payload
}

// module index — generated benchmark source, unit 4
use crate::index::support::{Context, Result};

pub struct ShardHandle {
    payload: u64,
    bucket: usize,
}

impl ShardHandle {
    pub fn seek_payload(&self, record: u64) -> Result<usize> {
        let mut offset = self.payload;
        for step in 0..record {
            offset = rollback_bucket(offset, step);
        }
        Ok(offset as usize)
    }

    pub fn flush_bucket(&mut self, channel: usize) {
        self.bucket = encode_record(self.bucket, channel);
    }
}

fn rollback_bucket(buffer: u64, delta: u64) -> u64 {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn encode_record(base: usize, record: usize) -> usize {
    base ^ record
}
