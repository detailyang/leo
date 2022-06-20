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

//! Methods to enforce constraints on input field values in a compiled Leo program.

use crate::{number_string_typing, value::ConstrainedValue, FieldType, GroupType};
use leo_ast::InputValue;
use leo_errors::{CompilerError, Result, Span};

use snarkvm_fields::PrimeField;
use snarkvm_gadgets::traits::alloc::AllocGadget;
use snarkvm_r1cs::{ConstraintSystem, SynthesisError};

pub(crate) fn allocate_field<F: PrimeField, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    name: &str,
    option: Option<String>,
    span: &Span,
) -> Result<FieldType<F>> {
    match option {
        Some(string) => {
            let number_info = number_string_typing(&string);

            match number_info {
                (number, neg) if neg => FieldType::alloc(
                    cs.ns(|| format!("`{}: field` {}:{}", name, span.line_start, span.col_start)),
                    || Some(number).ok_or(SynthesisError::AssignmentMissing),
                )
                .map(|value| value.negate(cs, span))
                .map_err(|_| CompilerError::field_value_missing_field(format!("{}: field", name), span))?,
                (number, _) => Ok(FieldType::alloc(
                    cs.ns(|| format!("`{}: field` {}:{}", name, span.line_start, span.col_start)),
                    || Some(number).ok_or(SynthesisError::AssignmentMissing),
                )
                .map_err(|_| CompilerError::field_value_missing_field(format!("{}: field", name), span))?),
            }
        }
        None => {
            return Err(CompilerError::field_value_missing_field(format!("{}: field", name), span).into());
        }
    }
}

pub(crate) fn field_from_input<'a, F: PrimeField, G: GroupType<F>, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    name: &str,
    input_value: Option<InputValue>,
    span: &Span,
) -> Result<ConstrainedValue<'a, F, G>> {
    // Check that the parameter value is the correct type
    let option = match input_value {
        Some(input) => {
            if let InputValue::Field(string) = input {
                Some(string)
            } else {
                return Err(CompilerError::field_value_invalid_field(input, span).into());
            }
        }
        None => None,
    };

    let field = allocate_field(cs, name, option, span)?;

    Ok(ConstrainedValue::Field(field))
}
