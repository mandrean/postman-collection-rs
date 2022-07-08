use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Spec {
    #[serde(rename = "auth")]
    pub auth: Option<Auth>,

    /// Provide a long description of this collection using this field. This field supports
    /// markdown syntax to better format the description.
    #[serde(rename = "description")]
    pub description: Option<String>,

    #[serde(rename = "events")]
    pub events: Option<Vec<Event>>,

    /// Folders are the way to go if you want to group your requests and to keep things
    /// organised. Folders can also be useful in sequentially requesting a part of the entire
    /// collection by using [Postman Collection
    /// Runner](https://www.getpostman.com/docs/jetpacks_running_collections) or
    /// [Newman](https://github.com/postmanlabs/newman) on a particular folder.
    #[serde(rename = "folders")]
    pub folders: Option<Vec<Folder>>,

    /// The folders order array ensures that your requests and folders don't randomly get
    /// shuffled up. It holds a sequence of
    /// [UUIDs](https://en.wikipedia.org/wiki/Universally_unique_identifier) corresponding to
    /// folders and requests.
    /// *Note that if a folder ID or a request ID (if the request is not already part of a
    /// folder) is not included in the order array, the request or the folder will not show up in
    /// the collection.*
    #[serde(rename = "folders_order")]
    pub folders_order: Option<Vec<String>>,

    /// Every collection is identified by the unique value of this field. The value of this field
    /// is usually easiest to generate using a
    /// [UID](https://tools.ietf.org/html/rfc4122#section-4.4%29) generator function. If you
    /// already have a collection, it is recommended that you maintain the same id since changing
    /// the id usually implies that this is a different collection than it was originally.
    #[serde(rename = "id")]
    pub id: String,

    /// A collection's friendly name is defined by this field. You would want to set this field
    /// to a value that would allow you to easily identify this collection among a bunch of other
    /// collections, as such outlining its usage or content.
    #[serde(rename = "name")]
    pub name: String,

    /// The order array ensures that your requests and folders don't randomly get shuffled up. It
    /// holds a sequence of [UUIDs](https://en.wikipedia.org/wiki/Universally_unique_identifier)
    /// corresponding to folders and requests.
    /// *Note that if a folder ID or a request ID (if the request is not already part of a
    /// folder) is not included in the order array, the request or the folder will not show up in
    /// the collection.*
    #[serde(rename = "order")]
    pub order: Vec<String>,

    #[serde(rename = "requests")]
    pub requests: Vec<Request>,

    #[serde(rename = "timestamp")]
    pub timestamp: Option<f64>,

    #[serde(rename = "variables")]
    pub variables: Option<Vec<Variable>>,
}

/// Represents authentication helpers provided by Postman
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Auth {
    /// The auth type. Either `noauth`, `awsv4`, `basic`, `bearer`, `digest`, `hawk`, `ntlm`, `oauth1`, or `oauth2`.
    #[serde(rename = "type")]
    pub auth_type: AuthType,

    /// No authentication
    #[serde(rename = "noauth")]
    pub noauth: Option<serde_json::Value>,

    /// The attributes for [AWS Auth](http://docs.aws.amazon.com/AmazonS3/latest/dev/RESTAuthentication.html).
    #[serde(rename = "awsv4")]
    pub awsv4: Option<Vec<AuthAttribute>>,

    /// The attributes for [Basic Authentication](https://en.wikipedia.org/wiki/Basic_access_authentication).
    #[serde(rename = "basic")]
    pub basic: Option<Vec<AuthAttribute>>,

    /// The helper attributes for [Bearer Token Authentication](https://tools.ietf.org/html/rfc6750).
    #[serde(rename = "bearer")]
    pub bearer: Option<Vec<AuthAttribute>>,

    /// The attributes for [Digest Authentication](https://en.wikipedia.org/wiki/Digest_access_authentication).
    #[serde(rename = "digest")]
    pub digest: Option<Vec<AuthAttribute>>,

    /// The attributes for [Hawk Authentication](https://github.com/hueniverse/hawk).
    #[serde(rename = "hawk")]
    pub hawk: Option<Vec<AuthAttribute>>,

    /// The attributes for [NTLM Authentication](https://msdn.microsoft.com/en-us/library/cc237488.aspx).
    #[serde(rename = "ntlm")]
    pub ntlm: Option<Vec<AuthAttribute>>,

    /// The attributes for [OAuth2](https://oauth.net/1/).
    #[serde(rename = "oauth1")]
    pub oauth1: Option<Vec<AuthAttribute>>,

    /// Helper attributes for [OAuth2](https://oauth.net/2/).
    #[serde(rename = "oauth2")]
    pub oauth2: Option<Vec<AuthAttribute>>,
}

/// Represents an attribute for any authorization method provided by Postman. For example
/// `username` and `password` are set as auth attributes for Basic Authentication method.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct AuthAttribute {
    #[serde(rename = "key")]
    pub key: String,

    #[serde(rename = "type")]
    pub auth_type: Option<String>,

    #[serde(rename = "value")]
    pub value: Option<serde_json::Value>,
}

/// Defines a script associated with an associated event name
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Event {
    /// Indicates whether the event is disabled. If absent, the event is assumed to be enabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,

    /// A unique identifier for the enclosing event.
    #[serde(rename = "id")]
    pub id: Option<String>,

    /// Can be set to `test` or `prerequest` for test scripts or pre-request scripts respectively.
    #[serde(rename = "listen")]
    pub listen: String,

    #[serde(rename = "script")]
    pub script: Option<Script>,
}

/// A script is a snippet of Javascript code that can be used to to perform setup or teardown
/// operations on a particular response.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Script {
    #[serde(rename = "exec")]
    pub exec: Option<Host>,

    /// A unique, user defined identifier that can  be used to refer to this script from requests.
    #[serde(rename = "id")]
    pub id: Option<String>,

    /// Script name
    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "src")]
    pub src: Option<Url>,

    /// Type of the script. E.g: 'text/javascript'
    #[serde(rename = "type")]
    pub script_type: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct UrlClass {
    /// Contains the URL fragment (if any). Usually this is not transmitted over the network, but
    /// it could be useful to store this in some cases.
    #[serde(rename = "hash")]
    pub hash: Option<String>,

    /// The host for the URL, E.g: api.yourdomain.com. Can be stored as a string or as an array
    /// of strings.
    #[serde(rename = "host")]
    pub host: Option<Host>,

    #[serde(rename = "path")]
    pub path: Option<UrlPath>,

    /// The port number present in this URL. An empty value implies 80/443 depending on whether
    /// the protocol field contains http/https.
    #[serde(rename = "port")]
    pub port: Option<String>,

    /// The protocol associated with the request, E.g: 'http'
    #[serde(rename = "protocol")]
    pub protocol: Option<String>,

    /// An array of QueryParams, which is basically the query string part of the URL, parsed into
    /// separate variables
    #[serde(rename = "query")]
    pub query: Option<Vec<QueryParam>>,

    /// The string representation of the request URL, including the protocol, host, path, hash,
    /// query parameter(s) and path variable(s).
    #[serde(rename = "raw")]
    pub raw: Option<String>,

    /// Postman supports path variables with the syntax `/path/:variableName/to/somewhere`. These
    /// variables are stored in this field.
    #[serde(rename = "variable")]
    pub variable: Option<Vec<Variable>>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct PathClass {
    #[serde(rename = "type")]
    pub path_type: Option<String>,

    #[serde(rename = "value")]
    pub value: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct QueryParam {
    #[serde(rename = "description")]
    pub description: Option<Description>,

    /// If set to true, the current query parameter will not be sent with the request.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,

    #[serde(rename = "key")]
    pub key: Option<String>,

    #[serde(rename = "value")]
    pub value: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct DescriptionClass {
    /// The content of the description goes here, as a raw string.
    #[serde(rename = "content")]
    pub content: Option<String>,

    /// Holds the mime type of the raw description content. E.g: 'text/markdown' or 'text/html'.
    /// The type is used to correctly render the description when generating documentation, or in
    /// the Postman app.
    #[serde(rename = "type")]
    pub description_type: Option<String>,

    /// Description can have versions associated with it, which should be put in this property.
    #[serde(rename = "version")]
    pub version: Option<serde_json::Value>,
}

/// Collection variables allow you to define a set of variables, that are a *part of the
/// collection*, as opposed to environments, which are separate entities.
/// *Note: Collection variables must not contain any sensitive information.*
///
/// Using variables in your Postman requests eliminates the need to duplicate requests, which
/// can save a lot of time. Variables can be defined, and referenced to from any part of a
/// request.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Variable {
    #[serde(rename = "description")]
    pub description: Option<Description>,

    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,

    /// A variable ID is a unique user-defined value that identifies the variable within a
    /// collection. In traditional terms, this would be a variable name.
    #[serde(rename = "id")]
    pub id: Option<String>,

    /// A variable key is a human friendly value that identifies the variable within a
    /// collection. In traditional terms, this would be a variable name.
    #[serde(rename = "key")]
    pub key: Option<String>,

    /// Variable name
    #[serde(rename = "name")]
    pub name: Option<String>,

    /// When set to true, indicates that this variable has been set by Postman
    #[serde(rename = "system")]
    pub system: Option<bool>,

    /// A variable may have multiple types. This field specifies the type of the variable.
    #[serde(rename = "type")]
    pub variable_type: Option<VariableType>,

    /// The value that a variable holds in this collection. Ultimately, the variables will be
    /// replaced by this value, when say running a set of requests from a collection
    #[serde(rename = "value")]
    pub value: Option<serde_json::Value>,
}

/// One of the primary goals of Postman is to organize the development of APIs. To this end,
/// it is necessary to be able to group requests together. This can be achived using
/// 'Folders'. A folder just is an ordered set of requests.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Folder {
    #[serde(rename = "auth")]
    pub auth: Option<Auth>,

    /// Postman folders are always a part of a collection. That collection's unique ID (which is
    /// a [UUID](https://en.wikipedia.org/wiki/Globally_unique_identifier)) is stored in this
    /// field.
    #[serde(rename = "collection")]
    pub collection: Option<String>,

    /// Postman folders are always a part of a collection. That collection's unique ID (which is
    /// a [UUID](https://en.wikipedia.org/wiki/Globally_unique_identifier)) is stored in this
    /// field.
    #[serde(rename = "collection_id")]
    pub collection_id: Option<String>,

    /// Essays about the folder go into this field!
    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "events")]
    pub events: Option<Vec<Event>>,

    /// Postman preserves the order of your folders within each folder. This field holds a
    /// sequence of [UUIDs](https://en.wikipedia.org/wiki/Globally_unique_identifier), where each
    /// ID corresponds to a particular collection folder.
    #[serde(rename = "folders_order")]
    pub folders_order: Option<Vec<String>>,

    /// In order to be able to uniquely identify different folders within a collection, Postman
    /// assigns each folder a unique ID (a
    /// [UUID](https://en.wikipedia.org/wiki/Globally_unique_identifier)). This field contains
    /// that value.
    #[serde(rename = "id")]
    pub id: String,

    /// A folder's friendly name is defined by this field. You would want to set this field to a
    /// value that would allow you to easily identify this folder.
    #[serde(rename = "name")]
    pub name: String,

    /// Postman preserves the order of your requests within each folder. This field holds a
    /// sequence of [UUIDs](https://en.wikipedia.org/wiki/Globally_unique_identifier), where each
    /// ID corresponds to a particular Postman request.
    #[serde(rename = "order")]
    pub order: Vec<String>,

    #[serde(rename = "variables")]
    pub variables: Option<Vec<Variable>>,
}

/// A request represents an HTTP request.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Request {
    #[serde(rename = "auth")]
    pub auth: Option<Auth>,

    /// This field contains the unique ID of the collection to which this request belongs.
    #[serde(rename = "collection")]
    pub collection: Option<String>,

    /// This field contains the unique ID of the collection to which this request belongs.
    #[serde(rename = "collectionId")]
    pub collection_id: Option<String>,

    #[serde(rename = "currentHelper")]
    pub current_helper: Option<String>,

    #[serde(rename = "data")]
    pub data: Option<Vec<Datum>>,

    /// When set to true, prevents request body from being sent.
    #[serde(rename = "dataDisabled")]
    pub data_disabled: Option<bool>,

    #[serde(rename = "dataMode")]
    pub data_mode: Option<DataMode>,

    /// The description of this request. Can be as long as you want. Postman also supports two
    /// formats for your description, ``markdown`` and ``html``.
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// A request can have an associated description text. Since description is meant to be long,
    /// it can be in either ``html`` or ``markdown`` formats. This field specifies that format.
    #[serde(rename = "descriptionFormat")]
    pub description_format: Option<DescriptionFormat>,

    #[serde(rename = "events")]
    pub events: Option<Vec<Event>>,

    /// Postman requests may or may not be a part of a folder. If this request belongs to a
    /// folder, that folder's unique ID (which is a
    /// [UUID](https://en.wikipedia.org/wiki/Globally_unique_identifier)) is stored in this field.
    #[serde(rename = "folder")]
    pub folder: Option<String>,

    #[serde(rename = "headerData")]
    pub header_data: Option<Vec<Option<Header>>>,

    /// No HTTP request is complete without its headers, and the same is true for a Postman
    /// request. This field contains all the HTTP Headers in a raw string format.
    #[serde(rename = "headers")]
    pub headers: String,

    #[serde(rename = "helperAttributes")]
    pub helper_attributes: Option<HelperAttributes>,

    /// Postman can store a number of requests in each collection. In order to preserve the order
    /// of each request, we need to be able to identify requests uniquely. This field is a UUID
    /// assigned to each request.
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "method")]
    pub method: String,

    /// Sometimes, you just need to call your request 'Bob'. Postman will let you do that, and
    /// store the name you give in this field.
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "pathVariableData")]
    pub path_variable_data: Option<Vec<Option<PathVariable>>>,

    /// A Postman request allows you to use Path Variables in a request, e.g:
    /// ``/search/:bookId``. This field stores these variables.
    #[serde(rename = "pathVariables")]
    pub path_variables: Option<PathVariables>,

    #[serde(rename = "preRequestScript")]
    pub pre_request_script: Option<String>,

    /// Set of configurations used to alter the usual behavior of sending the request
    #[serde(rename = "protocolProfileBehavior")]
    pub protocol_profile_behavior: Option<ProtocolProfileBehavior>,

    #[serde(rename = "queryParams")]
    pub query_params: Option<Vec<Option<UrlParam>>>,

    /// Contains the raw data (parameters) that Postman sends to the server
    #[serde(rename = "rawModeData")]
    pub raw_mode_data: Option<RawModeData>,

    /// A Postman request can have multiple responses associated with it. These responses are
    /// stored in this field.
    #[serde(rename = "responses")]
    pub responses: Option<Vec<Option<Response>>>,

    #[serde(rename = "tests")]
    pub tests: Option<String>,

    /// The timestamp for this request.
    #[serde(rename = "time")]
    pub time: Option<f64>,

    /// Contains the complete URL for this request, along with the path variables, if any.
    #[serde(rename = "url")]
    pub url: String,

    #[serde(rename = "variables")]
    pub variables: Option<Vec<Variable>>,
}

/// Data is an array of key-values that the request goes with. POST data, PUT data, etc goes
/// here.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Datum {
    /// Override Content-Type header of this form data entity.
    #[serde(rename = "contentType")]
    pub content_type: Option<String>,

    #[serde(rename = "description")]
    pub description: Option<String>,

    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,

    #[serde(rename = "key")]
    pub key: Option<String>,

    #[serde(rename = "type")]
    pub datum_type: Option<serde_json::Value>,

    #[serde(rename = "value")]
    pub value: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct HeaderClass {
    /// You can associate descriptions with headers too.
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// Name of the header goes here. e.g: `Content-Type`
    #[serde(rename = "key")]
    pub key: Option<String>,

    /// The value of the header
    #[serde(rename = "value")]
    pub value: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct HelperClass {
    /// The helper type. Either `awsSigV4`, `basic`, `bearer`, `digest`, `hawk`, `ntlm`, `oAuth1`, or `oAuth2`.
    #[serde(rename = "id")]
    pub id: Option<HelperType>,
    /* TODO: Get boxed raw values working
    /// The attributes for [AWS Auth](http://docs.aws.amazon.com/AmazonS3/latest/dev/RESTAuthentication.html). e.g.
    /// accessKey, secretKey, region, service.
    ///
    /// The attributes for [Digest Authentication](https://en.wikipedia.org/wiki/Digest_access_authentication). e.g.
    /// username, password, realm, nonce, nonceCount, algorithm, qop, opaque, clientNonce.
    ///
    /// The attributes for [Hawk Authentication](https://github.com/hueniverse/hawk). e.g.
    /// authId, authKey, algorith, user, nonce, extraData, appId, delegation, timestamp.
    ///
    /// The attributes for [NTLM Authentication](https://msdn.microsoft.com/en-us/library/cc237488.aspx). e.g. username,
    /// password, domain, workstation.
    ///
    /// The attributes for [Basic Authentication](https://en.wikipedia.org/wiki/Basic_access_authentication). e.g.
    /// username, password.
    ///
    /// The attributes for [Bearer Token Authentication](https://tools.ietf.org/html/rfc6750).
    /// e.g. token.
    ///
    /// The attributes for [OAuth1](https://oauth.net/1/). e.g. consumerKey, consumerSecret,
    /// token, tokenSecret, signatureMethod, timestamp, nonce, version, realm, encodeOAuthSign.
    ///
    /// The attributes for [OAuth2](https://oauth.net/2/). e.g. accessToken, addTokenTo.
    #[serde(flatten)]
    pub attributes: Box<RawValue>,
    */
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct PathVariableClass {
    /// Extra description about a path variable may be added in this field.
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// The identifier of a path variable goes here.
    #[serde(rename = "key")]
    pub key: Option<String>,

    /// The value of the path variable will be substituted in place of the key.
    #[serde(rename = "value")]
    pub value: Option<String>,
}

/// Set of configurations used to alter the usual behavior of sending the request
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct ProtocolProfileBehavior {
    /// Disable body pruning for GET, COPY, HEAD, PURGE and UNLOCK request methods.
    #[serde(rename = "disableBodyPruning")]
    pub disable_body_pruning: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct UrlParamClass {
    /// You can associate descriptions with URL parameters, which are stored in this field.
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// The key of a URL parameter.
    #[serde(rename = "key")]
    pub key: Option<String>,

    /// The value of a URL parameter
    #[serde(rename = "value")]
    pub value: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct ResponseClass {
    #[serde(rename = "cookies")]
    pub cookies: Option<Vec<Cookie>>,

    #[serde(rename = "headers")]
    pub headers: Option<Vec<HeaderElement>>,

    /// In order to unambiguously identify a response, Postman assigns a UUID to it, and stores
    /// it in this field.
    #[serde(rename = "id")]
    pub id: String,

    /// The language associated with the response.
    #[serde(rename = "language")]
    pub language: Option<Language>,

    /// Mimetype of the response.
    #[serde(rename = "mime")]
    pub mime: Option<String>,

    /// A response can have a friendly name, which goes here.
    #[serde(rename = "name")]
    pub name: Option<String>,

    /// The data type of the raw response.
    #[serde(rename = "rawDataType")]
    pub raw_data_type: Option<String>,

    /// A response is associated with a request. This fields contains the UUID of the request
    /// corresponding to this response.
    #[serde(rename = "request")]
    pub request: Option<PathVariables>,

    #[serde(rename = "responseCode")]
    pub response_code: ResponseCode,

    #[serde(rename = "status")]
    pub status: Option<String>,

    /// The raw text of the response.
    #[serde(rename = "text")]
    pub text: Option<String>,

    /// The time taken by this particular HTTP transaction to complete is stored in this field.
    /// For manually created responses, this field can be set to ``null``.
    #[serde(rename = "time")]
    pub time: Option<Time>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Cookie {
    /// The domain for which this cookie is valid.
    #[serde(rename = "domain")]
    pub domain: String,

    /// The timestamp of the time when the cookie expires.
    #[serde(rename = "expirationDate")]
    pub expiration_date: f64,

    /// Human readable expiration time.
    #[serde(rename = "expires")]
    pub expires: String,

    /// Indicates if this cookie is Host Only.
    #[serde(rename = "hostOnly")]
    pub host_only: bool,

    /// Indicates if this cookie is HTTP Only.
    #[serde(rename = "httpOnly")]
    pub http_only: bool,

    /// This is the name of the Cookie.
    #[serde(rename = "name")]
    pub name: String,

    /// The path associated with the Cookie.
    #[serde(rename = "path")]
    pub path: String,

    /// Indicates if the 'secure' flag is set on the Cookie.
    #[serde(rename = "secure")]
    pub secure: bool,

    /// True if the cookie is a session cookie.
    #[serde(rename = "session")]
    pub session: bool,

    /// The ID of the cookie store containing this cookie.
    #[serde(rename = "storeId")]
    pub store_id: String,

    /// The value of the Cookie.
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct HeaderElement {
    /// An optional description about the header.
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// The left hand side (LHS) or 'key' of the header.
    #[serde(rename = "key")]
    pub key: String,

    /// Some headers can have names associated with them, which are stored in this field.
    #[serde(rename = "name")]
    pub name: Option<String>,

    /// Value of the header, or the right hand side (RHS).
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct ResponseCode {
    /// The numeric HTTP response code.
    #[serde(rename = "code")]
    pub code: f64,

    /// Detailed explanation of the response code.
    #[serde(rename = "detail")]
    pub detail: Option<String>,

    /// The textual HTTP response code.
    #[serde(rename = "name")]
    pub name: String,
}

/// The host for the URL, E.g: api.yourdomain.com. Can be stored as a string or as an array
/// of strings.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Host {
    String(String),

    StringArray(Vec<String>),
}

/// If object, contains the complete broken-down URL for this request. If string, contains
/// the literal request URL.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Url {
    String(String),

    UrlClass(UrlClass),
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum UrlPath {
    String(String),

    UnionArray(Vec<PathElement>),
}

/// The complete path of the current url, broken down into segments. A segment could be a
/// string, or a path variable.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum PathElement {
    String(String),

    PathClass(PathClass),
}

/// A Description can be a raw text, or be an object, which holds the description along with
/// its format.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Description {
    String(String),

    DescriptionClass(DescriptionClass),
}

/// A response represents an HTTP response.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Header {
    AnythingArray(Vec<Option<serde_json::Value>>),

    Bool(bool),

    Double(f64),

    HeaderClass(HeaderClass),

    Integer(i64),

    String(String),
}

/// A helper may require a number of parameters to actually be helpful. The parameters used
/// by the helper can be stored in this field, as an object. E.g when using Basic
/// Authentication, the username and password will be stored here.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum HelperAttributes {
    String(String),

    HelperClass(HelperClass),
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum HelperType {
    #[serde(rename = "awsSigV4")]
    AwsSigV4,

    #[serde(rename = "basic")]
    Basic,

    #[serde(rename = "bearer")]
    Bearer,

    #[serde(rename = "digest")]
    Digest,

    #[serde(rename = "hawk")]
    Hawk,

    #[serde(rename = "ntlm")]
    Ntlm,

    #[serde(rename = "oAuth1")]
    OAuth1,

    #[serde(rename = "oAuth2")]
    OAuth2,
}

/// A request URL may contain one or more path variables (e.g: `:varname`)
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum PathVariable {
    AnythingArray(Vec<Option<serde_json::Value>>),

    Bool(bool),

    Double(f64),

    Integer(i64),

    PathVariableClass(PathVariableClass),

    String(String),
}

/// A Postman request allows you to use Path Variables in a request, e.g:
/// ``/search/:bookId``. This field stores these variables.
///
/// A response is associated with a request. This fields contains the UUID of the request
/// corresponding to this response.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum PathVariables {
    AnythingMap(HashMap<String, Option<serde_json::Value>>),

    String(String),
}

/// A response represents an HTTP response.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum UrlParam {
    AnythingArray(Vec<Option<serde_json::Value>>),

    Bool(bool),

    Double(f64),

    Integer(i64),

    String(String),

    UrlParamClass(UrlParamClass),
}

/// Contains the raw data (parameters) that Postman sends to the server
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum RawModeData {
    AnythingArray(Vec<Option<serde_json::Value>>),

    String(String),
}

/// A response represents an HTTP response.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Response {
    AnythingArray(Vec<Option<serde_json::Value>>),

    Bool(bool),

    Double(f64),

    Integer(i64),

    ResponseClass(ResponseClass),

    String(String),
}

/// The time taken by this particular HTTP transaction to complete is stored in this field.
/// For manually created responses, this field can be set to ``null``.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Time {
    Double(f64),

    String(String),
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum AuthType {
    #[serde(rename = "awsv4")]
    Awsv4,

    #[serde(rename = "basic")]
    Basic,

    #[serde(rename = "bearer")]
    Bearer,

    #[serde(rename = "digest")]
    Digest,

    #[serde(rename = "hawk")]
    Hawk,

    #[serde(rename = "noauth")]
    Noauth,

    #[serde(rename = "ntlm")]
    Ntlm,

    #[serde(rename = "oauth1")]
    Oauth1,

    #[serde(rename = "oauth2")]
    Oauth2,
}

/// Returns `Noauth` for AuthType by default
impl Default for AuthType {
    fn default() -> AuthType {
        AuthType::Noauth
    }
}

/// A variable may have multiple types. This field specifies the type of the variable.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum VariableType {
    #[serde(rename = "any")]
    Any,

    #[serde(rename = "boolean")]
    Boolean,

    #[serde(rename = "number")]
    Number,

    #[serde(rename = "string")]
    String,
}

/// A request can have a specific data mode, and Postman supports these.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum DataMode {
    #[serde(rename = "binary")]
    Binary,

    #[serde(rename = "params")]
    Params,

    #[serde(rename = "raw")]
    Raw,

    #[serde(rename = "urlencoded")]
    Urlencoded,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum DescriptionFormat {
    #[serde(rename = "html")]
    Html,

    #[serde(rename = "markdown")]
    Markdown,
}

/// The language associated with the response.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum Language {
    #[serde(rename = "html")]
    Html,

    #[serde(rename = "javascript")]
    Javascript,

    #[serde(rename = "Text")]
    Text,

    #[serde(rename = "xml")]
    Xml,
}
