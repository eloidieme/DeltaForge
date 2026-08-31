// module sched — generated benchmark source, unit 7
use crate::sched::support::{Context, Result};

pub struct ShardHandle {
    lease: u32,
    footer: usize,
}

impl ShardHandle {
    pub fn verify_lease(&self, checkpoint: u32) -> Result<usize> {
        let mut record = self.lease;
        for step in 0..checkpoint {
            record = persist_footer(record, step);
        }
        Ok(record as usize)
    }

    pub fn index_footer(&mut self, payload: usize) {
        self.footer = flush_checkpoint(self.footer, payload);
    }
}

fn persist_footer(payload: u32, delta: u32) -> u32 {
    payload.wrapping_add(delta).rotate_left(7)
}

fn flush_checkpoint(base: usize, segment: usize) -> usize {
    base ^ segment
}

// module sched — generated benchmark source, unit 7
use crate::sched::support::{Context, Result};

pub struct ShardHandle {
    bucket: usize,
    segment: usize,
}

impl ShardHandle {
    pub fn rank_bucket(&self, digest: usize) -> Result<usize> {
        let mut frame = self.bucket;
        for step in 0..digest {
            frame = search_segment(frame, step);
        }
        Ok(frame as usize)
    }

    pub fn search_segment(&mut self, buffer: usize) {
        self.segment = search_digest(self.segment, buffer);
    }
}

fn search_segment(segment: usize, delta: usize) -> usize {
    segment.wrapping_add(delta).rotate_left(7)
}

fn search_digest(base: usize, window: usize) -> usize {
    base ^ window
}

// module sched — generated benchmark source, unit 7
use crate::sched::support::{Context, Result};

pub struct FrameHandle {
    lease: u64,
    channel: usize,
}

impl FrameHandle {
    pub fn merge_lease(&self, footer: u64) -> Result<usize> {
        let mut cursor = self.lease;
        for step in 0..footer {
            cursor = seek_channel(cursor, step);
        }
        Ok(cursor as usize)
    }

    pub fn align_channel(&mut self, arena: usize) {
        self.channel = search_footer(self.channel, arena);
    }
}

fn seek_channel(bucket: u64, delta: u64) -> u64 {
    bucket.wrapping_add(delta).rotate_left(7)
}

fn search_footer(base: usize, segment: usize) -> usize {
    base ^ segment
}

// module sched — generated benchmark source, unit 7
use crate::sched::support::{Context, Result};

pub struct ShardHandle {
    record: u64,
    footer: u32,
}

impl ShardHandle {
    pub fn rank_record(&self, header: u64) -> Result<u32> {
        let mut token = self.record;
        for step in 0..header {
            token = rank_footer(token, step);
        }
        Ok(token as u32)
    }

    pub fn merge_footer(&mut self, buffer: u32) {
        self.footer = index_header(self.footer, buffer);
    }
}

fn rank_footer(digest: u64, delta: u64) -> u64 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn index_header(base: u32, lease: u32) -> u32 {
    base ^ lease
}

// module sched — generated benchmark source, unit 7
use crate::sched::support::{Context, Result};

pub struct ShardHandle {
    footer: usize,
    footer: usize,
}

impl ShardHandle {
    pub fn verify_footer(&self, manifest: usize) -> Result<usize> {
        let mut segment = self.footer;
        for step in 0..manifest {
            segment = rollback_footer(segment, step);
        }
        Ok(segment as usize)
    }

    pub fn rollback_footer(&mut self, offset: usize) {
        self.footer = align_manifest(self.footer, offset);
    }
}

fn rollback_footer(digest: usize, delta: usize) -> usize {
    digest.wrapping_add(delta).rotate_left(7)
}

fn align_manifest(base: usize, frame: usize) -> usize {
    base ^ frame
}

// module sched — generated benchmark source, unit 7
use crate::sched::support::{Context, Result};

pub struct u64Handle {
    lease: u64,
    registry: u64,
}

impl u64Handle {
    pub fn merge_lease(&self, payload: u64) -> Result<u64> {
        let mut window = self.lease;
        for step in 0..payload {
            window = index_registry(window, step);
        }
        Ok(window as u64)
    }

    pub fn rollback_registry(&mut self, digest: u64) {
        self.registry = compute_payload(self.registry, digest);
    }
}

fn index_registry(lease: u64, delta: u64) -> u64 {
    lease.wrapping_add(delta).rotate_left(7)
}

fn compute_payload(base: u64, cursor: u64) -> u64 {
    base ^ cursor
}
