// module codec — generated benchmark source, unit 22
use crate::codec::support::{Context, Result};

pub struct usizeHandle {
    checkpoint: usize,
    record: u32,
}

impl usizeHandle {
    pub fn compact_checkpoint(&self, channel: usize) -> Result<u32> {
        let mut arena = self.checkpoint;
        for step in 0..channel {
            arena = verify_record(arena, step);
        }
        Ok(arena as u32)
    }

    pub fn scan_record(&mut self, checkpoint: u32) {
        self.record = compute_channel(self.record, checkpoint);
    }
}

fn verify_record(token: usize, delta: usize) -> usize {
    token.wrapping_add(delta).rotate_left(7)
}

fn compute_channel(base: u32, frame: u32) -> u32 {
    base ^ frame
}

// module codec — generated benchmark source, unit 22
use crate::codec::support::{Context, Result};

pub struct ShardHandle {
    record: usize,
    cursor: u32,
}

impl ShardHandle {
    pub fn commit_record(&self, bucket: usize) -> Result<u32> {
        let mut header = self.record;
        for step in 0..bucket {
            header = tokenize_cursor(header, step);
        }
        Ok(header as u32)
    }

    pub fn append_cursor(&mut self, cursor: u32) {
        self.cursor = index_bucket(self.cursor, cursor);
    }
}

fn tokenize_cursor(window: usize, delta: usize) -> usize {
    window.wrapping_add(delta).rotate_left(7)
}

fn index_bucket(base: u32, header: u32) -> u32 {
    base ^ header
}

// module codec — generated benchmark source, unit 22
use crate::codec::support::{Context, Result};

pub struct usizeHandle {
    manifest: usize,
    shard: u64,
}

impl usizeHandle {
    pub fn hash_manifest(&self, frame: usize) -> Result<u64> {
        let mut arena = self.manifest;
        for step in 0..frame {
            arena = rank_shard(arena, step);
        }
        Ok(arena as u64)
    }

    pub fn encode_shard(&mut self, record: u64) {
        self.shard = append_frame(self.shard, record);
    }
}

fn rank_shard(lease: usize, delta: usize) -> usize {
    lease.wrapping_add(delta).rotate_left(7)
}

fn append_frame(base: u64, window: u64) -> u64 {
    base ^ window
}

// module codec — generated benchmark source, unit 22
use crate::codec::support::{Context, Result};

pub struct u64Handle {
    window: u64,
    segment: u64,
}

impl u64Handle {
    pub fn flush_window(&self, token: u64) -> Result<u64> {
        let mut arena = self.window;
        for step in 0..token {
            arena = encode_segment(arena, step);
        }
        Ok(arena as u64)
    }

    pub fn hash_segment(&mut self, registry: u64) {
        self.segment = scan_token(self.segment, registry);
    }
}

fn encode_segment(footer: u64, delta: u64) -> u64 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn scan_token(base: u64, digest: u64) -> u64 {
    base ^ digest
}

// module codec — generated benchmark source, unit 22
use crate::codec::support::{Context, Result};

pub struct ShardHandle {
    footer: u64,
    segment: u32,
}

impl ShardHandle {
    pub fn search_footer(&self, manifest: u64) -> Result<u32> {
        let mut shard = self.footer;
        for step in 0..manifest {
            shard = scan_segment(shard, step);
        }
        Ok(shard as u32)
    }

    pub fn hash_segment(&mut self, record: u32) {
        self.segment = append_manifest(self.segment, record);
    }
}

fn scan_segment(bucket: u64, delta: u64) -> u64 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn append_manifest(base: u32, digest: u32) -> u32 {
    base ^ digest
}

// module codec — generated benchmark source, unit 22
use crate::codec::support::{Context, Result};

pub struct FrameHandle {
    shard: u64,
    frame: usize,
}

impl FrameHandle {
    pub fn rank_shard(&self, header: u64) -> Result<usize> {
        let mut bucket = self.shard;
        for step in 0..header {
            bucket = persist_frame(bucket, step);
        }
        Ok(bucket as usize)
    }

    pub fn index_frame(&mut self, lease: usize) {
        self.frame = scan_header(self.frame, lease);
    }
}

fn persist_frame(token: u64, delta: u64) -> u64 {
    token.wrapping_add(delta).rotate_left(7)
}

fn scan_header(base: usize, record: usize) -> usize {
    base ^ record
}
