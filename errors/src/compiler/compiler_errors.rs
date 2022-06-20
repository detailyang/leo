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

use crate::create_errors;

use std::{
    error::Error as ErrorArg,
    fmt::{Debug, Display},
};

create_errors!(
    /// CompilerError enum that represents all the errors for the `leo-compiler` crate.
    CompilerError,
    exit_code_mask: 6000i32,
    error_code_prefix: "CMP",

    /// For when the test function has invalid test context.
    @backtraced
    invalid_test_context {
        args: (name: impl Display),
        msg: format!("Cannot find input files with context name `{}`", name),
        help: None,
    }

    /// For when the compiler can't read a file from the provided path.
    @backtraced
    file_read_error {
        args: (path: impl Debug, error: impl ErrorArg),
        msg: format!("Cannot read from the provided file path '{:?}': {}", path, error),
        help: None,
    }

     /// For when there is no main function in a Leo program.
    @backtraced
    no_main_function {
        args: (),
        msg: "There must be a function named `main`",
        help: None,
    }

     /// For when the compiler can't find the test input files with the specified name.
    @backtraced
    no_test_input {
        args: (),
        msg: "Failed to find input files for the current test",
        help: None,
    }

    /// For when the console formatter expected a left or right brace after a left brace.
    @formatted
    console_fmt_expected_left_or_right_brace {
        args: (),
        msg: "Formatter given a {. Expected a { or } after",
        help: None,
    }

    /// For when the console formatter expected a right brace after a right brace.
    @formatted
    console_fmt_expected_escaped_right_brace {
        args: (),
        msg: "Formatter given a }. Expected a container {} or }}",
        help: None,
    }

    /// For when the amount of arguments, and number of containers mismatch
    /// in a format statement.
    @formatted
    console_container_parameter_length_mismatch {
        args: (containers: impl Display, parameters: impl Display),
        msg: format!(
            "Formatter given {} containers and found {} parameters",
            containers, parameters
        ),
        help: None,
    }

    /// For when a user tries to user console assert outside of a test function.
    @formatted
    console_assertion_depends_on_input {
        args: (),
        msg: "console.assert() does not produce constraints and cannot use inputs. \
        Assertions should only be used in @test functions",
        help: None,
    }

    /// For when a console assert fails.
    @formatted
    console_assertion_failed {
        args: (),
        msg:  "console.assert(...) failed",
        help: None,
    }

    /// For when a console assert value is not a boolean.
    @formatted
    console_assertion_must_be_boolean {
        args: (),
        msg: "Assertion expression must evaluate to a boolean value",
        help: None,
    }

    /// For when a experssion gadget oepration cannot be enforced due to a SnarkVM syntehsis error.
    @formatted
    cannot_enforce_expression {
        args: (operation: impl Display, error: impl ErrorArg),
        msg: format!(
            "the gadget operation `{}` failed due to synthesis error `{:?}`",
            operation, error,
        ),
        help: None,
    }

    /// For when an expression has mismatching types for an operation.
    @formatted
    cannot_evaluate_expression {
        args: (operation: impl Display),
        msg: format!("Mismatched types found for operation `{}`", operation),
        help: None,
    }

    /// For when an array length goes out of the legal bounds of 2^32.
    @formatted
    array_length_out_of_bounds {
        args: (),
        msg:  "array length cannot be >= 2^32",
        help: None,
    }

    /// For when an array index goes out of the legal bounds of 2^32.
    @formatted
    array_index_out_of_legal_bounds {
        args: (),
        msg: "array index cannot be >= 2^32",
        help: None,
    }

    /// For when a boolean expression does not resolve to a boolean type.
    @formatted
    conditional_boolean_expression_fails_to_resolve_to_bool {
        args: (actual: impl Display),
        msg: format!("if, else conditional must resolve to a boolean, found `{}`", actual),
        help: None,
    }

    /// For when the expected circuit member could not be found.
    @formatted
    expected_circuit_member {
        args: (expected: impl Display),
        msg: format!("expected circuit member `{}`, not found", expected),
        help: None,
    }

    /// For when the operation has no implementation on the type of variable received.
    @formatted
    incompatible_types {
        args: (operation: impl Display),
        msg: format!("no implementation for `{}`", operation),
        help: None,
    }

    /// For when a tuple index goes out of the tuples size bounds.
    @formatted
    tuple_index_out_of_bounds {
        args: (index: impl Display),
        msg: format!("cannot access index {} of tuple out of bounds", index),
        help: None,
    }

    /// For when an array index goes out of the arrays size bounds.
    @formatted
    array_index_out_of_bounds {
        args: (index: impl Display),
        msg: format!("cannot access index {} of array out of bounds", index),
        help: None,
    }

    /// For when a invalid array slice length is requested.
    @formatted
    array_invalid_slice_length {
        args: (),
        msg: "illegal length of slice",
        help: None,
    }

    /// For when an array index does not resolve to an integer type.
    @formatted
    invalid_index_expression {
        args: (actual: impl Display),
        msg: format!("index must resolve to an integer, found `{}`", actual),
        help: None,
    }

    /// For when a typed array receives an assignment of an array with a different length.
    @formatted
    unexpected_array_length {
        args: (expected: impl Display, actual: impl Display),
        msg: format!("expected array length {}, found one with length {}", expected, actual),
        help: None,
    }

    /// For when the circuit static member does not exist.
    @formatted
    invalid_circuit_static_member_access {
        args: (member: impl Display),
        msg: format!("invalid circuit static member `{}` must be accessed using `::` syntax", member),
        help: None,
    }

    /// For when a user is trying to use an array in access expression before it is declared.
    @formatted
    undefined_array {
        args: (actual: impl Display),
        msg: format!("array `{}` must be declared before it is used in an expression", actual),
        help: None,
    }

    /// For when the user is trying to us a circuit that is not yet declared.
    @formatted
    undefined_circuit {
        args: (actual: impl Display),
        msg:  format!(
            "circuit `{}` must be declared before it is used in an expression",
            actual
        ),
        help: None,
    }

    /// For when the user tries to use an identifier not declared in scope.
    @formatted
    undefined_identifier {
        args: (name: impl Display),
        msg: format!("Cannot find value `{}` in this scope", name),
        help: None,
    }

    /// For when the user tries to access an undefined circuit member.
    @formatted
    undefined_circuit_member_access {
        args: (circuit: impl Display, member: impl Display),
        msg: format!("Circuit `{}` has no member `{}`", circuit, member),
        help: None,
    }

    /// For when the input variable type mismatches the declared function input type.
    @formatted
    input_variable_type_mismatch {
        args: (expected: impl Display, actual: impl Display, variable: impl Display),
        msg: format!(
            "Expected input variable `{}` to be type `{}`, found type `{}`",
            variable, expected, actual
        ),
        help: None,
    }

    /// For when the declared function input variable was expected to be constant
    /// but the input file did not have it in the constants section.
    @formatted
    expected_const_input_variable {
        args: (variable: impl Display),
        msg:  format!(
            "Expected input variable `{}` to be constant. Move input variable `{}` to [constants] section of input file",
            variable, variable
        ),
        help: None,
    }

    /// For when the declared function input variable was expected to be mutable
    /// but the input file did not have it in the main section.
    @formatted
    expected_non_const_input_variable {
        args: (variable: impl Display),
        msg: format!(
            "Expected input variable `{}` to be non-constant. Move input variable `{}` to [main] section of input file",
            variable, variable
        ),
        help: None,
    }

    /// For when the declared function input variable was expected to be a valid array
    /// in the input file.
    @formatted
    invalid_function_input_array {
        args: (actual: impl Display),
        msg: format!("Expected function input array, found `{}`", actual),
        help: None,
    }

    /// For when the declared function input variable was expected to be an array with differing dimensions.
    @formatted
    invalid_input_array_dimensions {
        args: (expected: impl Display, actual: impl Display),
        msg: format!(
            "Input array dimensions mismatch expected {}, found array dimensions {}",
            expected, actual
        ),
        help: None,
    }

    /// For when the declared function input variable was expected to be a tuple
    /// with a different number of arguments.
    @formatted
    input_tuple_size_mismatch {
        args: (expected: impl Display, actual: impl Display),
        msg: format!(
            "Input tuple size mismatch expected {}, found tuple with length {}",
            expected, actual
        ),
        help: None,
    }

    /// For when the declared function input variable was expected to be a valid tuple
    /// in the input file.
    @formatted
    invalid_function_input_tuple {
        args: (actual: impl Display),
        msg: format!("Expected function input tuple, found `{}`", actual),
        help: None,
    }

    /// For when the declared function input variable was expected to be a valid tuple
    /// in the input file.
    @formatted
    function_input_not_found {
        args: (function: impl Display, expected: impl Display),
        msg: format!("function `{}` input {} not found", function, expected),
        help: None,
    }

    /// For when the declared function input variable was defined twice
    /// in the input file.
    @formatted
    double_input_declaration {
        args: (input_name: impl Display),
        msg: format!("Input variable {} declared twice", input_name),
        help: None,
    }

    /// For when the input file does not define enough registers.
    @formatted
    output_not_enough_registers {
        args: (),
        msg: "number of input registers must be greater than or equal to output registers",
        help: None,
    }

    /// For when the input file register types do not match the output types being generated.
    @formatted
    output_mismatched_types {
        args: (left: impl Display, right: impl Display),
        msg: format!(
            "Mismatched types. Expected register output type `{}`, found type `{}`.",
            left, right
        ),
        help: None,
    }

    /// For when there's an IO error with the output file.
    @backtraced
    output_file_io_error {
        args: (error: impl ErrorArg),
        msg: error,
        help: None,
    }

    /// For when the output file cannot be read.
    @backtraced
    output_file_cannot_read {
        args: (path: impl Debug),
        msg: format!("Cannot read the provided ouput file - {:?}", path),
        help: None,
    }

    /// For when the output file cannot be removed.
    @backtraced
    output_file_cannot_remove {
        args: (path: impl Debug),
        msg: format!("Cannot remove the provided ouput file - {:?}", path),
        help: None,
    }

    /// For when the user tries to index a single array more than once.
    @formatted
    statement_array_assign_index {
        args: (),
        msg: "Cannot assign single index to array of values",
        help: None,
    }

    /// For when the user tries to use a non const value as an index.
    @formatted
    statement_array_assign_index_const {
        args: (),
        msg: "Cannot assign to non-const array index",
        help: None,
    }

    /// For when the user tries to assign an index to something not an array of length >= 1;
    @formatted
    statement_array_assign_interior_index {
        args: (),
        msg: "Cannot assign single index to interior of array of values",
        help: None,
    }

    /// For when the user tries to assign a range of values to something that expected a single value.
    @formatted
    statement_array_assign_range {
        args: (),
        msg: "Cannot assign range of array values to single value",
        help: None,
    }

    /// For when the user tries to index a value from an array that is >= the array length.
    @formatted
    statement_array_assign_index_bounds {
        args: (index: impl Display, length: impl Display),
        msg: format!(
            "Array assign index `{}` out of range for array of length `{}`",
            index, length
        ),
        help: None,
    }

    /// For when the user defines an array range values that is >= the array length.
    @formatted
    statement_array_assign_range_order {
        args: (start: impl Display, stop: impl Display, length: impl Display),
        msg: format!(
            "Array assign range `{}`..`{}` out of range for array of length `{}`",
            start, stop, length
        ),
        help: None,
    }

    /// For when the statement conditional boolean fails to resolve to a boolean.
    @formatted
    statement_conditional_boolean_fails_to_resolve_to_boolean {
        args: (actual: impl Display),
        msg: format!("If, else conditional must resolve to a boolean, found `{}`", actual),
        help: None,
    }

    /// For when there was an error in SnarkVM trying to do a bit and operation.
    @formatted
    statement_indicator_calculation {
        args: (name: impl Display),
        msg: format!(
            "Constraint system failed to evaluate branch selection indicator `{}`",
            name
        ),
        help: None,
    }

    /// For when a multi definition statement found a differing number of values and variable names.
    @formatted
    statement_invalid_number_of_definitions {
        args: (expected: impl Display, actual: impl Display),
        msg: format!(
            "Multiple definition statement expected {} values, found {} variable names",
            expected, actual
        ),
        help: None,
    }

    /// For when the user tries to assign multiple variables to a single value.
    @formatted
    statement_multiple_definition {
        args: (value: impl Display),
        msg: format!("cannot assign multiple variables to a single value: {}", value),
        help: None,
    }

    /// For when a function returns multiple times.
    @formatted
    statement_multiple_returns {
        args: (),
        msg: "This function returns multiple times and produces unreachable circuits with undefined behavior.",
        help: None,
    }

    /// For when a function expects a return type and has no valid return statements.
    @formatted
    statement_no_returns {
        args: (expected: impl Display),
        msg: format!(
            "function expected `{}` return type but no valid branches returned a result",
            expected
        ),
        help: None,
    }

    /// For when SnarkVM fails to conditionally select between values for a gadget.
    @formatted
    statement_select_fail {
        args: (first: impl Display, second: impl Display),
        msg: format!(
            "Conditional select gadget failed to select between `{}` or `{}`",
            first, second
        ),
        help: None,
    }

    /// For when the user tries to index a single tuple more than once.
    @formatted
    statement_tuple_assign_index {
        args: (),
        msg: "Cannot assign single index to tuple of values",
        help: None,
    }

    /// For when the user tries to index a value from an tuple that is >= the tuple length.
    @formatted
    statement_tuple_assign_index_bounds {
        args: (index: impl Display, length: impl Display),
        msg: format!(
            "Tuple assign index `{}` out of range for tuple of length `{}`",
            index, length
        ),
        help: None,
    }

    /// For when the user doesn't assign or return the expression.
    @formatted
    statement_unassigned {
        args: (),
        msg: "Expected assignment of return values for expression",
        help: None,
    }

    /// For when a statement tries to use an unknown variable.
    @formatted
    statement_undefined_variable {
        args: (name: impl Display),
        msg: format!("Attempted to assign to unknown variable `{}`", name),
        help: None,
    }

    /// For when the user defines a statement that tries to access an unknown circuit member variable.
    @formatted
    statement_undefined_circuit_variable {
        args: (name: impl Display),
        msg: format!("Attempted to assign to unknown circuit member variable `{}`", name),
        help: None,
    }

    /// For when the user uses a nont const value for an iteration range.
    @formatted
    statement_loop_index_const {
        args: (),
        msg: "iteration range must be const",
        help: None,
    }

    /// For when there is an issue with an address value account.
    @formatted
    address_value_account_error {
        args: (error: impl ErrorArg),
        msg: format!("account creation failed due to `{}`", error),
        help: None,
    }

    /// For when there is an invalid address value.
    @formatted
    address_value_invalid_address {
        args: (actual: impl Display),
        msg: format!("expected address input type, found `{}`", actual),
        help: None,
    }

    /// For when an a address value was expected but none was found.
    @formatted
    address_value_missing_address {
        args: (),
        msg: "expected address input not found",
        help: None,
    }

    /// For when an a boolean operation cannot be enforced due to a SnarkVM synthesis error.
    @formatted
    boolean_value_cannot_enforce {
        args: (operation: impl Display, error: impl ErrorArg),
        msg: format!(
            "the boolean operation `{}` failed due to the synthesis error `{}`",
            operation, error,
        ),
        help: None,
    }

    /// For when an a invalid boolean operation is called.
    @formatted
    boolean_value_cannot_evaluate {
        args: (operation: impl Display),
        msg: format!("no implementation found for `{}`", operation),
        help: None,
    }

    /// For when there is an invalid boolean value.
    @formatted
    boolean_value_invalid_boolean {
        args: (actual: impl Display),
        msg: format!("expected boolean input type, found `{}`", actual),
        help: None,
    }

    /// For when an a boolean value was expected but none was found.
    @formatted
    boolean_value_missing_boolean {
        args: (expected: impl Display),
        msg: format!("expected boolean input `{}` not found", expected),
        help: None,
    }

    /// For when there is an invalid char value.
    @formatted
    char_value_invalid_char {
        args: (actual: impl Display),
        msg: format!("expected char element input type, found `{}`", actual),
        help: None,
    }

    /// For when negating a field value fails due to a SnarkVM synthesis error.
    @formatted
    field_value_negate_operation {
        args: (error: impl ErrorArg),
        msg: format!("field negation failed due to synthesis error `{}`", error),
        help: None,
    }

    /// For when an a field operation cannot be enforced due to a SnarkVM synthesis error.
    @formatted
    field_value_binary_operation {
        args: (operation: impl Display, error: impl ErrorArg),
        msg: format!(
            "the field binary operation `{}` failed due to synthesis error `{}`",
            operation, error,
        ),
        help: None,
    }

    /// For when there is an invalid field value.
    @formatted
    field_value_invalid_field {
        args: (actual: impl Display),
        msg: format!("expected field element input type, found `{}`", actual),
        help: None,
    }

    /// For when an a field value was expected but none was found.
    @formatted
    field_value_missing_field {
        args: (expected: impl Display),
        msg: format!("expected field input `{}` not found", expected),
        help: None,
    }

    /// For when negating a group value fails due to a SnarkVM synthesis error.
    @formatted
    group_value_negate_operation {
        args: (error: impl ErrorArg),
        msg: format!("group negation failed due to the synthesis error `{}`", error),
        help: None,
    }

    /// For when an a group operation cannot be enforced due to a SnarkVM synthesis error.
    @formatted
    group_value_binary_operation {
        args: (operation: impl Display, error: impl ErrorArg),
        msg: format!(
            "the group binary operation `{}` failed due to the synthesis error `{}`",
            operation, error,
        ),
        help: None,
    }

    /// For when there is an invalid group value.
    @formatted
    group_value_invalid_group {
        args: (actual: impl Display),
        msg: format!("expected group affine point input type, found `{}`", actual),
        help: None,
    }

    /// For when an a group value was expected but none was found.
    @formatted
    group_value_missing_group {
        args: (expected: impl Display),
        msg: format!("expected group input `{}` not found", expected),
        help: None,
    }

    /// For when the synthesis of a group failed due to a SnarkVM synthesis error.
    @formatted
    group_value_synthesis_error {
        args: (error: impl ErrorArg),
        msg: format!("compilation failed due to group synthesis error `{}`", error),
        help: None,
    }

    /// For when the x coordinate of a group is invalid.
    @formatted
    group_value_x_invalid {
        args: (x: impl Display),
        msg: format!("invalid x coordinate `{}`", x),
        help: None,
    }

    /// For when the y coordinate of a group is invalid.
    @formatted
    group_value_y_invalid {
        args: (y: impl Display),
        msg: format!("invalid y coordinate `{}`", y),
        help: None,
    }

    /// For when the current group value is not on the current supported curve.
    @formatted
    group_value_not_on_curve {
        args: (element: impl Display),
        msg: format!("group element `{}` is not on the supported curve", element),
        help: None,
    }

    /// For when the x coordinate of a group could not be recovered.
    @formatted
    group_value_x_recover {
        args: (),
        msg: "could not recover group element from x coordinate",
        help: None,
    }

    /// For when the y coordinate of a group could not be recovered.
    @formatted
    group_value_y_recover {
        args: (),
        msg: "could not recover group element from y coordinate",
        help: None,
    }

    /// For when a group generator cannot be multiplied by some number.
    @formatted
    group_value_n_group {
        args: (number: impl Display),
        msg: format!("cannot multiply group generator by \"{}\"", number),
        help: None,
    }

    /// For when an a signed integer operation cannot be enforced due to a SnarkVM synthesis error.
    @formatted
    integer_value_signed {
        args: (error: impl ErrorArg),
        msg: format!("integer operation failed due to the signed integer error `{}`", error),
        help: None,
    }

    /// For when an a unsigned integer operation cannot be enforced due to a SnarkVM synthesis error.
    @formatted
    integer_value_unsigned {
        args: (error: impl ErrorArg),
        msg: format!(
            "integer operation failed due to the unsigned integer error `{}`",
            error
        ),
        help: None,
    }

    /// For when an a integer operation cannot be enforced due to a SnarkVM synthesis error.
    @formatted
    integer_value_synthesis {
        args: (error: impl ErrorArg),
        msg: format!("integer operation failed due to the synthesis error `{}`", error),
        help: None,
    }

    /// For when negating a integer value fails due to a SnarkVM synthesis error.
    @formatted
    integer_value_negate_operation {
        args: (),
        msg: "integer negation can only be enforced on signed integers",
        help: None,
    }

    /// For when an a binary integer operation cannot be enforced due to a SnarkVM synthesis error.
    @formatted
    integer_value_binary_operation {
        args: (operation: impl Display),
        msg: format!(
            "the integer binary operation `{}` can only be enforced on integers of the same type",
            operation
        ),
        help: None,
    }

    /// For when there is an integer type mismatch, one kind was expected but another was received.
    @formatted
    integer_value_integer_type_mismatch {
        args: (expected: impl Display, received: impl Display),
        msg: format!("expected data type `{}`, found `{}`", expected, received),
        help: None,
    }

    /// For when there is an invalid integer value.
    @formatted
    integer_value_invalid_integer {
        args: (actual: impl Display),
        msg: format!("failed to parse `{}` as expected integer type", actual),
        help: None,
    }

    /// For when an a integer value was expected but none was found.
    @formatted
    integer_value_missing_integer {
        args: (expected: impl Display),
        msg: format!("expected integer input `{}` not found", expected),
        help: None,
    }

    /// For when an a integer operation has no implementation.
    @formatted
    integer_value_cannot_evaluate {
        args: (operation: impl Display),
        msg: format!("no implementation found for `{}`", operation),
        help: None,
    }

    /// For when .len() method is used on non-array values/variables.
    @formatted
    lengthof_can_only_be_used_on_arrays {
        args: (),
        msg: "len() can only be called on an array value".to_string(),
        help: None,
    }

    /// For when equality operator is used on arrays with different sizes.
    @formatted
    array_sizes_must_match_in_eq {
        args: (lhs: impl Display, rhs: impl Display),
        msg: format!("array sizes must match for comparison; left: {}, right: {}", lhs, rhs),
        help: None,
    }
);
