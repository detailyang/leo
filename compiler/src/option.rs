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

///
/// Toggles compiler optimizations on the program.
///
#[derive(Clone)]
pub struct CompilerOptions {
    pub canonicalization_enabled: bool,
    pub constant_folding_enabled: bool,
    pub dead_code_elimination_enabled: bool,
    pub type_inference_enabled: bool,
}

impl Default for CompilerOptions {
    ///
    /// All compiler optimizations are enabled by default.
    ///
    fn default() -> Self {
        CompilerOptions {
            canonicalization_enabled: true,
            constant_folding_enabled: true,
            dead_code_elimination_enabled: true,
            type_inference_enabled: true,
        }
    }
}
