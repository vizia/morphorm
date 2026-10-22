#[path = "pangui/fixture.rs"]
mod fixture;

use std::fs;
use std::path::PathBuf;

use criterion::{criterion_group, criterion_main, BatchSize, Criterion, Throughput};
use fixture::{Fixture, FixtureError};
use morphorm::Node;

fn fixture_paths() -> Vec<PathBuf> {
    let fixture_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("benches/pangui");
    let mut paths: Vec<_> = fs::read_dir(&fixture_directory)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", fixture_directory.display()))
        .map(|entry| entry.expect("failed to read Pangui fixture directory entry").path())
        .filter(|path| path.extension().is_some_and(|extension| extension == "txt"))
        .collect();
    paths.sort();
    paths
}

fn pangui_benchmarks(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("Pangui");
    group.sample_size(10);

    for path in fixture_paths() {
        let source =
            fs::read_to_string(&path).unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));
        let fixture = match Fixture::parse(&source) {
            Ok(fixture) => fixture,
            Err(error @ FixtureError::Unsupported { .. }) => {
                eprintln!(
                    "Skipping Pangui fixture `{}` because `{}` is unsupported",
                    path.display(),
                    error.unsupported_feature().unwrap()
                );
                continue;
            }
            Err(error) => panic!("failed to parse {}: {error}", path.display()),
        };
        let benchmark_name = path.file_stem().unwrap().to_string_lossy().into_owned();

        group.throughput(Throughput::Elements(fixture.expanded_node_count() as u64));
        group.bench_function(benchmark_name, |bencher| {
            bencher.iter_batched(
                || fixture.build_world(),
                |(mut world, root)| {
                    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());
                },
                BatchSize::LargeInput,
            );
        });
    }

    group.finish();
}

criterion_group!(benches, pangui_benchmarks);
criterion_main!(benches);
