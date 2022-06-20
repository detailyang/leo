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

//! Enforces array access in a compiled Leo program.

use crate::{program::ConstrainedProgram, value::ConstrainedValue, GroupType};
use leo_asg::Expression;
use leo_errors::{CompilerError, Result, Span};

use snarkvm_fields::PrimeField;
use snarkvm_r1cs::ConstraintSystem;

impl<'a, F: PrimeField, G: GroupType<F>> ConstrainedProgram<'a, F, G> {
    #[allow(clippy::too_many_arguments)]
    pub fn enforce_tuple_access<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        tuple: &'a Expression<'a>,
        index: usize,
        span: &Span,
    ) -> Result<ConstrainedValue<'a, F, G>> {
        // Get the tuple values.
        let tuple = match self.enforce_expression(cs, tuple)? {
            ConstrainedValue::Tuple(tuple) => tuple,
            value => return Err(CompilerError::undefined_array(value, span).into()),
        };

        // Check for out of bounds access.
        if index > tuple.len() - 1 {
            // probably safe to be a panic here
            return Err(CompilerError::tuple_index_out_of_bounds(index, span).into());
        }

        Ok(tuple[index].to_owned())
    }
}
