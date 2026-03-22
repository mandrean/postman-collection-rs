use std::{fs, io::Cursor, path::PathBuf};

#[cfg(feature = "yaml")]
use postman_collection::to_yaml;
use postman_collection::{
    Error, PostmanCollection, from_path, from_reader, from_slice, from_str, to_json, v1_0_0,
    v2_0_0, v2_1_0,
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
        serde_json::from_str(&read_fixture(name)).expect("fixture should deserialize");
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
fn supports_from_slice_for_json_inputs() {
    let json_fixture = read_fixture("swagger-petstore-v2.1.0.json");
    let parsed_from_json = from_slice(json_fixture.as_bytes()).expect("JSON bytes should parse");

    assert!(matches!(parsed_from_json, PostmanCollection::V2_1_0(_)));
}

#[cfg(feature = "yaml")]
#[test]
fn supports_from_slice_for_yaml_inputs_when_feature_is_enabled() {
    let json_fixture = read_fixture("swagger-petstore-v2.1.0.json");
    let parsed_from_json = from_slice(json_fixture.as_bytes()).expect("JSON bytes should parse");
    let yaml_fixture = to_yaml(&parsed_from_json).expect("collection should serialize to YAML");
    let parsed_from_yaml = from_slice(yaml_fixture.as_bytes()).expect("YAML bytes should parse");

    assert_eq!(parsed_from_yaml, parsed_from_json);
}

#[cfg(not(feature = "yaml"))]
#[test]
fn rejects_yaml_input_even_when_it_looks_like_a_collection() {
    let yaml = r#"
info:
  name: Example
  schema: https://schema.getpostman.com/json/collection/v2.1.0/collection.json
item: []
"#;

    let error = from_str(yaml).expect_err("YAML input should fail without JSON syntax");
    assert!(matches!(error, Error::Json(_)));
}

#[cfg(feature = "yaml")]
#[test]
fn accepts_yaml_input_when_feature_is_enabled() {
    let json_fixture = read_fixture("swagger-petstore-v2.1.0.json");
    let parsed_from_json = from_str(&json_fixture).expect("JSON fixture should parse");
    let yaml_fixture = to_yaml(&parsed_from_json).expect("collection should serialize to YAML");
    let parsed_from_yaml = from_str(&yaml_fixture).expect("YAML string should parse");

    assert_eq!(parsed_from_yaml, parsed_from_json);
}

#[cfg(not(feature = "yaml"))]
#[test]
fn returns_json_errors_for_bytes_that_are_not_json() {
    let error = from_slice(b"\xFF").expect_err("invalid bytes should fail to parse");
    assert!(matches!(error, Error::Json(_)));
}

#[cfg(feature = "yaml")]
#[test]
fn returns_parse_errors_for_bytes_that_are_not_json_or_yaml() {
    let error = from_slice(b"\xFF").expect_err("invalid bytes should fail to parse");
    assert!(matches!(error, Error::Parse { .. }));
}

#[test]
fn rejects_non_object_document_roots() {
    let error = from_str("[]").expect_err("array roots should be rejected");
    assert!(matches!(error, Error::InvalidDocumentShape));
}

#[test]
fn returns_json_errors_after_version_detection_for_invalid_collection_shapes() {
    let error = from_str(
        r#"{
            "info": {
                "name": "Broken fixture",
                "schema": "https://schema.getpostman.com/json/collection/v2.1.0/collection.json"
            },
            "item": "should have been an array"
        }"#,
    )
    .expect_err("invalid v2.1 shape should fail");

    assert!(matches!(error, Error::Json(_)));
}

#[test]
fn rejects_unrecognized_schema_versions() {
    let schema = "https://schema.getpostman.com/json/collection/current/collection.json";
    let error = from_str(&format!(
        r#"{{
            "info": {{
                "name": "Example",
                "schema": "{schema}"
            }},
            "item": []
        }}"#
    ))
    .expect_err("unrecognized schema should fail");

    match error {
        Error::UnrecognizedSpecFileVersion {
            schema: rejected_schema,
        } => {
            assert_eq!(rejected_schema, schema);
        }
        error => panic!("expected UnrecognizedSpecFileVersion, got {:?}", error),
    }
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

#[cfg(feature = "yaml")]
#[test]
fn serializes_to_yaml_when_feature_is_enabled() {
    let collection =
        from_path(fixture_path("swagger-petstore-v2.1.0.json")).expect("fixture should parse");
    let yaml = to_yaml(&collection).expect("collection should serialize to YAML");

    assert!(yaml.contains("info:"), "unexpected YAML output: {yaml}");
    assert!(
        yaml.contains("name: Swagger Petstore"),
        "unexpected YAML output: {yaml}"
    );
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

#[test]
fn parses_api_key_auth_across_supported_versions() {
    let v1_collection = from_path(fixture_path("apikey-v1.0.0.json")).unwrap();
    let PostmanCollection::V1_0_0(v1_spec) = v1_collection else {
        panic!("expected v1 collection");
    };
    let v1_auth = v1_spec.auth.as_ref().expect("v1 auth should exist");
    assert!(matches!(v1_auth.auth_type, v1_0_0::AuthType::Apikey));
    let v1_api_key = v1_auth
        .api_key
        .as_ref()
        .expect("v1 API key auth should exist");
    assert_eq!(v1_api_key.len(), 1);
    assert_eq!(v1_api_key[0].key, "key");
    assert_eq!(
        v1_api_key[0].value,
        Some(serde_json::Value::String("X-API-Key".to_owned()))
    );

    let v2_0_collection = from_path(fixture_path("apikey-v2.0.0.json")).unwrap();
    let PostmanCollection::V2_0_0(v2_0_spec) = v2_0_collection else {
        panic!("expected v2.0.0 collection");
    };
    let v2_0_auth = v2_0_spec.auth.as_ref().expect("v2.0 auth should exist");
    assert!(matches!(v2_0_auth.auth_type, v2_0_0::AuthType::Apikey));
    let v2_0_api_key = v2_0_auth
        .api_key
        .as_ref()
        .expect("v2.0 API key auth should exist");
    assert_eq!(
        v2_0_api_key.get("key"),
        Some(&Some(serde_json::Value::String("X-API-Key".to_owned())))
    );

    let v2_1_collection = from_path(fixture_path("apikey-v2.1.0.json")).unwrap();
    let PostmanCollection::V2_1_0(v2_1_spec) = v2_1_collection else {
        panic!("expected v2.1.0 collection");
    };
    let v2_1_auth = v2_1_spec.auth.as_ref().expect("v2.1 auth should exist");
    assert!(matches!(v2_1_auth.auth_type, v2_1_0::AuthType::Apikey));
    let v2_1_api_key = v2_1_auth
        .api_key
        .as_ref()
        .expect("v2.1 API key auth should exist");
    assert_eq!(v2_1_api_key.len(), 1);
    assert_eq!(v2_1_api_key[0].key, "key");
    assert_eq!(
        v2_1_api_key[0].value,
        Some(serde_json::Value::String("X-API-Key".to_owned()))
    );
}

#[test]
fn parses_v2_1_edgegrid_auth_attributes_explicitly() {
    let collection = from_path(fixture_path("edgegrid-v2.1.0.json")).unwrap();
    let PostmanCollection::V2_1_0(spec) = collection else {
        panic!("expected v2.1.0 collection");
    };
    let auth = spec.auth.as_ref().expect("edgegrid auth should exist");

    assert!(matches!(auth.auth_type, v2_1_0::AuthType::Edgegrid));

    let edgegrid = auth
        .edgegrid
        .as_ref()
        .expect("edgegrid attributes should exist");
    assert_eq!(edgegrid.len(), 2);
    assert_eq!(edgegrid[0].key, "accessToken");
    assert_eq!(
        edgegrid[0].value,
        Some(serde_json::Value::String(
            "edgegrid-access-token".to_owned()
        ))
    );
    assert_eq!(edgegrid[1].key, "clientToken");
    assert_eq!(
        edgegrid[1].value,
        Some(serde_json::Value::String(
            "edgegrid-client-token".to_owned()
        ))
    );
}

#[test]
fn parses_v2_1_graphql_body_options_and_response_timings_explicitly() {
    let collection = from_path(fixture_path("graphql-query-v2.1.0.json")).unwrap();
    let PostmanCollection::V2_1_0(spec) = collection else {
        panic!("expected v2.1.0 collection");
    };
    let v2_1_0::Items::Item(item) = &spec.item[0] else {
        panic!("expected a top-level item");
    };
    let v2_1_0::RequestUnion::RequestClass(request) = &item.request else {
        panic!("expected a structured request");
    };
    let body = request
        .body
        .as_ref()
        .expect("GraphQL request body should exist");

    assert_eq!(body.mode.as_ref(), Some(&v2_1_0::Mode::Graphql));

    let graphql = body
        .graphql
        .as_ref()
        .expect("GraphQL payload should be preserved");
    assert_eq!(graphql["query"], "query Viewer { viewer { id name } }");
    assert_eq!(graphql["variables"]["includeInactive"], false);

    let options = body
        .options
        .as_ref()
        .expect("GraphQL body options should be preserved");
    assert_eq!(options["graphql"]["output"], "json");

    let response = item
        .response
        .as_ref()
        .and_then(|responses| responses.first())
        .expect("sample response should exist");
    assert_eq!(
        response.response_time.as_ref(),
        Some(&v2_1_0::ResponseTime::Integer(123))
    );

    let timings = response
        .timings
        .as_ref()
        .expect("response timings should be preserved");
    assert_eq!(timings["response"], 123);
    assert_eq!(timings["dns"], 4);
}
