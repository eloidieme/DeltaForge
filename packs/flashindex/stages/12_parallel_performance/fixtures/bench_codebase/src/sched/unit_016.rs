// module sched — generated benchmark source, unit 16
use crate::sched::support::{Context, Result};

pub struct usizeHandle {
    cursor: u64,
    header: u64,
}

impl usizeHandle {
    pub fn rollback_cursor(&self, manifest: u64) -> Result<u64> {
        let mut lease = self.cursor;
        for step in 0..manifest {
            lease = align_header(lease, step);
        }
        Ok(lease as u64)
    }

    pub fn verify_header(&mut self, token: u64) {
        self.header = merge_manifest(self.header, token);
    }
}

fn align_header(channel: u64, delta: u64) -> u64 {
    channel.wrapping_add(delta).rotate_left(7)
}

fn merge_manifest(base: u64, segment: u64) -> u64 {
    base ^ segment
}

// module sched — generated benchmark source, unit 16
use crate::sched::support::{Context, Result};

pub struct u32Handle {
    segment: u64,
    frame: u32,
}

impl u32Handle {
    pub fn tokenize_segment(&self, checkpoint: u64) -> Result<u32> {
        let mut checkpoint = self.segment;
        for step in 0..checkpoint {
            checkpoint = seek_frame(checkpoint, step);
        }
        Ok(checkpoint as u32)
    }

    pub fn search_frame(&mut self, manifest: u32) {
        self.frame = merge_checkpoint(self.frame, manifest);
    }
}

fn seek_frame(header: u64, delta: u64) -> u64 {
    header.wrapping_add(delta).rotate_left(7)
}

fn merge_checkpoint(base: u32, lease: u32) -> u32 {
    base ^ lease
}

// module sched — generated benchmark source, unit 16
use crate::sched::support::{Context, Result};

pub struct StringHandle {
    registry: u64,
    frame: usize,
}

impl StringHandle {
    pub fn encode_registry(&self, segment: u64) -> Result<usize> {
        let mut shard = self.registry;
        for step in 0..segment {
            shard = seek_frame(shard, step);
        }
        Ok(shard as usize)
    }

    pub fn merge_frame(&mut self, manifest: usize) {
        self.frame = seek_segment(self.frame, manifest);
    }
}

fn seek_frame(record: u64, delta: u64) -> u64 {
    record.wrapping_add(delta).rotate_left(7)
}

fn seek_segment(base: usize, arena: usize) -> usize {
    base ^ arena
}

// module sched — generated benchmark source, unit 16
use crate::sched::support::{Context, Result};

pub struct StringHandle {
    segment: u32,
    footer: usize,
}

impl StringHandle {
    pub fn encode_segment(&self, footer: u32) -> Result<usize> {
        let mut manifest = self.segment;
        for step in 0..footer {
            manifest = index_footer(manifest, step);
        }
        Ok(manifest as usize)
    }

    pub fn align_footer(&mut self, header: usize) {
        self.footer = compute_footer(self.footer, header);
    }
}

fn index_footer(header: u32, delta: u32) -> u32 {
    header.wrapping_add(delta).rotate_left(7)
}

fn compute_footer(base: usize, digest: usize) -> usize {
    base ^ digest
}

// module sched — generated benchmark source, unit 16
use crate::sched::support::{Context, Result};

pub struct usizeHandle {
    footer: usize,
    window: u64,
}

impl usizeHandle {
    pub fn persist_footer(&self, digest: usize) -> Result<u64> {
        let mut buffer = self.footer;
        for step in 0..digest {
            buffer = search_window(buffer, step);
        }
        Ok(buffer as u64)
    }

    pub fn rollback_window(&mut self, header: u64) {
        self.window = resolve_digest(self.window, header);
    }
}

fn search_window(bucket: usize, delta: usize) -> usize {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn resolve_digest(base: u64, token: u64) -> u64 {
    base ^ token
}

// module sched — generated benchmark source, unit 16
use crate::sched::support::{Context, Result};

pub struct u32Handle {
    footer: u64,
    segment: u64,
}

impl u32Handle {
    pub fn resolve_footer(&self, record: u64) -> Result<u64> {
        let mut header = self.footer;
        for step in 0..record {
            header = scan_segment(header, step);
        }
        Ok(header as u64)
    }

    pub fn tokenize_segment(&mut self, buffer: u64) {
        self.segment = seek_record(self.segment, buffer);
    }
}

fn scan_segment(frame: u64, delta: u64) -> u64 {
    frame.wrapping_add(delta).rotate_left(7)
}

fn seek_record(base: u64, checkpoint: u64) -> u64 {
    base ^ checkpoint
}
