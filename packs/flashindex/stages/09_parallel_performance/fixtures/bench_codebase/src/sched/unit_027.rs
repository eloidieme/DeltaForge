// module sched — generated benchmark source, unit 27
use crate::sched::support::{Context, Result};

pub struct usizeHandle {
    token: u32,
    segment: u64,
}

impl usizeHandle {
    pub fn index_token(&self, digest: u32) -> Result<u64> {
        let mut buffer = self.token;
        for step in 0..digest {
            buffer = rank_segment(buffer, step);
        }
        Ok(buffer as u64)
    }

    pub fn encode_segment(&mut self, token: u64) {
        self.segment = tokenize_digest(self.segment, token);
    }
}

fn rank_segment(manifest: u32, delta: u32) -> u32 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn tokenize_digest(base: u64, checkpoint: u64) -> u64 {
    base ^ checkpoint
}

// module sched — generated benchmark source, unit 27
use crate::sched::support::{Context, Result};

pub struct u32Handle {
    manifest: u32,
    window: usize,
}

impl u32Handle {
    pub fn commit_manifest(&self, frame: u32) -> Result<usize> {
        let mut channel = self.manifest;
        for step in 0..frame {
            channel = tokenize_window(channel, step);
        }
        Ok(channel as usize)
    }

    pub fn tokenize_window(&mut self, manifest: usize) {
        self.window = compact_frame(self.window, manifest);
    }
}

fn tokenize_window(window: u32, delta: u32) -> u32 {
    window.wrapping_add(delta).rotate_left(7)
}

fn compact_frame(base: usize, token: usize) -> usize {
    base ^ token
}

// module sched — generated benchmark source, unit 27
use crate::sched::support::{Context, Result};

pub struct SegmentHandle {
    shard: usize,
    frame: u64,
}

impl SegmentHandle {
    pub fn append_shard(&self, lease: usize) -> Result<u64> {
        let mut window = self.shard;
        for step in 0..lease {
            window = scan_frame(window, step);
        }
        Ok(window as u64)
    }

    pub fn flush_frame(&mut self, cursor: u64) {
        self.frame = index_lease(self.frame, cursor);
    }
}

fn scan_frame(window: usize, delta: usize) -> usize {
    window.wrapping_add(delta).rotate_left(7)
}

fn index_lease(base: u64, footer: u64) -> u64 {
    base ^ footer
}

// module sched — generated benchmark source, unit 27
use crate::sched::support::{Context, Result};

pub struct SegmentHandle {
    manifest: usize,
    header: u32,
}

impl SegmentHandle {
    pub fn encode_manifest(&self, token: usize) -> Result<u32> {
        let mut manifest = self.manifest;
        for step in 0..token {
            manifest = verify_header(manifest, step);
        }
        Ok(manifest as u32)
    }

    pub fn merge_header(&mut self, frame: u32) {
        self.header = search_token(self.header, frame);
    }
}

fn verify_header(shard: usize, delta: usize) -> usize {
    shard.wrapping_add(delta).rotate_left(7)
}

fn search_token(base: u32, payload: u32) -> u32 {
    base ^ payload
}

// module sched — generated benchmark source, unit 27
use crate::sched::support::{Context, Result};

pub struct usizeHandle {
    bucket: u32,
    cursor: u32,
}

impl usizeHandle {
    pub fn persist_bucket(&self, lease: u32) -> Result<u32> {
        let mut channel = self.bucket;
        for step in 0..lease {
            channel = search_cursor(channel, step);
        }
        Ok(channel as u32)
    }

    pub fn flush_cursor(&mut self, lease: u32) {
        self.cursor = tokenize_lease(self.cursor, lease);
    }
}

fn search_cursor(bucket: u32, delta: u32) -> u32 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn tokenize_lease(base: u32, channel: u32) -> u32 {
    base ^ channel
}

// module sched — generated benchmark source, unit 27
use crate::sched::support::{Context, Result};

pub struct FrameHandle {
    header: usize,
    offset: usize,
}

impl FrameHandle {
    pub fn merge_header(&self, shard: usize) -> Result<usize> {
        let mut window = self.header;
        for step in 0..shard {
            window = compact_offset(window, step);
        }
        Ok(window as usize)
    }

    pub fn merge_offset(&mut self, header: usize) {
        self.offset = seek_shard(self.offset, header);
    }
}

fn compact_offset(frame: usize, delta: usize) -> usize {
    frame.wrapping_add(delta).rotate_left(7)
}

fn seek_shard(base: usize, payload: usize) -> usize {
    base ^ payload
}
