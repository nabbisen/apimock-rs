use std::{env, fs, path::Path};

pub mod constant;

use constant::*;

/// env args passed at startup
#[derive(Clone)]
pub struct EnvArgs {
    /// config .toml file path
    pub config_file_path: Option<String>,
    /// overwrites value in config file
    pub port: Option<u16>,
    /// middleware .rhai file path
    pub middleware_file_path: Option<String>,
}

impl EnvArgs {
    /// generate from env args, and set default if not specified at each arg
    pub fn init_with_default() -> Self {
        let mut ret = EnvArgs::init();

        ret.default_config_file_path();
        ret.default_middleware_file_path();

        let init_with_default_files =
            args_option_value(INIT_WITH_DEFAULT_FILES_OPTION_NAMES.to_vec().as_ref()).is_some();
        if init_with_default_files {
            ret.init_with_default_files();
        }

        match ret.validate() {
            Ok(_) => ret,
            Err(_) => panic!("something wrong in env args"),
        }
    }

    pub fn validate(&self) -> Result<(), ()> {
        if let Some(config_file_path) = self.config_file_path.as_ref() {
            if !Path::new(config_file_path.as_str()).exists() {
                panic!(
                    "config file was specified but didn't exist: {}",
                    config_file_path
                );
            }
        }

        Ok(())
    }

    /// generate from env args
    fn init() -> Self {
        let port =
            if let Some(port) =
                args_option_value(CONFIG_LISTENER_PORT_OPTION_NAMES.to_vec().as_ref())
            {
                Some(port.parse::<u16>().expect(
                    format!("env arg `port` is {} - specified but not number", port).as_str(),
                ))
            } else {
                None
            };

        let ret = EnvArgs {
            config_file_path: args_option_value(CONFIG_FILE_PATH_OPTION_NAMES.to_vec().as_ref()),
            port,
            middleware_file_path: args_option_value(
                MIDDLEWARE_FILE_PATH_OPTION_NAMES.to_vec().as_ref(),
            ),
        };

        ret
    }

    fn init_with_default_files(&mut self) {
        if self.config_file_path.is_none() {
            let config_file_path = DEFAULT_CONFIG_FILE_PATH;
            let config_content = include_str!("../../examples/config/default/apimock.toml");
            let _ = fs::write(config_file_path, config_content);

            if !Path::new(DEFAULT_RULE_SET_FILE_PATH).exists() {
                let rule_set_file_path = DEFAULT_RULE_SET_FILE_PATH;
                let rule_set_content =
                    include_str!("../../examples/config/default/apimock-rule-set.toml");
                let _ = fs::write(rule_set_file_path, rule_set_content);
            }

            self.config_file_path = Some(config_file_path.to_owned());
        }

        if self.middleware_file_path.is_none() {
            let file_path = DEFAULT_MIDDLEWARE_FILE_PATH;
            let content = include_str!("../../examples/config/default/apimock-middleware.rhai");
            let _ = fs::write(file_path, content);
            self.middleware_file_path = Some(file_path.to_owned());
        }
    }

    /// app config file path
    ///
    /// - if specified in arguments, use it
    /// - else if default file exists, use it
    /// - else miss it
    fn default_config_file_path(&mut self) {
        if self.config_file_path.is_some() {
            return;
        }
        if !Path::new(DEFAULT_CONFIG_FILE_PATH).exists() {
            return;
        }

        self.config_file_path = Some(DEFAULT_CONFIG_FILE_PATH.to_owned());
    }

    /// app middleware file path
    ///
    /// - if specified in arguments, use it
    /// - else if default file exists, use it
    /// - else miss it
    fn default_middleware_file_path(&mut self) {
        if self.middleware_file_path.is_some() {
            return;
        }
        if !Path::new(DEFAULT_MIDDLEWARE_FILE_PATH).exists() {
            return;
        }

        self.middleware_file_path = Some(DEFAULT_MIDDLEWARE_FILE_PATH.to_owned());
    }
}

/// arguments: get option value if the option name is found
fn args_option_value(option_names: &Vec<&str>) -> Option<String> {
    let args: Vec<String> = env::args().collect();

    let name_index = args.iter().position(|arg| {
        option_names
            .iter()
            .any(|option_name| arg.as_str().eq(*option_name))
    });

    if let Some(name_index) = name_index {
        let name_value = args.get(name_index + 1);
        return match name_value {
            Some(name_value) if !name_value.starts_with("-") => Some(name_value.to_owned()),
            _ => Some(String::new()),
        };
    }

    None
}
