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

use super::build::Build;
use crate::{api::Publish as PublishRoute, commands::Command, context::Context};
use leo_package::{
    outputs::OutputsDirectory,
    root::{ZipFile, AUTHOR_PLACEHOLDER},
};

use anyhow::{anyhow, Result};
use structopt::StructOpt;

/// Publish package to Aleo Package Manager
#[derive(StructOpt, Debug)]
#[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
pub struct Publish {}

impl Command for Publish {
    type Input = <Build as Command>::Output;
    type Output = String;

    /// Build program before publishing
    fn prelude(&self, context: Context) -> Result<Self::Input> {
        (Build {
            compiler_options: Default::default(),
        })
        .execute(context)
    }

    fn apply(self, context: Context, _input: Self::Input) -> Result<Self::Output> {
        // Get the package manifest
        let path = context.dir()?;
        let manifest = context.manifest()?;

        let package_name = manifest.get_package_name();
        let package_version = manifest.get_package_version();

        match (
            manifest.get_package_description(),
            manifest.get_package_license(),
            manifest.get_package_remote(),
        ) {
            (None, _, _) => return Err(anyhow!("No package description")),
            (_, None, _) => return Err(anyhow!("Missing package license")),
            (_, _, None) => return Err(anyhow!("Missing package remote")),
            (_, _, _) => (),
        };

        let package_remote = manifest.get_package_remote().unwrap();
        let username = package_remote.clone().author;

        // Prevent most common error before accessing API.
        if username == AUTHOR_PLACEHOLDER {
            return Err(anyhow!(
                "Package author is not set. Specify package author in [remote] section of Leo.toml"
            ));
        }

        // Create the output directory.
        OutputsDirectory::create(&path)?;

        // Create zip file.
        let zip_file = ZipFile::new(&package_name);
        if zip_file.exists_at(&path) {
            tracing::debug!("Existing package zip file found. Clearing it to regenerate.");
            // Remove the existing package zip file
            zip_file.remove(&path)?;
        }
        zip_file.write(&path)?;

        // Make an API request with zip file and package data.
        let package_id = context.api.run_route(PublishRoute {
            name: package_name.clone(),
            remote: format!("{}/{}", package_remote.author, package_name),
            version: package_version,
            file: zip_file.get_file_path(&path).into(),
        })?;

        tracing::info!("Package published successfully with id: {}", &package_id);
        Ok(package_id)
    }
}
