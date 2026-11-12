//! Reproduces arewelayoutyet's `propagation_1373` incremental benchmark.
//! Run with `cargo run --release --example propagation -- 20`.

mod adapter;

use std::{error::Error, hint::black_box, time::Instant};

use adapter::{Bounds, LayoutCache, NodeId, NodeStyle, Tree};
use morphorm::{LayoutType, Node, Units};

const VIEWPORT: f32 = 16384.0;

struct Benchmark {
    tree: Tree,
    store: Vec<NodeStyle>,
    cache: LayoutCache,
    viewport: NodeId,
    leaves: Vec<NodeId>,
}

impl Benchmark {
    fn new() -> Self {
        let mut benchmark = Self {
            tree: Tree::default(),
            store: Vec::new(),
            cache: LayoutCache::default(),
            viewport: NodeId(0),
            leaves: Vec::new(),
        };
        let viewport_style = NodeStyle {
            layout_type: LayoutType::Column,
            width: Units::Pixels(VIEWPORT),
            height: Units::Pixels(VIEWPORT),
            ..Default::default()
        };
        let viewport = benchmark.add_node(viewport_style, None);
        let mut parent = benchmark.add_node(
            NodeStyle { width: Units::Pixels(VIEWPORT), height: Units::Pixels(VIEWPORT), ..Self::template_style() },
            Some(viewport),
        );
        for _ in 0..7 {
            parent = benchmark.add_node(Self::template_style(), Some(parent));
        }
        benchmark.add_branch(parent, 5);
        benchmark.cache.0.resize(benchmark.store.len(), Bounds::default());
        benchmark
    }

    fn template_style() -> NodeStyle {
        NodeStyle {
            layout_type: LayoutType::Column,
            padding_left: Units::Pixels(0.0),
            padding_right: Units::Pixels(0.0),
            padding_top: Units::Pixels(0.0),
            padding_bottom: Units::Pixels(0.0),
            gap: Units::Pixels(0.0),
            ..Default::default()
        }
    }

    fn add_node(&mut self, style: NodeStyle, parent: Option<NodeId>) -> NodeId {
        let node = NodeId(self.store.len());
        self.store.push(style);
        self.tree.children.push(Vec::new());
        self.tree.parents.push(parent);
        if let Some(parent) = parent {
            self.tree.children[parent.0].push(node);
        }
        node
    }

    fn add_branch(&mut self, parent: NodeId, depth: usize) {
        let leaf = depth == 0;
        let node = self.add_node(
            NodeStyle {
                layout_type: if !leaf && depth % 2 == 0 { LayoutType::Row } else { LayoutType::Column },
                width: if leaf { Units::Pixels(16.0) } else { Units::Auto },
                height: if leaf { Units::Pixels(16.0) } else { Units::Auto },
                ..Self::template_style()
            },
            Some(parent),
        );
        if leaf {
            self.leaves.push(node);
        } else {
            for _ in 0..4 {
                self.add_branch(node, depth - 1);
            }
        }
    }

    fn compute_layout(&mut self) {
        self.viewport.layout(&mut self.cache, &self.tree, &self.store, &mut ());
        black_box(&self.cache);
    }

    fn mutate_and_relayout(&mut self, height: f32) {
        let mut dirty = self.tree.children[self.viewport.0][0];
        for index in [0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0] {
            dirty = self.tree.children[dirty.0][index];
        }
        self.store[dirty.0].height = Units::Pixels(height);
        dirty.layout(&mut self.cache, &self.tree, &self.store, &mut ());
        black_box(&self.cache);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let iterations = std::env::args().nth(1).map(|value| value.parse::<u32>()).transpose()?.unwrap_or(20);
    if iterations == 0 {
        return Err("iterations must be greater than zero".into());
    }

    let mut benchmark = Benchmark::new();
    benchmark.compute_layout();
    if benchmark.store.len() != 1374 || benchmark.leaves.len() != 1024 {
        return Err("unexpected workload dimensions".into());
    }
    for height in [24.0, 16.0, 24.0, 16.0] {
        benchmark.mutate_and_relayout(height);
        let mut fresh = Benchmark::new();
        fresh.store[fresh.leaves[512].0].height = Units::Pixels(height);
        fresh.compute_layout();
        for (actual, expected) in benchmark.cache.0.iter().zip(&fresh.cache.0) {
            let actual = [actual.x, actual.y, actual.width, actual.height];
            let expected = [expected.x, expected.y, expected.width, expected.height];
            for (actual, expected) in actual.into_iter().zip(expected) {
                let tolerance = 0.001 + expected.abs() * 0.00001;
                if !actual.is_finite() || (actual - expected).abs() > tolerance {
                    return Err("incremental layout differs from a fresh full layout".into());
                }
            }
        }
    }

    let mut elapsed = std::time::Duration::ZERO;
    for iteration in 0..iterations {
        let height = if iteration % 2 == 0 { 24.0 } else { 16.0 };
        let start = Instant::now();
        benchmark.mutate_and_relayout(height);
        elapsed += start.elapsed();
    }

    println!("propagation: {} nodes (+1 viewport), mutating leaf 512", benchmark.store.len() - 1);
    println!("{iterations} incremental layouts: {elapsed:?}, average: {:?}", elapsed / iterations);
    Ok(())
}
