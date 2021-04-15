// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use std::{
    fs::{
        create_dir_all,
        File,
        {self},
    },
    io,
    io::prelude::*,
    path::{Path, PathBuf},
};

use anyhow::Error;
use dirs::home_dir;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

pub const LEO_CREDENTIALS_FILE: &str = "credentials";
pub const LEO_CONFIG_FILE: &str = "config.toml";
pub const LEO_USERNAME_FILE: &str = "username";

lazy_static! {
    pub static ref LEO_CONFIG_DIRECTORY: PathBuf = {
        let mut path = home_dir().expect("Invalid home directory");
        path.push(".leo");
        path
    };
    pub static ref LEO_CREDENTIALS_PATH: PathBuf = {
        let mut path = LEO_CONFIG_DIRECTORY.to_path_buf();
        path.push(LEO_CREDENTIALS_FILE);
        path
    };
    pub static ref LEO_USERNAME_PATH: PathBuf = {
        let mut path = LEO_CONFIG_DIRECTORY.to_path_buf();
        path.push(LEO_USERNAME_FILE);
        path
    };
    pub static ref LEO_CONFIG_PATH: PathBuf = {
        let mut path = LEO_CONFIG_DIRECTORY.to_path_buf();
        path.push(LEO_CONFIG_FILE);
        path
    };
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Update {
    pub automatic: bool,
}

impl Default for Update {
    fn default() -> Self {
        Self { automatic: true }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub update: Update,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            update: Update::default(),
        }
    }
}

impl Config {
    /// Read the config from the `config.toml` file
    pub fn read_config() -> Result<Self, Error> {
        let config_dir = LEO_CONFIG_DIRECTORY.clone();
        let config_path = LEO_CONFIG_PATH.clone();

        if !Path::exists(&config_path) {
            // Create a new default `config.toml` file if it doesn't already exist
            create_dir_all(&config_dir)?;

            let default_config_string = toml::to_string(&Config::default())?;

            fs::write(&config_path, default_config_string)?;
        }

        let toml_string = match fs::read_to_string(&config_path) {
            Ok(mut toml) => {
                // If the config is using an incorrect format, rewrite it.
                if toml::from_str::<Config>(&toml).is_err() {
                    let default_config_string = toml::to_string(&Config::default())?;
                    fs::write(&config_path, default_config_string.clone())?;
                    toml = default_config_string;
                }

                toml
            }
            Err(_) => {
                create_dir_all(&config_dir)?;
                toml::to_string(&Config::default())?
            }
        };

        // Parse the contents into the `Config` struct
        let config: Config = toml::from_str(&toml_string)?;

        Ok(config)
    }

    /// Update the `automatic` configuration in the `config.toml` file.
    pub fn set_update_automatic(automatic: bool) -> Result<(), Error> {
        let mut config = Self::read_config()?;

        if config.update.automatic != automatic {
            config.update.automatic = automatic;

            // Update the config file
            let config_path = LEO_CONFIG_PATH.clone();
            fs::write(&config_path, toml::to_string(&config)?)?;
        }

        Ok(())
    }
}

pub fn write_token_and_username(token: &str, username: &str) -> Result<(), io::Error> {
    let config_dir = LEO_CONFIG_DIRECTORY.clone();

    // Create Leo config directory if it not exists
    if !Path::new(&config_dir).exists() {
        create_dir_all(&config_dir)?;
    }

    let mut credentials = File::create(&LEO_CREDENTIALS_PATH.to_path_buf())?;
    credentials.write_all(&token.as_bytes())?;

    let mut username_file = File::create(&LEO_USERNAME_PATH.to_path_buf())?;
    username_file.write_all(&username.as_bytes())?;

    Ok(())
}

pub fn read_token() -> Result<String, io::Error> {
    let mut credentials = File::open(&LEO_CREDENTIALS_PATH.to_path_buf())?;
    let mut buf = String::new();
    credentials.read_to_string(&mut buf)?;
    Ok(buf)
}

pub fn read_username() -> Result<String, io::Error> {
    let mut username = File::open(&LEO_USERNAME_PATH.to_path_buf())?;
    let mut buf = String::new();
    username.read_to_string(&mut buf)?;
    Ok(buf)
}

pub fn remove_token_and_username() -> Result<(), io::Error> {
    fs::remove_file(&LEO_CREDENTIALS_PATH.to_path_buf())?;
    fs::remove_file(&LEO_USERNAME_PATH.to_path_buf())?;
    Ok(())
}
