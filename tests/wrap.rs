use morphorm::*;
use morphorm_ecs::*;

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

// TODO: wrap_with_different_line_heights test - currently fails because the line-wrapping
// logic in layout_wrap doesn't properly wrap items when they exceed available space.
// This is a known issue with the Phase 2 line assignment algorithm that needs fixing.

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

