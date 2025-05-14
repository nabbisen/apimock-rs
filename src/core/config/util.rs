use console::style;

use std::{fs, path::Path};

use super::Config;

/// api url full path
pub fn fullpath(path: &str, path_prefix: &Option<String>, is_raw_paths: bool) -> String {
    let possibly_w_trailing_slash = if is_raw_paths {
        format!("/{}/", path.to_string())
    } else {
        if let Some(path_prefix) = path_prefix {
            format!("/{}/{}/", path_prefix, path.to_string())
        } else {
            format!("/{}/", path.to_string())
        }
    }
    .replace("//", "/");

    (&possibly_w_trailing_slash[..possibly_w_trailing_slash.len() - 1]).to_owned()
}

/// `data_src` path on static json responses
pub fn data_src_path(file: &str, data_dir: &Option<String>) -> String {
    let data_dir = if let Some(x) = data_dir.clone() {
        x.to_owned()
    } else {
        String::new()
    };
    let path = Path::new(data_dir.as_str())
        .join(file)
        .display()
        .to_string();
    let _ = fs::metadata(&path).expect(format!("`{}` is missing", path).as_str());
    path
}

/// wholly print out config
pub fn print(config: &Config) {
    // if let Some(always) = config.always {
    //     log::info!("[always] {}", always);
    // }
    // log::info!(
    //     "[response wait] {}",
    //     if 0 < config.response_wait_millis {
    //         format!("{} milliseconds", config.response_wait_millis)
    //     } else {
    //         "-".to_owned()
    //     }
    // );
    log::info!(
        "[log.verbose] header = {}, body = {}",
        if config.log.verbose.header {
            "Yes"
        } else {
            "No"
        },
        if config.log.verbose.body { "Yes" } else { "No" }
    );
    log::info!("------");
    // if let Some(data_dir) = &config.data_dir {
    //     log::info!("[data_dir] {}", data_dir);
    // }
    // if let Some(data_dir_query_path) = &config.data_dir_query_path {
    //     log::info!(
    //         "[data_dir_query_url] http://{}/{}",
    //         &config.listener_address(),
    //         data_dir_query_path
    //     );
    // }
    // if let Some(path_prefix) = &config.path_prefix {
    //     log::info!("[path_prefix] {}", path_prefix);
    // }
    // let _ = match &config.headers {
    //     Some(headers) if 0 < headers.len() => {
    //         log::info!("------");
    //         let mut keys: Vec<_> = headers.keys().collect();
    //         keys.sort();
    //         for key in keys {
    //             log::info!(
    //                 "[header] {} = {}{}",
    //                 style(headers.get_key_value(key).unwrap().0).magenta(),
    //                 headers.get(key).unwrap().key.clone(),
    //                 if let Some(value) = headers.get(key).unwrap().value.clone() {
    //                     format!(": {}", value)
    //                 } else {
    //                     String::new()
    //                 }
    //             );
    //         }
    //     }
    //     _ => (),
    // };
    // let _ = match &config.paths {
    //     Some(paths) if 0 < paths.len() => {
    //         log::info!("------");
    //         config.print_paths();
    //         log::info!("------");
    //     }
    //     _ => (),
    // };
    let p = Path::new(config.service.fallback_response_dir.as_str());
    let p = if p.is_relative() {
        let absolute_path = fs::canonicalize(config.service.fallback_response_dir.as_str()).expect(
            format!(
                "{} does not exist",
                config.service.fallback_response_dir.as_str()
            )
            .as_str(),
        );
        format!(
            "{} ({})",
            style(config.service.fallback_response_dir.as_str()).green(),
            absolute_path.to_str().expect(
                format!(
                    "logger failed to print out: {}",
                    config.service.fallback_response_dir.as_str()
                )
                .as_str()
            )
        )
    } else {
        style(config.service.fallback_response_dir.clone())
            .green()
            .to_string()
    };
    log::info!("[service.fallback_response_dir] {}", p);
}
