use std::{env, fs, path::Path};

use super::constant::args;

/// env args passed at startup
#[derive(Clone)]
pub struct EnvArgs {
    /// config .toml file path
    pub config_filepath: Option<String>,
    /// overwrites value in config file
    pub port: Option<u16>,
    /// middleware .rhai file path
    pub middleware_filepath: Option<String>,
}

impl EnvArgs {
    /// generate from env args, and set default if not specified at each arg
    pub fn init_with_default() -> Self {
        let mut ret = EnvArgs::init();

        ret.default_config_filepath();
        ret.default_middleware_filepath();

        let init_with_default_files =
            args_option_value(args::INIT_WITH_DEFAULT_FILES_OPTION_NAMES.to_vec().as_ref())
                .is_some();
        if init_with_default_files {
            ret.init_with_default_files();
        }

        match ret.validate() {
            Ok(_) => ret,
            Err(_) => panic!("something wrong in env args"),
        }
    }

    pub fn validate(&self) -> Result<(), ()> {
        if let Some(config_filepath) = self.config_filepath.as_ref() {
            if !Path::new(config_filepath.as_str()).exists() {
                panic!(
                    "config file was specified but didn't exist: {}",
                    config_filepath
                );
            }
        }

        if let Some(middleware_filepath) = self.middleware_filepath.as_ref() {
            if !Path::new(middleware_filepath.as_str()).exists() {
                panic!(
                    "middleware file was specified but didn't exist: {}",
                    middleware_filepath
                );
            }
        }

        Ok(())
    }

    /// generate from env args
    fn init() -> Self {
        let port =
            if let Some(port) =
                args_option_value(args::CONFIG_LISTENER_PORT_OPTION_NAMES.to_vec().as_ref())
            {
                Some(port.parse::<u16>().expect(
                    format!("env arg `port` is {} - specified but not number", port).as_str(),
                ))
            } else {
                None
            };

        let ret = EnvArgs {
            config_filepath: args_option_value(
                args::CONFIG_FILEPATH_OPTION_NAMES.to_vec().as_ref(),
            ),
            port,
            middleware_filepath: args_option_value(
                args::MIDDLEWARE_FILEPATH_OPTION_NAMES.to_vec().as_ref(),
            ),
        };

        ret
    }

    fn init_with_default_files(&mut self) {
        if self.config_filepath.is_none() {
            let filepath = args::DEFAULT_CONFIG_FILEPATH;
            let content = include_str!("../../examples/config/default/apimock.toml");
            let _ = fs::write(filepath, content);
            self.config_filepath = Some(filepath.to_owned());
        }

        if self.middleware_filepath.is_none() {
            let filepath = args::DEFAULT_MIDDLEWARE_FILEPATH;
            let content = include_str!("../../examples/config/default/apimock-middleware.rhai");
            let _ = fs::write(filepath, content);
            self.middleware_filepath = Some(filepath.to_owned());
        }
    }

    /// app config file path
    ///
    /// - if specified in arguments, use it
    /// - else if default file exists, use it
    /// - else miss it
    fn default_config_filepath(&mut self) {
        if self.config_filepath.is_some() {
            return;
        }
        if !Path::new(args::DEFAULT_CONFIG_FILEPATH).exists() {
            return;
        }

        self.config_filepath = Some(args::DEFAULT_CONFIG_FILEPATH.to_owned());
    }

    /// app middleware file path
    ///
    /// - if specified in arguments, use it
    /// - else if default file exists, use it
    /// - else miss it
    fn default_middleware_filepath(&mut self) {
        if self.middleware_filepath.is_some() {
            return;
        }
        if !Path::new(args::DEFAULT_MIDDLEWARE_FILEPATH).exists() {
            return;
        }

        self.middleware_filepath = Some(args::DEFAULT_MIDDLEWARE_FILEPATH.to_owned());
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
