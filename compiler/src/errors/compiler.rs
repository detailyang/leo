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

use crate::errors::{FunctionError, ImportError, OutputBytesError, OutputFileError};
use leo_asg::{AsgConvertError, FormattedError};
use leo_ast::LeoError;
use leo_imports::ImportParserError;
use leo_input::InputParserError;
use leo_parser::SyntaxError;
use leo_state::LocalDataVerificationError;

use bincode::Error as SerdeError;
use std::path::PathBuf;

#[derive(Debug, Error)]
pub enum CompilerError {
    #[error("{}", _0)]
    SyntaxError(#[from] SyntaxError),

    #[error("{}", _0)]
    AsgPassError(FormattedError),

    #[error("{}", _0)]
    ImportError(#[from] ImportError),

    #[error("{}", _0)]
    ImportParserError(#[from] ImportParserError),

    #[error("{}", _0)]
    InputParserError(#[from] InputParserError),

    #[error("Cannot find input files with context name `{}`", _0)]
    InvalidTestContext(String),

    #[error("{}", _0)]
    FunctionError(#[from] FunctionError),

    #[error("Cannot read from the provided file path '{:?}': {}", _0, _1)]
    FileReadError(PathBuf, std::io::Error),

    #[error("{}", _0)]
    LocalDataVerificationError(#[from] LocalDataVerificationError),

    #[error("`main` function not found")]
    NoMain,

    #[error("`main` must be a function")]
    NoMainFunction,

    #[error("Failed to find input files for the current test")]
    NoTestInput,

    #[error("{}", _0)]
    OutputError(#[from] OutputFileError),

    #[error("{}", _0)]
    OutputStringError(#[from] OutputBytesError),

    #[error("{}", _0)]
    SerdeError(#[from] SerdeError),

    #[error("{}", _0)]
    AsgConvertError(#[from] AsgConvertError),
}

impl LeoError for CompilerError {
    fn get_path(&self) -> Option<&str> {
        match self {
            CompilerError::SyntaxError(error) => error.get_path(),
            CompilerError::ImportError(error) => error.get_path(),
            CompilerError::ImportParserError(error) => error.get_path(),
            CompilerError::InputParserError(error) => error.get_path(),
            CompilerError::FunctionError(error) => error.get_path(),
            CompilerError::OutputStringError(error) => error.get_path(),
            CompilerError::AsgConvertError(error) => error.get_path(),
            _ => None,
        }
    }

    fn set_path(&mut self, path: &str, contents: &[String]) {
        match self {
            CompilerError::SyntaxError(error) => error.set_path(path, contents),
            CompilerError::ImportError(error) => error.set_path(path, contents),
            CompilerError::ImportParserError(error) => error.set_path(path, contents),
            CompilerError::InputParserError(error) => error.set_path(path, contents),
            CompilerError::FunctionError(error) => error.set_path(path, contents),
            CompilerError::OutputStringError(error) => error.set_path(path, contents),
            CompilerError::AsgConvertError(error) => error.set_path(path, contents),
            _ => {}
        }
    }
}
