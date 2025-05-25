// todo

// /// print out paths on `data_dir` (static json responses)
// pub fn print_paths(config: &Config) {
//     let paths = &self.paths.clone().unwrap();
//     let mut keys: Vec<_> = paths.keys().collect();
//     keys.sort();
//     for key in keys {
//         log::info!(
//             "[path] {} => [{}]{}{}",
//             style(paths.get_key_value(key).unwrap().0).yellow(),
//             paths.get(key).unwrap().status.as_u16(),
//             if let Some(data_src) = &paths.get(key).unwrap().data_src {
//                 style(format!(" {}", data_src.as_str())).green()
//             } else {
//                 if let Some(_) = &paths.get(key).unwrap().data_text {
//                     style(" (text)".to_owned()).green()
//                 } else {
//                     style(String::new()).green()
//                 }
//             },
//             if let Some(headers) = &paths.get(key).unwrap().headers {
//                 let printed_outs = headers
//                     .iter()
//                     .map(|x| format!("{}", style(x.to_owned()).magenta()))
//                     .collect::<Vec<String>>()
//                     .join(", ");
//                 format!(" {{{}}}", printed_outs)
//             } else {
//                 String::new()
//             },
//         );

//         if let Some(path_jsonpath_patterns) = &self.paths_jsonpath_patterns {
//             if let Some(jsonpath_patterns) = path_jsonpath_patterns.get(key) {
//                 let mut keys: Vec<_> = jsonpath_patterns.keys().collect();
//                 keys.sort();
//                 log::info!(
//                     " jsonpath {}",
//                     keys.iter()
//                         .map(|&jsonpath| {
//                             jsonpath_patterns
//                                 .get(jsonpath)
//                                 .unwrap()
//                                 .iter()
//                                 .map(|pattern| {
//                                     format!(
//                                         "case {} = \"{}\"\n            => {}",
//                                         style(jsonpath).yellow(),
//                                         style(pattern.value.to_owned()).magenta(),
//                                         style(pattern.data_src.to_owned()).green()
//                                     )
//                                 })
//                                 .collect::<Vec<String>>()
//                                 .join("\n          ")
//                         })
//                         .collect::<Vec<String>>()
//                         .join("\n          ")
//                 );
//             }
//         }
//     }
// }
