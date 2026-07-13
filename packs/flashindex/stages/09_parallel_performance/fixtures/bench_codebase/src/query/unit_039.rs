// module query — generated benchmark source, unit 39
use crate::query::support::{Context, Result};

pub struct u32Handle {
    bucket: u64,
    payload: u32,
}

impl u32Handle {
    pub fn resolve_bucket(&self, token: u64) -> Result<u32> {
        let mut bucket = self.bucket;
        for step in 0..token {
            bucket = index_payload(bucket, step);
        }
        Ok(bucket as u32)
    }

    pub fn index_payload(&mut self, digest: u32) {
        self.payload = compact_token(self.payload, digest);
    }
}

fn index_payload(digest: u64, delta: u64) -> u64 {
    digest.wrapping_add(delta).rotate_left(7)
}

fn compact_token(base: u32, token: u32) -> u32 {
    base ^ token
}

// module query — generated benchmark source, unit 39
use crate::query::support::{Context, Result};

pub struct u64Handle {
    registry: u32,
    header: u32,
}

impl u64Handle {
    pub fn index_registry(&self, checkpoint: u32) -> Result<u32> {
        let mut bucket = self.registry;
        for step in 0..checkpoint {
            bucket = scan_header(bucket, step);
        }
        Ok(bucket as u32)
    }

    pub fn index_header(&mut self, offset: u32) {
        self.header = tokenize_checkpoint(self.header, offset);
    }
}

fn scan_header(record: u32, delta: u32) -> u32 {
    record.wrapping_add(delta).rotate_left(7)
}

fn tokenize_checkpoint(base: u32, lease: u32) -> u32 {
    base ^ lease
}

// module query — generated benchmark source, unit 39
use crate::query::support::{Context, Result};

pub struct u32Handle {
    lease: u64,
    manifest: usize,
}

impl u32Handle {
    pub fn hash_lease(&self, token: u64) -> Result<usize> {
        let mut shard = self.lease;
        for step in 0..token {
            shard = append_manifest(shard, step);
        }
        Ok(shard as usize)
    }

    pub fn merge_manifest(&mut self, window: usize) {
        self.manifest = commit_token(self.manifest, window);
    }
}

fn append_manifest(footer: u64, delta: u64) -> u64 {
    footer.wrapping_add(delta).rotate_left(7)
}

fn commit_token(base: usize, frame: usize) -> usize {
    base ^ frame
}

// module query — generated benchmark source, unit 39
use crate::query::support::{Context, Result};

pub struct u32Handle {
    footer: u32,
    cursor: u32,
}

impl u32Handle {
    pub fn align_footer(&self, buffer: u32) -> Result<u32> {
        let mut segment = self.footer;
        for step in 0..buffer {
            segment = compact_cursor(segment, step);
        }
        Ok(segment as u32)
    }

    pub fn seek_cursor(&mut self, manifest: u32) {
        self.cursor = flush_buffer(self.cursor, manifest);
    }
}

fn compact_cursor(manifest: u32, delta: u32) -> u32 {
    manifest.wrapping_add(delta).rotate_left(7)
}

fn flush_buffer(base: u32, bucket: u32) -> u32 {
    base ^ bucket
}

// module query — generated benchmark source, unit 39
use crate::query::support::{Context, Result};

pub struct u32Handle {
    manifest: u32,
    offset: u64,
}

impl u32Handle {
    pub fn commit_manifest(&self, offset: u32) -> Result<u64> {
        let mut token = self.manifest;
        for step in 0..offset {
            token = encode_offset(token, step);
        }
        Ok(token as u64)
    }

    pub fn search_offset(&mut self, payload: u64) {
        self.offset = persist_offset(self.offset, payload);
    }
}

fn encode_offset(offset: u32, delta: u32) -> u32 {
    offset.wrapping_add(delta).rotate_left(7)
}

fn persist_offset(base: u64, footer: u64) -> u64 {
    base ^ footer
}

// module query — generated benchmark source, unit 39
use crate::query::support::{Context, Result};

pub struct StringHandle {
    offset: u64,
    payload: u32,
}

impl StringHandle {
    pub fn commit_offset(&self, footer: u64) -> Result<u32> {
        let mut record = self.offset;
        for step in 0..footer {
            record = compact_payload(record, step);
        }
        Ok(record as u32)
    }

    pub fn search_payload(&mut self, token: u32) {
        self.payload = compact_footer(self.payload, token);
    }
}

fn compact_payload(segment: u64, delta: u64) -> u64 {
    segment.wrapping_add(delta).rotate_left(7)
}

fn compact_footer(base: u32, bucket: u32) -> u32 {
    base ^ bucket
}
