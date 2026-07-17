use morphorm::*;
use morphorm_ecs::*;

// ────────────────────────────────────────────────────────────────────────────
// Auto columns
// ────────────────────────────────────────────────────────────────────────────

/// A single Auto column is sized to the natural width of its child.
/// `layout_grid` stores CELL dimensions in the cache, so we assert the cell bounds.
#[test]
fn auto_column_single_pixels_child() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(400.0));
    world.set_height(root, Units::Pixels(200.0));
    world.set_layout_type(root, LayoutType::Grid);
    world.set_alignment(root, Alignment::TopLeft);
    world.set_grid_columns(root, vec![Units::Auto]);
    world.set_grid_rows(root, vec![Units::Pixels(200.0)]);

    let child = world.add(Some(root));
    world.set_column_start(child, 0);
    world.set_row_start(child, 0);
    world.set_width(child, Units::Pixels(120.0));
    world.set_height(child, Units::Pixels(50.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    // Cell: column=120 (auto measured from child), row=200 (pixels).
    assert_eq!(world.cache.bounds(child), Some(&Rect { posx: 0.0, posy: 0.0, width: 120.0, height: 200.0 }));
}

/// Two Auto columns, each sized by the widest child in its column.
#[test]
fn auto_columns_two_columns_different_widths() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(200.0));
    world.set_layout_type(root, LayoutType::Grid);
    world.set_alignment(root, Alignment::TopLeft);
    world.set_grid_columns(root, vec![Units::Auto, Units::Auto]);
    world.set_grid_rows(root, vec![Units::Pixels(200.0)]);

    let child0 = world.add(Some(root));
    world.set_column_start(child0, 0);
    world.set_row_start(child0, 0);
    world.set_width(child0, Units::Pixels(80.0));
    world.set_height(child0, Units::Pixels(50.0));

    let child1 = world.add(Some(root));
    world.set_column_start(child1, 1);
    world.set_row_start(child1, 0);
    world.set_width(child1, Units::Pixels(150.0));
    world.set_height(child1, Units::Pixels(50.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(child0), Some(&Rect { posx: 0.0, posy: 0.0, width: 80.0, height: 200.0 }));
    assert_eq!(world.cache.bounds(child1), Some(&Rect { posx: 80.0, posy: 0.0, width: 150.0, height: 200.0 }));
}

/// Auto column mixed with a Pixels column.
#[test]
fn auto_column_mixed_with_pixels_column() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(200.0));
    world.set_layout_type(root, LayoutType::Grid);
    world.set_alignment(root, Alignment::TopLeft);
    world.set_grid_columns(root, vec![Units::Auto, Units::Pixels(200.0)]);
    world.set_grid_rows(root, vec![Units::Pixels(200.0)]);

    let child0 = world.add(Some(root));
    world.set_column_start(child0, 0);
    world.set_row_start(child0, 0);
    world.set_width(child0, Units::Pixels(100.0));
    world.set_height(child0, Units::Pixels(50.0));

    let child1 = world.add(Some(root));
    world.set_column_start(child1, 1);
    world.set_row_start(child1, 0);
    world.set_width(child1, Units::Stretch(1.0));
    world.set_height(child1, Units::Pixels(50.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    assert_eq!(world.cache.bounds(child0), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 200.0 }));
    assert_eq!(world.cache.bounds(child1), Some(&Rect { posx: 100.0, posy: 0.0, width: 200.0, height: 200.0 }));
}

/// Auto column mixed with a Stretch column; the stretch column takes remaining space.
#[test]
fn auto_column_mixed_with_stretch_column() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(200.0));
    world.set_layout_type(root, LayoutType::Grid);
    world.set_alignment(root, Alignment::TopLeft);
    world.set_grid_columns(root, vec![Units::Auto, Units::Stretch(1.0)]);
    world.set_grid_rows(root, vec![Units::Pixels(200.0)]);

    let child0 = world.add(Some(root));
    world.set_column_start(child0, 0);
    world.set_row_start(child0, 0);
    world.set_width(child0, Units::Pixels(100.0));
    world.set_height(child0, Units::Pixels(50.0));

    let child1 = world.add(Some(root));
    world.set_column_start(child1, 1);
    world.set_row_start(child1, 0);
    world.set_width(child1, Units::Stretch(1.0));
    world.set_height(child1, Units::Pixels(50.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    // Auto col=100; stretch col=600-100=500.
    assert_eq!(world.cache.bounds(child0), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 200.0 }));
    assert_eq!(world.cache.bounds(child1), Some(&Rect { posx: 100.0, posy: 0.0, width: 500.0, height: 200.0 }));
}

/// Auto column with a horizontal gap between columns.
#[test]
fn auto_column_with_gap() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(200.0));
    world.set_layout_type(root, LayoutType::Grid);
    world.set_alignment(root, Alignment::TopLeft);
    world.set_grid_columns(root, vec![Units::Auto, Units::Pixels(200.0)]);
    world.set_grid_rows(root, vec![Units::Pixels(200.0)]);
    world.set_horizontal_gap(root, Units::Pixels(20.0));

    let child0 = world.add(Some(root));
    world.set_column_start(child0, 0);
    world.set_row_start(child0, 0);
    world.set_width(child0, Units::Pixels(100.0));
    world.set_height(child0, Units::Pixels(50.0));

    let child1 = world.add(Some(root));
    world.set_column_start(child1, 1);
    world.set_row_start(child1, 0);
    world.set_width(child1, Units::Stretch(1.0));
    world.set_height(child1, Units::Pixels(50.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    // Auto col=100, gap=20, pixels col=200 → child1 starts at 120.
    assert_eq!(world.cache.bounds(child0), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 200.0 }));
    assert_eq!(world.cache.bounds(child1), Some(&Rect { posx: 120.0, posy: 0.0, width: 200.0, height: 200.0 }));
}

/// Multiple children in the same Auto column: column takes the maximum natural width.
#[test]
fn auto_column_multiple_children_max_width() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(300.0));
    world.set_layout_type(root, LayoutType::Grid);
    world.set_alignment(root, Alignment::TopLeft);
    world.set_grid_columns(root, vec![Units::Auto]);
    world.set_grid_rows(root, vec![Units::Pixels(100.0), Units::Pixels(100.0), Units::Pixels(100.0)]);

    for (row, width) in [(0usize, 60.0f32), (1, 120.0f32), (2, 90.0f32)] {
        let child = world.add(Some(root));
        world.set_column_start(child, 0);
        world.set_row_start(child, row);
        world.set_width(child, Units::Pixels(width));
        world.set_height(child, Units::Pixels(50.0));
    }

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    // Column = max(60, 120, 90) = 120. All cells are 120 wide.
    let children: Vec<_> = root.children(&world.tree).filter(|c| c.visible(&world.store)).collect();

    assert_eq!(world.cache.bounds(*children[0]), Some(&Rect { posx: 0.0, posy: 0.0, width: 120.0, height: 100.0 }));
    assert_eq!(world.cache.bounds(*children[1]), Some(&Rect { posx: 0.0, posy: 100.0, width: 120.0, height: 100.0 }));
    assert_eq!(world.cache.bounds(*children[2]), Some(&Rect { posx: 0.0, posy: 200.0, width: 120.0, height: 100.0 }));
}

// ────────────────────────────────────────────────────────────────────────────
// Auto rows
// ────────────────────────────────────────────────────────────────────────────

/// A single Auto row is sized to the natural height of its child.
#[test]
fn auto_row_single_pixels_child() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(400.0));
    world.set_height(root, Units::Pixels(400.0));
    world.set_layout_type(root, LayoutType::Grid);
    world.set_alignment(root, Alignment::TopLeft);
    world.set_grid_columns(root, vec![Units::Pixels(200.0)]);
    world.set_grid_rows(root, vec![Units::Auto]);

    let child = world.add(Some(root));
    world.set_column_start(child, 0);
    world.set_row_start(child, 0);
    world.set_width(child, Units::Pixels(80.0));
    world.set_height(child, Units::Pixels(75.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    // Cell: col=200 (pixels), row=75 (auto from child Pixels height).
    assert_eq!(world.cache.bounds(child), Some(&Rect { posx: 0.0, posy: 0.0, width: 200.0, height: 75.0 }));
}

/// Two Auto rows, each sized by the tallest child in that row.
#[test]
fn auto_rows_two_rows_different_heights() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(400.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_layout_type(root, LayoutType::Grid);
    world.set_alignment(root, Alignment::TopLeft);
    world.set_grid_columns(root, vec![Units::Pixels(200.0)]);
    world.set_grid_rows(root, vec![Units::Auto, Units::Auto]);

    let child0 = world.add(Some(root));
    world.set_column_start(child0, 0);
    world.set_row_start(child0, 0);
    world.set_width(child0, Units::Pixels(80.0));
    world.set_height(child0, Units::Pixels(60.0));

    let child1 = world.add(Some(root));
    world.set_column_start(child1, 0);
    world.set_row_start(child1, 1);
    world.set_width(child1, Units::Pixels(80.0));
    world.set_height(child1, Units::Pixels(40.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    // Row 0=60, row 1=40; no gap.
    assert_eq!(world.cache.bounds(child0), Some(&Rect { posx: 0.0, posy: 0.0, width: 200.0, height: 60.0 }));
    assert_eq!(world.cache.bounds(child1), Some(&Rect { posx: 0.0, posy: 60.0, width: 200.0, height: 40.0 }));
}

/// Auto row mixed with a Pixels row.
#[test]
fn auto_row_mixed_with_pixels_row() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(400.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_layout_type(root, LayoutType::Grid);
    world.set_alignment(root, Alignment::TopLeft);
    world.set_grid_columns(root, vec![Units::Pixels(200.0)]);
    world.set_grid_rows(root, vec![Units::Auto, Units::Pixels(100.0)]);

    let child0 = world.add(Some(root));
    world.set_column_start(child0, 0);
    world.set_row_start(child0, 0);
    world.set_width(child0, Units::Pixels(80.0));
    world.set_height(child0, Units::Pixels(55.0));

    let child1 = world.add(Some(root));
    world.set_column_start(child1, 0);
    world.set_row_start(child1, 1);
    world.set_width(child1, Units::Pixels(80.0));
    world.set_height(child1, Units::Stretch(1.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    // Auto row=55, pixels row=100.
    assert_eq!(world.cache.bounds(child0), Some(&Rect { posx: 0.0, posy: 0.0, width: 200.0, height: 55.0 }));
    assert_eq!(world.cache.bounds(child1), Some(&Rect { posx: 0.0, posy: 55.0, width: 200.0, height: 100.0 }));
}

/// Auto row mixed with a Stretch row; the stretch row takes remaining space.
#[test]
fn auto_row_mixed_with_stretch_row() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(400.0));
    world.set_height(root, Units::Pixels(500.0));
    world.set_layout_type(root, LayoutType::Grid);
    world.set_alignment(root, Alignment::TopLeft);
    world.set_grid_columns(root, vec![Units::Pixels(200.0)]);
    world.set_grid_rows(root, vec![Units::Auto, Units::Stretch(1.0)]);

    let child0 = world.add(Some(root));
    world.set_column_start(child0, 0);
    world.set_row_start(child0, 0);
    world.set_width(child0, Units::Pixels(80.0));
    world.set_height(child0, Units::Pixels(80.0));

    let child1 = world.add(Some(root));
    world.set_column_start(child1, 0);
    world.set_row_start(child1, 1);
    world.set_width(child1, Units::Pixels(80.0));
    world.set_height(child1, Units::Stretch(1.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    // Auto row=80; stretch row=500-80=420.
    assert_eq!(world.cache.bounds(child0), Some(&Rect { posx: 0.0, posy: 0.0, width: 200.0, height: 80.0 }));
    assert_eq!(world.cache.bounds(child1), Some(&Rect { posx: 0.0, posy: 80.0, width: 200.0, height: 420.0 }));
}

/// Auto row with a vertical gap between rows.
#[test]
fn auto_row_with_gap() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(400.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_layout_type(root, LayoutType::Grid);
    world.set_alignment(root, Alignment::TopLeft);
    world.set_grid_columns(root, vec![Units::Pixels(200.0)]);
    world.set_grid_rows(root, vec![Units::Auto, Units::Pixels(100.0)]);
    world.set_vertical_gap(root, Units::Pixels(10.0));

    let child0 = world.add(Some(root));
    world.set_column_start(child0, 0);
    world.set_row_start(child0, 0);
    world.set_width(child0, Units::Pixels(80.0));
    world.set_height(child0, Units::Pixels(50.0));

    let child1 = world.add(Some(root));
    world.set_column_start(child1, 0);
    world.set_row_start(child1, 1);
    world.set_width(child1, Units::Pixels(80.0));
    world.set_height(child1, Units::Stretch(1.0));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    // Auto row=50, gap=10, pixels row=100 → child1 at y=60.
    assert_eq!(world.cache.bounds(child0), Some(&Rect { posx: 0.0, posy: 0.0, width: 200.0, height: 50.0 }));
    assert_eq!(world.cache.bounds(child1), Some(&Rect { posx: 0.0, posy: 60.0, width: 200.0, height: 100.0 }));
}

/// Multiple children in the same Auto row: row takes the maximum natural height.
#[test]
fn auto_row_multiple_children_max_height() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(400.0));
    world.set_layout_type(root, LayoutType::Grid);
    world.set_alignment(root, Alignment::TopLeft);
    world.set_grid_columns(root, vec![Units::Pixels(100.0), Units::Pixels(100.0), Units::Pixels(100.0)]);
    world.set_grid_rows(root, vec![Units::Auto]);

    for (col, height) in [(0usize, 40.0f32), (1, 80.0f32), (2, 55.0f32)] {
        let child = world.add(Some(root));
        world.set_column_start(child, col);
        world.set_row_start(child, 0);
        world.set_width(child, Units::Pixels(80.0));
        world.set_height(child, Units::Pixels(height));
    }

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    // Row = max(40, 80, 55) = 80. All cells 100-wide and 80-tall.
    let children: Vec<_> = root.children(&world.tree).filter(|c| c.visible(&world.store)).collect();

    assert_eq!(world.cache.bounds(*children[0]), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 80.0 }));
    assert_eq!(world.cache.bounds(*children[1]), Some(&Rect { posx: 100.0, posy: 0.0, width: 100.0, height: 80.0 }));
    assert_eq!(world.cache.bounds(*children[2]), Some(&Rect { posx: 200.0, posy: 0.0, width: 100.0, height: 80.0 }));
}

// ────────────────────────────────────────────────────────────────────────────
// Auto columns AND Auto rows together
// ────────────────────────────────────────────────────────────────────────────

/// 2×2 grid with all Auto tracks. Columns and rows each size independently.
#[test]
fn auto_columns_and_rows_2x2() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_layout_type(root, LayoutType::Grid);
    world.set_alignment(root, Alignment::TopLeft);
    world.set_grid_columns(root, vec![Units::Auto, Units::Auto]);
    world.set_grid_rows(root, vec![Units::Auto, Units::Auto]);

    // (col, row, w, h)
    for &(col, row, w, h) in
        &[(0usize, 0usize, 100.0f32, 60.0f32), (1, 0, 80.0, 60.0), (0, 1, 100.0, 40.0), (1, 1, 80.0, 40.0)]
    {
        let child = world.add(Some(root));
        world.set_column_start(child, col);
        world.set_row_start(child, row);
        world.set_width(child, Units::Pixels(w));
        world.set_height(child, Units::Pixels(h));
    }

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    // col0=100, col1=80, row0=60, row1=40.
    let children: Vec<_> = root.children(&world.tree).filter(|c| c.visible(&world.store)).collect();

    assert_eq!(world.cache.bounds(*children[0]), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 60.0 }));
    assert_eq!(world.cache.bounds(*children[1]), Some(&Rect { posx: 100.0, posy: 0.0, width: 80.0, height: 60.0 }));
    assert_eq!(world.cache.bounds(*children[2]), Some(&Rect { posx: 0.0, posy: 60.0, width: 100.0, height: 40.0 }));
    assert_eq!(world.cache.bounds(*children[3]), Some(&Rect { posx: 100.0, posy: 60.0, width: 80.0, height: 40.0 }));
}

/// Auto column is sized from a child that reports its width via content_size.
#[test]
fn auto_column_content_size_child() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(400.0));
    world.set_layout_type(root, LayoutType::Grid);
    world.set_alignment(root, Alignment::TopLeft);
    world.set_grid_columns(root, vec![Units::Auto]);
    world.set_grid_rows(root, vec![Units::Pixels(100.0)]);

    let child = world.add(Some(root));
    world.set_column_start(child, 0);
    world.set_row_start(child, 0);
    // Child always reports content width=180 regardless of parent constraint.
    world.set_width(child, Units::Auto);
    world.set_height(child, Units::Auto);
    world.set_layout_type(child, LayoutType::Row);
    world.set_content_size(child, |_, width, height| (width.unwrap_or(180.0).max(180.0), height.unwrap_or(40.0)));

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    // Auto column = 180 (content width). Cell: (0, 0, 180, 100).
    assert_eq!(world.cache.bounds(child), Some(&Rect { posx: 0.0, posy: 0.0, width: 180.0, height: 100.0 }));
}

/// Auto row height is determined by a child whose height depends on the column width
/// (simulates text wrapping: height = ceil(2000 / width)).
#[test]
fn auto_row_height_depends_on_column_width() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_layout_type(root, LayoutType::Grid);
    world.set_alignment(root, Alignment::TopLeft);
    world.set_grid_columns(root, vec![Units::Pixels(200.0)]);
    world.set_grid_rows(root, vec![Units::Auto]);

    let child = world.add(Some(root));
    world.set_column_start(child, 0);
    world.set_row_start(child, 0);
    world.set_width(child, Units::Stretch(1.0));
    world.set_height(child, Units::Auto);
    world.set_layout_type(child, LayoutType::Row);
    world.set_content_size(child, |_, width, _| {
        let w = width.unwrap_or(1.0).max(1.0);
        (w, (2000.0f32 / w).ceil())
    });

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());

    // Auto row height = ceil(2000/200) = 10. Cell: (0, 0, 200, 10).
    assert_eq!(world.cache.bounds(child), Some(&Rect { posx: 0.0, posy: 0.0, width: 200.0, height: 10.0 }));
}
