use morphorm::*;
use morphorm_ecs::*;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

// Helper function for building a tree of nodes.
fn build_tree(world: &mut World, parent: Option<Entity>, children_per_node: usize, depth: usize) -> Entity {
    if (depth + 1) > 0 {
        let node = world.add(parent);
        if parent.is_none() {
            world.set_width(node, Units::Pixels(1000.0));
            world.set_height(node, Units::Pixels(1000.0));
        } else {
            world.set_all_stretch(node);
        }
        for _ in 0..children_per_node {
            build_tree(world, Some(node), children_per_node, depth - 1);
        }

        return node;
    }

    Entity(0)
}

fn morphorm_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("Shallow Tree");
    group.sample_size(10);

    // 3 - 1111, 4 - 11,111, 5 - 111,111, 6 - 1,111,111
    for depth in [3, 4, 5, 6].iter() {
        let benchmark_id = BenchmarkId::new(
            format!("Wide Shallow. 10 children per node, depth: {depth}. Total nodes: {}.", 10u32.pow(*depth as u32)),
            depth,
        );
        group.bench_with_input(benchmark_id, depth, |b, &depth| {
            b.iter_batched(
                || {
                    let mut world = World::default();
                    let root = build_tree(&mut world, None, 10, depth);
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

    // 10 - 1023, 12 - 8191, 16 - 131,071, 19 - 1,048,575
    for depth in [9, 12, 16, 19].iter() {
        let benchmark_id = BenchmarkId::new(
            format!("Narrow Deep. 2 children per node, depth: {depth}. Total nodes: {}.", 2u32.pow(*depth as u32)),
            depth,
        );
        group.bench_with_input(benchmark_id, depth, |b, &depth| {
            b.iter_batched(
                || {
                    let mut world = World::default();
                    let root = build_tree(&mut world, None, 2, depth);
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
