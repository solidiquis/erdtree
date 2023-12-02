use crate::{
    error::prelude::*,
    file::{tree::Tree, File},
    user::{
        args::{FileType, Layout},
        Context, Globbing,
    },
};
use ahash::HashSet;
use ignore::overrides::OverrideBuilder;
use indextree::NodeId;
use regex::Regex;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("A regex pattern was not provided")]
    MissingRegexPattern,

    #[error("{0}")]
    InvalidRegex(regex::Error),
}

/// Predicate used for filtering a [`File`] based on [`FileType`].
pub type FileTypeFilter = dyn Fn(&File) -> bool;

impl Tree {
    /// Filter [`File`]s in the [`indextree::Arena`] based on provided [`Context`]. The order in
    /// which the filters are applied matters.
    pub fn filter_nodes(&mut self, ctx: &Context) -> Result<()> {
        if !ctx.file_type.is_empty() {
            self.filter_file_type(ctx);
        }

        if ctx.pattern.is_some() {
            let Globbing { glob, iglob } = ctx.globbing;

            if glob || iglob {
                self.filter_glob(ctx)?;
            } else {
                self.filter_regex(ctx)?;
            }
        }

        if ctx.prune {
            self.prune();
        }

        Ok(())
    }

    /// Remove all directories that have no children.
    fn prune(&mut self) {
        let mut pruning = true;

        while pruning {
            let mut to_remove = vec![];

            for n in self.root_id.descendants(&self.arena).skip(1) {
                if !n.is_removed(&self.arena)
                    && self.arena[n].get().is_dir()
                    && n.children(&self.arena).count() == 0
                {
                    to_remove.push(n);
                }
            }

            if !to_remove.is_empty() {
                to_remove
                    .into_iter()
                    .for_each(|n| n.remove_subtree(&mut self.arena));
                continue;
            }
            pruning = false;
        }
    }

    /// Updates the [`Tree`]'s inner [`indextree::Arena`] to only contain files of certain
    /// file-types. This should not affect disk-usage calculations.
    ///
    /// TODO: Consider using Rayon for parallel filtering.
    pub fn filter_file_type(
        &mut self,
        Context {
            layout, file_type, ..
        }: &Context,
    ) {
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
            .for_each(|n| n.remove_subtree(&mut self.arena));
    }

    /// Remove nodes/sub-trees that don't match the provided regular expression, `pattern`.
    pub fn filter_regex(
        &mut self,
        Context {
            pattern, layout, ..
        }: &Context,
    ) -> Result<()> {
        let re_pattern = pattern
            .as_ref()
            .ok_or(Error::MissingRegexPattern)
            .into_report(ErrorCategory::User)?;

        let regex = Regex::new(re_pattern)
            .map_err(Error::InvalidRegex)
            .into_report(ErrorCategory::User)?;

        match layout {
            Layout::Flat => {
                let to_remove = self
                    .root_id
                    .descendants(&self.arena)
                    .skip(1)
                    .filter(|node_id| {
                        !regex
                            .is_match(self.arena[*node_id].get().path().to_string_lossy().as_ref())
                    })
                    .collect::<Vec<_>>();

                to_remove
                    .into_iter()
                    .for_each(|n| n.remove(&mut self.arena));
            },
            _ => {
                let to_remove = self
                    .root_id
                    .descendants(&self.arena)
                    .skip(1)
                    .filter(|node_id| {
                        let node = self.arena[*node_id].get();
                        !node.is_dir() && !regex.is_match(node.path().to_string_lossy().as_ref())
                    })
                    .collect::<Vec<_>>();

                to_remove
                    .into_iter()
                    .for_each(|n| n.remove_subtree(&mut self.arena));
            },
        };

        Ok(())
    }

    /// Filtering via globbing
    fn filter_glob(&mut self, ctx: &Context) -> Result<()> {
        let Context {
            globbing: Globbing { iglob, .. },
            pattern,
            layout,
            ..
        } = ctx;

        let dir = ctx.dir_canonical()?;
        let mut override_builder = OverrideBuilder::new(dir);

        let mut negated_glob = false;

        let overrides = {
            if *iglob {
                override_builder
                    .case_insensitive(true)
                    .into_report(ErrorCategory::Internal)
                    .context(error_source!())?;
            }

            if let Some(ref glob) = pattern {
                let trim = glob.trim_start();
                negated_glob = trim.starts_with('!');

                if negated_glob {
                    override_builder
                        .add(trim.trim_start_matches('!'))
                        .into_report(ErrorCategory::Internal)
                        .context(error_source!())?;
                } else {
                    override_builder
                        .add(trim)
                        .into_report(ErrorCategory::Internal)
                        .context(error_source!())?;
                }
            }

            override_builder.build().into_report(ErrorCategory::User)?
        };

        match layout {
            Layout::Flat => {
                let to_remove = self
                    .root_id
                    .descendants(&self.arena)
                    .skip(1)
                    .filter(|node_id| {
                        let dirent = self.arena[*node_id].get();
                        let matched = overrides.matched(dirent.path(), dirent.is_dir());
                        !(negated_glob ^ matched.is_whitelist())
                    })
                    .collect::<Vec<_>>();

                to_remove
                    .into_iter()
                    .for_each(|n| n.remove(&mut self.arena));
            },
            _ => {
                let to_remove = self
                    .root_id
                    .descendants(&self.arena)
                    .skip(1)
                    .filter(|node_id| {
                        let dirent = self.arena[*node_id].get();

                        if dirent.is_dir() {
                            false
                        } else {
                            let matched = overrides.matched(dirent.path(), dirent.is_dir());
                            !(negated_glob ^ matched.is_whitelist())
                        }
                    })
                    .collect::<Vec<_>>();

                to_remove
                    .into_iter()
                    .for_each(|n| n.remove_subtree(&mut self.arena));
            },
        }

        Ok(())
    }
}
