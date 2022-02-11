# Morphorm

Morphorm is a crate for determining the position and size of UI elements which are organised in a visual tree.
The algorithm is written from scratch but is based on the [subform layout system](https://subformapp.com/articles/why-not-flexbox/).

# How to use
To try and keep things as generic as possible Morphorm does not provide any containers for representing the layout properties or visual tree.
Instead, three traits must be implemented by the users' containers in order to utilise the layout algorithm:

 - `Node` represents a UI element which can be sized and positioned. The node itself could contain the desired layout properties, or the properties
    can be provided by an external source (such as an ECS component store), which is provided by the `Data` associated type.
 - `Hierarchy` represents the visual tree of nodes. Morphorm requires three iterators to be provided: an upward iterator which iterates the tree
    from bottom to top in depth first order; a downward iterator which iterates the tree from the root in depth first order; and a child iterator
    which iterates through the children of a specified node.
 - `Cache` represents a store for the output of the layout computation as well as intermediate values used. The store is indexed by a reference
    to the node type and so the computed results cannot be stored within the node itself (due to the borrow checker).
 
Once the appropriate traits have been implmented, layout can be performed on the whole tree, e.g.
```rs
layout(&mut world.cache, &world.tree, &world.store);
```
In this example the cache, tree, and a store for the node properties are kept in an ECS world struct and a node is simply an entity id.

See examples for details.

# Layout system description
An overview of stacks can be found here: [overview (stacks)](https://geom3trik.github.io/tuix-book/section_2_layout/chapter_1.html)

## Layout Type
The layout type of an element determines whether its children should be arranged into a vertical stack (column), a horizontal stack (row), or a grid.

## Position Type
The position type of an element determines whether it should be arranged by the parent with its siblings (parent-directed) or whether it should arrange itself relative to the top-left corner of its parent and ignore its siblings (self-directed).

## Units
Both space and size are expressed in `Units`, which is an enumeration of 4 possible variants:
### Pixels
Specifies a number of pixels for space or size.

### Percentage
Specifies the space or size to be a fraction of the parent size.

This is direction dependent. For example, specifying `left: 30%` will be a percentage of the parents width, while `top: 30%` will be a percentage of the parents height.

### Stretch 
Specifies the space or size as a ratio of the remaining free space.

The ratio is calculated as the stretch value of an element to the sum of the stretch values of all flexible elements. For example, two children with a stretch width of 1 would each have a ratio of 1:2 and thus stretch to fill half of the remaining free space.

The remaining free space is the space left after subtracting the sizes of the fixed-sized children from the size of the parent. For example, an element with a width of 300 pixels and a single child with width 100 pixels would have 200 pixels of remaining free space.

### Auto
Specifies that the space or size should be automatically determined from either the parent or the children of an element. See descriptions of space and size below for details.

## Space
The position of an element is determined by the space applied to each side of its bounding box. Space can be added to the `left`, `right`, `top` and `bottom` of an element and is expressed in `Units`. This is conceptually the same as applying margins to an element.

Auto spacing causes the space around an element to be determined by the child-space of the parent.

## Child Space
To achieve padding between the parent and its children the child space property can be used and is expressed in `Units`. This property applies the specified space to the child elements if their individual space is specified as `Auto`.

For example, a vertical stack with 3 children can apply 10 pixels of padding by setting the `child-left`, `child-right`, `child-top`, and `child-bottom` properties to `Units::Pixels(10.0)`, and by setting the `left`, `right`, `top`, and `bottom` properties of each child to `Auto`.

## Size
The size of an element is determined by specifying the `width` and `height` of an element and is expressed in `Units`.

Auto size causes the size of an element to be determined by its children. For example, for a vertical stack, the height would be the sum of the heights of the child elements, while the width would be the max of the widths of the child elements, both of which include any spacing on the child elements. 

// Image here

## Constraints
Both space and size can have minimum and maximum constraints applied and are expressed in `Units`.

// Image here


## Grid
It was mentioned before that an element can use a grid layout by setting its `LAyoutType` property to `Grid`. Then, to position child elements within the grid, the index and span, for row and column, should be specified.

// Image here
