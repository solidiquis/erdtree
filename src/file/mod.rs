use ignore::DirEntry;
use std::fs::Metadata;

pub struct File {
    inner: DirEntry,
    metadata: Metadata,
}
