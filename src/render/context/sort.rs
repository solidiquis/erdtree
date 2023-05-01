use clap::ValueEnum;

/// Order in which to print nodes.
#[derive(Copy, Clone, Debug, ValueEnum, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum Type {
    /// Sort entries by file name in lexicographical order.
    Name,
    /// Sort entries by file name in reversed lexicographical order.
    NameRev,

    /// Sort entries by size smallest to largest, top to bottom
    #[default]
    Size,

    /// Sort entries by size largest to smallest, bottom to top
    SizeRev,

    // Sort entries by newer to older Accessing Date
    //Access,

    // Sort entries by older to newer Accessing Date
    //AccessRev,

    // Sort entries by newer to older Creation Date
    //Creation,

    // Sort entries by older to newer Creation Date
    //CreationRev,

    // Sort entries by newer to older Alteration Date
    //Modification,

    // Sort entries by older to newer Alteration Date
    //ModificationRev,
}
