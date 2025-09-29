pub enum ReadFileContentsResult {
    Directory,
    Found { contents: String },
    NotFound,
}
