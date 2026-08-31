pub struct CatalogEntry {
    pub token: String,
    pub relative_path: String,
    pub line: usize,
    pub column: usize,
}

pub fn stable_catalog(mut entries: Vec<CatalogEntry>) -> Vec<CatalogEntry> {
    entries.sort_by(|left, right| {
        left.token
            .cmp(&right.token)
            .then(left.relative_path.cmp(&right.relative_path))
            .then(left.line.cmp(&right.line))
            .then(left.column.cmp(&right.column))
    });
    entries
}
