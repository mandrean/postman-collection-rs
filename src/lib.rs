use std::{fs::File, io::Read, path::Path};

pub use errors::{Error, Result};
use serde::{Deserialize, Deserializer, Serialize, de};
use serde_json::{Map, Value};

pub mod v1_0_0;
pub mod v2_0_0;
pub mod v2_1_0;

const POSTMAN_COLLECTION_V2_0_0_SCHEMA: &str =
    "https://schema.getpostman.com/json/collection/v2.0.0/collection.json";
const POSTMAN_COLLECTION_V2_1_0_SCHEMA: &str =
    "https://schema.getpostman.com/json/collection/v2.1.0/collection.json";

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
        #[error("ambiguous Postman Collection file version; add a supported info.schema URL")]
        AmbiguousSpecFileVersion,
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
pub enum PostmanCollection {
    /// Version 1.0.0 of the Postman Collection specification.
    ///
    /// Refer to the official
    /// [specification](https://schema.getpostman.com/collection/json/v1.0.0/draft-07/docs/index.html)
    /// for more information.
    #[allow(non_camel_case_types)]
    V1_0_0(v1_0_0::Spec),
    /// Version 1.0.0 of the Postman Collection specification.
    ///
    /// Refer to the official
    /// [specification](https://schema.getpostman.com/collection/json/v2.0.0/draft-07/docs/index.html)
    /// for more information.
    #[allow(non_camel_case_types)]
    V2_0_0(v2_0_0::Spec),
    /// Version 1.0.0 of the Postman Collection specification.
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

    if is_v1_document(object) {
        return Ok(PostmanCollectionVersion::V1_0_0);
    }

    if let Some(version) = version_from_schema(object)? {
        return Ok(version);
    }

    let can_parse_v2_0 = serde_json::from_value::<v2_0_0::Spec>(value.clone()).is_ok();
    let can_parse_v2_1 = serde_json::from_value::<v2_1_0::Spec>(value.clone()).is_ok();

    match (can_parse_v2_0, can_parse_v2_1) {
        (true, false) => Ok(PostmanCollectionVersion::V2_0_0),
        (false, true) => Ok(PostmanCollectionVersion::V2_1_0),
        (true, true) => Err(Error::AmbiguousSpecFileVersion),
        (false, false) => Err(Error::UnsupportedSpecFileVersion {
            version: "unknown".to_owned(),
        }),
    }
}

fn is_v1_document(object: &Map<String, Value>) -> bool {
    ["requests", "folders", "order", "folders_order"]
        .iter()
        .any(|key| object.contains_key(*key))
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

    if schema == POSTMAN_COLLECTION_V2_0_0_SCHEMA {
        return Ok(Some(PostmanCollectionVersion::V2_0_0));
    }

    if schema == POSTMAN_COLLECTION_V2_1_0_SCHEMA {
        return Ok(Some(PostmanCollectionVersion::V2_1_0));
    }

    Err(Error::UnsupportedSpecFileVersion {
        version: schema.to_owned(),
    })
}
