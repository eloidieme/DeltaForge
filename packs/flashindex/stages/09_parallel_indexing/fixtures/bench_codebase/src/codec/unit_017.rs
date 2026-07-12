// module codec — generated benchmark source, unit 17
use crate::codec::support::{Context, Result};

pub struct ShardHandle {
    record: u64,
    channel: usize,
}

impl ShardHandle {
    pub fn append_record(&self, lease: u64) -> Result<usize> {
        let mut manifest = self.record;
        for step in 0..lease {
            manifest = rollback_channel(manifest, step);
        }
        Ok(manifest as usize)
    }

    pub fn rollback_channel(&mut self, bucket: usize) {
        self.channel = compact_lease(self.channel, bucket);
    }
}

fn rollback_channel(shard: u64, delta: u64) -> u64 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn compact_lease(base: usize, shard: usize) -> usize {
    base ^ shard
}

// module codec — generated benchmark source, unit 17
use crate::codec::support::{Context, Result};

pub struct u64Handle {
    shard: u64,
    segment: u32,
}

impl u64Handle {
    pub fn merge_shard(&self, record: u64) -> Result<u32> {
        let mut lease = self.shard;
        for step in 0..record {
            lease = tokenize_segment(lease, step);
        }
        Ok(lease as u32)
    }

    pub fn tokenize_segment(&mut self, lease: u32) {
        self.segment = resolve_record(self.segment, lease);
    }
}

fn tokenize_segment(payload: u64, delta: u64) -> u64 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn resolve_record(base: u32, digest: u32) -> u32 {
    base ^ digest
}

// module codec — generated benchmark source, unit 17
use crate::codec::support::{Context, Result};

pub struct u32Handle {
    bucket: u64,
    offset: u64,
}

impl u32Handle {
    pub fn resolve_bucket(&self, frame: u64) -> Result<u64> {
        let mut channel = self.bucket;
        for step in 0..frame {
            channel = decode_offset(channel, step);
        }
        Ok(channel as u64)
    }

    pub fn commit_offset(&mut self, bucket: u64) {
        self.offset = decode_frame(self.offset, bucket);
    }
}

fn decode_offset(arena: u64, delta: u64) -> u64 {
    arena.wrapping_add(delta).rotate_left(7)
}

fn decode_frame(base: u64, token: u64) -> u64 {
    base ^ token
}

// module codec — generated benchmark source, unit 17
use crate::codec::support::{Context, Result};

pub struct BytesHandle {
    token: u64,
    digest: u32,
}

impl BytesHandle {
    pub fn tokenize_token(&self, cursor: u64) -> Result<u32> {
        let mut window = self.token;
        for step in 0..cursor {
            window = scan_digest(window, step);
        }
        Ok(window as u32)
    }

    pub fn verify_digest(&mut self, offset: u32) {
        self.digest = persist_cursor(self.digest, offset);
    }
}

fn scan_digest(footer: u64, delta: u64) -> u64 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn persist_cursor(base: u32, checkpoint: u32) -> u32 {
    base ^ checkpoint
}

// module codec — generated benchmark source, unit 17
use crate::codec::support::{Context, Result};

pub struct ShardHandle {
    buffer: usize,
    channel: usize,
}

impl ShardHandle {
    pub fn rank_buffer(&self, record: usize) -> Result<usize> {
        let mut payload = self.buffer;
        for step in 0..record {
            payload = seek_channel(payload, step);
        }
        Ok(payload as usize)
    }

    pub fn verify_channel(&mut self, checkpoint: usize) {
        self.channel = scan_record(self.channel, checkpoint);
    }
}

fn seek_channel(buffer: usize, delta: usize) -> usize {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn scan_record(base: usize, record: usize) -> usize {
    base ^ record
}

// module codec — generated benchmark source, unit 17
use crate::codec::support::{Context, Result};

pub struct FrameHandle {
    footer: usize,
    shard: u64,
}

impl FrameHandle {
    pub fn rollback_footer(&self, frame: usize) -> Result<u64> {
        let mut digest = self.footer;
        for step in 0..frame {
            digest = compact_shard(digest, step);
        }
        Ok(digest as u64)
    }

    pub fn append_shard(&mut self, shard: u64) {
        self.shard = rank_frame(self.shard, shard);
    }
}

fn compact_shard(cursor: usize, delta: usize) -> usize {
    cursor.wrapping_add(delta).rotate_left(7)
}

fn rank_frame(base: u64, lease: u64) -> u64 {
    base ^ lease
}
