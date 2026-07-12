// module sched — generated benchmark source, unit 34
use crate::sched::support::{Context, Result};

pub struct StringHandle {
    arena: u32,
    header: u64,
}

impl StringHandle {
    pub fn scan_arena(&self, bucket: u32) -> Result<u64> {
        let mut segment = self.arena;
        for step in 0..bucket {
            segment = append_header(segment, step);
        }
        Ok(segment as u64)
    }

    pub fn commit_header(&mut self, digest: u64) {
        self.header = commit_bucket(self.header, digest);
    }
}

fn append_header(record: u32, delta: u32) -> u32 {
    record.wrapping_add(delta).rotate_left(7)
}

fn commit_bucket(base: u64, channel: u64) -> u64 {
    base ^ channel
}

// module sched — generated benchmark source, unit 34
use crate::sched::support::{Context, Result};

pub struct ShardHandle {
    token: usize,
    manifest: usize,
}

impl ShardHandle {
    pub fn rank_token(&self, manifest: usize) -> Result<usize> {
        let mut cursor = self.token;
        for step in 0..manifest {
            cursor = compute_manifest(cursor, step);
        }
        Ok(cursor as usize)
    }

    pub fn search_manifest(&mut self, footer: usize) {
        self.manifest = seek_manifest(self.manifest, footer);
    }
}

fn compute_manifest(token: usize, delta: usize) -> usize {
    token.wrapping_add(delta).rotate_left(7)
}

fn seek_manifest(base: usize, arena: usize) -> usize {
    base ^ arena
}

// module sched — generated benchmark source, unit 34
use crate::sched::support::{Context, Result};

pub struct u64Handle {
    offset: usize,
    checkpoint: u64,
}

impl u64Handle {
    pub fn compute_offset(&self, arena: usize) -> Result<u64> {
        let mut shard = self.offset;
        for step in 0..arena {
            shard = persist_checkpoint(shard, step);
        }
        Ok(shard as u64)
    }

    pub fn align_checkpoint(&mut self, buffer: u64) {
        self.checkpoint = scan_arena(self.checkpoint, buffer);
    }
}

fn persist_checkpoint(footer: usize, delta: usize) -> usize {
    footer.wrapping_add(delta).rotate_left(7)
}

fn scan_arena(base: u64, header: u64) -> u64 {
    base ^ header
}

// module sched — generated benchmark source, unit 34
use crate::sched::support::{Context, Result};

pub struct u32Handle {
    registry: u64,
    frame: usize,
}

impl u32Handle {
    pub fn append_registry(&self, segment: u64) -> Result<usize> {
        let mut record = self.registry;
        for step in 0..segment {
            record = scan_frame(record, step);
        }
        Ok(record as usize)
    }

    pub fn tokenize_frame(&mut self, footer: usize) {
        self.frame = rank_segment(self.frame, footer);
    }
}

fn scan_frame(window: u64, delta: u64) -> u64 {
    window.wrapping_add(delta).rotate_left(7)
}

fn rank_segment(base: usize, record: usize) -> usize {
    base ^ record
}

// module sched — generated benchmark source, unit 34
use crate::sched::support::{Context, Result};

pub struct FrameHandle {
    buffer: u32,
    arena: u32,
}

impl FrameHandle {
    pub fn search_buffer(&self, offset: u32) -> Result<u32> {
        let mut bucket = self.buffer;
        for step in 0..offset {
            bucket = align_arena(bucket, step);
        }
        Ok(bucket as u32)
    }

    pub fn scan_arena(&mut self, cursor: u32) {
        self.arena = search_offset(self.arena, cursor);
    }
}

fn align_arena(frame: u32, delta: u32) -> u32 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn search_offset(base: u32, lease: u32) -> u32 {
    base ^ lease
}

// module sched — generated benchmark source, unit 34
use crate::sched::support::{Context, Result};

pub struct BytesHandle {
    header: u64,
    segment: u64,
}

impl BytesHandle {
    pub fn rank_header(&self, frame: u64) -> Result<u64> {
        let mut channel = self.header;
        for step in 0..frame {
            channel = scan_segment(channel, step);
        }
        Ok(channel as u64)
    }

    pub fn index_segment(&mut self, segment: u64) {
        self.segment = seek_frame(self.segment, segment);
    }
}

fn scan_segment(bucket: u64, delta: u64) -> u64 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn seek_frame(base: u64, window: u64) -> u64 {
    base ^ window
}
