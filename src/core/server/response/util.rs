use crate::core::server::constant::DEFAULT_PLAIN_TEXT_CONTENT_TYPE;

/// content type from text file extension
pub fn text_file_content_type(ext: &str) -> String {
    let ret = match ext {
        "html" => "text/html",
        "css" => "text/css",
        "js" => "application/javascript",
        _ => DEFAULT_PLAIN_TEXT_CONTENT_TYPE,
    };
    ret.to_owned()
}
