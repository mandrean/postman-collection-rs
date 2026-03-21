use std::path::{Path, PathBuf};

use glob::glob;
use postman_collection::{
    Error, PostmanCollection, PostmanCollectionVersion, from_path, from_str, to_yaml,
};

fn collection_fixture_paths() -> Vec<PathBuf> {
    let pattern = format!(
        "{}/tests/fixtures/collection/*.json",
        env!("CARGO_MANIFEST_DIR")
    );
    let mut paths = glob(&pattern)
        .expect("Failed to read fixture glob pattern")
        .collect::<std::result::Result<Vec<_>, _>>()
        .expect("Failed to enumerate collection fixtures");
    paths.sort();
    paths
}

fn expected_version(path: &Path) -> PostmanCollectionVersion {
    let filename = path
        .file_name()
        .and_then(|name| name.to_str())
        .expect("Fixture filename should be valid UTF-8");

    if filename.contains("v1.0.0") {
        PostmanCollectionVersion::V1_0_0
    } else if filename.contains("v2.0.0") {
        PostmanCollectionVersion::V2_0_0
    } else if filename.contains("v2.1.0") {
        PostmanCollectionVersion::V2_1_0
    } else {
        panic!("Unexpected fixture filename: {filename}");
    }
}

#[test]
fn discovers_collection_fixtures() {
    let fixtures = collection_fixture_paths();
    assert!(
        !fixtures.is_empty(),
        "Expected at least one collection fixture under tests/fixtures/collection"
    );
}

#[test]
fn deserializes_collection_fixtures_with_expected_versions() {
    for path in collection_fixture_paths() {
        let collection = from_path(&path).expect("Fixture should deserialize");
        let expected = expected_version(&path);

        assert_eq!(
            collection.version(),
            expected,
            "Wrong collection version for {}",
            path.display()
        );

        match expected {
            PostmanCollectionVersion::V1_0_0 => {
                assert!(matches!(collection, PostmanCollection::V1_0_0(_)));
            }
            PostmanCollectionVersion::V2_0_0 => {
                assert!(matches!(collection, PostmanCollection::V2_0_0(_)));
            }
            PostmanCollectionVersion::V2_1_0 => {
                assert!(matches!(collection, PostmanCollection::V2_1_0(_)));
            }
        }
    }
}

#[test]
fn collection_fixtures_round_trip_without_losing_information() {
    for path in collection_fixture_paths() {
        let parsed = from_path(&path).expect("Fixture should deserialize");
        let json = postman_collection::to_json(&parsed).expect("Collection should serialize");
        let reparsed = from_str(&json).expect("Serialized JSON should deserialize");

        assert_eq!(reparsed, parsed, "Round-trip changed {}", path.display());
    }
}

#[test]
fn yaml_round_trip_still_works() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/collection/swagger-petstore-v2.1.0.json");
    let original = from_path(&fixture).expect("Fixture should deserialize");
    let yaml = to_yaml(&original).expect("Collection should serialize to YAML");
    let reparsed = from_str(&yaml).expect("YAML output should deserialize");

    assert_eq!(reparsed.version(), PostmanCollectionVersion::V2_1_0);
    assert_eq!(reparsed.name(), original.name());
    assert_eq!(
        serde_json::to_value(reparsed).expect("Collection should serialize to JSON"),
        serde_json::to_value(original).expect("Collection should serialize to JSON")
    );
}

#[test]
fn schema_tagged_v2_collections_take_precedence_over_v1_shape_heuristics() {
    for (schema, expected) in [
        (
            "https://schema.getpostman.com/json/collection/v2.0.0/collection.json",
            PostmanCollectionVersion::V2_0_0,
        ),
        (
            "https://schema.getpostman.com/json/collection/v2.1.0/collection.json",
            PostmanCollectionVersion::V2_1_0,
        ),
    ] {
        let input = format!(
            r#"{{
                "info": {{
                    "name": "Example",
                    "schema": "{schema}"
                }},
                "item": [],
                "order": ["custom-metadata"]
            }}"#
        );

        let collection = from_str(&input).expect("Schema-tagged v2 collection should deserialize");
        assert_eq!(collection.version(), expected);

        match expected {
            PostmanCollectionVersion::V1_0_0 => unreachable!("Only v2 schemas are under test"),
            PostmanCollectionVersion::V2_0_0 => {
                assert!(matches!(collection, PostmanCollection::V2_0_0(_)));
            }
            PostmanCollectionVersion::V2_1_0 => {
                assert!(matches!(collection, PostmanCollection::V2_1_0(_)));
            }
        }
    }
}

#[test]
fn rejects_unknown_schema_versions_even_with_v1_shape_keys_present() {
    let input = r#"{
        "info": {
            "name": "Example",
            "schema": "https://schema.getpostman.com/json/collection/v9.9.9/collection.json"
        },
        "item": [],
        "order": ["custom-metadata"]
    }"#;

    let error = from_str(input).expect_err("Unknown schema should fail");
    assert!(matches!(error, Error::UnsupportedSpecFileVersion { .. }));
}
