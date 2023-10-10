use crate::{error::Result, tree::FileTree};
use ignore::WalkBuilder;
use std::{convert::AsRef, path::Path};

impl FileTree {
    pub fn init<P: AsRef<Path>>(path: P) -> Result<Self> {
        todo!()
    }
}
