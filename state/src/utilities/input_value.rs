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

use leo_ast::{InputValue, Parameter};
use leo_errors::{Result, StateError};

use indexmap::IndexMap;

/// Returns the input parameter with the given name.
/// If a parameter with the given name does not exist, then an error is returned.
pub fn find_input(name: String, parameters: &IndexMap<Parameter, Option<InputValue>>) -> Result<InputValue> {
    let matched_parameter = parameters
        .iter()
        .find(|(parameter, _value)| parameter.variable.name.as_ref() == name);

    match matched_parameter {
        Some((_, Some(value))) => Ok(value.clone()),
        _ => Err(StateError::missing_parameter(name).into()),
    }
}

/// Returns the string of the integer input value.
/// If the input value is not an integer, then an error is returned.
pub fn input_to_integer_string(input: InputValue) -> Result<String> {
    match input {
        InputValue::Integer(_type, string) => Ok(string),
        value => Err(StateError::expected_int(value).into()),
    }
}

/// Returns the given input value as u8 bytes.
/// If the given input value cannot be serialized into bytes then an error is returned.
pub fn input_to_bytes(input: InputValue) -> Result<Vec<u8>> {
    let input_array = match input {
        InputValue::Array(values) => values,
        value => return Err(StateError::expected_bytes(value).into()),
    };

    let mut result_vec = Vec::with_capacity(input_array.len());

    for input in input_array {
        let integer_string = input_to_integer_string(input)?;
        let byte = integer_string.parse::<u8>().map_err(StateError::parse_int_error)?;

        result_vec.push(byte);
    }

    Ok(result_vec)
}

/// Returns the given input value as an array of u8 bytes.
/// If the given input value cannot be serialized into an array of bytes then an error is returned.
pub fn input_to_nested_bytes(input: InputValue) -> Result<Vec<Vec<u8>>> {
    let inner_arrays = match input {
        InputValue::Array(arrays) => arrays,
        value => return Err(StateError::expected_bytes(value).into()),
    };

    let mut result_vec = Vec::with_capacity(inner_arrays.len());

    for input_array in inner_arrays {
        let array = input_to_bytes(input_array)?;

        result_vec.push(array);
    }

    Ok(result_vec)
}
