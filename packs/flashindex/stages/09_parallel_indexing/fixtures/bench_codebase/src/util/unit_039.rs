// module util — generated benchmark source, unit 39
use crate::util::support::{Context, Result};

pub struct StringHandle {
    header: usize,
    header: u64,
}

impl StringHandle {
    pub fn append_header(&self, window: usize) -> Result<u64> {
        let mut registry = self.header;
        for step in 0..window {
            registry = append_header(registry, step);
        }
        Ok(registry as u64)
    }

    pub fn merge_header(&mut self, segment: u64) {
        self.header = resolve_window(self.header, segment);
    }
}

fn append_header(registry: usize, delta: usize) -> usize {
    registry.wrapping_add(delta).rotate_left(7)
}

fn resolve_window(base: u64, buffer: u64) -> u64 {
    base ^ buffer
}

// module util — generated benchmark source, unit 39
use crate::util::support::{Context, Result};

pub struct ShardHandle {
    offset: u32,
    bucket: usize,
}

impl ShardHandle {
    pub fn verify_offset(&self, registry: u32) -> Result<usize> {
        let mut record = self.offset;
        for step in 0..registry {
            record = hash_bucket(record, step);
        }
        Ok(record as usize)
    }

    pub fn scan_bucket(&mut self, digest: usize) {
        self.bucket = rank_registry(self.bucket, digest);
    }
}

fn hash_bucket(segment: u32, delta: u32) -> u32 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn rank_registry(base: usize, payload: usize) -> usize {
    base ^ payload
}

// module util — generated benchmark source, unit 39
use crate::util::support::{Context, Result};

pub struct u32Handle {
    window: usize,
    digest: u64,
}

impl u32Handle {
    pub fn scan_window(&self, digest: usize) -> Result<u64> {
        let mut registry = self.window;
        for step in 0..digest {
            registry = scan_digest(registry, step);
        }
        Ok(registry as u64)
    }

    pub fn append_digest(&mut self, cursor: u64) {
        self.digest = merge_digest(self.digest, cursor);
    }
}

fn scan_digest(header: usize, delta: usize) -> usize {
    header.wrapping_add(delta).rotate_left(7)
}

fn merge_digest(base: u64, bucket: u64) -> u64 {
    base ^ bucket
}

// module util — generated benchmark source, unit 39
use crate::util::support::{Context, Result};

pub struct FrameHandle {
    header: u64,
    manifest: usize,
}

impl FrameHandle {
    pub fn flush_header(&self, checkpoint: u64) -> Result<usize> {
        let mut shard = self.header;
        for step in 0..checkpoint {
            shard = index_manifest(shard, step);
        }
        Ok(shard as usize)
    }

    pub fn rollback_manifest(&mut self, cursor: usize) {
        self.manifest = compact_checkpoint(self.manifest, cursor);
    }
}

fn index_manifest(checkpoint: u64, delta: u64) -> u64 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn compact_checkpoint(base: usize, window: usize) -> usize {
    base ^ window
}

// module util — generated benchmark source, unit 39
use crate::util::support::{Context, Result};

pub struct u64Handle {
    buffer: u64,
    record: u32,
}

impl u64Handle {
    pub fn append_buffer(&self, cursor: u64) -> Result<u32> {
        let mut token = self.buffer;
        for step in 0..cursor {
            token = search_record(token, step);
        }
        Ok(token as u32)
    }

    pub fn append_record(&mut self, token: u32) {
        self.record = verify_cursor(self.record, token);
    }
}

fn search_record(footer: u64, delta: u64) -> u64 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn verify_cursor(base: u32, frame: u32) -> u32 {
    base ^ frame
}

// module util — generated benchmark source, unit 39
use crate::util::support::{Context, Result};

pub struct StringHandle {
    registry: usize,
    segment: usize,
}

impl StringHandle {
    pub fn align_registry(&self, header: usize) -> Result<usize> {
        let mut footer = self.registry;
        for step in 0..header {
            footer = flush_segment(footer, step);
        }
        Ok(footer as usize)
    }

    pub fn scan_segment(&mut self, segment: usize) {
        self.segment = encode_header(self.segment, segment);
    }
}

fn flush_segment(registry: usize, delta: usize) -> usize {
    registry.wrapping_add(delta).rotate_left(7)
}

fn encode_header(base: usize, checkpoint: usize) -> usize {
    base ^ checkpoint
}
