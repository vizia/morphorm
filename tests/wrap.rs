use morphorm::*;
use morphorm_ecs::*;
use std::{cell::Cell, rc::Rc};

#[test]
fn wrap_stretch_child_does_not_repeat_identical_sublayout() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(300.0));
    world.set_height(root, Units::Pixels(300.0));
    world.set_layout_type(root, LayoutType::Row);
    world.set_wrap(root, LayoutWrap::Wrap);
    world.set_alignment(root, Alignment::TopLeft);

    let card = world.add(Some(root));
    world.set_width(card, Units::Stretch(1.0));
    world.set_height(card, Units::Stretch(1.0));
    world.set_min_width(card, Units::Pixels(100.0));
    world.set_min_height(card, Units::Pixels(100.0));

    let measurements = Rc::new(Cell::new(0));
    let content = world.add(Some(card));
    world.set_width(content, Units::Stretch(1.0));
    world.set_height(content, Units::Auto);
    world.set_content_size(content, {
        let measurements = Rc::clone(&measurements);
        move |_, width, _| {
            measurements.set(measurements.get() + 1);
            (width.unwrap_or_default(), 20.0)
        }
    });

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(measurements.get(), 1);
    assert_eq!(world.cache.bounds(card), Some(&Rect { posx: 0.0, posy: 0.0, width: 300.0, height: 100.0 }));
    assert_eq!(world.cache.bounds(content), Some(&Rect { posx: 0.0, posy: 0.0, width: 300.0, height: 20.0 }));
}

#[test]
fn wrap_row_basic() {
    // Test basic row wrapping - when wrap is enabled, items should stay on one line
    // if they fit. Wrapping only occurs when items exceed available space.
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(300.0));
    world.set_height(root, Units::Pixels(300.0));
    world.set_layout_type(root, LayoutType::Row);
    world.set_wrap(root, LayoutWrap::Wrap);
    world.set_alignment(root, Alignment::TopLeft);

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Pixels(100.0));
    world.set_height(node1, Units::Pixels(50.0));

    let node2 = world.add(Some(root));
    world.set_width(node2, Units::Pixels(100.0));
    world.set_height(node2, Units::Pixels(50.0));

    let node3 = world.add(Some(root));
    world.set_width(node3, Units::Pixels(100.0));
    world.set_height(node3, Units::Pixels(50.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    // All items fit on one line (300px available, 300px used)
    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 50.0 }));
    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 100.0, posy: 0.0, width: 100.0, height: 50.0 }));
    assert_eq!(world.cache.bounds(node3), Some(&Rect { posx: 200.0, posy: 0.0, width: 100.0, height: 50.0 }));
}

#[test]
fn wrap_row_with_gap() {
    // Test row wrapping with horizontal gap between items and vertical gap between lines
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(300.0));
    world.set_height(root, Units::Pixels(300.0));
    world.set_layout_type(root, LayoutType::Row);
    world.set_wrap(root, LayoutWrap::Wrap);
    world.set_alignment(root, Alignment::TopLeft);
    world.set_horizontal_gap(root, Units::Pixels(20.0));
    world.set_vertical_gap(root, Units::Pixels(10.0));

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Pixels(80.0));
    world.set_height(node1, Units::Pixels(50.0));

    let node2 = world.add(Some(root));
    world.set_width(node2, Units::Pixels(80.0));
    world.set_height(node2, Units::Pixels(50.0));

    let node3 = world.add(Some(root));
    world.set_width(node3, Units::Pixels(80.0));
    world.set_height(node3, Units::Pixels(50.0));

    let node4 = world.add(Some(root));
    world.set_width(node4, Units::Pixels(80.0));
    world.set_height(node4, Units::Pixels(50.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    // First line: nodes 1, 2, 3 (80 + 20 + 80 + 20 + 80 = 280px fits in 300px)
    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 80.0, height: 50.0 }));
    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 100.0, posy: 0.0, width: 80.0, height: 50.0 }));
    assert_eq!(world.cache.bounds(node3), Some(&Rect { posx: 200.0, posy: 0.0, width: 80.0, height: 50.0 }));

    // Second line: node 4 (80px, offset by line_gap + line cross size)
    assert_eq!(world.cache.bounds(node4), Some(&Rect { posx: 0.0, posy: 60.0, width: 80.0, height: 50.0 }));
}

#[test]
fn wrap_column_basic() {
    // Test basic column wrapping with fixed-size items that exceed container height
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(200.0));
    world.set_height(root, Units::Pixels(250.0));
    world.set_layout_type(root, LayoutType::Column);
    world.set_wrap(root, LayoutWrap::Wrap);
    world.set_alignment(root, Alignment::TopLeft);

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Pixels(80.0));
    world.set_height(node1, Units::Pixels(100.0));

    let node2 = world.add(Some(root));
    world.set_width(node2, Units::Pixels(80.0));
    world.set_height(node2, Units::Pixels(100.0));

    let node3 = world.add(Some(root));
    world.set_width(node3, Units::Pixels(80.0));
    world.set_height(node3, Units::Pixels(100.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    // First column: nodes 1 and 2 (200px height fits in 250px)
    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 80.0, height: 100.0 }));
    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 0.0, posy: 100.0, width: 80.0, height: 100.0 }));

    // Second column: node 3
    assert_eq!(world.cache.bounds(node3), Some(&Rect { posx: 80.0, posy: 0.0, width: 80.0, height: 100.0 }));
}

#[test]
fn wrap_column_rtl_absolute_only_flips_horizontal_offsets() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(200.0));
    world.set_height(root, Units::Pixels(250.0));
    world.set_layout_type(root, LayoutType::Column);
    world.set_wrap(root, LayoutWrap::Wrap);
    world.set_direction(root, Direction::RightToLeft);

    let node = world.add(Some(root));
    world.set_position_type(node, PositionType::Absolute);
    world.set_width(node, Units::Pixels(80.0));
    world.set_height(node, Units::Pixels(100.0));
    world.set_left(node, Units::Pixels(20.0));
    world.set_top(node, Units::Pixels(10.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    // RTL should mirror only the horizontal offset in a wrapped column.
    assert_eq!(world.cache.bounds(node), Some(&Rect { posx: 100.0, posy: 10.0, width: 80.0, height: 100.0 }));
}

#[test]
fn wrap_row_with_stretch() {
    // Test row wrapping with stretch items filling available space on each line
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(300.0));
    world.set_height(root, Units::Pixels(300.0));
    world.set_layout_type(root, LayoutType::Row);
    world.set_wrap(root, LayoutWrap::Wrap);
    world.set_alignment(root, Alignment::TopLeft);

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Stretch(1.0));
    world.set_height(node1, Units::Pixels(50.0));

    let node2 = world.add(Some(root));
    world.set_width(node2, Units::Stretch(1.0));
    world.set_height(node2, Units::Pixels(50.0));

    let node3 = world.add(Some(root));
    world.set_width(node3, Units::Pixels(100.0));
    world.set_height(node3, Units::Pixels(50.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    // With wrapping, stretch items treat min size (0) for line-break decision
    // node1 and node2 both stretch to fill (no size contribution to line break)
    // All items fit on one line
    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 50.0 }));
    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 100.0, posy: 0.0, width: 100.0, height: 50.0 }));
    assert_eq!(world.cache.bounds(node3), Some(&Rect { posx: 200.0, posy: 0.0, width: 100.0, height: 50.0 }));
}

#[test]
fn wrap_row_stretch_children_resolve_to_line_max_cross() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(260.0));
    world.set_height(root, Units::Pixels(220.0));
    world.set_layout_type(root, LayoutType::Row);
    world.set_wrap(root, LayoutWrap::Wrap);
    world.set_alignment(root, Alignment::TopLeft);
    world.set_horizontal_gap(root, Units::Pixels(10.0));
    world.set_vertical_gap(root, Units::Pixels(8.0));

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Stretch(1.0));
    world.set_height(node1, Units::Stretch(1.0));
    world.set_min_width(node1, Units::Pixels(80.0));
    world.set_min_height(node1, Units::Pixels(30.0));

    let node2 = world.add(Some(root));
    world.set_width(node2, Units::Stretch(1.0));
    world.set_height(node2, Units::Stretch(1.0));
    world.set_min_width(node2, Units::Pixels(120.0));
    world.set_min_height(node2, Units::Pixels(70.0));

    let node3 = world.add(Some(root));
    world.set_width(node3, Units::Stretch(1.0));
    world.set_height(node3, Units::Stretch(1.0));
    world.set_min_width(node3, Units::Pixels(80.0));
    world.set_min_height(node3, Units::Pixels(30.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    // First line should resolve to node2's larger min-height, and node1 should
    // stretch to that same line height.
    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 125.0, height: 70.0 }));
    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 135.0, posy: 0.0, width: 125.0, height: 70.0 }));

    // Second line starts after first line cross size plus vertical gap.
    assert_eq!(world.cache.bounds(node3), Some(&Rect { posx: 0.0, posy: 78.0, width: 260.0, height: 30.0 }));
}

#[test]
fn wrap_row_stretch_mixed_min_cross_respects_parent_padding() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(280.0));
    world.set_height(root, Units::Pixels(240.0));
    world.set_padding_left(root, Units::Pixels(10.0));
    world.set_padding_right(root, Units::Pixels(10.0));
    world.set_padding_top(root, Units::Pixels(10.0));
    world.set_padding_bottom(root, Units::Pixels(10.0));
    world.set_layout_type(root, LayoutType::Row);
    world.set_wrap(root, LayoutWrap::Wrap);
    world.set_alignment(root, Alignment::TopLeft);
    world.set_horizontal_gap(root, Units::Pixels(10.0));
    world.set_vertical_gap(root, Units::Pixels(8.0));

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Stretch(1.0));
    world.set_height(node1, Units::Stretch(1.0));
    world.set_min_width(node1, Units::Pixels(80.0));
    world.set_min_height(node1, Units::Pixels(30.0));

    let node2 = world.add(Some(root));
    world.set_width(node2, Units::Stretch(1.0));
    world.set_height(node2, Units::Stretch(1.0));
    world.set_min_width(node2, Units::Pixels(120.0));
    world.set_min_height(node2, Units::Pixels(70.0));

    let node3 = world.add(Some(root));
    world.set_width(node3, Units::Stretch(1.0));
    world.set_height(node3, Units::Stretch(1.0));
    world.set_min_width(node3, Units::Pixels(80.0));
    world.set_min_height(node3, Units::Pixels(30.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    // Available width = 280 - left/right padding (20) = 260.
    // Line 1 has node1 + gap + node2; line height settles to 70.
    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 10.0, posy: 10.0, width: 125.0, height: 70.0 }));
    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 145.0, posy: 10.0, width: 125.0, height: 70.0 }));

    // Line 2 starts at padding_top + first_line_height + vertical_gap.
    assert_eq!(world.cache.bounds(node3), Some(&Rect { posx: 10.0, posy: 88.0, width: 260.0, height: 30.0 }));
}

#[test]
fn wrap_column_stretch_mixed_min_cross_respects_parent_padding() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(240.0));
    world.set_height(root, Units::Pixels(280.0));
    world.set_padding_left(root, Units::Pixels(10.0));
    world.set_padding_right(root, Units::Pixels(10.0));
    world.set_padding_top(root, Units::Pixels(10.0));
    world.set_padding_bottom(root, Units::Pixels(10.0));
    world.set_layout_type(root, LayoutType::Column);
    world.set_wrap(root, LayoutWrap::Wrap);
    world.set_alignment(root, Alignment::TopLeft);
    world.set_vertical_gap(root, Units::Pixels(10.0));
    world.set_horizontal_gap(root, Units::Pixels(8.0));

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Stretch(1.0));
    world.set_height(node1, Units::Stretch(1.0));
    world.set_min_width(node1, Units::Pixels(30.0));
    world.set_min_height(node1, Units::Pixels(80.0));

    let node2 = world.add(Some(root));
    world.set_width(node2, Units::Stretch(1.0));
    world.set_height(node2, Units::Stretch(1.0));
    world.set_min_width(node2, Units::Pixels(70.0));
    world.set_min_height(node2, Units::Pixels(120.0));

    let node3 = world.add(Some(root));
    world.set_width(node3, Units::Stretch(1.0));
    world.set_height(node3, Units::Stretch(1.0));
    world.set_min_width(node3, Units::Pixels(30.0));
    world.set_min_height(node3, Units::Pixels(80.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    // Available height = 280 - top/bottom padding (20) = 260.
    // First column has node1 + gap + node2; column width settles to 70.
    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 10.0, posy: 10.0, width: 70.0, height: 125.0 }));
    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 10.0, posy: 145.0, width: 70.0, height: 125.0 }));

    // Second column starts at padding_left + first_column_width + horizontal_gap.
    assert_eq!(world.cache.bounds(node3), Some(&Rect { posx: 88.0, posy: 10.0, width: 30.0, height: 260.0 }));
}

#[test]
fn wrap_row_stretch_large_min_main_preserves_line_padding() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(872.0));
    world.set_height(root, Units::Pixels(934.0));
    world.set_layout_type(root, LayoutType::Row);
    world.set_wrap(root, LayoutWrap::Wrap);
    world.set_alignment(root, Alignment::TopLeft);
    world.set_padding_left(root, Units::Pixels(10.0));
    world.set_padding_right(root, Units::Pixels(10.0));
    world.set_padding_top(root, Units::Pixels(10.0));
    world.set_padding_bottom(root, Units::Pixels(10.0));
    world.set_horizontal_gap(root, Units::Pixels(10.0));
    world.set_vertical_gap(root, Units::Pixels(10.0));

    let mins = [180.0, 180.0, 180.0, 300.0, 180.0, 180.0, 180.0];
    let mut nodes = Vec::new();
    for min in mins {
        let node = world.add(Some(root));
        world.set_width(node, Units::Stretch(1.0));
        world.set_height(node, Units::Stretch(1.0));
        world.set_min_width(node, Units::Pixels(min));
        world.set_min_height(node, Units::Pixels(min));
        nodes.push(node);
    }

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    // First line: three equal stretch children.
    assert_eq!(world.cache.bounds(nodes[0]), Some(&Rect { posx: 10.0, posy: 10.0, width: 277.0, height: 180.0 }));
    assert_eq!(world.cache.bounds(nodes[1]), Some(&Rect { posx: 297.0, posy: 10.0, width: 277.0, height: 180.0 }));
    assert_eq!(world.cache.bounds(nodes[2]), Some(&Rect { posx: 584.0, posy: 10.0, width: 277.0, height: 180.0 }));

    // Second line: larger min-width item should clamp to 300 and siblings re-resolve
    // so the line still respects right padding.
    assert_eq!(world.cache.bounds(nodes[3]), Some(&Rect { posx: 10.0, posy: 200.0, width: 300.0, height: 300.0 }));
    assert_eq!(world.cache.bounds(nodes[4]), Some(&Rect { posx: 320.0, posy: 200.0, width: 266.0, height: 300.0 }));
    assert_eq!(world.cache.bounds(nodes[5]), Some(&Rect { posx: 596.0, posy: 200.0, width: 266.0, height: 300.0 }));

    // Third line.
    assert_eq!(world.cache.bounds(nodes[6]), Some(&Rect { posx: 10.0, posy: 510.0, width: 852.0, height: 180.0 }));
}

#[test]
fn wrap_row_no_wrap_mode() {
    // Test that NoWrap mode (default) doesn't wrap items
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(300.0));
    world.set_height(root, Units::Pixels(300.0));
    world.set_layout_type(root, LayoutType::Row);
    world.set_wrap(root, LayoutWrap::NoWrap);
    world.set_alignment(root, Alignment::TopLeft);

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Pixels(100.0));
    world.set_height(node1, Units::Pixels(50.0));

    let node2 = world.add(Some(root));
    world.set_width(node2, Units::Pixels(100.0));
    world.set_height(node2, Units::Pixels(50.0));

    let node3 = world.add(Some(root));
    world.set_width(node3, Units::Pixels(100.0));
    world.set_height(node3, Units::Pixels(50.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    // All items on one line (no wrapping, even though they sum to 300px)
    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 50.0 }));
    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 100.0, posy: 0.0, width: 100.0, height: 50.0 }));
    assert_eq!(world.cache.bounds(node3), Some(&Rect { posx: 200.0, posy: 0.0, width: 100.0, height: 50.0 }));
}

#[test]
fn wrap_row_single_item_per_line() {
    // Test wrapping where items are so large that only one fits per line
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(250.0));
    world.set_height(root, Units::Pixels(300.0));
    world.set_layout_type(root, LayoutType::Row);
    world.set_wrap(root, LayoutWrap::Wrap);
    world.set_alignment(root, Alignment::TopLeft);

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Pixels(200.0));
    world.set_height(node1, Units::Pixels(50.0));

    let node2 = world.add(Some(root));
    world.set_width(node2, Units::Pixels(200.0));
    world.set_height(node2, Units::Pixels(50.0));

    let node3 = world.add(Some(root));
    world.set_width(node3, Units::Pixels(200.0));
    world.set_height(node3, Units::Pixels(50.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    // Each item on its own line
    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 200.0, height: 50.0 }));
    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 0.0, posy: 50.0, width: 200.0, height: 50.0 }));
    assert_eq!(world.cache.bounds(node3), Some(&Rect { posx: 0.0, posy: 100.0, width: 200.0, height: 50.0 }));
}

#[test]
fn wrap_with_alignment() {
    // Test wrapping with center alignment
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(300.0));
    world.set_height(root, Units::Pixels(300.0));
    world.set_layout_type(root, LayoutType::Row);
    world.set_wrap(root, LayoutWrap::Wrap);
    world.set_alignment(root, Alignment::Center);

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Pixels(100.0));
    world.set_height(node1, Units::Pixels(50.0));

    let node2 = world.add(Some(root));
    world.set_width(node2, Units::Pixels(100.0));
    world.set_height(node2, Units::Pixels(50.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    // Items centered on first line: (300 - 200) / 2 = 50px offset on main axis
    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 50.0, posy: 0.0, width: 100.0, height: 50.0 }));
    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 150.0, posy: 0.0, width: 100.0, height: 50.0 }));
}

#[test]
fn wrap_auto_container() {
    // Test wrap with fixed-size container and fixed-size children
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(250.0));
    world.set_height(root, Units::Pixels(250.0));
    world.set_layout_type(root, LayoutType::Row);
    world.set_wrap(root, LayoutWrap::Wrap);
    world.set_alignment(root, Alignment::TopLeft);

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Pixels(100.0));
    world.set_height(node1, Units::Pixels(50.0));

    let node2 = world.add(Some(root));
    world.set_width(node2, Units::Pixels(100.0));
    world.set_height(node2, Units::Pixels(50.0));

    let node3 = world.add(Some(root));
    world.set_width(node3, Units::Pixels(100.0));
    world.set_height(node3, Units::Pixels(50.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    // First line: nodes 1 and 2 (200px fits in 250px)
    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 50.0 }));
    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 100.0, posy: 0.0, width: 100.0, height: 50.0 }));

    // Second line: node 3 wraps (adding it would be 300px > 250px)
    assert_eq!(world.cache.bounds(node3), Some(&Rect { posx: 0.0, posy: 50.0, width: 100.0, height: 50.0 }));
}

// Regression test for wrapping when items on different lines have different heights.
// Verifies line assignment and vertical positioning use each line's maximum height.

#[test]
fn wrap_with_different_line_heights() {
    // Test wrapping where items have different heights
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(250.0));
    world.set_height(root, Units::Pixels(300.0));
    world.set_layout_type(root, LayoutType::Row);
    world.set_wrap(root, LayoutWrap::Wrap);
    world.set_alignment(root, Alignment::TopLeft);

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Pixels(100.0));
    world.set_height(node1, Units::Pixels(50.0));

    let node2 = world.add(Some(root));
    world.set_width(node2, Units::Pixels(100.0));
    world.set_height(node2, Units::Pixels(80.0));

    let node3 = world.add(Some(root));
    world.set_width(node3, Units::Pixels(100.0));
    world.set_height(node3, Units::Pixels(60.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    // First line: nodes 1 and 2 (200px fits in 250px, line height is max = 80px)
    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 50.0 }));
    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 100.0, posy: 0.0, width: 100.0, height: 80.0 }));

    // Second line: node 3 (line height is 60px)
    assert_eq!(world.cache.bounds(node3), Some(&Rect { posx: 0.0, posy: 80.0, width: 100.0, height: 60.0 }));
}

#[test]
fn wrap_row_with_padding() {
    // Test wrapping with padding on the container
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(320.0));
    world.set_height(root, Units::Pixels(320.0));
    world.set_padding_left(root, Units::Pixels(10.0));
    world.set_padding_right(root, Units::Pixels(10.0));
    world.set_padding_top(root, Units::Pixels(10.0));
    world.set_padding_bottom(root, Units::Pixels(10.0));
    world.set_layout_type(root, LayoutType::Row);
    world.set_wrap(root, LayoutWrap::Wrap);
    world.set_alignment(root, Alignment::TopLeft);

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Pixels(100.0));
    world.set_height(node1, Units::Pixels(50.0));

    let node2 = world.add(Some(root));
    world.set_width(node2, Units::Pixels(100.0));
    world.set_height(node2, Units::Pixels(50.0));

    let node3 = world.add(Some(root));
    world.set_width(node3, Units::Pixels(100.0));
    world.set_height(node3, Units::Pixels(50.0));

    let node4 = world.add(Some(root));
    world.set_width(node4, Units::Pixels(100.0));
    world.set_height(node4, Units::Pixels(50.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    // Available width is 320 - 10 - 10 = 300px
    // First line: nodes 1, 2, 3 (300px fills the available space)
    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 10.0, posy: 10.0, width: 100.0, height: 50.0 }));
    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 110.0, posy: 10.0, width: 100.0, height: 50.0 }));
    assert_eq!(world.cache.bounds(node3), Some(&Rect { posx: 210.0, posy: 10.0, width: 100.0, height: 50.0 }));

    // Second line: node 4 wraps (adding it would exceed 300px)
    assert_eq!(world.cache.bounds(node4), Some(&Rect { posx: 10.0, posy: 60.0, width: 100.0, height: 50.0 }));
}

#[test]
fn wrap_row_rtl() {
    // Test row wrapping with right-to-left direction
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(250.0));
    world.set_height(root, Units::Pixels(300.0));
    world.set_layout_type(root, LayoutType::Row);
    world.set_direction(root, Direction::RightToLeft);
    world.set_wrap(root, LayoutWrap::Wrap);
    world.set_alignment(root, Alignment::TopLeft);

    let node1 = world.add(Some(root));
    world.set_width(node1, Units::Pixels(100.0));
    world.set_height(node1, Units::Pixels(50.0));

    let node2 = world.add(Some(root));
    world.set_width(node2, Units::Pixels(100.0));
    world.set_height(node2, Units::Pixels(50.0));

    let node3 = world.add(Some(root));
    world.set_width(node3, Units::Pixels(100.0));
    world.set_height(node3, Units::Pixels(50.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    // RTL: wrapped line composition stays the same, but placement order per line is reversed.
    // With TopLeft alignment flipped to TopRight in RTL:
    // - Line 1: node1 (100px) + node2 (100px) = 200px fits in 250px
    //   Free space on left (50px), reversed placement: node2 at 50-150, node1 at 150-250
    // - Line 2: node3 (100px) alone, with free space on left (150px)
    //   node3 on right: node3 at 150-250
    assert_eq!(world.cache.bounds(node1), Some(&Rect { posx: 150.0, posy: 0.0, width: 100.0, height: 50.0 }));
    assert_eq!(world.cache.bounds(node2), Some(&Rect { posx: 50.0, posy: 0.0, width: 100.0, height: 50.0 }));
    assert_eq!(world.cache.bounds(node3), Some(&Rect { posx: 150.0, posy: 50.0, width: 100.0, height: 50.0 }));
}

#[test]
fn wrap_row_auto_height_includes_lines_gap_and_padding() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(400.0));
    world.set_height(root, Units::Pixels(400.0));
    world.set_layout_type(root, LayoutType::Column);
    world.set_alignment(root, Alignment::TopLeft);

    let wrap = world.add(Some(root));
    world.set_width(wrap, Units::Pixels(250.0));
    world.set_height(wrap, Units::Auto);
    world.set_layout_type(wrap, LayoutType::Row);
    world.set_wrap(wrap, LayoutWrap::Wrap);
    world.set_alignment(wrap, Alignment::TopLeft);
    world.set_padding_top(wrap, Units::Pixels(5.0));
    world.set_padding_bottom(wrap, Units::Pixels(5.0));
    world.set_vertical_gap(wrap, Units::Pixels(10.0));

    let a = world.add(Some(wrap));
    world.set_width(a, Units::Pixels(100.0));
    world.set_height(a, Units::Pixels(50.0));

    let b = world.add(Some(wrap));
    world.set_width(b, Units::Pixels(100.0));
    world.set_height(b, Units::Pixels(50.0));

    let c = world.add(Some(wrap));
    world.set_width(c, Units::Pixels(100.0));
    world.set_height(c, Units::Pixels(50.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    // Two lines: [a, b] then [c].
    // Height = 50 + 10 + 50 + 5 + 5 = 120.
    assert_eq!(world.cache.bounds(wrap), Some(&Rect { posx: 0.0, posy: 0.0, width: 250.0, height: 120.0 }));
    assert_eq!(world.cache.bounds(a), Some(&Rect { posx: 0.0, posy: 5.0, width: 100.0, height: 50.0 }));
    assert_eq!(world.cache.bounds(b), Some(&Rect { posx: 100.0, posy: 5.0, width: 100.0, height: 50.0 }));
    assert_eq!(world.cache.bounds(c), Some(&Rect { posx: 0.0, posy: 65.0, width: 100.0, height: 50.0 }));
}
