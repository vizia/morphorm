use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, Throughput};
use morphorm::*;
use morphorm_ecs::*;

fn root(world: &mut World, layout_type: LayoutType, width: f32, height: f32) -> Entity {
    let root = world.add(None);
    world.set_layout_type(root, layout_type);
    world.set_width(root, Units::Pixels(width));
    world.set_height(root, Units::Pixels(height));
    world.set_alignment(root, Alignment::TopLeft);
    root
}

fn build_stack(world: &mut World, parent: Entity, layout_type: LayoutType, breadth: usize, depth: usize) {
    if depth == 0 {
        return;
    }

    for index in 0..breadth {
        let child = world.add(Some(parent));
        world.set_layout_type(child, layout_type);
        world.set_width(child, Units::Stretch(1.0));
        world.set_height(child, if index % 3 == 0 { Units::Pixels(24.0) } else { Units::Stretch(1.0) });
        build_stack(world, child, layout_type, breadth, depth - 1);
    }
}

fn stack_world(layout_type: LayoutType) -> (World, Entity) {
    let mut world = World::default();
    let root = root(&mut world, layout_type, 1200.0, 900.0);
    build_stack(&mut world, root, layout_type, 8, 3);
    (world, root)
}

fn wrap_world(layout_type: LayoutType) -> (World, Entity) {
    let mut world = World::default();
    let root = root(&mut world, layout_type, 800.0, 600.0);
    world.set_wrap(root, LayoutWrap::Wrap);
    world.set_horizontal_gap(root, Units::Pixels(4.0));
    world.set_vertical_gap(root, Units::Pixels(4.0));

    for index in 0..256 {
        let child = world.add(Some(root));
        world.set_width(child, if index % 4 == 0 { Units::Stretch(1.0) } else { Units::Pixels(48.0) });
        world.set_height(child, if index % 5 == 0 { Units::Stretch(1.0) } else { Units::Pixels(24.0) });
        world.set_min_width(child, Units::Pixels(20.0));
        world.set_min_height(child, Units::Pixels(16.0));
    }

    (world, root)
}

fn overlay_world(auto_size: bool) -> (World, Entity) {
    let mut world = World::default();
    let root = root(&mut world, LayoutType::Overlay, 800.0, 600.0);
    world.set_alignment(root, Alignment::Center);

    if auto_size {
        world.set_width(root, Units::Auto);
        world.set_height(root, Units::Auto);
        world.set_min_width(root, Units::Auto);
        world.set_min_height(root, Units::Auto);
    }

    for index in 0..256 {
        let child = world.add(Some(root));
        world.set_width(child, Units::Pixels(32.0 + (index % 8) as f32 * 4.0));
        world.set_height(child, Units::Pixels(20.0 + (index % 5) as f32 * 3.0));
    }

    (world, root)
}

fn grid_world(auto_tracks: bool) -> (World, Entity) {
    const SIDE: usize = 16;

    let mut world = World::default();
    let root = root(&mut world, LayoutType::Grid, 1024.0, 768.0);
    let track = if auto_tracks { Units::Auto } else { Units::Stretch(1.0) };
    world.set_grid_columns(root, vec![track; SIDE]);
    world.set_grid_rows(root, vec![track; SIDE]);
    world.set_horizontal_gap(root, Units::Pixels(2.0));
    world.set_vertical_gap(root, Units::Pixels(2.0));

    for row in 0..SIDE {
        for column in 0..SIDE {
            let child = world.add(Some(root));
            world.set_column_start(child, column);
            world.set_row_start(child, row);
            world.set_width(child, Units::Pixels(24.0 + (column % 4) as f32 * 4.0));
            world.set_height(child, Units::Pixels(18.0 + (row % 3) as f32 * 3.0));
        }
    }

    (world, root)
}

fn incremental_world(auto_parent: bool) -> (World, Entity, Entity) {
    let mut world = World::default();
    let root = root(&mut world, LayoutType::Column, 1200.0, 900.0);
    let mut dirty = root;

    for branch in 0..8 {
        let parent = world.add(Some(root));
        world.set_layout_type(parent, LayoutType::Column);
        world.set_width(parent, Units::Pixels(120.0));
        world.set_height(parent, if auto_parent && branch == 0 { Units::Auto } else { Units::Pixels(100.0) });

        for child_index in 0..64 {
            let child = world.add(Some(parent));
            world.set_width(child, Units::Pixels(20.0));
            world.set_height(child, Units::Pixels(12.0));
            if branch == 0 && child_index == 0 {
                dirty = child;
            }
        }
    }

    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());
    world.set_height(dirty, Units::Pixels(16.0));
    (world, root, dirty)
}

fn benchmark_full_layout(c: &mut Criterion) {
    let mut group = c.benchmark_group("layout modes");
    group.sample_size(20);

    for layout_type in [LayoutType::Row, LayoutType::Column] {
        group.throughput(Throughput::Elements(585));
        group.bench_with_input(
            BenchmarkId::new("stack", format!("{layout_type:?}")),
            &layout_type,
            |b, &layout_type| {
                b.iter_batched(
                    || stack_world(layout_type),
                    |(mut world, root)| root.layout(&mut world.cache, &world.tree, &world.store, &mut ()),
                    BatchSize::SmallInput,
                );
            },
        );
    }

    for layout_type in [LayoutType::Row, LayoutType::Column] {
        group.throughput(Throughput::Elements(257));
        group.bench_with_input(
            BenchmarkId::new("wrap", format!("{layout_type:?}")),
            &layout_type,
            |b, &layout_type| {
                b.iter_batched(
                    || wrap_world(layout_type),
                    |(mut world, root)| root.layout(&mut world.cache, &world.tree, &world.store, &mut ()),
                    BatchSize::SmallInput,
                );
            },
        );
    }

    for auto_size in [false, true] {
        group.throughput(Throughput::Elements(257));
        group.bench_with_input(
            BenchmarkId::new("overlay", if auto_size { "auto" } else { "fixed" }),
            &auto_size,
            |b, &auto_size| {
                b.iter_batched(
                    || overlay_world(auto_size),
                    |(mut world, root)| root.layout(&mut world.cache, &world.tree, &world.store, &mut ()),
                    BatchSize::SmallInput,
                );
            },
        );
    }

    for auto_tracks in [false, true] {
        group.throughput(Throughput::Elements(257));
        group.bench_with_input(
            BenchmarkId::new("grid", if auto_tracks { "auto" } else { "stretch" }),
            &auto_tracks,
            |b, &auto_tracks| {
                b.iter_batched(
                    || grid_world(auto_tracks),
                    |(mut world, root)| root.layout(&mut world.cache, &world.tree, &world.store, &mut ()),
                    BatchSize::SmallInput,
                );
            },
        );
    }

    group.finish();
}

fn benchmark_incremental_layout(c: &mut Criterion) {
    let mut group = c.benchmark_group("incremental layout");
    group.sample_size(20);
    group.throughput(Throughput::Elements(521));

    for auto_parent in [false, true] {
        group.bench_with_input(
            BenchmarkId::new("dirty leaf", if auto_parent { "auto parent" } else { "fixed parent" }),
            &auto_parent,
            |b, &auto_parent| {
                b.iter_batched(
                    || incremental_world(auto_parent),
                    |(mut world, _root, dirty)| dirty.layout(&mut world.cache, &world.tree, &world.store, &mut ()),
                    BatchSize::SmallInput,
                );
            },
        );
    }

    group.finish();
}

criterion_group!(benches, benchmark_full_layout, benchmark_incremental_layout);
criterion_main!(benches);
