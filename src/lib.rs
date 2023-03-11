//! # Morphorm
//!
//! Morphorm is a crate for determining the position and size of UI elements which are organised in a visual tree.
//! The algorithm is written from scratch but is based on the [subform layout system](https://subformapp.com/articles/why-not-flexbox/).
//!
//! # How to use
//!
//! To try and keep things as generic as possible Morphorm does not provide any containers for representing the layout properties or visual tree.
//! Instead, two traits must be implemented by the users' containers in order to utilise the layout algorithm:
//!
//! - `Node` represents a UI element which can be sized and positioned. The node itself could contain the desired layout properties, or the properties
//!   can be provided by an external source (such as an ECS component store), which is provided by the `Store` associated type.
//!
//! - `Cache` represents a store for the output of the layout computation. The store is indexed by a key which is represented by the `CacheKey`
//! associated type on the `Node` trait.
//!
//! Once the appropriate traits have been implmented, layout can be performed on a particular node, recursing depth first down the visual tree, e.g.
//! ```no_run
//! layout(&root, LayoutType::Column, 600.0, 600.0, &mut cache, &tree, &store);
//! ```
//! In this example the cache, tree, and a store for the node properties are kept in an ECS world struct and a node is simply an entity id.
//!
//! See examples for details.
//!
//! # Layout system description
//!
//! TODO

pub mod types;
pub use types::*;

pub mod util;
pub use util::*;

pub mod cache;
pub use cache::*;

pub mod node;
pub use node::*;

mod layout;
use layout::layout;
