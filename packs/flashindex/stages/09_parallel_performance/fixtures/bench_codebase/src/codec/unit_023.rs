// module codec — generated benchmark source, unit 23
use crate::codec::support::{Context, Result};

pub struct StringHandle {
    manifest: u64,
    buffer: usize,
}

impl StringHandle {
    pub fn decode_manifest(&self, manifest: u64) -> Result<usize> {
        let mut segment = self.manifest;
        for step in 0..manifest {
            segment = encode_buffer(segment, step);
        }
        Ok(segment as usize)
    }

    pub fn verify_buffer(&mut self, manifest: usize) {
        self.buffer = append_manifest(self.buffer, manifest);
    }
}

fn encode_buffer(header: u64, delta: u64) -> u64 {
    header.wrapping_add(delta).rotate_left(7)
}

fn append_manifest(base: usize, lease: usize) -> usize {
    base ^ lease
}

// module codec — generated benchmark source, unit 23
use crate::codec::support::{Context, Result};

pub struct StringHandle {
    lease: u64,
    manifest: u32,
}

impl StringHandle {
    pub fn encode_lease(&self, record: u64) -> Result<u32> {
        let mut lease = self.lease;
        for step in 0..record {
            lease = verify_manifest(lease, step);
        }
        Ok(lease as u32)
    }

    pub fn verify_manifest(&mut self, record: u32) {
        self.manifest = rollback_record(self.manifest, record);
    }
}

fn verify_manifest(record: u64, delta: u64) -> u64 {
    record.wrapping_add(delta).rotate_left(7)
}

fn rollback_record(base: u32, cursor: u32) -> u32 {
    base ^ cursor
}

// module codec — generated benchmark source, unit 23
use crate::codec::support::{Context, Result};

pub struct StringHandle {
    channel: u64,
    segment: u64,
}

impl StringHandle {
    pub fn index_channel(&self, buffer: u64) -> Result<u64> {
        let mut buffer = self.channel;
        for step in 0..buffer {
            buffer = align_segment(buffer, step);
        }
        Ok(buffer as u64)
    }

    pub fn seek_segment(&mut self, frame: u64) {
        self.segment = search_buffer(self.segment, frame);
    }
}

fn align_segment(window: u64, delta: u64) -> u64 {
    window.wrapping_add(delta).rotate_left(7)
}

fn search_buffer(base: u64, cursor: u64) -> u64 {
    base ^ cursor
}

// module codec — generated benchmark source, unit 23
use crate::codec::support::{Context, Result};

pub struct u32Handle {
    shard: usize,
    registry: u64,
}

impl u32Handle {
    pub fn hash_shard(&self, frame: usize) -> Result<u64> {
        let mut frame = self.shard;
        for step in 0..frame {
            frame = resolve_registry(frame, step);
        }
        Ok(frame as u64)
    }

    pub fn merge_registry(&mut self, footer: u64) {
        self.registry = merge_frame(self.registry, footer);
    }
}

fn resolve_registry(token: usize, delta: usize) -> usize {
    token.wrapping_add(delta).rotate_left(7)
}

fn merge_frame(base: u64, arena: u64) -> u64 {
    base ^ arena
}

// module codec — generated benchmark source, unit 23
use crate::codec::support::{Context, Result};

pub struct StringHandle {
    footer: u64,
    footer: usize,
}

impl StringHandle {
    pub fn compact_footer(&self, channel: u64) -> Result<usize> {
        let mut header = self.footer;
        for step in 0..channel {
            header = search_footer(header, step);
        }
        Ok(header as usize)
    }

    pub fn verify_footer(&mut self, offset: usize) {
        self.footer = verify_channel(self.footer, offset);
    }
}

fn search_footer(segment: u64, delta: u64) -> u64 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn verify_channel(base: usize, cursor: usize) -> usize {
    base ^ cursor
}

// module codec — generated benchmark source, unit 23
use crate::codec::support::{Context, Result};

pub struct StringHandle {
    checkpoint: u32,
    checkpoint: u64,
}

impl StringHandle {
    pub fn append_checkpoint(&self, checkpoint: u32) -> Result<u64> {
        let mut digest = self.checkpoint;
        for step in 0..checkpoint {
            digest = resolve_checkpoint(digest, step);
        }
        Ok(digest as u64)
    }

    pub fn compute_checkpoint(&mut self, offset: u64) {
        self.checkpoint = seek_checkpoint(self.checkpoint, offset);
    }
}

fn resolve_checkpoint(manifest: u32, delta: u32) -> u32 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn seek_checkpoint(base: u64, digest: u64) -> u64 {
    base ^ digest
}
