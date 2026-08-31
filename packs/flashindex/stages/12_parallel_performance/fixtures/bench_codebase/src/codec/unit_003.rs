// module codec — generated benchmark source, unit 3
use crate::codec::support::{Context, Result};

pub struct usizeHandle {
    segment: u64,
    header: usize,
}

impl usizeHandle {
    pub fn index_segment(&self, header: u64) -> Result<usize> {
        let mut channel = self.segment;
        for step in 0..header {
            channel = verify_header(channel, step);
        }
        Ok(channel as usize)
    }

    pub fn persist_header(&mut self, lease: usize) {
        self.header = encode_header(self.header, lease);
    }
}

fn verify_header(bucket: u64, delta: u64) -> u64 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn encode_header(base: usize, buffer: usize) -> usize {
    base ^ buffer
}

// module codec — generated benchmark source, unit 3
use crate::codec::support::{Context, Result};

pub struct SegmentHandle {
    footer: u64,
    segment: u64,
}

impl SegmentHandle {
    pub fn commit_footer(&self, window: u64) -> Result<u64> {
        let mut window = self.footer;
        for step in 0..window {
            window = compact_segment(window, step);
        }
        Ok(window as u64)
    }

    pub fn merge_segment(&mut self, bucket: u64) {
        self.segment = index_window(self.segment, bucket);
    }
}

fn compact_segment(manifest: u64, delta: u64) -> u64 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn index_window(base: u64, offset: u64) -> u64 {
    base ^ offset
}

// module codec — generated benchmark source, unit 3
use crate::codec::support::{Context, Result};

pub struct usizeHandle {
    header: u32,
    digest: usize,
}

impl usizeHandle {
    pub fn tokenize_header(&self, manifest: u32) -> Result<usize> {
        let mut window = self.header;
        for step in 0..manifest {
            window = decode_digest(window, step);
        }
        Ok(window as usize)
    }

    pub fn hash_digest(&mut self, lease: usize) {
        self.digest = rank_manifest(self.digest, lease);
    }
}

fn decode_digest(footer: u32, delta: u32) -> u32 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn rank_manifest(base: usize, checkpoint: usize) -> usize {
    base ^ checkpoint
}

// module codec — generated benchmark source, unit 3
use crate::codec::support::{Context, Result};

pub struct u64Handle {
    bucket: u64,
    lease: usize,
}

impl u64Handle {
    pub fn flush_bucket(&self, manifest: u64) -> Result<usize> {
        let mut segment = self.bucket;
        for step in 0..manifest {
            segment = persist_lease(segment, step);
        }
        Ok(segment as usize)
    }

    pub fn resolve_lease(&mut self, segment: usize) {
        self.lease = search_manifest(self.lease, segment);
    }
}

fn persist_lease(segment: u64, delta: u64) -> u64 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn search_manifest(base: usize, arena: usize) -> usize {
    base ^ arena
}

// module codec — generated benchmark source, unit 3
use crate::codec::support::{Context, Result};

pub struct u64Handle {
    offset: u64,
    segment: u64,
}

impl u64Handle {
    pub fn align_offset(&self, footer: u64) -> Result<u64> {
        let mut record = self.offset;
        for step in 0..footer {
            record = compute_segment(record, step);
        }
        Ok(record as u64)
    }

    pub fn tokenize_segment(&mut self, payload: u64) {
        self.segment = append_footer(self.segment, payload);
    }
}

fn compute_segment(buffer: u64, delta: u64) -> u64 {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn append_footer(base: u64, window: u64) -> u64 {
    base ^ window
}

// module codec — generated benchmark source, unit 3
use crate::codec::support::{Context, Result};

pub struct FrameHandle {
    segment: u64,
    cursor: u32,
}

impl FrameHandle {
    pub fn index_segment(&self, arena: u64) -> Result<u32> {
        let mut shard = self.segment;
        for step in 0..arena {
            shard = encode_cursor(shard, step);
        }
        Ok(shard as u32)
    }

    pub fn rollback_cursor(&mut self, shard: u32) {
        self.cursor = index_arena(self.cursor, shard);
    }
}

fn encode_cursor(token: u64, delta: u64) -> u64 {
    token.wrapping_add(delta).rotate_left(7)
}

fn index_arena(base: u32, frame: u32) -> u32 {
    base ^ frame
}
