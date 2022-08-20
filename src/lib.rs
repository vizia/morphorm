//! # Morphorm
//!
//! Morphorm is a crate for determining the position and size of UI elements which are organised in a visual tree.
//! The algorithm is written from scratch but is based on the [subform layout system](https://subformapp.com/articles/why-not-flexbox/).
//!
//! # How to use
//!
//! To try and keep things as generic as possible Morphorm does not provide any containers for representing the layout properties or visual tree.
//! Instead, three traits must be implemented by the users' containers in order to utilise the layout algorithm:
//!
//! - `Node` represents a UI element which can be sized and positioned. The node itself could contain the desired layout properties, or the properties
//!   can be provided by an external source (such as an ECS component store), which is provided by the `Data` associated type.
//!
//! - `Hierarchy` represents the visual tree of nodes. Morphorm requires three iterators to be provided: an upward iterator which iterates the tree
//!   from bottom to top in depth first order; a downward iterator which iterates the tree from the root in depth first order; and a child iterator
//!   which iterates through the children of a specified node.
//!
//! - `Cache` represents a store for the output of the layout computation as well as intermediate values used. The store is indexed by a reference
//!   to the node type and so the computed results cannot be stored within the node itself (due to the borrow checker).
//!
//! Once the appropriate traits have been implmented, layout can be performed on the whole tree, e.g.
//! ```no_run
//! layout(&mut world.cache, &world.tree, &world.store);
//! ```
//! In this example the cache, tree, and a store for the node properties are kept in an ECS world struct and a node is simply an entity id.
//!
//! See examples for details.
//!
//! # Layout system description
//!
//! TODO

// pub mod cache;
// pub use cache::*;

pub mod types;
pub use types::*;

// pub mod node;
// pub use node::*;

// pub mod hierarchy;
// pub use hierarchy::*;

// pub mod layout;
// pub use layout::*;

pub mod cache2;
pub use cache2::*;

pub mod node2;
pub use node2::*;

pub mod layout2;
pub use layout2::*;
