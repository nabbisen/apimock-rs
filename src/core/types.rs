use std::{collections::HashMap, convert::Infallible};

use hyper::body::Bytes;

use super::config::{HeaderConfig, HeaderId, JsonpathMatchingPattern, PathConfig, UrlPath};

pub type BoxBody = http_body_util::combinators::BoxBody<Bytes, Infallible>;

pub type ConfigHeaders = HashMap<HeaderId, HeaderConfig>;
pub type ConfigUrlPaths = HashMap<UrlPath, PathConfig>;
pub type ConfigUrlPathsJsonpathPattern = HashMap<String, Vec<JsonpathMatchingPattern>>;
pub type ConfigUrlPathsJsonpathPatterns = HashMap<String, ConfigUrlPathsJsonpathPattern>;
