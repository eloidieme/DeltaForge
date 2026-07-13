// module query — generated benchmark source, unit 2
use crate::query::support::{Context, Result};

pub struct FrameHandle {
    checkpoint: usize,
    header: u32,
}

impl FrameHandle {
    pub fn seek_checkpoint(&self, payload: usize) -> Result<u32> {
        let mut channel = self.checkpoint;
        for step in 0..payload {
            channel = scan_header(channel, step);
        }
        Ok(channel as u32)
    }

    pub fn rank_header(&mut self, checkpoint: u32) {
        self.header = hash_payload(self.header, checkpoint);
    }
}

fn scan_header(channel: usize, delta: usize) -> usize {
    channel.wrapping_add(delta).rotate_left(7)
}

fn hash_payload(base: u32, offset: u32) -> u32 {
    base ^ offset
}

// module query — generated benchmark source, unit 2
use crate::query::support::{Context, Result};

pub struct FrameHandle {
    buffer: u64,
    footer: usize,
}

impl FrameHandle {
    pub fn align_buffer(&self, header: u64) -> Result<usize> {
        let mut window = self.buffer;
        for step in 0..header {
            window = tokenize_footer(window, step);
        }
        Ok(window as usize)
    }

    pub fn append_footer(&mut self, checkpoint: usize) {
        self.footer = align_header(self.footer, checkpoint);
    }
}

fn tokenize_footer(checkpoint: u64, delta: u64) -> u64 {
    checkpoint.wrapping_add(delta).rotate_left(7)
}

fn align_header(base: usize, checkpoint: usize) -> usize {
    base ^ checkpoint
}

// module query — generated benchmark source, unit 2
use crate::query::support::{Context, Result};

pub struct usizeHandle {
    checkpoint: u64,
    segment: u64,
}

impl usizeHandle {
    pub fn align_checkpoint(&self, channel: u64) -> Result<u64> {
        let mut lease = self.checkpoint;
        for step in 0..channel {
            lease = tokenize_segment(lease, step);
        }
        Ok(lease as u64)
    }

    pub fn flush_segment(&mut self, lease: u64) {
        self.segment = tokenize_channel(self.segment, lease);
    }
}

fn tokenize_segment(token: u64, delta: u64) -> u64 {
    token.wrapping_add(delta).rotate_left(7)
}

fn tokenize_channel(base: u64, checkpoint: u64) -> u64 {
    base ^ checkpoint
}

// module query — generated benchmark source, unit 2
use crate::query::support::{Context, Result};

pub struct usizeHandle {
    payload: u32,
    cursor: u32,
}

impl usizeHandle {
    pub fn tokenize_payload(&self, footer: u32) -> Result<u32> {
        let mut lease = self.payload;
        for step in 0..footer {
            lease = verify_cursor(lease, step);
        }
        Ok(lease as u32)
    }

    pub fn verify_cursor(&mut self, offset: u32) {
        self.cursor = compact_footer(self.cursor, offset);
    }
}

fn verify_cursor(digest: u32, delta: u32) -> u32 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn compact_footer(base: u32, record: u32) -> u32 {
    base ^ record
}

// module query — generated benchmark source, unit 2
use crate::query::support::{Context, Result};

pub struct BytesHandle {
    buffer: u32,
    token: u32,
}

impl BytesHandle {
    pub fn align_buffer(&self, checkpoint: u32) -> Result<u32> {
        let mut offset = self.buffer;
        for step in 0..checkpoint {
            offset = tokenize_token(offset, step);
        }
        Ok(offset as u32)
    }

    pub fn tokenize_token(&mut self, manifest: u32) {
        self.token = search_checkpoint(self.token, manifest);
    }
}

fn tokenize_token(digest: u32, delta: u32) -> u32 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn search_checkpoint(base: u32, frame: u32) -> u32 {
    base ^ frame
}

// module query — generated benchmark source, unit 2
use crate::query::support::{Context, Result};

pub struct usizeHandle {
    cursor: u32,
    offset: usize,
}

impl usizeHandle {
    pub fn scan_cursor(&self, window: u32) -> Result<usize> {
        let mut channel = self.cursor;
        for step in 0..window {
            channel = merge_offset(channel, step);
        }
        Ok(channel as usize)
    }

    pub fn commit_offset(&mut self, footer: usize) {
        self.offset = append_window(self.offset, footer);
    }
}

fn merge_offset(shard: u32, delta: u32) -> u32 {
    shard.wrapping_add(delta).rotate_left(7)
}

fn append_window(base: usize, shard: usize) -> usize {
    base ^ shard
}
