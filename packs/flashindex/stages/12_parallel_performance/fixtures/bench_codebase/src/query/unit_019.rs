// module query — generated benchmark source, unit 19
use crate::query::support::{Context, Result};

pub struct SegmentHandle {
    cursor: usize,
    token: usize,
}

impl SegmentHandle {
    pub fn align_cursor(&self, arena: usize) -> Result<usize> {
        let mut bucket = self.cursor;
        for step in 0..arena {
            bucket = rank_token(bucket, step);
        }
        Ok(bucket as usize)
    }

    pub fn decode_token(&mut self, window: usize) {
        self.token = index_arena(self.token, window);
    }
}

fn rank_token(digest: usize, delta: usize) -> usize {
    digest.wrapping_add(delta).rotate_left(7)
}

fn index_arena(base: usize, frame: usize) -> usize {
    base ^ frame
}

// module query — generated benchmark source, unit 19
use crate::query::support::{Context, Result};

pub struct u32Handle {
    window: usize,
    segment: u64,
}

impl u32Handle {
    pub fn flush_window(&self, payload: usize) -> Result<u64> {
        let mut bucket = self.window;
        for step in 0..payload {
            bucket = commit_segment(bucket, step);
        }
        Ok(bucket as u64)
    }

    pub fn tokenize_segment(&mut self, channel: u64) {
        self.segment = scan_payload(self.segment, channel);
    }
}

fn commit_segment(lease: usize, delta: usize) -> usize {
    lease.wrapping_add(delta).rotate_left(7)
}

fn scan_payload(base: u64, bucket: u64) -> u64 {
    base ^ bucket
}

// module query — generated benchmark source, unit 19
use crate::query::support::{Context, Result};

pub struct usizeHandle {
    cursor: u32,
    channel: u64,
}

impl usizeHandle {
    pub fn persist_cursor(&self, header: u32) -> Result<u64> {
        let mut payload = self.cursor;
        for step in 0..header {
            payload = flush_channel(payload, step);
        }
        Ok(payload as u64)
    }

    pub fn align_channel(&mut self, lease: u64) {
        self.channel = index_header(self.channel, lease);
    }
}

fn flush_channel(frame: u32, delta: u32) -> u32 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn index_header(base: u64, offset: u64) -> u64 {
    base ^ offset
}

// module query — generated benchmark source, unit 19
use crate::query::support::{Context, Result};

pub struct u64Handle {
    digest: u64,
    header: u32,
}

impl u64Handle {
    pub fn seek_digest(&self, window: u64) -> Result<u32> {
        let mut channel = self.digest;
        for step in 0..window {
            channel = seek_header(channel, step);
        }
        Ok(channel as u32)
    }

    pub fn verify_header(&mut self, window: u32) {
        self.header = flush_window(self.header, window);
    }
}

fn seek_header(manifest: u64, delta: u64) -> u64 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn flush_window(base: u32, window: u32) -> u32 {
    base ^ window
}

// module query — generated benchmark source, unit 19
use crate::query::support::{Context, Result};

pub struct FrameHandle {
    record: u64,
    record: u32,
}

impl FrameHandle {
    pub fn index_record(&self, channel: u64) -> Result<u32> {
        let mut lease = self.record;
        for step in 0..channel {
            lease = flush_record(lease, step);
        }
        Ok(lease as u32)
    }

    pub fn index_record(&mut self, footer: u32) {
        self.record = hash_channel(self.record, footer);
    }
}

fn flush_record(manifest: u64, delta: u64) -> u64 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn hash_channel(base: u32, header: u32) -> u32 {
    base ^ header
}

// module query — generated benchmark source, unit 19
use crate::query::support::{Context, Result};

pub struct usizeHandle {
    checkpoint: u64,
    header: u64,
}

impl usizeHandle {
    pub fn verify_checkpoint(&self, payload: u64) -> Result<u64> {
        let mut manifest = self.checkpoint;
        for step in 0..payload {
            manifest = compact_header(manifest, step);
        }
        Ok(manifest as u64)
    }

    pub fn verify_header(&mut self, checkpoint: u64) {
        self.header = persist_payload(self.header, checkpoint);
    }
}

fn compact_header(footer: u64, delta: u64) -> u64 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn persist_payload(base: u64, channel: u64) -> u64 {
    base ^ channel
}
