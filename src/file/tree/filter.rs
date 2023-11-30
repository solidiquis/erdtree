use crate::{
    file::{tree::Tree, File},
    user::{
        args::{FileType, Layout},
        Context,
    },
};
use ahash::HashSet;
use indextree::NodeId;

/// Predicate used for filtering a [`File`] based on [`FileType`].
pub type FileTypeFilter = dyn Fn(&File) -> bool;

impl Tree {
    /// Updates the [`Tree`]'s inner [`indextree::Arena`] to only contain files of certain
    /// file-types.
    pub fn filter_file_type(
        &mut self,
        Context {
            layout, file_type, ..
        }: &Context,
    ) {
        if file_type.is_empty() {
            return;
        }

        let mut filters = Vec::<Box<FileTypeFilter>>::new();

        for ft in HashSet::from_iter(file_type) {
            match ft {
                FileType::Dir if matches!(layout, Layout::Tree | Layout::InvertedTree) => {
                    filters.push(Box::new(|f| f.is_dir()))
                },
                FileType::Dir => filters.push(Box::new(|f| f.is_dir())),
                FileType::File => filters.push(Box::new(|f| f.is_file())),
                FileType::Symlink => filters.push(Box::new(|f| f.is_symlink())),

                #[cfg(unix)]
                FileType::Pipe => filters.push(Box::new(|f| f.is_fifo())),
                #[cfg(unix)]
                FileType::Socket => filters.push(Box::new(|f| f.is_socket())),
                #[cfg(unix)]
                FileType::Char => filters.push(Box::new(|f| f.is_char_device())),
                #[cfg(unix)]
                FileType::Block => filters.push(Box::new(|f| f.is_block_device())),
            }
        }

        let no_match = |node_id: &NodeId| !filters.iter().any(|f| f(self.arena[*node_id].get()));

        let to_remove = self
            .root_id
            .descendants(&self.arena)
            .filter(no_match)
            .collect::<Vec<_>>();

        to_remove
            .into_iter()
            .for_each(|n| n.detach(&mut self.arena));
    }
}
