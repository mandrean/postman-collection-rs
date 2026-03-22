use std::{fs, io::Cursor, path::PathBuf};

use postman_collection::{
    Error, PostmanCollection, from_path, from_reader, to_json, v1_0_0, v2_0_0, v2_1_0,
};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("collection")
        .join(name)
}

fn read_fixture(name: &str) -> String {
    fs::read_to_string(fixture_path(name)).expect("fixture should exist")
}

fn normalize_json_value(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Array(values) => {
            for value in values {
                normalize_json_value(value);
            }
        }
        serde_json::Value::Object(map) => {
            map.retain(|_, value| {
                normalize_json_value(value);
                !value.is_null()
            });
        }
        _ => {}
    }
}

fn normalized_fixture_json(name: &str) -> serde_json::Value {
    let mut value: serde_json::Value =
        serde_yaml::from_str(&read_fixture(name)).expect("fixture should deserialize");
    normalize_json_value(&mut value);
    value
}

fn normalized_collection_json(collection: &PostmanCollection) -> serde_json::Value {
    let mut value: serde_json::Value =
        serde_json::from_str(&to_json(collection).expect("collection should serialize"))
            .expect("serialized JSON should parse");
    normalize_json_value(&mut value);
    value
}

fn assert_round_trip_json(name: &str) {
    let collection = from_path(fixture_path(name)).expect("fixture should parse");
    assert_eq!(
        normalized_fixture_json(name),
        normalized_collection_json(&collection)
    );
}

#[test]
fn detects_versions_for_sample_fixtures_with_all_entrypoints() {
    assert!(matches!(
        from_path(fixture_path("swagger-petstore-v1.0.0.json")).unwrap(),
        PostmanCollection::V1_0_0(_)
    ));
    assert!(matches!(
        from_path(fixture_path("swagger-petstore-v2.0.0.json")).unwrap(),
        PostmanCollection::V2_0_0(_)
    ));
    assert!(matches!(
        from_path(fixture_path("swagger-petstore-v2.1.0.json")).unwrap(),
        PostmanCollection::V2_1_0(_)
    ));

    let v2_1_json = read_fixture("swagger-petstore-v2.1.0.json");
    assert!(matches!(
        serde_json::from_str::<PostmanCollection>(&v2_1_json).unwrap(),
        PostmanCollection::V2_1_0(_)
    ));
}

#[test]
fn rejects_unsupported_schema_versions() {
    let mut fixture: serde_json::Value =
        serde_json::from_str(&read_fixture("swagger-petstore-v2.1.0.json")).unwrap();
    fixture["info"]["schema"] = serde_json::Value::String(
        "https://schema.getpostman.com/json/collection/v2.2.0/collection.json".to_owned(),
    );

    let error = from_reader(Cursor::new(serde_json::to_vec(&fixture).unwrap())).unwrap_err();
    match error {
        Error::UnsupportedSpecFileVersion { version } => {
            assert_eq!(version, "2.2.0");
        }
        error => panic!("expected UnsupportedSpecFileVersion, got {:?}", error),
    }
}

#[test]
fn rejects_v2_documents_without_schema_metadata() {
    let mut fixture: serde_json::Value =
        serde_json::from_str(&read_fixture("swagger-petstore-v2.1.0.json")).unwrap();
    fixture["info"]
        .as_object_mut()
        .expect("fixture info should be an object")
        .remove("schema");

    let error = from_reader(Cursor::new(serde_json::to_vec(&fixture).unwrap())).unwrap_err();
    assert!(matches!(error, Error::MissingSpecFileVersion));
}

#[test]
fn round_trips_new_coverage_fixtures_losslessly() {
    for fixture in [
        "coverage/v1-helper-attributes-string.json",
        "coverage/v1-helper-attributes-empty-object.json",
        "coverage/v1-helper-attributes-object.json",
        "coverage/v2.0.0-item-group.json",
        "coverage/v2.1.0-item-group.json",
    ] {
        assert_round_trip_json(fixture);
    }
}

#[test]
fn preserves_v1_helper_attribute_objects() {
    let collection = from_path(fixture_path("coverage/v1-helper-attributes-object.json")).unwrap();

    let PostmanCollection::V1_0_0(spec) = collection else {
        panic!("expected v1 collection");
    };

    let Some(v1_0_0::HelperAttributes::Object(helper_attributes)) =
        spec.requests[0].helper_attributes.as_ref()
    else {
        panic!("expected helperAttributes object");
    };

    assert_eq!(
        helper_attributes.get("id"),
        Some(&serde_json::Value::String("basic".to_owned()))
    );
    assert_eq!(
        helper_attributes.get("username"),
        Some(&serde_json::Value::String("alice".to_owned()))
    );
    assert_eq!(
        helper_attributes.get("password"),
        Some(&serde_json::Value::String("secret".to_owned()))
    );
}

#[test]
fn parses_v2_item_group_branches_explicitly() {
    let v2_0_collection = from_path(fixture_path("coverage/v2.0.0-item-group.json")).unwrap();
    let PostmanCollection::V2_0_0(v2_0_spec) = v2_0_collection else {
        panic!("expected v2.0.0 collection");
    };
    let v2_0_0::Items::ItemGroup(v2_0_group) = &v2_0_spec.item[0] else {
        panic!("expected top-level item-group for v2.0.0");
    };
    assert!(matches!(v2_0_group.item[0], v2_0_0::Items::Item(_)));

    let v2_1_collection = from_path(fixture_path("coverage/v2.1.0-item-group.json")).unwrap();
    let PostmanCollection::V2_1_0(v2_1_spec) = v2_1_collection else {
        panic!("expected v2.1.0 collection");
    };
    let v2_1_0::Items::ItemGroup(v2_1_group) = &v2_1_spec.item[0] else {
        panic!("expected top-level item-group for v2.1.0");
    };
    assert!(matches!(v2_1_group.item[0], v2_1_0::Items::Item(_)));
}
