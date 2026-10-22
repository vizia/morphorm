#[path = "../benches/pangui/fixture.rs"]
mod fixture;

use std::fs;
use std::path::PathBuf;

use fixture::{Fixture, FixtureError};
use morphorm::{LayoutType, LayoutWrap, Node, Units};

#[test]
fn parses_repeats_and_maps_layout_properties() {
    let fixture = Fixture::parse(
        "Width(Pixels(200)).Height(Fit).Vertical().Padding(10).Gap(5) {\
            Repeat(3) { Width(Expand).Height(Pixels(10)) {} }\
        }",
    )
    .unwrap();

    assert_eq!(fixture.expanded_node_count(), 4);
    let (world, root) = fixture.build_world();
    assert_eq!(root.width(&world.store), Some(Units::Pixels(200.0)));
    assert_eq!(root.height(&world.store), Some(Units::Auto));
    assert_eq!(root.layout_type(&world.store), Some(LayoutType::Column));
    assert_eq!(root.padding_left(&world.store), Some(Units::Pixels(10.0)));
    assert_eq!(root.horizontal_gap(&world.store), Some(Units::Pixels(5.0)));
}

#[test]
fn maps_percentage_ratio_and_wrap() {
    let fixture = Fixture::parse(
        "Width(Pixels(1000)).Height(Pixels(1000)).Wrap(Auto).Horizontal() {\
            Width(Percentage(0.5)).Height(Ratio(0.5)) {}\
        }",
    )
    .unwrap();

    let (world, root) = fixture.build_world();
    let child = *world.tree.get_first_child(&root).unwrap();
    assert_eq!(root.wrap(&world.store), Some(LayoutWrap::Wrap));
    assert_eq!(child.width(&world.store), Some(Units::Percentage(50.0)));
    assert_eq!(child.height(&world.store), Some(Units::Auto));
    assert_eq!(child.aspect_ratio(&world.store), Some(2.0));
}

#[test]
fn classifies_margin_as_unsupported() {
    let error = Fixture::parse("Width(100).Margin(2) {}").unwrap_err();

    assert_eq!(error.unsupported_feature(), Some("Margin"));
    assert!(matches!(error, FixtureError::Unsupported { .. }));
}

#[test]
fn current_fixture_corpus_parses_and_representative_fixture_lays_out() {
    let fixture_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("benches/pangui");
    let mut paths: Vec<_> = fs::read_dir(fixture_directory)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|extension| extension == "txt"))
        .collect();
    paths.sort();

    let mut supported = 0;
    let mut unsupported = Vec::new();
    for path in paths {
        let source = fs::read_to_string(&path).unwrap();
        match Fixture::parse(&source) {
            Ok(fixture) => {
                assert!(fixture.expanded_node_count() > 0);
                if path.file_name().unwrap() == "wide_no_wrap_simple_few.txt" {
                    let (mut world, root) = fixture.build_world();
                    root.layout(&mut world.cache, &world.tree, &world.store, &mut ());
                }
                supported += 1;
            }
            Err(error @ FixtureError::Unsupported { .. }) => {
                unsupported
                    .push((path.file_name().unwrap().to_owned(), error.unsupported_feature().unwrap().to_string()));
            }
            Err(error) => panic!("failed to parse {}: {error}", path.display()),
        }
    }

    assert!(supported >= 12);
    assert_eq!(unsupported.len(), 1);
    assert_eq!(unsupported[0].0, "padding_and_margin.txt");
    assert_eq!(unsupported[0].1, "Margin");
}
