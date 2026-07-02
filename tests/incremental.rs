use morphorm::*;
use morphorm_ecs::*;

/// Collect the computed bounds of the given entities into a comparable snapshot.
fn snapshot(world: &World, entities: &[Entity]) -> Vec<Option<Rect>> {
    entities.iter().map(|entity| world.cache.bounds(*entity).copied()).collect()
}

/// Perform a full layout pass from the root.
fn full_layout(world: &mut World, root: Entity) {
    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());
}

/// Perform an incremental layout pass starting from the given (dirty) node.
fn incremental_layout(world: &mut World, node: Entity) {
    node.layout(&mut world.cache, &world.tree, &world.store, &mut ());
}

/// A change under a fixed (pixels) parent restarts layout at the parent and repositions siblings,
/// producing the same result as a full layout pass.
#[test]
fn incremental_matches_full_under_fixed_parent() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_alignment(root, Alignment::TopLeft);
    world.set_layout_type(root, LayoutType::Column);

    let parent = world.add(Some(root));
    world.set_width(parent, Units::Pixels(400.0));
    world.set_height(parent, Units::Pixels(400.0));
    world.set_layout_type(parent, LayoutType::Column);

    let a = world.add(Some(parent));
    world.set_width(a, Units::Pixels(100.0));
    world.set_height(a, Units::Pixels(100.0));

    let b = world.add(Some(parent));
    world.set_width(b, Units::Pixels(100.0));
    world.set_height(b, Units::Pixels(100.0));

    full_layout(&mut world, root);

    // `a` grows; its parent is a fixed-pixels node, so layout restarts at the parent.
    world.set_height(a, Units::Pixels(250.0));
    incremental_layout(&mut world, a);
    let incremental = snapshot(&world, &[root, parent, a, b]);

    // Ground truth: a full layout pass must produce the same result.
    full_layout(&mut world, root);
    let full = snapshot(&world, &[root, parent, a, b]);

    assert_eq!(incremental, full);
    // `b` should have been pushed down by `a` growing.
    assert_eq!(world.cache.bounds(b).unwrap().posy, 250.0);
}

/// A change under an `Auto`-sized parent bubbles up to the nearest fixed ancestor, because the
/// auto parent resizes to fit its children and therefore affects its own parent.
#[test]
fn incremental_bubbles_through_auto_parent() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_alignment(root, Alignment::TopLeft);
    world.set_layout_type(root, LayoutType::Column);

    let grandparent = world.add(Some(root));
    world.set_width(grandparent, Units::Pixels(500.0));
    world.set_height(grandparent, Units::Pixels(500.0));
    world.set_layout_type(grandparent, LayoutType::Column);

    // Auto height -> fits its children on the main (vertical) axis.
    let parent = world.add(Some(grandparent));
    world.set_width(parent, Units::Pixels(300.0));
    world.set_height(parent, Units::Auto);
    world.set_layout_type(parent, LayoutType::Column);

    // Sibling positioned below `parent` within `grandparent`.
    let sibling = world.add(Some(grandparent));
    world.set_width(sibling, Units::Pixels(50.0));
    world.set_height(sibling, Units::Pixels(50.0));

    let child = world.add(Some(parent));
    world.set_width(child, Units::Pixels(100.0));
    world.set_height(child, Units::Pixels(100.0));

    full_layout(&mut world, root);
    // Auto parent initially fits the 100px child, so the sibling sits at y = 100.
    assert_eq!(world.cache.bounds(sibling).unwrap().posy, 100.0);

    // Growing the child grows the auto parent, which shifts the sibling: must restart high enough.
    world.set_height(child, Units::Pixels(220.0));
    incremental_layout(&mut world, child);
    let incremental = snapshot(&world, &[grandparent, parent, sibling, child]);

    full_layout(&mut world, root);
    let full = snapshot(&world, &[grandparent, parent, sibling, child]);

    assert_eq!(incremental, full);
    assert_eq!(world.cache.bounds(sibling).unwrap().posy, 220.0);
}

/// A `Stretch`-sized ancestor is a valid restart point: its size is determined by the parent's
/// allocation (unaffected by its own descendants) and is reproduced from the cached size.
#[test]
fn incremental_restarts_at_stretch_ancestor() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_alignment(root, Alignment::TopLeft);
    world.set_layout_type(root, LayoutType::Row);

    // Stretch on both axes -> fills the root.
    let stretch = world.add(Some(root));
    world.set_width(stretch, Units::Stretch(1.0));
    world.set_height(stretch, Units::Stretch(1.0));
    world.set_layout_type(stretch, LayoutType::Column);

    let child = world.add(Some(stretch));
    world.set_width(child, Units::Pixels(100.0));
    world.set_height(child, Units::Pixels(100.0));

    let child2 = world.add(Some(stretch));
    world.set_width(child2, Units::Pixels(100.0));
    world.set_height(child2, Units::Pixels(100.0));

    full_layout(&mut world, root);
    assert_eq!(world.cache.bounds(child2).unwrap().posy, 100.0);

    world.set_height(child, Units::Pixels(180.0));
    incremental_layout(&mut world, child);
    let incremental = snapshot(&world, &[root, stretch, child, child2]);

    full_layout(&mut world, root);
    let full = snapshot(&world, &[root, stretch, child, child2]);

    assert_eq!(incremental, full);
    assert_eq!(world.cache.bounds(child2).unwrap().posy, 180.0);
}

/// A `Percentage`-sized ancestor cannot be reproduced from the cached size alone, so the search
/// bubbles past it up to the fixed root, still matching a full layout pass.
#[test]
fn incremental_bubbles_through_percentage_ancestor() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_alignment(root, Alignment::TopLeft);
    world.set_layout_type(root, LayoutType::Column);

    let percent = world.add(Some(root));
    world.set_width(percent, Units::Percentage(50.0));
    world.set_height(percent, Units::Percentage(50.0));
    world.set_layout_type(percent, LayoutType::Column);

    let child = world.add(Some(percent));
    world.set_width(child, Units::Pixels(100.0));
    world.set_height(child, Units::Pixels(100.0));

    let child2 = world.add(Some(percent));
    world.set_width(child2, Units::Pixels(100.0));
    world.set_height(child2, Units::Pixels(100.0));

    full_layout(&mut world, root);

    world.set_height(child, Units::Pixels(150.0));
    incremental_layout(&mut world, child);
    let incremental = snapshot(&world, &[root, percent, child, child2]);

    full_layout(&mut world, root);
    let full = snapshot(&world, &[root, percent, child, child2]);

    assert_eq!(incremental, full);
    // The percentage container keeps its size (50% of 600 = 300) while children reflow.
    assert_eq!(world.cache.bounds(percent).unwrap().height, 300.0);
    assert_eq!(world.cache.bounds(child2).unwrap().posy, 150.0);
}

/// Calling layout on the root is a full layout pass (regression: root behavior is unchanged).
#[test]
fn incremental_on_root_is_full_layout() {
    let mut world = World::default();

    let root = world.add(None);
    world.set_width(root, Units::Pixels(600.0));
    world.set_height(root, Units::Pixels(600.0));
    world.set_alignment(root, Alignment::TopLeft);
    world.set_layout_type(root, LayoutType::Row);

    let a = world.add(Some(root));
    world.set_width(a, Units::Pixels(100.0));
    world.set_height(a, Units::Pixels(150.0));

    let b = world.add(Some(root));
    world.set_width(b, Units::Pixels(120.0));
    world.set_height(b, Units::Pixels(150.0));

    incremental_layout(&mut world, root);

    assert_eq!(world.cache.bounds(a), Some(&Rect { posx: 0.0, posy: 0.0, width: 100.0, height: 150.0 }));
    assert_eq!(world.cache.bounds(b), Some(&Rect { posx: 100.0, posy: 0.0, width: 120.0, height: 150.0 }));
}
