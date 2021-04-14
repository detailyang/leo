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

use crate::{
    assert_satisfied,
    expect_asg_error,
    expect_compiler_error,
    generate_main_input,
    integers::{expect_computation_error, IntegerTester},
    parse_program,
};
use leo_ast::InputValue;
use leo_input::types::{I8Type, IntegerType, SignedIntegerType};

test_int!(
    TestI8,
    i8,
    IntegerType::Signed(SignedIntegerType::I8Type(I8Type {})),
    Int8
);

#[test]
fn test_i8_min() {
    TestI8::test_min();
}

#[test]
fn test_i8_min_fail() {
    TestI8::test_min_fail();
}

#[test]
fn test_i8_max() {
    TestI8::test_max();
}

#[test]
fn test_i8_max_fail() {
    TestI8::test_max_fail();
}

#[test]
fn test_i8_neg() {
    TestI8::test_negate();
}

#[test]
fn test_i8_neg_max_fail() {
    TestI8::test_negate_min_fail();
}

#[test]
fn test_i8_neg_zero() {
    TestI8::test_negate_zero();
}

#[test]
fn test_i8_add() {
    TestI8::test_add();
}

#[test]
fn test_i8_sub() {
    TestI8::test_sub();
}

#[test]
fn test_i8_mul() {
    TestI8::test_mul();
}

#[test]
fn test_i8_div() {
    TestI8::test_div();
}

#[test]
fn test_i8_pow() {
    TestI8::test_pow();
}

#[test]
fn test_i8_eq() {
    TestI8::test_eq();
}

#[test]
fn test_i8_ne() {
    TestI8::test_ne();
}

#[test]
fn test_i8_ge() {
    TestI8::test_ge();
}

#[test]
fn test_i8_gt() {
    TestI8::test_gt();
}

#[test]
fn test_i8_le() {
    TestI8::test_le();
}

#[test]
fn test_i8_lt() {
    TestI8::test_lt();
}

#[test]
fn test_i8_console_assert() {
    TestI8::test_console_assert();
}

#[test]
fn test_i8_ternary() {
    TestI8::test_ternary();
}
