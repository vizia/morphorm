use morphorm::*;
use morphorm_ecs::*;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

// Helper function for building a tree of nodes.
fn build_tree(world: &mut World, parent: Option<Entity>, children_per_node: usize, depth: usize) -> Entity {
    let node = world.add(parent);
    if parent.is_none() {
        world.set_width(node, Units::Pixels(1000.0));
        world.set_height(node, Units::Pixels(1000.0));
    } else {
        world.set_all_stretch(node);
    }

    let depth = depth.saturating_sub(1);

    if depth > 0 {
        for _ in 0..children_per_node {
            build_tree(world, Some(node), children_per_node, depth);
        }

        return node;
    }

    Entity(0)
}

// FIXME: There's almost certainly a formula for this geometric series.
fn compute_node_count(children_per_node: usize, depth: usize, node_count: &mut usize) -> usize {
    *node_count += 1;
    let depth = depth.saturating_sub(1);
    if depth > 0 {
        for _ in 0..children_per_node {
            compute_node_count(children_per_node, depth, node_count);
        }
    }

    return *node_count;
}

fn morphorm_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("Shallow Tree");
    group.sample_size(10);

    let children_per_node = 10;

    // 3 - 111, 4 - 1,111, 5 - 11,111, 6 - 111,111 7 - 1,111,111
    for depth in [4, 5, 6, 7].iter() {
        let benchmark_id = BenchmarkId::new(
            format!(
                "Wide Shallow Bench. {children_per_node} children per node, depth: {depth}. Total nodes: {}.",
                compute_node_count(children_per_node, *depth, &mut 0)
            ),
            depth,
        );
        group.bench_with_input(benchmark_id, depth, |b, &depth| {
            b.iter_batched(
                || {
                    let mut world = World::default();
                    let root = build_tree(&mut world, None, children_per_node, depth);
                    (world, root)
                },
                |(mut world, root)| root.layout(&mut world.cache, &world.tree, &world.store, &mut ()),
                criterion::BatchSize::SmallInput,
            )
        });
    }

    group.finish();

    let mut group = c.benchmark_group("Deep Tree");
    group.sample_size(10);

    let children_per_node = 2;

    // 10 - 1023, 12 - 8191, 16 - 131,071, 19 - 1,048,575
    for depth in [10, 13, 17, 20].iter() {
        let benchmark_id = BenchmarkId::new(
            format!(
                "Narrow Deep Bench. {children_per_node} children per node, depth: {depth}. Total nodes: {}.",
                compute_node_count(children_per_node, *depth, &mut 0)
            ),
            depth,
        );
        group.bench_with_input(benchmark_id, depth, |b, &depth| {
            b.iter_batched(
                || {
                    let mut world = World::default();
                    let root = build_tree(&mut world, None, children_per_node, depth);
                    (world, root)
                },
                |(mut world, root)| root.layout(&mut world.cache, &world.tree, &world.store, &mut ()),
                criterion::BatchSize::SmallInput,
            )
        });
    }

    group.finish();

    let mut group = c.benchmark_group("Super Deep Tree");
    group.sample_size(10);

    let children_per_node = 1;

    let depth = 1000usize;
    let benchmark_id = BenchmarkId::new(
        format!(
            "Super Deep Bench. {children_per_node} child per node, depth: {depth}. Total nodes: {}.",
            compute_node_count(children_per_node, depth, &mut 0)
        ),
        depth,
    );
    group.bench_with_input(benchmark_id, &depth, |b, &depth| {
        b.iter_batched(
            || {
                let mut world = World::default();
                let root = build_tree(&mut world, None, children_per_node, depth);
                (world, root)
            },
            |(mut world, root)| root.layout(&mut world.cache, &world.tree, &world.store, &mut ()),
            criterion::BatchSize::SmallInput,
        )
    });

    group.finish();
}

criterion_group!(benches, morphorm_benchmarks);
criterion_main!(benches);
