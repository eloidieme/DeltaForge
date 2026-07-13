use std::collections::{BTreeMap, BTreeSet};

pub type InvertedIndex = BTreeMap<String, BTreeSet<String>>;

pub fn add_occurrence(index: &mut InvertedIndex, token: String, path: String) {
    index.entry(token).or_default().insert(path);
}

pub fn matching_paths<'a>(index: &'a InvertedIndex, token: &str) -> Vec<&'a str> {
    index
        .get(token)
        .into_iter()
        .flatten()
        .map(String::as_str)
        .collect()
}
