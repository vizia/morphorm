use morphorm::*;
use morphorm_ecs::*;

use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};

// Helper function for building a tree of nodes.
fn build_tree(world: &mut World, parent: Option<Entity>, children_per_node: usize, depth: usize) {
    if depth > 0 {
        let node = world.add(parent);
        world.set_all_stretch(node);
        for _ in 0..children_per_node {
            build_tree(world, Some(node), children_per_node, depth - 1)
        }
    }
}

fn morphorm_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("Shallow Tree");
    group.sample_size(10);

    // 3 - 1000, 4 - 10,000, 5 - 100,000, 6 - 1,000,000
    for depth in [3, 4, 5, 6].iter() {
        let benchmark_id = BenchmarkId::new(format!("Wide Shallow. 10 children per node, depth: {depth}. Total nodes: {}.", 10u32.pow(*depth as u32)), depth);
        group.bench_with_input(benchmark_id, depth, |b, &depth| {
            b.iter_batched(
                || {
                    let mut world = World::default();
                    let root = world.add(None);
                    world.set_width(root, Units::Pixels(1000.0));
                    world.set_height(root, Units::Pixels(1000.0));
                    build_tree(&mut world, Some(root), 10, depth);
                    (world, root)
                },
                |(mut world, root)| root.layout(&mut world.cache, &world.tree, &world.store),
                criterion::BatchSize::SmallInput,
            )
        });
    }

    group.finish();

    let mut group = c.benchmark_group("Deep Tree");
    group.sample_size(10);

    // 10 - 1024, 13 - 8192, 17 - 131,072, 20 - 1,048,576
    for depth in [10, 13, 17, 20].iter() {
        let benchmark_id = BenchmarkId::new(format!("Narrow Deep. 2 children per node, depth: {depth}. Total nodes: {}.", 2u32.pow(*depth as u32)), depth);
        group.bench_with_input(benchmark_id, depth, |b, &depth| {
            b.iter_batched(
                || {
                    let mut world = World::default();
                    let root = world.add(None);
                    world.set_width(root, Units::Pixels(1000.0));
                    world.set_height(root, Units::Pixels(1000.0));
                    build_tree(&mut world, Some(root), 2, depth);
                    (world, root)
                },
                |(mut world, root)| root.layout(&mut world.cache, &world.tree, &world.store),
                criterion::BatchSize::SmallInput,
            )
        });
    }

    group.finish()

}

criterion_group!(benches, morphorm_benchmarks);
criterion_main!(benches);
