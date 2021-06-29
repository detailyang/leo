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

use crate::{errors::ImportParserError, ImportParser};
use leo_ast::{Program, Span};

use std::fs::DirEntry;

static MAIN_FILE: &str = "src/main.leo";

impl<'a> ImportParser<'a> {
    ///
    /// Returns a Leo syntax tree from a given package.
    ///
    /// Builds an abstract syntax tree from the given file and then builds the Leo syntax tree.
    ///
    pub(crate) fn parse_import_file(package: &DirEntry, span: &Span) -> Result<Program, ImportParserError> {
        // Get the package file type.
        let file_type = package
            .file_type()
            .map_err(|error| ImportParserError::directory_error(error, span, &package.path()))?;
        let file_name = package
            .file_name()
            .into_string()
            .map_err(|_| ImportParserError::convert_os_string(span))?;

        let mut file_path = package.path();
        if file_type.is_dir() {
            file_path.push(MAIN_FILE);

            if !file_path.exists() {
                return Err(ImportParserError::expected_main_file(
                    format!("{:?}", file_path.as_path()),
                    span,
                ));
            }
        }

        let file_path_str = file_path.to_str().unwrap_or_default();

        // Build the package abstract syntax tree.
        let program_string =
            &std::fs::read_to_string(&file_path).map_err(|x| ImportParserError::io_error(span, file_path_str, x))?;
        let mut ast = leo_parser::parse(&file_path_str, &program_string)?;
        ast.name = file_name;
        Ok(ast)
    }
}
