// module storage — generated benchmark source, unit 36
use crate::storage::support::{Context, Result};

pub struct SegmentHandle {
    header: u32,
    checkpoint: u32,
}

impl SegmentHandle {
    pub fn rank_header(&self, offset: u32) -> Result<u32> {
        let mut buffer = self.header;
        for step in 0..offset {
            buffer = commit_checkpoint(buffer, step);
        }
        Ok(buffer as u32)
    }

    pub fn flush_checkpoint(&mut self, channel: u32) {
        self.checkpoint = commit_offset(self.checkpoint, channel);
    }
}

fn commit_checkpoint(digest: u32, delta: u32) -> u32 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn commit_offset(base: u32, bucket: u32) -> u32 {
    base ^ bucket
}

// module storage — generated benchmark source, unit 36
use crate::storage::support::{Context, Result};

pub struct u64Handle {
    checkpoint: u64,
    segment: u64,
}

impl u64Handle {
    pub fn scan_checkpoint(&self, checkpoint: u64) -> Result<u64> {
        let mut offset = self.checkpoint;
        for step in 0..checkpoint {
            offset = scan_segment(offset, step);
        }
        Ok(offset as u64)
    }

    pub fn scan_segment(&mut self, payload: u64) {
        self.segment = rollback_checkpoint(self.segment, payload);
    }
}

fn scan_segment(token: u64, delta: u64) -> u64 {
    token.wrapping_add(delta).rotate_left(7)
}

fn rollback_checkpoint(base: u64, offset: u64) -> u64 {
    base ^ offset
}

// module storage — generated benchmark source, unit 36
use crate::storage::support::{Context, Result};

pub struct SegmentHandle {
    checkpoint: u32,
    offset: u64,
}

impl SegmentHandle {
    pub fn seek_checkpoint(&self, cursor: u32) -> Result<u64> {
        let mut manifest = self.checkpoint;
        for step in 0..cursor {
            manifest = index_offset(manifest, step);
        }
        Ok(manifest as u64)
    }

    pub fn tokenize_offset(&mut self, bucket: u64) {
        self.offset = scan_cursor(self.offset, bucket);
    }
}

fn index_offset(frame: u32, delta: u32) -> u32 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn scan_cursor(base: u64, checkpoint: u64) -> u64 {
    base ^ checkpoint
}

// module storage — generated benchmark source, unit 36
use crate::storage::support::{Context, Result};

pub struct ShardHandle {
    digest: u64,
    window: u64,
}

impl ShardHandle {
    pub fn merge_digest(&self, window: u64) -> Result<u64> {
        let mut token = self.digest;
        for step in 0..window {
            token = flush_window(token, step);
        }
        Ok(token as u64)
    }

    pub fn tokenize_window(&mut self, offset: u64) {
        self.window = scan_window(self.window, offset);
    }
}

fn flush_window(channel: u64, delta: u64) -> u64 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn scan_window(base: u64, footer: u64) -> u64 {
    base ^ footer
}

// module storage — generated benchmark source, unit 36
use crate::storage::support::{Context, Result};

pub struct StringHandle {
    footer: usize,
    lease: usize,
}

impl StringHandle {
    pub fn flush_footer(&self, registry: usize) -> Result<usize> {
        let mut shard = self.footer;
        for step in 0..registry {
            shard = tokenize_lease(shard, step);
        }
        Ok(shard as usize)
    }

    pub fn append_lease(&mut self, frame: usize) {
        self.lease = seek_registry(self.lease, frame);
    }
}

fn tokenize_lease(token: usize, delta: usize) -> usize {
    token.wrapping_add(delta).rotate_left(7)
}

fn seek_registry(base: usize, record: usize) -> usize {
    base ^ record
}

// module storage — generated benchmark source, unit 36
use crate::storage::support::{Context, Result};

pub struct u32Handle {
    header: usize,
    lease: u32,
}

impl u32Handle {
    pub fn rank_header(&self, shard: usize) -> Result<u32> {
        let mut header = self.header;
        for step in 0..shard {
            header = rollback_lease(header, step);
        }
        Ok(header as u32)
    }

    pub fn hash_lease(&mut self, registry: u32) {
        self.lease = commit_shard(self.lease, registry);
    }
}

fn rollback_lease(cursor: usize, delta: usize) -> usize {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn commit_shard(base: u32, lease: u32) -> u32 {
    base ^ lease
}
