use morphorm_ecs::*;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn build_shallow_tree(world: &mut World, parent: Option<Entity>, depth: usize) {
    if depth > 0 {
        let node = world.add(parent);
        world.set_all_stetch(node);
        for _ in 0..1000 {
            build_shallow_tree(world, Some(node), depth - 1)
        }
    }
}

fn wide_shallow_tree(c: &mut Criterion) {
    let mut world = World::default();
    build_shallow_tree(&mut world, None, 3);
    // c.bench_with_input("fib 20", |b| b.iter_batched(|| build_shallow_tree(&mut world, parent, depth)|| fibonacci(black_box(20))));
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
