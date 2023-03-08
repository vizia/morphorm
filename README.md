# Morphorm

Morphorm is a crate for determining the size and position of UI elements which are organised into a tree structure.

## Description

### Layout Type
The layout type property determines how children of a node will be arranged. There are two variants:
- LayoutType::Row - The node will arrange its children into a horizontal row.
- LayoutType::Column - The node will arrange its children into a vertical column.

### Position Type
The position type property determines whether a node should be positioned in-line with its siblings in a stack, or out-of-line and independently of its siblings. There are two variants:
- PositionType::ParentDirected - The node will be positioned relative to its in-line position with its siblings.
- PositionType::SelfDirected - The node will be positioned out-of-line and relative to the top-left corner of its parent.

### Space
The position of a node within a stack can be adjusted by the spacing applied to each of its four sides: left, top, right, bottom.
Spacing is specified with `Units`, which has four variants:
- Units::Pixels(val) - Sets the spacing to a fixed number of pixels.
- Units::Percentage(val) - Sets the spacing to a percentage of the nodes parent size.
- Units::Stretch(factor) - Sets the spacing to a proportion of the free space of the parent within the same axis.
- Units::Auto - Sets the spacing to inherit the child spacing of the parent.

### Child Space
The child space of a node is used to override the individual auto spacing of the nodes children and is akin to padding.

### Size
The size of a node is determined by its width and height properties, which are also specified with `Units`:
- Units::Pixels(val) - Sets the size to a fixed number of pixels.
- Units::Percentage(val) - Sets the size to a percentage of the nodes parent size.
- Units::Stretch(factor) - Sets the size to a proportion of the free space of the parent within the same axis.
- Units::Auto - Sets the size to either hug the nodes children, or to inherit the content size of the node.

### Content Size
Content size is used to determine the size of a node which has no children but may have an intrinsic size due to contents which do not correspond to nodes in the layout tree. For example, a node which contains text has an intrinsic size of the bounds of the text, which may introduce a dependency between the width and height (i.e. when text wraps). Similarly, content size can be used to size a node with a particular aspect ratio by constraining the height to be some proportion of the width (or conversely).

### Constraints
All spacing and size properties can be constrained with corresponding minimum and maximum properties, which are also specified using `Units`. For example, the `width` of a node can be constrained with the `min_width` and `max_width` properties.

## List of Properties

| **Property**  | **Type**       | **Description**                                                                                                                                                                                                        |
|---------------|----------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| layout-type   | `LayoutType`   | Determines whether the container should arrange its children into a vertical stack or horizontal stack.                                                                                                                |
| position-type | `PositionType` | Determines whether the element should be positioned relative to its stack position, or relative to the parent's top-left corner.                                                                                       |
| width         | `Units`        | The desired width of a node.                                                                                                                                                                                           |
| height        | `Units`        | The desired height of a node.                                                                                                                                                                                          |
| left          | `Units`        | The space that should be applied to the left side of a node.                                                                                                                                                           |
| right         | `Units`        | The space that should be applied to the right side of a node.                                                                                                                                                          |
| top           | `Units`        | The space that should be applied to the top side of (above) a node.                                                                                                                                                    |
| bottom        | `Units`        | The space that should be applied to the bottom side of (below) a node.                                                                                                                                                 |
| child-left    | `Units`        | The space that should be applied between the left side of the view and its children with individual `Auto` left spacing. Applies to all children in a vertical stack and to the first child in a horizontal stack.     |
| chld-right    | `Units`        | The space that should be applied between the right side of the view and its children with individual `Auto` right spacing. Applies to all children in a vertical stack and to the first child in a horizontal stack.   |
| chld-top      | `Units`        | The space that should be applied between the top side of the view and its children with individual `Auto` top spacing. Applies to all children in a horizontal stack and to the first child in a vertical stack.       |
| child-bottom  | `Units`        | The space that should be applied between the bottom side of the view and its children with individual `Auto` bottom spacing. Applies to all children in a horizontal stack and to the first child in a vertical stack. |
|               |                |                                                                                                                                                                                                                        |

## How to use

To try and keep things as generic as possible Morphorm does not provide any containers for representing the layout properties or the tree.
Instead, two traits must be implemented by the users' containers in order to utilise the layout algorithm:

 - `Node` represents a UI element which can be sized and positioned. The node itself could contain the desired layout properties, or the properties can be provided by an external source (such as an ECS component store), which is provided by the `Store` associated type. The node must also provide an iterator over its children, specified by the `ChildIter` associated type, and to allow the children to be stored externally as well, there is a `Tree` associated type.
 - `Cache` represents a store for the output of the layout computation. The store is indexed by a reference to the node type, however, to allow store types which cannot use the node reference as a key, the `Node` trait provides a `CacheKey` associated type.
 
Layout can then be performed on the whole tree, e.g.
```rs
layout(&root, None, 600.0, 600.0, &mut cache, &tree, &store);
```
In this example the cache, tree, and a store for the node properties are kept in an ECS world struct and a node is simply an entity id.

See examples for details.