use morphorm::*;
use morphorm_ecs::*;

use criterion::{criterion_group, criterion_main, Criterion};

fn build_shallow_tree(world: &mut World, parent: Option<Entity>, depth: usize) {
    if depth > 0 {
        let node = world.add(parent);
        world.set_all_stretch(node);
        for _ in 0..10 {
            build_shallow_tree(world, Some(node), depth - 1)
        }
    }
}

fn wide_shallow_tree(c: &mut Criterion) {
    // let mut world = World::default();
    // let root = world.add(None);
    // build_shallow_tree(&mut world, Some(root), 3);
    // print_node(&world, &root, true, false, String::new());
    c.bench_function("lvl3", |b| {
        b.iter_batched(
            || {
                let mut world = World::default();
                let root = world.add(None);
                world.set_width(root, Units::Pixels(1000.0));
                world.set_height(root, Units::Pixels(1000.0));
                build_shallow_tree(&mut world, Some(root), 3);
                (world, root)
            },
            |(mut world, root)| layout(&root, None, None, None, &mut world.cache, &world.tree, &world.store),
            criterion::BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, wide_shallow_tree);
criterion_main!(benches);
