use std::convert::Infallible;

use hyper::body::Bytes;

pub type BoxBody = http_body_util::combinators::BoxBody<Bytes, Infallible>;
