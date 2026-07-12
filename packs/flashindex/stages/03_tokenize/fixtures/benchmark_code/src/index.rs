pub struct PostingList {
    token: String,
    documents: Vec<String>,
}

impl PostingList {
    pub fn insert_document(&mut self, relative_path: String) {
        if !self.documents.contains(&relative_path) {
            self.documents.push(relative_path);
            self.documents.sort();
        }
    }

    pub fn document_frequency(&self) -> usize {
        self.documents.len()
    }
}

pub fn merge_postings(left: PostingList, right: PostingList) -> PostingList {
    let mut merged = left;
    for relative_path in right.documents {
        merged.insert_document(relative_path);
    }
    merged
}
