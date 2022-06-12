use crate::node::Node;

/// Describes how the visual tree of nodes can be iterated through for
/// the layout engine.
///
/// You need to get the nodes of the layout in depth first order (tree pre order).
/// Thats what you usually get when you just walk down the tree in a recursive
/// naive manner.
///
/// The layout algorithm also requires the reverse of that order, to walk
/// upwards the tree from the leaf nodes. The easiest to achieve that is by
/// collecting your nodes in a vector by recursively walking through
/// your widget tree.
///
/// Here is a quick example on how to implement this trait:
///
///```
/// use morphorm::{Node, Hierarchy};
///
/// #[derive(Debug, Clone, Copy)]
/// pub struct Id(usize);
///
/// impl Node<'_> for Id {
///     type Data = ();
/// }
///
/// struct MyWidgetHierarchy {
///     widgets_in_dfs_pre_order: Vec<Id>,
/// }
///
/// impl<'a> Hierarchy<'a> for MyWidgetHierarchy {
///     type Item = Id;
///
///     fn up_iter<F: FnMut(Self::Item)>(&'a self, mut f: F) {
///         for id in self.widgets_in_dfs_pre_order.iter().rev() {
///             (f)(*id);
///         }
///     }
///
///     fn down_iter<F: FnMut(Self::Item)>(&'a self, mut f: F) {
///         for id in self.widgets_in_dfs_pre_order.iter() {
///             (f)(*id);
///         }
///     }
///
///     fn child_iter<F: FnMut(Self::Item)>(&'a self, node: Self::Item, mut f: F) {
///         // Rough example:
///         // let widget = self.store.get(node);
///
///         // widget.for_each_child(|child_widget_id| {
///         //     (f)(child_widget_id);
///         // });
///     }
///
///     fn parent(&self, node: Self::Item) -> Option<Self::Item> {
///         None // left as an exercise to the student ;-)
///     }
///
///     fn is_first_child(&self, node: Self::Item) -> bool {
///         false // left as an exercise to the student ;-)
///     }
///
///     fn is_last_child(&self, node: Self::Item) -> bool {
///         false // left as an exercise to the student ;-)
///     }
/// }
///```
pub trait Hierarchy<'a> {
    /// A type respresenting a node in the visual tree
    type Item: 'a + for<'b> Node<'b>;

    /// Iterates upwards the hierarchy, the exact reverse of [Hierarchy::down_iter].
    fn up_iter<F: FnMut(Self::Item)>(&'a self, f: F);

    /// Iterates downward the hierarchy, depth first search with pre-order.
    fn down_iter<F: FnMut(Self::Item)>(&'a self, f: F);

    /// Iterates through the child of `node`.
    fn child_iter<F: FnMut(Self::Item)>(&'a self, node: Self::Item, f: F);

    /// Get the parent node of the specified node
    fn parent(&self, node: Self::Item) -> Option<Self::Item>;

    /// Returns true if the specified node is the first child of its parent
    fn is_first_child(&self, node: Self::Item) -> bool;

    /// Returns true if the specified node is the last child of its parent
    fn is_last_child(&self, node: Self::Item) -> bool;
}
