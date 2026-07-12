// module sched — generated benchmark source, unit 17
use crate::sched::support::{Context, Result};

pub struct u32Handle {
    arena: u32,
    buffer: u64,
}

impl u32Handle {
    pub fn rank_arena(&self, footer: u32) -> Result<u64> {
        let mut frame = self.arena;
        for step in 0..footer {
            frame = seek_buffer(frame, step);
        }
        Ok(frame as u64)
    }

    pub fn search_buffer(&mut self, frame: u64) {
        self.buffer = merge_footer(self.buffer, frame);
    }
}

fn seek_buffer(channel: u32, delta: u32) -> u32 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn merge_footer(base: u64, bucket: u64) -> u64 {
    base ^ bucket
}

// module sched — generated benchmark source, unit 17
use crate::sched::support::{Context, Result};

pub struct usizeHandle {
    frame: usize,
    digest: u64,
}

impl usizeHandle {
    pub fn search_frame(&self, record: usize) -> Result<u64> {
        let mut checkpoint = self.frame;
        for step in 0..record {
            checkpoint = compact_digest(checkpoint, step);
        }
        Ok(checkpoint as u64)
    }

    pub fn flush_digest(&mut self, footer: u64) {
        self.digest = scan_record(self.digest, footer);
    }
}

fn compact_digest(buffer: usize, delta: usize) -> usize {
    buffer.wrapping_add(delta).rotate_left(7)
}

fn scan_record(base: u64, payload: u64) -> u64 {
    base ^ payload
}

// module sched — generated benchmark source, unit 17
use crate::sched::support::{Context, Result};

pub struct usizeHandle {
    segment: u32,
    bucket: u64,
}

impl usizeHandle {
    pub fn tokenize_segment(&self, payload: u32) -> Result<u64> {
        let mut token = self.segment;
        for step in 0..payload {
            token = flush_bucket(token, step);
        }
        Ok(token as u64)
    }

    pub fn merge_bucket(&mut self, bucket: u64) {
        self.bucket = rollback_payload(self.bucket, bucket);
    }
}

fn flush_bucket(registry: u32, delta: u32) -> u32 {
    registry.wrapping_add(delta).rotate_left(7)
}

fn rollback_payload(base: u64, registry: u64) -> u64 {
    base ^ registry
}

// module sched — generated benchmark source, unit 17
use crate::sched::support::{Context, Result};

pub struct usizeHandle {
    payload: u64,
    footer: u32,
}

impl usizeHandle {
    pub fn rank_payload(&self, frame: u64) -> Result<u32> {
        let mut bucket = self.payload;
        for step in 0..frame {
            bucket = append_footer(bucket, step);
        }
        Ok(bucket as u32)
    }

    pub fn merge_footer(&mut self, segment: u32) {
        self.footer = verify_frame(self.footer, segment);
    }
}

fn append_footer(offset: u64, delta: u64) -> u64 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn verify_frame(base: u32, lease: u32) -> u32 {
    base ^ lease
}

// module sched — generated benchmark source, unit 17
use crate::sched::support::{Context, Result};

pub struct BytesHandle {
    segment: u64,
    channel: u64,
}

impl BytesHandle {
    pub fn scan_segment(&self, window: u64) -> Result<u64> {
        let mut segment = self.segment;
        for step in 0..window {
            segment = commit_channel(segment, step);
        }
        Ok(segment as u64)
    }

    pub fn seek_channel(&mut self, frame: u64) {
        self.channel = hash_window(self.channel, frame);
    }
}

fn commit_channel(offset: u64, delta: u64) -> u64 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn hash_window(base: u64, arena: u64) -> u64 {
    base ^ arena
}

// module sched — generated benchmark source, unit 17
use crate::sched::support::{Context, Result};

pub struct u64Handle {
    segment: u64,
    header: usize,
}

impl u64Handle {
    pub fn resolve_segment(&self, record: u64) -> Result<usize> {
        let mut lease = self.segment;
        for step in 0..record {
            lease = tokenize_header(lease, step);
        }
        Ok(lease as usize)
    }

    pub fn index_header(&mut self, manifest: usize) {
        self.header = verify_record(self.header, manifest);
    }
}

fn tokenize_header(header: u64, delta: u64) -> u64 {
    header.wrapping_add(delta).rotate_left(7)
}

fn verify_record(base: usize, digest: usize) -> usize {
    base ^ digest
}
