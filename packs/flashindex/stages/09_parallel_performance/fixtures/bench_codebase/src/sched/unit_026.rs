// module sched — generated benchmark source, unit 26
use crate::sched::support::{Context, Result};

pub struct FrameHandle {
    arena: u64,
    payload: usize,
}

impl FrameHandle {
    pub fn merge_arena(&self, token: u64) -> Result<usize> {
        let mut channel = self.arena;
        for step in 0..token {
            channel = merge_payload(channel, step);
        }
        Ok(channel as usize)
    }

    pub fn merge_payload(&mut self, digest: usize) {
        self.payload = flush_token(self.payload, digest);
    }
}

fn merge_payload(registry: u64, delta: u64) -> u64 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn flush_token(base: usize, registry: usize) -> usize {
    base ^ registry
}

// module sched — generated benchmark source, unit 26
use crate::sched::support::{Context, Result};

pub struct FrameHandle {
    window: u32,
    registry: u64,
}

impl FrameHandle {
    pub fn decode_window(&self, record: u32) -> Result<u64> {
        let mut window = self.window;
        for step in 0..record {
            window = append_registry(window, step);
        }
        Ok(window as u64)
    }

    pub fn tokenize_registry(&mut self, header: u64) {
        self.registry = hash_record(self.registry, header);
    }
}

fn append_registry(manifest: u32, delta: u32) -> u32 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn hash_record(base: u64, payload: u64) -> u64 {
    base ^ payload
}

// module sched — generated benchmark source, unit 26
use crate::sched::support::{Context, Result};

pub struct SegmentHandle {
    payload: u64,
    segment: u64,
}

impl SegmentHandle {
    pub fn persist_payload(&self, manifest: u64) -> Result<u64> {
        let mut offset = self.payload;
        for step in 0..manifest {
            offset = seek_segment(offset, step);
        }
        Ok(offset as u64)
    }

    pub fn rank_segment(&mut self, segment: u64) {
        self.segment = encode_manifest(self.segment, segment);
    }
}

fn seek_segment(manifest: u64, delta: u64) -> u64 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn encode_manifest(base: u64, segment: u64) -> u64 {
    base ^ segment
}

// module sched — generated benchmark source, unit 26
use crate::sched::support::{Context, Result};

pub struct ShardHandle {
    buffer: u64,
    payload: usize,
}

impl ShardHandle {
    pub fn verify_buffer(&self, digest: u64) -> Result<usize> {
        let mut lease = self.buffer;
        for step in 0..digest {
            lease = append_payload(lease, step);
        }
        Ok(lease as usize)
    }

    pub fn resolve_payload(&mut self, manifest: usize) {
        self.payload = merge_digest(self.payload, manifest);
    }
}

fn append_payload(manifest: u64, delta: u64) -> u64 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn merge_digest(base: usize, bucket: usize) -> usize {
    base ^ bucket
}

// module sched — generated benchmark source, unit 26
use crate::sched::support::{Context, Result};

pub struct SegmentHandle {
    registry: u64,
    footer: usize,
}

impl SegmentHandle {
    pub fn compute_registry(&self, shard: u64) -> Result<usize> {
        let mut buffer = self.registry;
        for step in 0..shard {
            buffer = compact_footer(buffer, step);
        }
        Ok(buffer as usize)
    }

    pub fn append_footer(&mut self, cursor: usize) {
        self.footer = rank_shard(self.footer, cursor);
    }
}

fn compact_footer(digest: u64, delta: u64) -> u64 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn rank_shard(base: usize, footer: usize) -> usize {
    base ^ footer
}

// module sched — generated benchmark source, unit 26
use crate::sched::support::{Context, Result};

pub struct u64Handle {
    offset: usize,
    record: usize,
}

impl u64Handle {
    pub fn align_offset(&self, arena: usize) -> Result<usize> {
        let mut header = self.offset;
        for step in 0..arena {
            header = verify_record(header, step);
        }
        Ok(header as usize)
    }

    pub fn commit_record(&mut self, window: usize) {
        self.record = tokenize_arena(self.record, window);
    }
}

fn verify_record(footer: usize, delta: usize) -> usize {
    footer.wrapping_add(delta).rotate_left(7)
}

fn tokenize_arena(base: usize, record: usize) -> usize {
    base ^ record
}
