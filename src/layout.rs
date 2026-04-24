use smallvec::SmallVec;

use crate::{
    Alignment, Cache, CacheExt, Direction, LayoutType, LayoutWrap, Node, NodeExt, PositionType, Size, Units, Units::*,
};

const DEFAULT_MIN: f32 = -f32::MAX;
const DEFAULT_MAX: f32 = f32::MAX;
const DEFAULT_BORDER_WIDTH: f32 = 0.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ItemType {
    Size,
    After,
}

/// Represents a space or size which has stretch units.
#[derive(Copy, Clone)]
struct StretchItem {
    // The child index of the item.
    index: usize,
    // The stretch factor of the item.
    factor: f32,
    // The type of stretch item, either space-before, size, or space-after.
    item_type: ItemType,
    // The violation of the stretch item after clamping.
    violation: f32,
    // The computed size of the stretch item.
    computed: f32,
    // The measured size from the current flex iteration.
    measured: f32,
    // Whether or not the stretch item is frozen.
    frozen: bool,
    // The minimum size of the stretch item.
    min: f32,
    // The maximum size of the stretch item.
    max: f32,
}

impl StretchItem {
    pub fn new(index: usize, factor: f32, item_type: ItemType, min: f32, max: f32) -> Self {
        Self { index, factor, item_type, violation: 0.0, computed: 0.0, measured: 0.0, frozen: false, min, max }
    }
}

#[derive(Debug, Copy, Clone)]
struct ChildNode<'a, N: Node> {
    // A reference to the node.
    node: &'a N,
    // Computed cross size of the node.
    cross: f32,
    // Computed main size of the node.
    main: f32,

    main_after: f32,
    // Last parent constraints used to lay out this child.
    last_layout_main: f32,
    last_layout_cross: f32,
    has_layout_constraints: bool,
}

fn flip_alignment_horizontal(alignment: Alignment) -> Alignment {
    match alignment {
        Alignment::TopLeft => Alignment::TopRight,
        Alignment::TopRight => Alignment::TopLeft,
        Alignment::Left => Alignment::Right,
        Alignment::Right => Alignment::Left,
        Alignment::BottomLeft => Alignment::BottomRight,
        Alignment::BottomRight => Alignment::BottomLeft,
        alignment => alignment,
    }
}

#[inline]
fn same_f32(a: f32, b: f32) -> bool {
    a.to_bits() == b.to_bits()
}

fn alignment_fractions(alignment: Alignment) -> (f32, f32) {
    // Convert alignment into normalized horizontal/vertical fractions in [0, 1].
    // These fractions are later multiplied by available free space.
    match alignment {
        Alignment::TopLeft => (0.0, 0.0),
        Alignment::TopCenter => (0.5, 0.0),
        Alignment::TopRight => (1.0, 0.0),
        Alignment::Left => (0.0, 0.5),
        Alignment::Center => (0.5, 0.5),
        Alignment::Right => (1.0, 0.5),
        Alignment::BottomLeft => (0.0, 1.0),
        Alignment::BottomCenter => (0.5, 1.0),
        Alignment::BottomRight => (1.0, 1.0),
    }
}

fn absolute_axis_position(before: Units, after: Units, parent_size: f32, child_size: f32) -> f32 {
    // Resolve a child position on one axis from before/after offsets.
    // This is shared by absolute positioning in stack and overlay layouts.
    match (before, after) {
        (Pixels(val), _) => val,
        (Percentage(val), _) => val * 0.01 * parent_size,
        (_, Pixels(val)) => parent_size - val - child_size,
        (_, Percentage(val)) => parent_size - child_size - val * 0.01 * parent_size,
        (Stretch(b), Stretch(a)) => {
            if b == a {
                (parent_size - child_size) * 0.5
            } else {
                (parent_size - child_size) * (b / (b + a))
            }
        }
        (Stretch(_), Auto) => parent_size - child_size,
        (Auto, Stretch(_)) => 0.0,
        (Auto, Auto) => 0.0,
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn layout_overlay<N, C>(
    node: &N,
    parent_layout_type: LayoutType,
    parent_main: f32,
    parent_cross: f32,
    cache: &mut C,
    tree: &<N as Node>::Tree,
    store: &<N as Node>::Store,
    sublayout: &mut <N as Node>::SubLayout<'_>,
) -> Size
where
    N: Node,
    C: Cache<Node = N>,
{
    // Interpret parent-provided main/cross as concrete width/height for overlay.
    let (mut computed_width, mut computed_height) = match parent_layout_type {
        LayoutType::Column => (parent_cross, parent_main),
        LayoutType::Row | LayoutType::Overlay | LayoutType::Grid => (parent_main, parent_cross),
    };

    // Resolve this node's own size constraints in physical width/height axes.
    let width = node.width(store).unwrap_or(Stretch(1.0));
    let height = node.height(store).unwrap_or(Stretch(1.0));
    let mut min_width = node.min_width(store).unwrap_or(Pixels(0.0)).to_px(computed_width, DEFAULT_MIN);
    let mut max_width = node.max_width(store).unwrap_or(Pixels(f32::MAX)).to_px(computed_width, DEFAULT_MAX);
    let mut min_height = node.min_height(store).unwrap_or(Pixels(0.0)).to_px(computed_height, DEFAULT_MIN);
    let mut max_height = node.max_height(store).unwrap_or(Pixels(f32::MAX)).to_px(computed_height, DEFAULT_MAX);

    let border_left = node.border_left(store).unwrap_or_default().to_px(computed_width, DEFAULT_BORDER_WIDTH);
    let border_right = node.border_right(store).unwrap_or_default().to_px(computed_width, DEFAULT_BORDER_WIDTH);
    let border_top = node.border_top(store).unwrap_or_default().to_px(computed_height, DEFAULT_BORDER_WIDTH);
    let border_bottom = node.border_bottom(store).unwrap_or_default().to_px(computed_height, DEFAULT_BORDER_WIDTH);

    let padding_left = node.padding_left(store).unwrap_or_default().to_px(computed_width, 0.0);
    let padding_right = node.padding_right(store).unwrap_or_default().to_px(computed_width, 0.0);
    let padding_top = node.padding_top(store).unwrap_or_default().to_px(computed_height, 0.0);
    let padding_bottom = node.padding_bottom(store).unwrap_or_default().to_px(computed_height, 0.0);

    // Split visible children by position type; relative children participate in
    // overlay alignment, absolute children keep explicit edge-based positioning.
    let mut relative_children = SmallVec::<[&N; 32]>::new();
    let mut absolute_children = SmallVec::<[&N; 8]>::new();
    for child in node.children(tree).filter(|child| child.visible(store)) {
        match child.position_type(store).unwrap_or_default() {
            PositionType::Relative => relative_children.push(child),
            PositionType::Absolute => absolute_children.push(child),
        }
    }

    let num_children = relative_children.len() + absolute_children.len();
    let num_relative_children = relative_children.len();

    let mut children = SmallVec::<[ChildNode<N>; 32]>::with_capacity(num_children);

    // Two-pass stabilization:
    // pass 1 measures children with initial size,
    // pass 2 remeasures only if auto/min/max constraints changed the container size.
    for _ in 0..2 {
        children.clear();

        // Relative children are laid out against the parent content box.
        let available_width = computed_width - padding_left - padding_right - border_left - border_right;
        let available_height = computed_height - padding_top - padding_bottom - border_top - border_bottom;

        for child in relative_children.iter().copied() {
            let child_width = child.width(store).unwrap_or(Stretch(1.0));
            let child_height = child.height(store).unwrap_or(Stretch(1.0));

            let child_min_width = child.min_width(store).unwrap_or(Pixels(0.0));
            let child_max_width = child.max_width(store).unwrap_or(Pixels(f32::MAX));
            let child_min_height = child.min_height(store).unwrap_or(Pixels(0.0));
            let child_max_height = child.max_height(store).unwrap_or(Pixels(f32::MAX));

            // Stretch children are constrained directly to available space,
            // while non-stretch children are measured against that space.
            let target_width = if child_width.is_stretch() {
                available_width.clamp(
                    child_min_width.to_px(available_width, DEFAULT_MIN),
                    child_max_width.to_px(available_width, DEFAULT_MAX),
                )
            } else {
                available_width
            };

            let target_height = if child_height.is_stretch() {
                available_height.clamp(
                    child_min_height.to_px(available_height, DEFAULT_MIN),
                    child_max_height.to_px(available_height, DEFAULT_MAX),
                )
            } else {
                available_height
            };

            // Overlay children recurse with Overlay parent semantics so descendants
            // inherit overlay axis behavior when needed.
            let child_size =
                layout(child, LayoutType::Overlay, target_width, target_height, cache, tree, store, sublayout);

            children.push(ChildNode {
                node: child,
                cross: child_size.cross,
                main: child_size.main,
                main_after: 0.0,
                last_layout_main: target_width,
                last_layout_cross: target_height,
                has_layout_constraints: true,
            });
        }

        if num_relative_children == 0 {
            break;
        }

        let max_child_width = children.iter().map(|child| child.main).reduce(f32::max).unwrap_or_default();
        let max_child_height = children.iter().map(|child| child.cross).reduce(f32::max).unwrap_or_default();

        // Auto-size in overlay is based on max extents (not sums), because
        // children can overlap and are independently aligned in the same box.
        if width.is_auto() || node.min_width(store).unwrap_or(Pixels(0.0)).is_auto() {
            min_width = max_child_width + padding_left + padding_right + border_left + border_right;
        }

        if node.max_width(store).unwrap_or(Pixels(f32::MAX)).is_auto() && max_child_width != 0.0 {
            max_width = max_child_width + padding_left + padding_right + border_left + border_right;
        }

        if height.is_auto() || node.min_height(store).unwrap_or(Pixels(0.0)).is_auto() {
            min_height = max_child_height + padding_top + padding_bottom + border_top + border_bottom;
        }

        if node.max_height(store).unwrap_or(Pixels(f32::MAX)).is_auto() && max_child_height != 0.0 {
            max_height = max_child_height + padding_top + padding_bottom + border_top + border_bottom;
        }

        let next_width = computed_width.max(min_width).min(max_width);
        let next_height = computed_height.max(min_height).min(max_height);

        if same_f32(next_width, computed_width) && same_f32(next_height, computed_height) {
            break;
        }

        computed_width = next_width;
        computed_height = next_height;
    }

    // Final clamped container size after stabilization.
    computed_width = computed_width.max(min_width).min(max_width);
    computed_height = computed_height.max(min_height).min(max_height);

    // Relative children are positioned inside the padded content box.
    let available_width = computed_width - padding_left - padding_right - border_left - border_right;
    let available_height = computed_height - padding_top - padding_bottom - border_top - border_bottom;

    let mut alignment = node.alignment(store).unwrap_or_default();
    if node.direction(store).unwrap_or_default() == Direction::RightToLeft {
        alignment = flip_alignment_horizontal(alignment);
    }
    // Alignment gives each child its own anchor point in the same content box,
    // which is what enables intentional overlap.
    let (align_x, align_y) = alignment_fractions(alignment);

    for child in &children {
        let mut child_posx = align_x * (available_width - child.main);
        let mut child_posy = align_y * (available_height - child.cross);

        // Parent scroll offsets can override alignment-derived positions.
        if let Some(scroll_x) = node.horizontal_scroll(store) {
            child_posx = scroll_x;
        }

        if let Some(scroll_y) = node.vertical_scroll(store) {
            child_posy = scroll_y;
        }

        cache.set_rect(
            child.node,
            LayoutType::Overlay,
            child_posx + padding_left + border_left,
            child_posy + padding_top + border_top,
            child.main,
            child.cross,
        );
    }

    // Absolute children are sized in the same box model used by stack/wrap:
    // padding box (content + padding), excluding border.
    let abs_width = computed_width - border_left - border_right;
    let abs_height = computed_height - border_top - border_bottom;

    for child in absolute_children.into_iter() {
        // Stretch sizing for absolute children consumes remaining axis size after offsets.
        let child_width = if child.width(store).unwrap_or(Stretch(1.0)).is_stretch() {
            let child_min_width = child.min_width(store).unwrap_or(Pixels(0.0)).to_px(abs_width, DEFAULT_MIN);
            let child_max_width = child.max_width(store).unwrap_or(Pixels(f32::MAX)).to_px(abs_width, DEFAULT_MAX);
            let child_left = child.left(store).unwrap_or_default().to_px(abs_width, 0.0);
            let child_right = child.right(store).unwrap_or_default().to_px(abs_width, 0.0);

            abs_width.clamp(child_min_width, child_max_width) - child_left - child_right
        } else {
            abs_width
        };

        let child_height = if child.height(store).unwrap_or(Stretch(1.0)).is_stretch() {
            let child_min_height = child.min_height(store).unwrap_or(Pixels(0.0)).to_px(abs_height, DEFAULT_MIN);
            let child_max_height = child.max_height(store).unwrap_or(Pixels(f32::MAX)).to_px(abs_height, DEFAULT_MAX);
            let child_top = child.top(store).unwrap_or_default().to_px(abs_height, 0.0);
            let child_bottom = child.bottom(store).unwrap_or_default().to_px(abs_height, 0.0);

            abs_height.clamp(child_min_height, child_max_height) - child_top - child_bottom
        } else {
            abs_height
        };

        // Recurse first to resolve intrinsic/auto behavior under the resolved constraints.
        let child_size = layout(child, LayoutType::Overlay, child_width, child_height, cache, tree, store, sublayout);

        // Then resolve explicit absolute offsets for final placement.
        let child_posx = absolute_axis_position(
            child.left(store).unwrap_or_default(),
            child.right(store).unwrap_or_default(),
            abs_width,
            child_size.main,
        );
        let child_posy = absolute_axis_position(
            child.top(store).unwrap_or_default(),
            child.bottom(store).unwrap_or_default(),
            abs_height,
            child_size.cross,
        );

        cache.set_rect(
            child,
            LayoutType::Overlay,
            child_posx + border_left,
            child_posy + border_top,
            child_size.main,
            child_size.cross,
        );
    }

    // Return in caller's axis orientation (main/cross abstraction).
    match parent_layout_type {
        LayoutType::Column => Size { main: computed_height, cross: computed_width },
        LayoutType::Row | LayoutType::Overlay | LayoutType::Grid => {
            Size { main: computed_width, cross: computed_height }
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn layout_grid<N, C>(
    node: &N,
    parent_layout_type: LayoutType,
    parent_main: f32,
    parent_cross: f32,
    cache: &mut C,
    tree: &<N as Node>::Tree,
    store: &<N as Node>::Store,
    sublayout: &mut <N as Node>::SubLayout<'_>,
) -> Size
where
    N: Node,
    C: Cache<Node = N>,
{
    let computed_main = parent_main;
    let computed_cross = parent_cross;

    let (mut parent_width, mut parent_height) = match parent_layout_type {
        LayoutType::Column => (parent_cross, parent_main),
        LayoutType::Row | LayoutType::Overlay | LayoutType::Grid => (parent_main, parent_cross),
    };

    let padding_left = node.padding_left(store).unwrap_or_default().to_px(parent_width, 0.0);
    let padding_right = node.padding_right(store).unwrap_or_default().to_px(parent_width, 0.0);
    let padding_top = node.padding_top(store).unwrap_or_default().to_px(parent_height, 0.0);
    let padding_bottom = node.padding_bottom(store).unwrap_or_default().to_px(parent_height, 0.0);

    parent_width -= padding_left + padding_right;
    parent_height -= padding_top + padding_bottom;

    let grid_cols = node.grid_columns(store).unwrap_or_default();
    let grid_rows = node.grid_rows(store).unwrap_or_default();

    let mut computed_grid_cols = vec![0.0; 2 * grid_cols.len() + 2];
    let mut computed_grid_rows = vec![0.0; 2 * grid_rows.len() + 2];

    let horizontal_gap = node.horizontal_gap(store).unwrap_or_default();
    let vertical_gap = node.vertical_gap(store).unwrap_or_default();

    // Sum of all space and size flex factors on the col-axis of the node.
    let mut col_flex_sum = 0.0;

    // List of stretch nodes for the col axis.
    let mut col_axis = SmallVec::<[StretchItem; 32]>::new();

    // Sum of all space and size flex factors on the row-axis of the node.
    let mut row_flex_sum = 0.0;

    // List of stretch nodes for the row axis.
    let mut row_axis = SmallVec::<[StretchItem; 32]>::new();

    for (i, col) in grid_cols.iter().enumerate() {
        let idx = 2 * i + 1;
        computed_grid_cols[idx] = col.to_px(parent_width, 0.0);

        if let Stretch(val) = col {
            col_flex_sum += val;
            col_axis.push(StretchItem::new(idx, *val, ItemType::Size, 0.0, 1000.0));
        }

        if i < grid_cols.len() - 1 {
            let gutter_idx = 2 * i + 2;
            computed_grid_cols[gutter_idx] = horizontal_gap.to_px(parent_width, 0.0);

            if let Stretch(val) = horizontal_gap {
                col_flex_sum += val;
                col_axis.push(StretchItem::new(gutter_idx, val, ItemType::Size, 0.0, 1000.0));
            }
        }
    }

    for (i, row) in grid_rows.iter().enumerate() {
        let idx = 2 * i + 1;
        computed_grid_rows[idx] = row.to_px(parent_height, 0.0);

        if let Stretch(val) = row {
            row_flex_sum += val;
            row_axis.push(StretchItem::new(idx, *val, ItemType::Size, 0.0, 1000.0));
        }

        if i < grid_rows.len() - 1 {
            let gutter_idx = 2 * i + 2;
            computed_grid_rows[gutter_idx] = vertical_gap.to_px(parent_height, 0.0);

            if let Stretch(val) = vertical_gap {
                row_flex_sum += val;
                row_axis.push(StretchItem::new(gutter_idx, val, ItemType::Size, 0.0, 1000.0));
            }
        }
    }

    let mut width_sum: f32 = computed_grid_cols.iter().sum();
    let mut height_sum: f32 = computed_grid_rows.iter().sum();

    if !col_axis.is_empty() {
        loop {
            // If all stretch items are frozen, exit the loop.
            if col_axis.iter().all(|item| item.frozen) {
                break;
            }

            // Calculate free space on the main-axis.
            let free_col_space = parent_width - width_sum;

            let mut total_violation = 0.0;

            for item in col_axis.iter_mut().filter(|item| !item.frozen) {
                let actual_main = (item.factor * free_col_space / col_flex_sum).round();

                let clamped = actual_main.min(item.max).max(item.min);
                item.violation = clamped - actual_main;
                total_violation += item.violation;
                item.measured = actual_main;
                item.computed = clamped;
            }

            for item in col_axis.iter_mut().filter(|item| !item.frozen) {
                // Freeze over-stretched items.
                item.frozen = match total_violation {
                    total if total > 0.0 => item.violation > 0.0,
                    total if total < 0.0 => item.violation < 0.0,
                    _ => true,
                };

                // If the item is frozen, adjust the used_space and sum of cross stretch factors.
                if item.frozen {
                    col_flex_sum -= item.factor;
                    let prev = computed_grid_cols[item.index];
                    computed_grid_cols[item.index] = item.computed;
                    width_sum += item.computed - prev;
                }
            }
        }
    }

    if !row_axis.is_empty() {
        loop {
            // If all stretch items are frozen, exit the loop.
            if row_axis.iter().all(|item| item.frozen) {
                break;
            }

            // Calculate free space on the main-axis.
            let free_row_space = parent_height - height_sum;

            let mut total_violation = 0.0;

            for item in row_axis.iter_mut().filter(|item| !item.frozen) {
                let actual_main = (item.factor * free_row_space / row_flex_sum).round();

                let clamped = actual_main.min(item.max).max(item.min);
                item.violation = clamped - actual_main;
                total_violation += item.violation;
                item.measured = actual_main;
                item.computed = clamped;
            }

            for item in row_axis.iter_mut().filter(|item| !item.frozen) {
                // Freeze over-stretched items.
                item.frozen = match total_violation {
                    total if total > 0.0 => item.violation > 0.0,
                    total if total < 0.0 => item.violation < 0.0,
                    _ => true,
                };

                // If the item is frozen, adjust the used_space and sum of cross stretch factors.
                if item.frozen {
                    row_flex_sum -= item.factor;
                    let prev = computed_grid_rows[item.index];
                    computed_grid_rows[item.index] = item.computed;
                    height_sum += item.computed - prev;
                }
            }
        }
    }

    // println!("{:?} {:?}", computed_grid_cols, computed_grid_rows);

    let mut current_col_pos = 0.0;
    for col in &mut computed_grid_cols {
        current_col_pos += *col;
        *col = current_col_pos;
    }

    let mut current_row_pos = 0.0;
    for row in &mut computed_grid_rows {
        current_row_pos += *row;
        *row = current_row_pos;
    }

    // println!("{:?} {:?}", computed_grid_cols, computed_grid_rows);

    let mut alignment = node.alignment(store).unwrap_or_default();

    if node.direction(store).unwrap_or_default() == Direction::RightToLeft {
        alignment = flip_alignment_horizontal(alignment);
    }

    let (mut child_posx, mut child_posy) = match alignment {
        Alignment::TopLeft => (0.0, 0.0),
        Alignment::TopCenter => (0.0, 0.5),
        Alignment::TopRight => (0.0, 1.0),
        Alignment::Left => (0.5, 0.0),
        Alignment::Center => (0.5, 0.5),
        Alignment::Right => (0.5, 1.0),
        Alignment::BottomLeft => (1.0, 0.0),
        Alignment::BottomCenter => (1.0, 0.5),
        Alignment::BottomRight => (1.0, 1.0),
    };

    child_posx *= parent_width - width_sum;
    child_posy *= parent_height - height_sum;

    let node_children = node
        .children(tree)
        .filter(|child| child.visible(store))
        .filter(|child| child.position_type(store).unwrap_or_default() == PositionType::Relative);

    // Compute space and size of non-flexible relative children.
    for child in node_children {
        let column_start = 2 * child.column_start(store).unwrap_or_default();
        let column_span = 2 * child.column_span(store).unwrap_or(1) - 1;
        let column_end = column_start + column_span;

        let row_start = 2 * child.row_start(store).unwrap_or_default();
        let row_span = 2 * child.row_span(store).unwrap_or(1) - 1;
        let row_end = row_start + row_span;

        let posx = computed_grid_cols[column_start];
        let width = computed_grid_cols[column_end] - posx;

        let posy = computed_grid_rows[row_start];
        let height = computed_grid_rows[row_end] - posy;

        layout(child, LayoutType::Row, width, height, cache, tree, store, sublayout);

        cache.set_rect(
            child,
            LayoutType::Row,
            posx + padding_left + child_posx,
            posy + padding_top + child_posy,
            width,
            height,
        );
    }

    Size { main: computed_main, cross: computed_cross }
}

/// Performs wrapped layout on the given node, arranging children into multiple lines
/// along the main axis when they overflow the available space.
///
/// Called from [`layout`] when a node has [`LayoutWrap::Wrap`] set.
#[allow(clippy::too_many_arguments)]
pub(crate) fn layout_wrap<N, C>(
    node: &N,
    parent_layout_type: LayoutType,
    parent_main: f32,
    parent_cross: f32,
    cache: &mut C,
    tree: &<N as Node>::Tree,
    store: &<N as Node>::Store,
    sublayout: &mut <N as Node>::SubLayout<'_>,
) -> Size
where
    N: Node,
    C: Cache<Node = N>,
{
    let layout_type = node.layout_type(store).unwrap_or_default();

    // Convert parent-provided main/cross (which are in parent layout axes)
    // into this node's layout axes.
    let (parent_main, parent_cross) =
        if parent_layout_type == layout_type { (parent_main, parent_cross) } else { (parent_cross, parent_main) };

    let border_main_before = node.border_main_before(store, layout_type).to_px(parent_main, DEFAULT_BORDER_WIDTH);
    let border_main_after = node.border_main_after(store, layout_type).to_px(parent_main, DEFAULT_BORDER_WIDTH);
    let border_cross_before = node.border_cross_before(store, layout_type).to_px(parent_cross, DEFAULT_BORDER_WIDTH);
    let border_cross_after = node.border_cross_after(store, layout_type).to_px(parent_cross, DEFAULT_BORDER_WIDTH);

    let padding_main_before = node.padding_main_before(store, layout_type).to_px(parent_main, 0.0);
    let padding_main_after = node.padding_main_after(store, layout_type).to_px(parent_main, 0.0);
    let padding_cross_before = node.padding_cross_before(store, layout_type).to_px(parent_cross, 0.0);
    let padding_cross_after = node.padding_cross_after(store, layout_type).to_px(parent_cross, 0.0);

    // Available space for children after subtracting padding and border.
    let avail_main = parent_main - padding_main_before - padding_main_after - border_main_before - border_main_after;
    let avail_cross =
        parent_cross - padding_cross_before - padding_cross_after - border_cross_before - border_cross_after;

    // Gap between items within a line (on the main axis).
    let min_main_between = node.min_main_between(store, layout_type);
    let max_main_between = node.max_main_between(store, layout_type);
    let item_gap_px =
        node.main_between(store, layout_type).to_px_clamped(avail_main, 0.0, min_main_between, max_main_between);

    // Gap between lines (on the cross axis).
    let line_gap_px = node.cross_between(store, layout_type).to_px(avail_cross, 0.0);

    let is_inline_rtl = matches!(layout_type, LayoutType::Row | LayoutType::Column)
        && node.direction(store).unwrap_or_default() == Direction::RightToLeft;

    let relative_children = node
        .children(tree)
        .filter(|c| c.visible(store))
        .filter(|c| c.position_type(store).unwrap_or_default() == PositionType::Relative)
        .collect::<SmallVec<[&N; 32]>>();

    let num_rel = relative_children.len();

    // Per-item data used during layout.
    struct WrapItem {
        main: f32,
        cross: f32,
        /// Non-zero when this item has Stretch units on the main axis.
        stretch_main_factor: f32,
        cross_is_stretch: bool,
        min_main: f32,
        max_main: f32,
        min_cross: f32,
        max_cross: f32,
    }

    // Phase 1: Compute sizes for all relative children.
    // Stretch-main items are deferred; their sizes are resolved per-line in phase 3.
    let mut items: SmallVec<[WrapItem; 32]> = SmallVec::with_capacity(num_rel);
    for child in relative_children.iter() {
        let child_main_units = child.main(store, layout_type);
        let child_min_main = child.min_main(store, layout_type);
        let child_max_main = child.max_main(store, layout_type);
        let child_min_cross = child.min_cross(store, layout_type);
        let child_max_cross = child.max_cross(store, layout_type);
        let child_cross_is_stretch = child.cross(store, layout_type).is_stretch();

        let min_main_px = child_min_main.to_px(avail_main, DEFAULT_MIN);
        let max_main_px = child_max_main.to_px(avail_main, DEFAULT_MAX);
        let min_cross_px = child_min_cross.to_px(avail_cross, DEFAULT_MIN);
        let max_cross_px = child_max_cross.to_px(avail_cross, DEFAULT_MAX);

        if let Stretch(factor) = child_main_units {
            // Use min size as base for line-break decisions; actual size resolved later.
            let base = min_main_px.max(0.0);
            items.push(WrapItem {
                main: base,
                cross: 0.0,
                stretch_main_factor: factor,
                cross_is_stretch: child_cross_is_stretch,
                min_main: min_main_px,
                max_main: max_main_px,
                min_cross: min_cross_px,
                max_cross: max_cross_px,
            });
        } else {
            let size = layout(*child, layout_type, avail_main, avail_cross, cache, tree, store, sublayout);
            items.push(WrapItem {
                main: size.main,
                cross: size.cross,
                stretch_main_factor: 0.0,
                cross_is_stretch: child_cross_is_stretch,
                min_main: min_main_px,
                max_main: max_main_px,
                min_cross: min_cross_px,
                max_cross: max_cross_px,
            });
        }
    }

    // Phase 2: Assign children to lines.
    // A new line begins when adding the next child would exceed avail_main.
    // Stretch-main children use their min contribution for break decisions.
    // If avail_main <= 0 (auto-width container), no breaks occur.
    let mut lines: SmallVec<[std::ops::Range<usize>; 8]> = SmallVec::new();
    if num_rel > 0 {
        let mut line_start = 0usize;
        let mut line_main_used = 0.0f32;
        let mut items_in_line = 0usize;

        for i in 0..num_rel {
            // Stretch items contribute their min size to the line-break decision (0 if no min set).
            let size_contribution = items[i].main;
            let gap_before = if items_in_line > 0 { item_gap_px } else { 0.0 };
            let projected = line_main_used + gap_before + size_contribution;

            if avail_main > 0.0 && items_in_line > 0 && projected > avail_main {
                // Finish current line and start a new one.
                lines.push(line_start..i);
                line_start = i;
                line_main_used = size_contribution;
                items_in_line = 1;
            } else {
                line_main_used = projected;
                items_in_line += 1;
            }
        }
        lines.push(line_start..num_rel);
    }

    // Phase 3: Per-line flex resolution for stretch-main items.
    for line in lines.iter() {
        let start = line.start;
        let end = line.end;
        let count = line.len();
        if count == 0 {
            continue;
        }

        let mut stretch_sum = 0.0f32;
        let mut fixed_sum = 0.0f32;
        for i in start..end {
            if items[i].stretch_main_factor > 0.0 {
                stretch_sum += items[i].stretch_main_factor;
            } else {
                fixed_sum += items[i].main;
            }
        }

        if stretch_sum > 0.0 {
            let gap_total = (count - 1) as f32 * item_gap_px;
            let free_main = (avail_main - fixed_sum - gap_total).max(0.0);

            for i in start..end {
                let factor = items[i].stretch_main_factor;
                if factor > 0.0 {
                    let allocated = (factor / stretch_sum * free_main).round();
                    let clamped = allocated.clamp(items[i].min_main, items[i].max_main);
                    let size =
                        layout(relative_children[i], layout_type, clamped, avail_cross, cache, tree, store, sublayout);
                    items[i].main = size.main;
                    items[i].cross = size.cross;
                }
            }
        }
    }

    // Phase 4: Compute the cross extent of each line from non-cross-stretch children.
    let mut line_cross: SmallVec<[f32; 8]> = SmallVec::with_capacity(lines.len());
    for line in lines.iter() {
        let start = line.start;
        let end = line.end;
        let mut max_cross = 0.0f32;
        for i in start..end {
            if !items[i].cross_is_stretch {
                max_cross = max_cross.max(items[i].cross);
            }
        }
        line_cross.push(max_cross);
    }

    // Phase 5: Resolve cross-stretch children to fill their line's cross extent.
    for (line_idx, line) in lines.iter().enumerate() {
        let start = line.start;
        let end = line.end;
        let lc = line_cross[line_idx];
        for i in start..end {
            if items[i].cross_is_stretch {
                let child = relative_children[i];
                let clamped_cross = lc.clamp(items[i].min_cross, items[i].max_cross);
                let size = layout(child, layout_type, items[i].main, clamped_cross, cache, tree, store, sublayout);
                items[i].main = size.main;
                items[i].cross = size.cross;
            }
        }
        // Re-compute line cross to include cross-stretch items in case they changed.
        let mut max_cross = 0.0f32;
        for i in start..end {
            max_cross = max_cross.max(items[i].cross);
        }
        line_cross[line_idx] = max_cross;
    }

    // Phase 6: Determine the final cross size of the container.
    let num_lines = lines.len();
    let total_content_cross = if num_lines > 0 {
        line_cross.iter().sum::<f32>() + (num_lines.saturating_sub(1)) as f32 * line_gap_px
    } else {
        0.0
    };

    let cross_units = node.cross(store, layout_type);
    let final_cross = if cross_units.is_auto() || parent_cross == 0.0 {
        let raw =
            total_content_cross + padding_cross_before + padding_cross_after + border_cross_before + border_cross_after;
        let min_c = node.min_cross(store, layout_type).to_px(0.0, DEFAULT_MIN);
        let max_c = node.max_cross(store, layout_type).to_px(0.0, DEFAULT_MAX);
        raw.max(min_c).min(max_c)
    } else {
        parent_cross
    };

    // Recompute auto main size (for containers with Auto main axis).
    let main_units = node.main(store, layout_type);
    let final_main = if main_units.is_auto() || parent_main == 0.0 {
        let raw = lines
            .iter()
            .map(|line| {
                let mut sum = 0.0f32;
                for i in line.start..line.end {
                    sum += items[i].main;
                }
                sum + (line.len().saturating_sub(1)) as f32 * item_gap_px
            })
            .fold(0.0f32, f32::max);
        let raw = raw + padding_main_before + padding_main_after + border_main_before + border_main_after;
        let min_m = node.min_main(store, layout_type).to_px(0.0, DEFAULT_MIN);
        let max_m = node.max_main(store, layout_type).to_px(0.0, DEFAULT_MAX);
        raw.max(min_m).min(max_m)
    } else {
        parent_main
    };

    // Phase 7: Lay out absolute children against the container bounds.
    // Absolute children are sized against the padding box (content box + padding, excluding border).
    let abs_avail_main = final_main - border_main_before - border_main_after;
    let abs_avail_cross = final_cross - border_cross_before - border_cross_after;

    let abs_children = node
        .children(tree)
        .filter(|c| c.position_type(store).unwrap_or_default() == PositionType::Absolute)
        .filter(|c| c.visible(store));

    let mut abs_items: SmallVec<[ChildNode<N>; 8]> = SmallVec::new();
    for child in abs_children {
        let main = if child.main(store, layout_type).is_stretch() {
            let child_min_main = child.min_main(store, layout_type).to_px(abs_avail_main, DEFAULT_MIN);
            let child_max_main = child.max_main(store, layout_type).to_px(abs_avail_main, DEFAULT_MAX);
            let main_before = child.main_before(store, layout_type).to_px(abs_avail_main, 0.0);
            let main_after = child.main_after(store, layout_type).to_px(abs_avail_main, 0.0);
            abs_avail_main.clamp(child_min_main, child_max_main) - main_before - main_after
        } else {
            abs_avail_main
        };

        let cross = if child.cross(store, layout_type).is_stretch() {
            let child_min_cross = child.min_cross(store, layout_type).to_px(abs_avail_cross, DEFAULT_MIN);
            let child_max_cross = child.max_cross(store, layout_type).to_px(abs_avail_cross, DEFAULT_MAX);
            let cross_before = child.cross_before(store, layout_type).to_px(abs_avail_cross, 0.0);
            let cross_after = child.cross_after(store, layout_type).to_px(abs_avail_cross, 0.0);
            abs_avail_cross.clamp(child_min_cross, child_max_cross) - cross_before - cross_after
        } else {
            abs_avail_cross
        };

        let size = layout(child, layout_type, main, cross, cache, tree, store, sublayout);
        abs_items.push(ChildNode {
            node: child,
            main: size.main,
            cross: size.cross,
            main_after: 0.0,
            last_layout_main: 0.0,
            last_layout_cross: 0.0,
            has_layout_constraints: false,
        });
    }

    // Phase 8: Position all children.
    // Decompose alignment into (main-fraction, cross-fraction) where the
    // main-fraction offsets the whole group and cross-fraction aligns each item
    // within its line's cross extent.  The swap mirrors what the non-wrap layout
    // does so that `TopLeft` means the same visual position regardless of
    // layout_type.
    let mut alignment = node.alignment(store).unwrap_or_default();

    // For RTL inline layouts, flip horizontal alignment so TopLeft becomes TopRight.
    if is_inline_rtl {
        alignment = flip_alignment_horizontal(alignment);
    }

    let (mut main_align_frac, mut cross_align_frac) = match alignment {
        Alignment::TopLeft => (0.0f32, 0.0f32),
        Alignment::TopCenter => (0.0, 0.5),
        Alignment::TopRight => (0.0, 1.0),
        Alignment::Left => (0.5, 0.0),
        Alignment::Center => (0.5, 0.5),
        Alignment::Right => (0.5, 1.0),
        Alignment::BottomLeft => (1.0, 0.0),
        Alignment::BottomCenter => (1.0, 0.5),
        Alignment::BottomRight => (1.0, 1.0),
    };
    if layout_type == LayoutType::Row {
        std::mem::swap(&mut main_align_frac, &mut cross_align_frac);
    }

    let mut cross_cursor = padding_cross_before + border_cross_before;

    for (line_idx, line) in lines.iter().enumerate() {
        let start = line.start;
        let end = line.end;
        let lc = line_cross[line_idx];
        let count = line.len();
        let gap_total = (count.saturating_sub(1)) as f32 * item_gap_px;
        let mut line_main_sum = 0.0f32;
        for i in start..end {
            line_main_sum += items[i].main;
        }
        let free_main = (avail_main - line_main_sum - gap_total).max(0.0);

        if layout_type == LayoutType::Row && node.direction(store).unwrap_or_default() == Direction::RightToLeft {
            // RTL positioning: place items in reverse order within each wrapped line.
            // Alignment is flipped above so TopLeft maps to TopRight semantics.
            let mut main_cursor = padding_main_before + border_main_before + main_align_frac * free_main;

            for (item_idx, i) in (start..end).rev().enumerate() {
                let item = &items[i];
                let child = relative_children[i];
                let item_cross_offset = cross_align_frac * (lc - item.cross);

                cache.set_rect(
                    child,
                    layout_type,
                    main_cursor,
                    cross_cursor + item_cross_offset,
                    item.main,
                    item.cross,
                );

                main_cursor += item.main;
                if item_idx + 1 < count {
                    main_cursor += item_gap_px;
                }
            }
        } else {
            // LTR positioning: items are positioned left-to-right within the line
            let mut main_cursor = padding_main_before + border_main_before + main_align_frac * free_main;

            for i in start..end {
                let item = &items[i];
                let child = relative_children[i];

                let item_cross_offset = cross_align_frac * (lc - item.cross);

                cache.set_rect(
                    child,
                    layout_type,
                    main_cursor,
                    cross_cursor + item_cross_offset,
                    item.main,
                    item.cross,
                );

                main_cursor += item.main;
                if i + 1 < end {
                    main_cursor += item_gap_px;
                }
            }
        }

        cross_cursor += lc;
        if line_idx + 1 < num_lines {
            cross_cursor += line_gap_px;
        }
    }

    // Position absolute children.
    for abs_child in &abs_items {
        let (child_main_before, child_main_after) = if is_inline_rtl {
            (abs_child.node.main_after(store, layout_type), abs_child.node.main_before(store, layout_type))
        } else {
            (abs_child.node.main_before(store, layout_type), abs_child.node.main_after(store, layout_type))
        };
        let child_cross_before = abs_child.node.cross_before(store, layout_type);
        let child_cross_after = abs_child.node.cross_after(store, layout_type);

        let pma = abs_avail_main;
        let pca = abs_avail_cross;

        let child_main_pos = match (child_main_before, child_main_after) {
            (Pixels(val), _) => val,
            (Percentage(val), _) => val * 0.01 * pma,
            (_, Pixels(val)) => pma - val - abs_child.main,
            (_, Percentage(val)) => pma - abs_child.main - val * 0.01 * pma,
            (Stretch(b), Stretch(a)) => {
                if b == a {
                    (pma - abs_child.main) * 0.5
                } else {
                    (pma - abs_child.main) * (b / (b + a))
                }
            }
            (Stretch(_), Auto) => pma - abs_child.main,
            (Auto, Stretch(_)) => 0.0,
            (Auto, Auto) => 0.0,
        };

        let child_cross_pos = match (child_cross_before, child_cross_after) {
            (Pixels(val), _) => val,
            (Percentage(val), _) => val * 0.01 * pca,
            (_, Pixels(val)) => pca - val - abs_child.cross,
            (_, Percentage(val)) => pca - abs_child.cross - val * 0.01 * pca,
            (Stretch(b), Stretch(a)) => {
                if b == a {
                    (pca - abs_child.cross) * 0.5
                } else {
                    (pca - abs_child.cross) * (b / (b + a))
                }
            }
            (Stretch(_), Auto) => pca - abs_child.cross,
            (Auto, Stretch(_)) => 0.0,
            (Auto, Auto) => 0.0,
        };

        cache.set_rect(
            abs_child.node,
            layout_type,
            child_main_pos + border_main_before,
            child_cross_pos + border_cross_before,
            abs_child.main,
            abs_child.cross,
        );
    }

    if parent_layout_type == layout_type {
        Size { main: final_main, cross: final_cross }
    } else {
        Size { main: final_cross, cross: final_main }
    }
}

/// Performs layout on the given node returning its computed size.
///
/// The algorithm recurses down the tree, in depth-first order, and performs
/// layout on every node starting from the input `node`.
///
/// # Arguments
///
/// * `node` - Root node to start layout from.
/// * `parent_layout_type` - The [`LayoutType`] of the parent of the `node`.
/// * `parent_main` - The size of the parent of the `node` on its main axis or the main-size of the node if the node is stretch (determined by parent).
/// * `parent_cross` - The size of the parent of the `node` on its cross axis or the cross-size of the node if the node is stretch (determined by parent).
/// * `cache` - A mutable reference to the [`Cache`].
/// * `tree` - A mutable reference to the [`Tree`](crate::Node::Tree).
/// * `store` - A mutable reference to the [`Store`](crate::Node::Store).
/// * `sublayout` - A mutable reference to the [`SubLayout`](crate::Node::SubLayout) context.
///
/// # Example
///
/// ```
/// layout(&root, LayoutType::Column, 600.0, 600.0, &mut cache, &tree, &store, &mut sublayout);
/// ```
#[allow(clippy::too_many_arguments)]
pub(crate) fn layout<N, C>(
    node: &N,
    parent_layout_type: LayoutType,
    parent_main: f32,
    parent_cross: f32,
    cache: &mut C,
    tree: &<N as Node>::Tree,
    store: &<N as Node>::Store,
    sublayout: &mut <N as Node>::SubLayout<'_>,
) -> Size
where
    N: Node,
    C: Cache<Node = N>,
{
    // The layout type of the node. Determines the main and cross axes of the children.
    let layout_type = node.layout_type(store).unwrap_or_default();

    // The desired main-axis and cross-axis sizes of the node.
    let main = node.main(store, parent_layout_type);
    let cross = node.cross(store, parent_layout_type);

    let mut min_main = if main.is_stretch() {
        DEFAULT_MIN
    } else {
        node.min_main(store, parent_layout_type).to_px(parent_main, DEFAULT_MIN)
    };

    let mut max_main = if main.is_stretch() {
        DEFAULT_MAX
    } else {
        node.max_main(store, parent_layout_type).to_px(parent_main, DEFAULT_MAX)
    };

    let mut min_cross = node.min_cross(store, parent_layout_type).to_px(parent_cross, DEFAULT_MIN);

    let mut max_cross = node.max_cross(store, parent_layout_type).to_px(parent_cross, DEFAULT_MAX);

    // Compute main-axis size.
    let mut computed_main = match main {
        Pixels(val) => val,
        Percentage(val) => (parent_main * (val / 100.0)).round(),
        Stretch(_) => parent_main,
        Auto => 0.0,
    };

    // Compute cross-axis size.
    let mut computed_cross = match cross {
        Pixels(val) => val,
        Percentage(val) => (parent_cross * (val / 100.0)).round(),
        Stretch(_) => parent_cross,
        Auto => 0.0,
    };

    let border_main_before =
        node.border_main_before(store, parent_layout_type).to_px(computed_main, DEFAULT_BORDER_WIDTH);
    let border_main_after =
        node.border_main_after(store, parent_layout_type).to_px(computed_main, DEFAULT_BORDER_WIDTH);
    let border_cross_before =
        node.border_cross_before(store, parent_layout_type).to_px(computed_cross, DEFAULT_BORDER_WIDTH);
    let border_cross_after =
        node.border_cross_after(store, parent_layout_type).to_px(computed_cross, DEFAULT_BORDER_WIDTH);

    // Classify visible children once to avoid repeated tree traversals.
    let mut relative_children = SmallVec::<[&N; 32]>::new();
    let mut absolute_children = SmallVec::<[&N; 8]>::new();
    for child in node.children(tree).filter(|child| child.visible(store)) {
        match child.position_type(store).unwrap_or_default() {
            PositionType::Relative => relative_children.push(child),
            PositionType::Absolute => absolute_children.push(child),
        }
    }

    // Get the total number of children of the node.
    let num_children = relative_children.len() + absolute_children.len();

    // Get the total number of relative children of the node.
    let num_parent_directed_children = relative_children.len();

    // Apply content sizing.
    if (node.min_main(store, parent_layout_type).is_auto() || node.min_cross(store, parent_layout_type).is_auto())
        && num_parent_directed_children == 0
    {
        let p_main = if node.min_main(store, parent_layout_type).is_auto() { None } else { Some(computed_main) };
        let p_cross = if node.min_cross(store, parent_layout_type).is_auto() { None } else { Some(computed_cross) };

        if let Some(content_size) = node.content_sizing(store, sublayout, parent_layout_type, p_main, p_cross) {
            min_main = content_size.0;
            min_cross = content_size.1;
        }
    }

    if (main.is_auto() || cross.is_auto()) && num_parent_directed_children == 0 {
        let p_main = if main.is_auto() { None } else { Some(computed_main) };
        let p_cross = if cross.is_auto() { None } else { Some(computed_cross) };

        if let Some(content_size) = node.content_sizing(store, sublayout, parent_layout_type, p_main, p_cross) {
            computed_main = content_size.0;
            computed_cross = content_size.1;
        }
    }

    computed_main = computed_main.max(min_main).min(max_main);
    computed_cross = computed_cross.max(min_cross).min(max_cross);

    if layout_type == LayoutType::Grid {
        return layout_grid(node, parent_layout_type, computed_main, computed_cross, cache, tree, store, sublayout);
    }

    if layout_type == LayoutType::Overlay {
        return layout_overlay(node, parent_layout_type, computed_main, computed_cross, cache, tree, store, sublayout);
    }

    if node.wrap(store).unwrap_or_default() == LayoutWrap::Wrap {
        return layout_wrap(node, parent_layout_type, computed_main, computed_cross, cache, tree, store, sublayout);
    }

    // Determine the parent_main/cross size to pass to the children based on the layout type of the parent and the node.
    // i.e. if the parent layout type and the node layout type are different, swap the main and the cross axes.
    let (mut parent_main, mut parent_cross) = if parent_layout_type == layout_type {
        (computed_main, computed_cross)
    } else {
        (computed_cross, computed_main)
    };

    // Sum of all space and size flex factors on the main-axis of the node.
    let mut main_flex_sum = 0.0;

    // List of child nodes for the current node.
    let mut children = SmallVec::<[ChildNode<N>; 32]>::with_capacity(num_children);

    // List of stretch nodes for the current node.
    // A stretch node is any flexible space/size. e.g. main_before, main, and main_after are separate stretch nodes
    let mut main_axis = SmallVec::<[StretchItem; 32]>::new();

    // Parent overrides for child auto space.
    let padding_main_before = node.padding_main_before(store, layout_type).to_px(parent_main, 0.0);
    let padding_main_after = node.padding_main_after(store, layout_type).to_px(parent_main, 0.0);
    let padding_cross_before = node.padding_cross_before(store, layout_type).to_px(parent_cross, 0.0);
    let padding_cross_after = node.padding_cross_after(store, layout_type).to_px(parent_cross, 0.0);

    let min_main_between = node.min_main_between(store, layout_type);
    let max_main_between = node.max_main_between(store, layout_type);

    parent_main = parent_main - padding_main_before - padding_main_after - border_main_before - border_main_after;
    parent_cross = parent_cross - padding_cross_before - padding_cross_after - border_cross_before - border_cross_after;

    let is_row_rtl =
        layout_type == LayoutType::Row && node.direction(store).unwrap_or_default() == Direction::RightToLeft;

    let is_rtl = matches!(layout_type, LayoutType::Row | LayoutType::Column)
        && node.direction(store).unwrap_or_default() == Direction::RightToLeft;

    if is_row_rtl {
        relative_children.reverse();
    }

    let last = relative_children.len().checked_sub(1);

    // Compute space and size of non-flexible relative children.
    for (index, child) in relative_children.into_iter().enumerate() {
        let child_main = child.main(store, layout_type);
        let child_cross = child.cross(store, layout_type);

        // Get fixed-size constraints.
        let child_min_main = child.min_main(store, layout_type);
        let child_max_main = child.max_main(store, layout_type);

        let child_min_cross = child.min_cross(store, layout_type);
        let child_max_cross = child.max_cross(store, layout_type);

        let mut computed_child_main_after = 0.0f32;

        if last != Some(index) {
            let child_main_after = node.main_between(store, layout_type);

            if let Stretch(factor) = child_main_after {
                main_flex_sum += factor;
                main_axis.push(StretchItem::new(
                    index,
                    factor,
                    ItemType::After,
                    min_main_between.to_px(parent_main, DEFAULT_MIN),
                    max_main_between.to_px(parent_main, DEFAULT_MAX),
                ));
            } else {
                computed_child_main_after =
                    child_main_after.to_px_clamped(parent_main, 0.0, min_main_between, max_main_between);
            }
        }

        let mut computed_child_main = 0.0;

        // Collect stretch main items.
        if let Stretch(factor) = child_main {
            main_flex_sum += factor;
            main_axis.push(StretchItem::new(
                index,
                factor,
                ItemType::Size,
                child_min_main.to_px(parent_main, DEFAULT_MIN),
                child_max_main.to_px(parent_main, DEFAULT_MAX),
            ));
        } else {
            computed_child_main = child_main.to_px_clamped(parent_cross, 0.0, child_min_main, child_max_main);
        }

        let mut computed_child_cross = child_cross.to_px_clamped(parent_cross, 0.0, child_min_cross, child_max_cross);

        // Compute fixed-size child main and cross.
        let mut has_layout_constraints = false;
        let mut last_layout_main = 0.0;
        let mut last_layout_cross = 0.0;

        if !child_main.is_stretch() && (!child_cross.is_stretch() || child_min_cross.is_auto()) {
            let child_size = layout(child, layout_type, parent_main, parent_cross, cache, tree, store, sublayout);

            computed_child_main = child_size.main;
            computed_child_cross = child_size.cross;
            has_layout_constraints = true;
            last_layout_main = parent_main;
            last_layout_cross = parent_cross;
        }

        children.push(ChildNode {
            node: child,
            cross: computed_child_cross,
            main: computed_child_main,
            main_after: computed_child_main_after,
            last_layout_main,
            last_layout_cross,
            has_layout_constraints,
        });
    }

    // Sum of all child nodes on the main-axis.
    let mut main_sum: f32 = children.iter().map(|child| child.main + child.main_after).sum();

    // Maximum of all child nodes on the cross-axis.
    let mut cross_max: f32 = children.iter().map(|child| child.cross).reduce(f32::max).unwrap_or_default();

    // Determine auto main and cross size from space and size of children.
    if num_parent_directed_children != 0 {
        if main.is_auto() || node.min_main(store, parent_layout_type).is_auto() {
            if parent_layout_type == layout_type {
                min_main = main_sum + border_main_before + border_main_after + padding_main_before + padding_main_after;
            } else {
                min_main =
                    cross_max + border_main_before + border_main_after + padding_cross_before + padding_cross_after;
            }
        }

        if node.max_main(store, parent_layout_type).is_auto() {
            if parent_layout_type == layout_type && main_sum != 0.0 {
                max_main = main_sum + border_main_before + border_main_after + padding_main_before + padding_main_after;
            } else if cross_max != 0.0 {
                max_main =
                    cross_max + border_main_before + border_main_after + padding_cross_before + padding_cross_after;
            }
        }

        if cross.is_auto() || node.min_cross(store, parent_layout_type).is_auto() {
            if parent_layout_type == layout_type {
                min_cross =
                    cross_max + border_cross_before + border_cross_after + padding_cross_before + padding_cross_after;
            } else {
                min_cross =
                    main_sum + border_cross_before + border_cross_after + padding_main_before + padding_main_after;
            }
        }

        if node.max_cross(store, parent_layout_type).is_auto() {
            if parent_layout_type == layout_type && cross_max != 0.0 {
                max_cross =
                    cross_max + border_cross_before + border_cross_after + padding_cross_before + padding_cross_after;
            } else if main_sum != 0.0 {
                max_cross =
                    main_sum + border_cross_before + border_cross_after + padding_main_before + padding_main_after;
            }
        }
    }

    computed_main = computed_main.max(min_main).min(max_main);
    computed_cross = computed_cross.max(min_cross).min(max_cross);

    let (mut parent_main, mut parent_cross) = if parent_layout_type == layout_type {
        (computed_main, computed_cross)
    } else {
        (computed_cross, computed_main)
    };

    parent_main = parent_main - padding_main_before - padding_main_after - border_main_before - border_main_after;
    parent_cross = parent_cross - padding_cross_before - padding_cross_after - border_cross_before - border_cross_after;

    // Compute stretch size on the cross-axis for relative children.
    for child in children
        .iter_mut()
        .filter(|child| child.node.position_type(store).unwrap_or_default() == PositionType::Relative)
        .filter(|child| child.node.cross(store, layout_type).is_stretch())
    {
        if !child.node.main(store, layout_type).is_stretch() {
            if !child.has_layout_constraints
                || !same_f32(child.last_layout_main, parent_main)
                || !same_f32(child.last_layout_cross, parent_cross)
            {
                let child_size =
                    layout(child.node, layout_type, parent_main, parent_cross, cache, tree, store, sublayout);
                child.main = child_size.main;
                child.cross = child_size.cross;
                child.last_layout_main = parent_main;
                child.last_layout_cross = parent_cross;
                child.has_layout_constraints = true;
            }
        } else {
            let child_min_cross = if child.node.min_cross(store, layout_type).is_auto() {
                child.cross
            } else {
                child.node.min_cross(store, layout_type).to_px(parent_cross, DEFAULT_MIN)
            };

            let child_max_cross = child.node.max_cross(store, layout_type).to_px(parent_cross, DEFAULT_MAX);

            child.cross = parent_cross.clamp(child_min_cross, child_max_cross);
        }
    }

    main_sum = children.iter().map(|child| child.main + child.main_after).sum();
    cross_max = children.iter().map(|child| child.cross).reduce(f32::max).unwrap_or_default();

    // Determine auto main and cross size from space and size of children.
    if num_parent_directed_children != 0 {
        if main.is_auto() || node.min_main(store, parent_layout_type).is_auto() {
            if parent_layout_type == layout_type {
                min_main = main_sum + border_main_before + border_main_after + padding_main_before + padding_main_after;
            } else {
                min_main =
                    cross_max + border_main_before + border_main_after + padding_cross_before + padding_cross_after;
            }
        }

        if node.max_main(store, parent_layout_type).is_auto() {
            if parent_layout_type == layout_type && main_sum != 0.0 {
                max_main = main_sum + border_main_before + border_main_after + padding_main_before + padding_main_after;
            } else if cross_max != 0.0 {
                max_main =
                    cross_max + border_main_before + border_main_after + padding_cross_before + padding_cross_after;
            }
        }

        if cross.is_auto() || node.min_cross(store, parent_layout_type).is_auto() {
            if parent_layout_type == layout_type {
                min_cross =
                    cross_max + border_cross_before + border_cross_after + padding_cross_before + padding_cross_after;
            } else {
                min_cross =
                    main_sum + border_cross_before + border_cross_after + padding_main_before + padding_main_after;
            }
        }

        if node.max_cross(store, parent_layout_type).is_auto() {
            if parent_layout_type == layout_type && cross_max != 0.0 {
                max_cross =
                    cross_max + border_cross_before + border_cross_after + padding_cross_before + padding_cross_after;
            } else if main_sum != 0.0 {
                max_cross =
                    main_sum + border_cross_before + border_cross_after + padding_main_before + padding_main_after;
            }
        }
    }

    computed_main = computed_main.max(min_main).min(max_main);
    computed_cross = computed_cross.max(min_cross).min(max_cross);

    // Compute flexible space and size on the main axis for relative children.
    if !main_axis.is_empty() {
        loop {
            // If all stretch items are frozen, exit the loop.
            if main_axis.iter().all(|item| item.frozen) {
                break;
            }

            // Calculate free space on the main-axis.
            let free_main_space = parent_main - main_sum;

            let mut total_violation = 0.0;

            for item in main_axis.iter_mut().filter(|item| !item.frozen) {
                let input_main = (item.factor * free_main_space / main_flex_sum).round();
                let mut actual_main = input_main;

                let child = &mut children[item.index];

                if item.item_type == ItemType::Size {
                    let target_cross =
                        if child.node.cross(store, layout_type).is_stretch() { child.cross } else { parent_cross };

                    if !child.has_layout_constraints
                        || !same_f32(child.last_layout_main, input_main)
                        || !same_f32(child.last_layout_cross, target_cross)
                    {
                        let child_size =
                            layout(child.node, layout_type, input_main, target_cross, cache, tree, store, sublayout);
                        child.cross = child_size.cross;
                        actual_main = child_size.main;
                        item.measured = actual_main;
                        child.last_layout_main = input_main;
                        child.last_layout_cross = target_cross;
                        child.has_layout_constraints = true;
                    } else {
                        actual_main = item.measured;
                    }

                    if child.node.min_main(store, layout_type).is_auto() {
                        item.min = child.main;
                    }
                }

                let clamped = actual_main.min(item.max).max(item.min);
                item.violation = clamped - actual_main;
                total_violation += item.violation;
                item.computed = clamped;
            }

            for item in main_axis.iter_mut().filter(|item| !item.frozen) {
                let child = &mut children[item.index];

                // Freeze over-stretched items.
                item.frozen = match total_violation {
                    total if total > 0.0 => item.violation > 0.0,
                    total if total < 0.0 => item.violation < 0.0,
                    _ => true,
                };

                // If the item is frozen, adjust the used_space and sum of cross stretch factors.
                if item.frozen {
                    main_flex_sum -= item.factor;
                    let previous_total = child.main + child.main_after;

                    match item.item_type {
                        ItemType::Size => {
                            if (item.computed - item.measured).abs() > f32::EPSILON {
                                let target_cross = if child.node.cross(store, layout_type).is_stretch() {
                                    child.cross
                                } else {
                                    parent_cross
                                };

                                if !child.has_layout_constraints
                                    || !same_f32(child.last_layout_main, item.computed)
                                    || !same_f32(child.last_layout_cross, target_cross)
                                {
                                    let child_size = layout(
                                        child.node,
                                        layout_type,
                                        item.computed,
                                        target_cross,
                                        cache,
                                        tree,
                                        store,
                                        sublayout,
                                    );

                                    child.cross = child_size.cross;
                                    item.measured = child_size.main;
                                    child.last_layout_main = item.computed;
                                    child.last_layout_cross = target_cross;
                                    child.has_layout_constraints = true;
                                }
                            }

                            child.main = item.computed;
                        }

                        ItemType::After => {
                            child.main_after = item.computed;
                        }
                    }

                    main_sum += (child.main + child.main_after) - previous_total;
                }
            }
        }
    }

    main_sum = children.iter().map(|child| child.main + child.main_after).sum();
    cross_max = children.iter().map(|child| child.cross).reduce(f32::max).unwrap_or_default();

    // Determine auto main and cross size from space and size of children.
    if num_parent_directed_children != 0 {
        if main.is_auto() || node.min_main(store, parent_layout_type).is_auto() {
            if parent_layout_type == layout_type {
                min_main = main_sum + border_main_before + border_main_after + padding_main_before + padding_main_after;
            } else {
                min_main =
                    cross_max + border_main_before + border_main_after + padding_cross_before + padding_cross_after;
            }
        }

        if node.max_main(store, parent_layout_type).is_auto() {
            if parent_layout_type == layout_type && main_sum != 0.0 {
                max_main = main_sum + border_main_before + border_main_after + padding_main_before + padding_main_after;
            } else if cross_max != 0.0 {
                max_main =
                    cross_max + border_main_before + border_main_after + padding_cross_before + padding_cross_after;
            }
        }

        if cross.is_auto() || node.min_cross(store, parent_layout_type).is_auto() {
            if parent_layout_type == layout_type {
                min_cross =
                    cross_max + border_cross_before + border_cross_after + padding_cross_before + padding_cross_after;
            } else {
                min_cross =
                    main_sum + border_cross_before + border_cross_after + padding_main_before + padding_main_after;
            }
        }

        if node.max_cross(store, parent_layout_type).is_auto() {
            if parent_layout_type == layout_type && cross_max != 0.0 {
                max_cross =
                    cross_max + border_cross_before + border_cross_after + padding_cross_before + padding_cross_after;
            } else if main_sum != 0.0 {
                max_cross =
                    main_sum + border_cross_before + border_cross_after + padding_main_before + padding_main_after;
            }
        }
    }

    computed_main = computed_main.max(min_main).min(max_main);
    computed_cross = computed_cross.max(min_cross).min(max_cross);

    let (mut parent_main, mut parent_cross) = if parent_layout_type == layout_type {
        (computed_main, computed_cross)
    } else {
        (computed_cross, computed_main)
    };

    parent_main = parent_main - padding_main_before - padding_main_after - border_main_before - border_main_after;
    parent_cross = parent_cross - padding_cross_before - padding_cross_after - border_cross_before - border_cross_after;

    for child in children
        .iter_mut()
        .filter(|child| child.node.position_type(store).unwrap_or_default() == PositionType::Relative)
        .filter(|child| child.node.cross(store, layout_type).is_stretch())
    {
        let child_min_cross = if child.node.min_cross(store, layout_type).is_auto() {
            child.cross
        } else {
            child.node.min_cross(store, layout_type).to_px(parent_cross, DEFAULT_MIN)
        };

        let child_max_cross = child.node.max_cross(store, layout_type).to_px(parent_cross, DEFAULT_MAX);

        child.cross = parent_cross.clamp(child_min_cross, child_max_cross);
    }

    // Re-run relative children with their final resolved constraints so descendant
    // layout uses the same dimensions that are ultimately cached for each child.
    // Only rerun children that have descendants (skip leaf nodes).
    for child in children
        .iter_mut()
        .filter(|child| child.node.position_type(store).unwrap_or_default() == PositionType::Relative)
        .filter(|child| child.node.children(tree).next().is_some())
    {
        let child_main_is_stretch = child.node.main(store, layout_type).is_stretch();
        let child_cross_is_stretch = child.node.cross(store, layout_type).is_stretch();

        let target_main = if child_main_is_stretch { child.main } else { parent_main };
        let target_cross = if child_cross_is_stretch { child.cross } else { parent_cross };

        if child.has_layout_constraints
            && same_f32(child.last_layout_main, target_main)
            && same_f32(child.last_layout_cross, target_cross)
        {
            continue;
        }

        let child_size = layout(child.node, layout_type, target_main, target_cross, cache, tree, store, sublayout);

        child.last_layout_main = target_main;
        child.last_layout_cross = target_cross;
        child.has_layout_constraints = true;

        if !child_main_is_stretch {
            child.main = child_size.main;
        }

        child.cross = child_size.cross;
    }

    // Absolute Children

    // Absolute children are sized against the padding box (content box + padding, excluding border).
    let abs_size_main = parent_main + padding_main_before + padding_main_after;
    let abs_size_cross = parent_cross + padding_cross_before + padding_cross_after;

    // Compute space and size of non-flexible absolute children.
    for child in absolute_children.into_iter() {
        let main = if child.main(store, layout_type).is_stretch() {
            let child_min_main = child.min_main(store, layout_type).to_px(abs_size_main, DEFAULT_MIN);
            let child_max_main = child.max_main(store, layout_type).to_px(abs_size_main, DEFAULT_MAX);

            let child_main_before = child.main_before(store, layout_type).to_px(abs_size_main, 0.0);
            let child_main_after = child.main_after(store, layout_type).to_px(abs_size_main, 0.0);

            abs_size_main.clamp(child_min_main, child_max_main) - child_main_before - child_main_after
        } else {
            abs_size_main
        };

        let cross = if child.cross(store, layout_type).is_stretch() {
            let child_min_cross = child.min_cross(store, layout_type).to_px(abs_size_cross, DEFAULT_MIN);
            let child_max_cross = child.max_cross(store, layout_type).to_px(abs_size_cross, DEFAULT_MAX);

            let child_cross_before = child.cross_before(store, layout_type).to_px(abs_size_cross, 0.0);
            let child_cross_after = child.cross_after(store, layout_type).to_px(abs_size_cross, 0.0);

            abs_size_cross.clamp(child_min_cross, child_max_cross) - child_cross_before - child_cross_after
        } else {
            abs_size_cross
        };

        let child_size = layout(child, layout_type, main, cross, cache, tree, store, sublayout);

        let computed_child_main = child_size.main;
        let computed_child_cross = child_size.cross;

        children.push(ChildNode {
            node: child,
            cross: computed_child_cross,
            main: computed_child_main,
            main_after: 0.0,
            last_layout_main: 0.0,
            last_layout_cross: 0.0,
            has_layout_constraints: false,
        });
    }

    let mut alignment = node.alignment(store).unwrap_or_default();

    if matches!(layout_type, LayoutType::Row | LayoutType::Column)
        && node.direction(store).unwrap_or_default() == Direction::RightToLeft
    {
        alignment = flip_alignment_horizontal(alignment);
    }

    // Set size and position of children in the cache.
    let mut main_pos = padding_main_before + border_main_before;
    for child in children.iter() {
        let child_position = child.node.position_type(store).unwrap_or_default();

        match child_position {
            PositionType::Absolute => {
                let (child_main_before, child_main_after) = if is_rtl {
                    (child.node.main_after(store, layout_type), child.node.main_before(store, layout_type))
                } else {
                    (child.node.main_before(store, layout_type), child.node.main_after(store, layout_type))
                };
                let child_cross_before = child.node.cross_before(store, layout_type);
                let child_cross_after = child.node.cross_after(store, layout_type);

                let parent_main = parent_main + padding_main_before + padding_main_after;
                let parent_cross = parent_cross + padding_cross_before + padding_cross_after;

                let child_main_pos =
                    absolute_axis_position(child_main_before, child_main_after, parent_main, child.main);

                let child_cross_pos =
                    absolute_axis_position(child_cross_before, child_cross_after, parent_cross, child.cross);

                cache.set_rect(
                    child.node,
                    layout_type,
                    child_main_pos + border_main_before,
                    child_cross_pos + border_cross_before,
                    child.main,
                    child.cross,
                );
            }

            PositionType::Relative => {
                let (mut child_main_pos, mut child_cross_pos) = match alignment {
                    Alignment::TopLeft => (0.0, 0.0),
                    Alignment::TopCenter => (0.0, 0.5),
                    Alignment::TopRight => (0.0, 1.0),
                    Alignment::Left => (0.5, 0.0),
                    Alignment::Center => (0.5, 0.5),
                    Alignment::Right => (0.5, 1.0),
                    Alignment::BottomLeft => (1.0, 0.0),
                    Alignment::BottomCenter => (1.0, 0.5),
                    Alignment::BottomRight => (1.0, 1.0),
                };

                if layout_type == LayoutType::Row {
                    std::mem::swap(&mut child_main_pos, &mut child_cross_pos);
                }

                child_main_pos *= parent_main - main_sum;
                child_cross_pos *= parent_cross - child.cross;

                if let Some(main_scroll) = node.main_scroll(store, layout_type) {
                    child_main_pos = main_scroll
                }

                if let Some(cross_scroll) = node.cross_scroll(store, layout_type) {
                    child_cross_pos = cross_scroll
                }

                cache.set_rect(
                    child.node,
                    layout_type,
                    main_pos + child_main_pos,
                    child_cross_pos + padding_cross_before + border_cross_before,
                    child.main,
                    child.cross,
                );
                main_pos += child.main + child.main_after;
            }
        };
    }

    // Return the computed size, propagating it back up the tree.
    Size { main: computed_main, cross: computed_cross }
}
