use std::{fs::File, io::Read, path::Path};

pub use errors::{Error, Result};
use serde::{Deserialize, Deserializer, Serialize, de};
use serde_json::{Map, Value};

pub mod v1_0_0;
pub mod v2_0_0;
pub mod v2_1_0;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct SchemaVersion {
    major: u64,
    minor: u64,
    patch: u64,
}

/// Errors that Postman Collection functions may return
pub mod errors {
    use thiserror::Error;

    pub type Result<T> = std::result::Result<T, Error>;

    #[derive(Debug, Error)]
    pub enum Error {
        #[error("I/O error: {0}")]
        Io(#[from] std::io::Error),
        #[error("JSON error: {0}")]
        Json(#[from] serde_json::Error),
        #[error("YAML error: {0}")]
        Yaml(#[from] serde_yaml::Error),
        #[error("failed to parse collection as JSON ({json}) or YAML ({yaml})")]
        Parse {
            json: serde_json::Error,
            yaml: serde_yaml::Error,
        },
        #[error("expected the Postman Collection document root to be an object")]
        InvalidDocumentShape,
        #[error(
            "missing Postman Collection file version; expected a supported v2 info.schema value or the v1 collection shape"
        )]
        MissingSpecFileVersion,
        #[error("could not determine Postman Collection file version from schema ({schema})")]
        UnrecognizedSpecFileVersion { schema: String },
        #[error("unsupported Postman Collection file version: {version}")]
        UnsupportedSpecFileVersion { version: String },
    }
}

/// Supported versions of Postman Collection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PostmanCollectionVersion {
    #[allow(non_camel_case_types)]
    V1_0_0,
    #[allow(non_camel_case_types)]
    V2_0_0,
    #[allow(non_camel_case_types)]
    V2_1_0,
}

/// Supported versions of Postman Collection.
#[derive(Clone, Debug, Serialize, PartialEq)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum PostmanCollection {
    /// Version 1.0.0 of the Postman Collection specification.
    ///
    /// Refer to the official
    /// [specification](https://schema.getpostman.com/collection/json/v1.0.0/draft-07/docs/index.html)
    /// for more information.
    #[allow(non_camel_case_types)]
    V1_0_0(v1_0_0::Spec),
    /// Version 2.0.0 of the Postman Collection specification.
    ///
    /// Refer to the official
    /// [specification](https://schema.getpostman.com/collection/json/v2.0.0/draft-07/docs/index.html)
    /// for more information.
    #[allow(non_camel_case_types)]
    V2_0_0(v2_0_0::Spec),
    /// Version 2.1.0 of the Postman Collection specification.
    ///
    /// Refer to the official
    /// [specification](https://schema.getpostman.com/collection/json/v2.1.0/draft-07/docs/index.html)
    /// for more information.
    #[allow(non_camel_case_types)]
    V2_1_0(v2_1_0::Spec),
}

impl<'de> Deserialize<'de> for PostmanCollection {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        Self::from_value(value).map_err(de::Error::custom)
    }
}

impl PostmanCollection {
    fn from_value(value: Value) -> Result<Self> {
        match detect_version(&value)? {
            PostmanCollectionVersion::V1_0_0 => {
                Ok(Self::V1_0_0(serde_json::from_value::<v1_0_0::Spec>(value)?))
            }
            PostmanCollectionVersion::V2_0_0 => {
                Ok(Self::V2_0_0(serde_json::from_value::<v2_0_0::Spec>(value)?))
            }
            PostmanCollectionVersion::V2_1_0 => {
                Ok(Self::V2_1_0(serde_json::from_value::<v2_1_0::Spec>(value)?))
            }
        }
    }

    pub fn version(&self) -> PostmanCollectionVersion {
        match self {
            Self::V1_0_0(_) => PostmanCollectionVersion::V1_0_0,
            Self::V2_0_0(_) => PostmanCollectionVersion::V2_0_0,
            Self::V2_1_0(_) => PostmanCollectionVersion::V2_1_0,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::V1_0_0(spec) => &spec.name,
            Self::V2_0_0(spec) => &spec.info.name,
            Self::V2_1_0(spec) => &spec.info.name,
        }
    }
}

/// Deserialize a Postman Collection from a path
pub fn from_path<P>(path: P) -> Result<PostmanCollection>
where
    P: AsRef<Path>,
{
    from_reader(File::open(path)?)
}

/// Deserialize a Postman Collection from a string slice
pub fn from_str(input: &str) -> Result<PostmanCollection> {
    from_slice(input.as_bytes())
}

/// Deserialize a Postman Collection from a byte slice
pub fn from_slice(input: &[u8]) -> Result<PostmanCollection> {
    let value = match serde_json::from_slice::<Value>(input) {
        Ok(value) => value,
        Err(json) => match serde_yaml::from_slice::<Value>(input) {
            Ok(value) => value,
            Err(yaml) => return Err(Error::Parse { json, yaml }),
        },
    };

    PostmanCollection::from_value(value)
}

/// Deserialize a Postman Collection from type which implements Read
pub fn from_reader<R>(mut read: R) -> Result<PostmanCollection>
where
    R: Read,
{
    let mut bytes = Vec::new();
    read.read_to_end(&mut bytes)?;
    from_slice(&bytes)
}

/// Serialize Postman Collection spec to a YAML string
pub fn to_yaml(spec: &PostmanCollection) -> Result<String> {
    Ok(serde_yaml::to_string(spec)?)
}

/// Serialize Postman Collection spec to JSON string
pub fn to_json(spec: &PostmanCollection) -> Result<String> {
    Ok(serde_json::to_string_pretty(spec)?)
}

fn detect_version(value: &Value) -> Result<PostmanCollectionVersion> {
    let object = value.as_object().ok_or(Error::InvalidDocumentShape)?;

    if let Some(version) = version_from_schema(object)? {
        return Ok(version);
    }

    if is_v1_document(object) {
        return Ok(PostmanCollectionVersion::V1_0_0);
    }

    if looks_like_v2_document(object) {
        return Err(Error::MissingSpecFileVersion);
    }

    Err(Error::InvalidDocumentShape)
}

fn is_v1_document(object: &Map<String, Value>) -> bool {
    object.contains_key("id")
        && object.contains_key("name")
        && object.contains_key("order")
        && object.contains_key("requests")
}

fn looks_like_v2_document(object: &Map<String, Value>) -> bool {
    object.contains_key("info") || object.contains_key("item")
}

fn version_from_schema(object: &Map<String, Value>) -> Result<Option<PostmanCollectionVersion>> {
    let Some(schema) = object
        .get("info")
        .and_then(Value::as_object)
        .and_then(|info| info.get("schema"))
        .and_then(Value::as_str)
    else {
        return Ok(None);
    };

    let version =
        extract_schema_version(schema).ok_or_else(|| Error::UnrecognizedSpecFileVersion {
            schema: schema.to_owned(),
        })?;

    match version {
        SchemaVersion {
            major: 2,
            minor: 0,
            patch: 0,
        } => Ok(Some(PostmanCollectionVersion::V2_0_0)),
        SchemaVersion {
            major: 2,
            minor: 1,
            patch: 0,
        } => Ok(Some(PostmanCollectionVersion::V2_1_0)),
        version => Err(Error::UnsupportedSpecFileVersion {
            version: format!("{}.{}.{}", version.major, version.minor, version.patch),
        }),
    }
}

fn extract_schema_version(schema: &str) -> Option<SchemaVersion> {
    schema
        .split(|character: char| !(character.is_ascii_alphanumeric() || character == '.'))
        .filter_map(|segment| segment.strip_prefix('v'))
        .find_map(parse_schema_version)
}

fn parse_schema_version(candidate: &str) -> Option<SchemaVersion> {
    let mut parts = candidate.split('.');
    let major = parts.next()?.parse().ok()?;
    let minor = parts.next()?.parse().ok()?;
    let patch = parts.next()?.parse().ok()?;

    if parts.next().is_some() {
        return None;
    }

    Some(SchemaVersion {
        major,
        minor,
        patch,
    })
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;

    use glob::glob;

    use super::*;

    fn collection_fixture_glob() -> String {
        format!(
            "{}/tests/fixtures/collection/**/*.json",
            env!("CARGO_MANIFEST_DIR")
        )
    }

    /// Helper function for reading a file to string.
    fn read_file<P>(path: P) -> String
    where
        P: AsRef<Path>,
    {
        let mut f = File::open(path).unwrap();
        let mut content = String::new();
        f.read_to_string(&mut content).unwrap();
        content
    }

    /// Helper function to write string to file.
    fn write_to_file<P>(path: P, filename: &str, data: &str)
    where
        P: AsRef<Path> + std::fmt::Debug,
    {
        println!("    Saving string to {:?}...", path);
        std::fs::create_dir_all(&path).unwrap();
        let full_filename = path.as_ref().to_path_buf().join(filename);
        let mut f = File::create(&full_filename).unwrap();
        f.write_all(data.as_bytes()).unwrap();
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

    /// Convert a YAML `&str` to a normalized JSON `String`.
    fn convert_yaml_str_to_json(yaml_str: &str) -> String {
        let yaml: serde_yaml::Value = serde_yaml::from_str(yaml_str).unwrap();
        let mut json: serde_json::Value = serde_yaml::from_value(yaml).unwrap();
        normalize_json_value(&mut json);
        serde_json::to_string_pretty(&json).unwrap()
    }

    /// Deserialize and re-serialize the input file to a JSON string through two different
    /// paths, comparing the result.
    fn compare_spec_through_json(
        input_file: &Path,
        save_path_base: &Path,
    ) -> (String, String, String) {
        let spec_yaml_str = read_file(input_file);
        let spec_json_str = convert_yaml_str_to_json(&spec_yaml_str);

        let parsed_spec = from_path(input_file).unwrap();
        let mut parsed_spec_json: serde_json::Value = serde_json::to_value(parsed_spec).unwrap();
        normalize_json_value(&mut parsed_spec_json);
        let parsed_spec_json_str = serde_json::to_string_pretty(&parsed_spec_json).unwrap();

        let api_filename = input_file
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .replace(".yaml", ".json");

        let mut save_path = save_path_base.to_path_buf();
        save_path.push("yaml_to_json");
        write_to_file(&save_path, &api_filename, &spec_json_str);

        let mut save_path = save_path_base.to_path_buf();
        save_path.push("yaml_to_spec_to_json");
        write_to_file(&save_path, &api_filename, &parsed_spec_json_str);

        (api_filename, parsed_spec_json_str, spec_json_str)
    }

    #[test]
    fn can_deserialize() {
        for entry in glob(&collection_fixture_glob()).expect("Failed to read glob pattern") {
            let entry = entry.unwrap();
            let path = entry.as_path();
            println!("Testing if {:?} is deserializable", path);
            from_path(path).unwrap();
        }
    }

    #[test]
    fn can_deserialize_and_reserialize() {
        let save_path_base: std::path::PathBuf =
            ["target", "tests", "can_deserialize_and_reserialize"]
                .iter()
                .collect();
        let mut invalid_diffs = Vec::new();

        for entry in glob(&collection_fixture_glob()).expect("Failed to read glob pattern") {
            let entry = entry.unwrap();
            let path = entry.as_path();

            println!("Testing if {:?} is deserializable", path);

            let (api_filename, parsed_spec_json_str, spec_json_str) =
                compare_spec_through_json(path, &save_path_base);

            if parsed_spec_json_str != spec_json_str {
                invalid_diffs.push((api_filename, parsed_spec_json_str, spec_json_str));
            }
        }

        for invalid_diff in &invalid_diffs {
            println!("File {} failed JSON comparison!", invalid_diff.0);
        }
        assert_eq!(invalid_diffs.len(), 0);
    }

    #[test]
    fn detects_versions_for_sample_collections() {
        let fixture_root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("fixtures")
            .join("collection");

        assert!(matches!(
            from_path(fixture_root.join("swagger-petstore-v1.0.0.json")).unwrap(),
            PostmanCollection::V1_0_0(_)
        ));
        assert!(matches!(
            from_path(fixture_root.join("swagger-petstore-v2.0.0.json")).unwrap(),
            PostmanCollection::V2_0_0(_)
        ));
        assert!(matches!(
            from_path(fixture_root.join("swagger-petstore-v2.1.0.json")).unwrap(),
            PostmanCollection::V2_1_0(_)
        ));
    }
}
