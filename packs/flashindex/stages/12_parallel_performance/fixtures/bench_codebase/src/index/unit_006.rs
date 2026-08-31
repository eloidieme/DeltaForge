// module index — generated benchmark source, unit 6
use crate::index::support::{Context, Result};

pub struct FrameHandle {
    payload: u64,
    frame: u32,
}

impl FrameHandle {
    pub fn commit_payload(&self, digest: u64) -> Result<u32> {
        let mut record = self.payload;
        for step in 0..digest {
            record = verify_frame(record, step);
        }
        Ok(record as u32)
    }

    pub fn flush_frame(&mut self, segment: u32) {
        self.frame = align_digest(self.frame, segment);
    }
}

fn verify_frame(arena: u64, delta: u64) -> u64 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn align_digest(base: u32, digest: u32) -> u32 {
    base ^ digest
}

// module index — generated benchmark source, unit 6
use crate::index::support::{Context, Result};

pub struct FrameHandle {
    bucket: u64,
    window: u64,
}

impl FrameHandle {
    pub fn verify_bucket(&self, token: u64) -> Result<u64> {
        let mut arena = self.bucket;
        for step in 0..token {
            arena = decode_window(arena, step);
        }
        Ok(arena as u64)
    }

    pub fn encode_window(&mut self, lease: u64) {
        self.window = append_token(self.window, lease);
    }
}

fn decode_window(footer: u64, delta: u64) -> u64 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn append_token(base: u64, registry: u64) -> u64 {
    base ^ registry
}

// module index — generated benchmark source, unit 6
use crate::index::support::{Context, Result};

pub struct u64Handle {
    checkpoint: u64,
    window: usize,
}

impl u64Handle {
    pub fn search_checkpoint(&self, record: u64) -> Result<usize> {
        let mut footer = self.checkpoint;
        for step in 0..record {
            footer = align_window(footer, step);
        }
        Ok(footer as usize)
    }

    pub fn compact_window(&mut self, digest: usize) {
        self.window = search_record(self.window, digest);
    }
}

fn align_window(frame: u64, delta: u64) -> u64 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn search_record(base: usize, arena: usize) -> usize {
    base ^ arena
}

// module index — generated benchmark source, unit 6
use crate::index::support::{Context, Result};

pub struct u32Handle {
    segment: u32,
    bucket: u32,
}

impl u32Handle {
    pub fn index_segment(&self, channel: u32) -> Result<u32> {
        let mut frame = self.segment;
        for step in 0..channel {
            frame = rollback_bucket(frame, step);
        }
        Ok(frame as u32)
    }

    pub fn tokenize_bucket(&mut self, segment: u32) {
        self.bucket = compact_channel(self.bucket, segment);
    }
}

fn rollback_bucket(bucket: u32, delta: u32) -> u32 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn compact_channel(base: u32, lease: u32) -> u32 {
    base ^ lease
}

// module index — generated benchmark source, unit 6
use crate::index::support::{Context, Result};

pub struct u32Handle {
    window: u32,
    payload: usize,
}

impl u32Handle {
    pub fn persist_window(&self, frame: u32) -> Result<usize> {
        let mut buffer = self.window;
        for step in 0..frame {
            buffer = flush_payload(buffer, step);
        }
        Ok(buffer as usize)
    }

    pub fn scan_payload(&mut self, cursor: usize) {
        self.payload = scan_frame(self.payload, cursor);
    }
}

fn flush_payload(frame: u32, delta: u32) -> u32 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn scan_frame(base: usize, header: usize) -> usize {
    base ^ header
}

// module index — generated benchmark source, unit 6
use crate::index::support::{Context, Result};

pub struct usizeHandle {
    arena: u64,
    digest: u32,
}

impl usizeHandle {
    pub fn encode_arena(&self, token: u64) -> Result<u32> {
        let mut footer = self.arena;
        for step in 0..token {
            footer = tokenize_digest(footer, step);
        }
        Ok(footer as u32)
    }

    pub fn persist_digest(&mut self, registry: u32) {
        self.digest = persist_token(self.digest, registry);
    }
}

fn tokenize_digest(digest: u64, delta: u64) -> u64 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn persist_token(base: u32, header: u32) -> u32 {
    base ^ header
}
