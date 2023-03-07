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

/// Prints a debug representation of the computed layout for a tree of nodes, starting with the given root node.
fn print_node(
    world: &World,
    node: &impl Node<Tree = Tree, CacheKey = Entity>,
    is_root: bool,
    has_sibling: bool,
    lines_string: String,
) {
    let entity = node.key();

    let fork_string = if is_root {
        "│"
    } else if has_sibling {
        "├───┤"
    } else {
        "└───┤"
    };
    println!(
        "{lines}{fork}{id}| {x:#3} {y:#3} {w:#3} {h:#3}│",
        lines = lines_string,
        fork = fork_string,
        id = entity.0,
        x = world.cache.posx(entity),
        y = world.cache.posx(entity),
        w = world.cache.width(entity),
        h = world.cache.height(entity),
    );
    let bar = if is_root {
        ""
    } else if has_sibling {
        "│   "
    } else {
        "    "
    };
    let new_string = lines_string + bar;

    for child in node.children(&world.tree) {
        let has_sibling = world.tree.get_next_sibling(&child.key()).is_some();
        print_node(world, child, false, has_sibling, new_string.clone());
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
            |(mut world, root)| {
                layout(&root, None, None, None, &mut world.cache, &world.tree, &world.store)
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, wide_shallow_tree);
criterion_main!(benches);
