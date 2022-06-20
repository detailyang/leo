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

use crate::{number_string_typing, GroupType};
use leo_asg::{GroupCoordinate, GroupValue};
use leo_errors::{CompilerError, Result, Span};

use snarkvm_curves::{
    edwards_bls12::{EdwardsAffine, EdwardsParameters, Fq},
    templates::twisted_edwards_extended::Affine,
    AffineCurve, TwistedEdwardsParameters,
};
use snarkvm_fields::{Fp256, One, Zero};
use snarkvm_gadgets::{
    bits::{ToBitsBEGadget, ToBytesGadget},
    boolean::Boolean,
    curves::edwards_bls12::EdwardsBls12Gadget,
    fields::{AllocatedFp, FpGadget},
    integers::uint::UInt8,
    traits::{
        alloc::AllocGadget,
        curves::GroupGadget,
        eq::{ConditionalEqGadget, EqGadget, EvaluateEqGadget},
        fields::FieldGadget,
    },
    CondSelectGadget,
};
use snarkvm_r1cs::{ConstraintSystem, SynthesisError};
use std::{
    borrow::Borrow,
    ops::{Add, Mul, Neg, Sub},
    str::FromStr,
};

#[derive(Clone, Debug)]
pub enum EdwardsGroupType {
    Constant(EdwardsAffine),
    Allocated(Box<EdwardsBls12Gadget>),
}

impl GroupType<Fq> for EdwardsGroupType {
    fn constant(group: &GroupValue, span: &Span) -> Result<Self> {
        let value = Self::edwards_affine_from_value(group, span)?;

        Ok(EdwardsGroupType::Constant(value))
    }

    fn to_allocated<CS: ConstraintSystem<Fq>>(&self, mut cs: CS, span: &Span) -> Result<Self> {
        Ok(self
            .allocated(cs.ns(|| format!("allocate affine point {}:{}", span.line_start, span.col_start)))
            .map(|ebg| EdwardsGroupType::Allocated(Box::new(ebg)))
            .map_err(|e| CompilerError::group_value_synthesis_error(e, span))?)
    }

    fn negate<CS: ConstraintSystem<Fq>>(&self, cs: CS, span: &Span) -> Result<Self> {
        match self {
            EdwardsGroupType::Constant(group) => Ok(EdwardsGroupType::Constant(group.neg())),
            EdwardsGroupType::Allocated(group) => {
                let result = <EdwardsBls12Gadget as GroupGadget<Affine<EdwardsParameters>, Fq>>::negate(group, cs)
                    .map_err(|e| CompilerError::group_value_negate_operation(e, span))?;

                Ok(EdwardsGroupType::Allocated(Box::new(result)))
            }
        }
    }

    fn add<CS: ConstraintSystem<Fq>>(&self, cs: CS, other: &Self, span: &Span) -> Result<Self> {
        match (self, other) {
            (EdwardsGroupType::Constant(self_value), EdwardsGroupType::Constant(other_value)) => {
                Ok(EdwardsGroupType::Constant(self_value.add(other_value)))
            }

            (EdwardsGroupType::Allocated(self_value), EdwardsGroupType::Allocated(other_value)) => {
                let result = <EdwardsBls12Gadget as GroupGadget<Affine<EdwardsParameters>, Fq>>::add(
                    self_value,
                    cs,
                    other_value,
                )
                .map_err(|e| CompilerError::group_value_binary_operation("+", e, span))?;

                Ok(EdwardsGroupType::Allocated(Box::new(result)))
            }

            (EdwardsGroupType::Constant(constant_value), EdwardsGroupType::Allocated(allocated_value))
            | (EdwardsGroupType::Allocated(allocated_value), EdwardsGroupType::Constant(constant_value)) => {
                Ok(EdwardsGroupType::Allocated(Box::new(
                    allocated_value
                        .add_constant(cs, constant_value)
                        .map_err(|e| CompilerError::group_value_binary_operation("+", e, span))?,
                )))
            }
        }
    }

    fn sub<CS: ConstraintSystem<Fq>>(&self, cs: CS, other: &Self, span: &Span) -> Result<Self> {
        match (self, other) {
            (EdwardsGroupType::Constant(self_value), EdwardsGroupType::Constant(other_value)) => {
                Ok(EdwardsGroupType::Constant(self_value.sub(other_value)))
            }

            (EdwardsGroupType::Allocated(self_value), EdwardsGroupType::Allocated(other_value)) => {
                let result = <EdwardsBls12Gadget as GroupGadget<Affine<EdwardsParameters>, Fq>>::sub(
                    self_value,
                    cs,
                    other_value,
                )
                .map_err(|e| CompilerError::group_value_binary_operation("-", e, span))?;

                Ok(EdwardsGroupType::Allocated(Box::new(result)))
            }

            (EdwardsGroupType::Constant(constant_value), EdwardsGroupType::Allocated(allocated_value))
            | (EdwardsGroupType::Allocated(allocated_value), EdwardsGroupType::Constant(constant_value)) => {
                Ok(EdwardsGroupType::Allocated(Box::new(
                    allocated_value
                        .sub_constant(cs, constant_value)
                        .map_err(|e| CompilerError::group_value_binary_operation("-", e, span))?,
                )))
            }
        }
    }
}

impl EdwardsGroupType {
    pub fn edwards_affine_from_value(value: &GroupValue, span: &Span) -> Result<EdwardsAffine> {
        match value {
            GroupValue::Single(number, ..) => Self::edwards_affine_from_single(number, span),
            GroupValue::Tuple(x, y) => Self::edwards_affine_from_tuple(x, y, span),
        }
    }

    pub fn edwards_affine_from_single(number: &str, span: &Span) -> Result<EdwardsAffine> {
        let number_info = number_string_typing(number);

        if number_info.0.eq("0") {
            Ok(EdwardsAffine::zero())
        } else {
            let one = edwards_affine_one();
            let number_value = match number_info {
                (number, neg) if neg => {
                    -Fp256::from_str(&number).map_err(|_| CompilerError::group_value_n_group(number, span))?
                }
                (number, _) => {
                    Fp256::from_str(&number).map_err(|_| CompilerError::group_value_n_group(number, span))?
                }
            };

            let result: EdwardsAffine = one.mul(number_value);

            Ok(result)
        }
    }

    pub fn edwards_affine_from_tuple(x: &GroupCoordinate, y: &GroupCoordinate, span: &Span) -> Result<EdwardsAffine> {
        let x = x.clone();
        let y = y.clone();

        match (x, y) {
            // (x, y)
            (GroupCoordinate::Number(x_string), GroupCoordinate::Number(y_string)) => Self::edwards_affine_from_pair(
                number_string_typing(&x_string),
                number_string_typing(&y_string),
                span,
                span,
                span,
            ),
            // (x, +)
            (GroupCoordinate::Number(x_string), GroupCoordinate::SignHigh) => {
                Self::edwards_affine_from_x_str(number_string_typing(&x_string), span, Some(true), span)
            }
            // (x, -)
            (GroupCoordinate::Number(x_string), GroupCoordinate::SignLow) => {
                Self::edwards_affine_from_x_str(number_string_typing(&x_string), span, Some(false), span)
            }
            // (x, _)
            (GroupCoordinate::Number(x_string), GroupCoordinate::Inferred) => {
                Self::edwards_affine_from_x_str(number_string_typing(&x_string), span, None, span)
            }
            // (+, y)
            (GroupCoordinate::SignHigh, GroupCoordinate::Number(y_string)) => {
                Self::edwards_affine_from_y_str(number_string_typing(&y_string), span, Some(true), span)
            }
            // (-, y)
            (GroupCoordinate::SignLow, GroupCoordinate::Number(y_string)) => {
                Self::edwards_affine_from_y_str(number_string_typing(&y_string), span, Some(false), span)
            }
            // (_, y)
            (GroupCoordinate::Inferred, GroupCoordinate::Number(y_string)) => {
                Self::edwards_affine_from_y_str(number_string_typing(&y_string), span, None, span)
            }
            // Invalid
            (x, y) => {
                return Err(CompilerError::group_value_invalid_group(format!("({}, {})", x, y), span).into());
            }
        }
    }

    pub fn edwards_affine_from_x_str(
        x_info: (String, bool),
        x_span: &Span,
        greatest: Option<bool>,
        element_span: &Span,
    ) -> Result<EdwardsAffine> {
        let x = match x_info {
            (x_str, neg) if neg => {
                -Fq::from_str(&x_str).map_err(|_| CompilerError::group_value_x_invalid(x_str, x_span))?
            }
            (x_str, _) => Fq::from_str(&x_str).map_err(|_| CompilerError::group_value_x_invalid(x_str, x_span))?,
        };

        match greatest {
            // Sign provided
            Some(greatest) => Ok(EdwardsAffine::from_x_coordinate(x, greatest)
                .ok_or_else(|| CompilerError::group_value_x_recover(element_span))?),
            // Sign inferred
            None => {
                // Attempt to recover with a sign_low bit.
                if let Some(element) = EdwardsAffine::from_x_coordinate(x, false) {
                    return Ok(element);
                }

                // Attempt to recover with a sign_high bit.
                if let Some(element) = EdwardsAffine::from_x_coordinate(x, true) {
                    return Ok(element);
                }

                // Otherwise return error.
                Err(CompilerError::group_value_x_recover(element_span).into())
            }
        }
    }

    pub fn edwards_affine_from_y_str(
        y_info: (String, bool),
        y_span: &Span,
        greatest: Option<bool>,
        element_span: &Span,
    ) -> Result<EdwardsAffine> {
        let y = match y_info {
            (y_str, neg) if neg => {
                -Fq::from_str(&y_str).map_err(|_| CompilerError::group_value_y_invalid(y_str, y_span))?
            }
            (y_str, _) => Fq::from_str(&y_str).map_err(|_| CompilerError::group_value_y_invalid(y_str, y_span))?,
        };

        match greatest {
            // Sign provided
            Some(greatest) => Ok(EdwardsAffine::from_y_coordinate(y, greatest)
                .ok_or_else(|| CompilerError::group_value_y_recover(element_span))?),
            // Sign inferred
            None => {
                // Attempt to recover with a sign_low bit.
                if let Some(element) = EdwardsAffine::from_y_coordinate(y, false) {
                    return Ok(element);
                }

                // Attempt to recover with a sign_high bit.
                if let Some(element) = EdwardsAffine::from_y_coordinate(y, true) {
                    return Ok(element);
                }

                // Otherwise return error.
                Err(CompilerError::group_value_y_recover(element_span).into())
            }
        }
    }

    pub fn edwards_affine_from_pair(
        x_info: (String, bool),
        y_info: (String, bool),
        x_span: &Span,
        y_span: &Span,
        element_span: &Span,
    ) -> Result<EdwardsAffine> {
        let x = match x_info {
            (x_str, neg) if neg => {
                -Fq::from_str(&x_str).map_err(|_| CompilerError::group_value_x_invalid(x_str, x_span))?
            }
            (x_str, _) => Fq::from_str(&x_str).map_err(|_| CompilerError::group_value_x_invalid(x_str, x_span))?,
        };

        let y = match y_info {
            (y_str, neg) if neg => {
                -Fq::from_str(&y_str).map_err(|_| CompilerError::group_value_y_invalid(y_str, y_span))?
            }
            (y_str, _) => Fq::from_str(&y_str).map_err(|_| CompilerError::group_value_y_invalid(y_str, y_span))?,
        };

        let element = EdwardsAffine::new(x, y);

        if element.is_on_curve() {
            Ok(element)
        } else {
            Err(CompilerError::group_value_not_on_curve(element, element_span).into())
        }
    }

    pub fn alloc_helper<Fn: FnOnce() -> Result<T, SynthesisError>, T: Borrow<GroupValue>>(
        value_gen: Fn,
    ) -> Result<EdwardsAffine, SynthesisError> {
        let group_value = match value_gen() {
            Ok(value) => {
                let group_value = value.borrow().clone();
                Ok(group_value)
            }
            _ => Err(SynthesisError::AssignmentMissing),
        }?;

        Self::edwards_affine_from_value(&group_value, &Span::default()).map_err(|_| SynthesisError::AssignmentMissing)
    }

    pub fn allocated<CS: ConstraintSystem<Fq>>(&self, mut cs: CS) -> Result<EdwardsBls12Gadget, SynthesisError> {
        match self {
            EdwardsGroupType::Constant(constant) => {
                <EdwardsBls12Gadget as AllocGadget<Affine<EdwardsParameters>, Fq>>::alloc(
                    &mut cs.ns(|| format!("{:?}", constant)),
                    || Ok(constant),
                )
            }
            EdwardsGroupType::Allocated(allocated) => {
                let x_value = allocated.x.get_value();
                let y_value = allocated.y.get_value();

                let x_allocated = FpGadget::alloc(cs.ns(|| "x"), || x_value.ok_or(SynthesisError::AssignmentMissing))?;
                let y_allocated = FpGadget::alloc(cs.ns(|| "y"), || y_value.ok_or(SynthesisError::AssignmentMissing))?;

                Ok(EdwardsBls12Gadget::new(x_allocated, y_allocated))
            }
        }
    }
}

impl AllocGadget<GroupValue, Fq> for EdwardsGroupType {
    fn alloc<Fn: FnOnce() -> Result<T, SynthesisError>, T: Borrow<GroupValue>, CS: ConstraintSystem<Fq>>(
        cs: CS,
        value_gen: Fn,
    ) -> Result<Self, SynthesisError> {
        let value = <EdwardsBls12Gadget as AllocGadget<Affine<EdwardsParameters>, Fq>>::alloc(cs, || {
            Self::alloc_helper(value_gen)
        })?;

        Ok(EdwardsGroupType::Allocated(Box::new(value)))
    }

    fn alloc_input<Fn: FnOnce() -> Result<T, SynthesisError>, T: Borrow<GroupValue>, CS: ConstraintSystem<Fq>>(
        cs: CS,
        value_gen: Fn,
    ) -> Result<Self, SynthesisError> {
        let value = <EdwardsBls12Gadget as AllocGadget<Affine<EdwardsParameters>, Fq>>::alloc_input(cs, || {
            Self::alloc_helper(value_gen)
        })?;

        Ok(EdwardsGroupType::Allocated(Box::new(value)))
    }
}

impl PartialEq for EdwardsGroupType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (EdwardsGroupType::Constant(self_value), EdwardsGroupType::Constant(other_value)) => {
                self_value == other_value
            }

            (EdwardsGroupType::Allocated(self_value), EdwardsGroupType::Allocated(other_value)) => {
                self_value.eq(other_value)
            }

            (EdwardsGroupType::Constant(constant_value), EdwardsGroupType::Allocated(allocated_value))
            | (EdwardsGroupType::Allocated(allocated_value), EdwardsGroupType::Constant(constant_value)) => {
                <EdwardsBls12Gadget as GroupGadget<Affine<EdwardsParameters>, Fq>>::get_value(allocated_value)
                    .map(|allocated_value| allocated_value == *constant_value)
                    .unwrap_or(false)
            }
        }
    }
}

impl Eq for EdwardsGroupType {}

// fn compare_allocated_edwards_bls_gadgets<CS: ConstraintSystem<Fq>>(
//     mut cs: CS,
//     first: &EdwardsBls12Gadget,
//     second: &EdwardsBls12Gadget,
// ) -> Result<Boolean, SynthesisError> {
//     // compare x coordinates
//     let x_first = &first.x;
//     let x_second = &second.x;
//
//     let compare_x = x_first.evaluate_equal(&mut cs.ns(|| format!("compare x")), x_second)?;
//
//     // compare y coordinates
//     let y_first = &first.y;
//     let y_second = &second.y;
//
//     let compare_y = y_first.evaluate_equal(&mut cs.ns(|| format!("compare y")), y_second)?;
//
//     Boolean::and(
//         &mut cs.ns(|| format!("compare x and y results")),
//         &compare_x,
//         &compare_y,
//     )
// }

impl EvaluateEqGadget<Fq> for EdwardsGroupType {
    fn evaluate_equal<CS: ConstraintSystem<Fq>>(&self, mut _cs: CS, other: &Self) -> Result<Boolean, SynthesisError> {
        match (self, other) {
            (EdwardsGroupType::Constant(self_value), EdwardsGroupType::Constant(other_value)) => {
                Ok(Boolean::constant(self_value.eq(other_value)))
            }
            _ => unimplemented!(),
            // (EdwardsGroupType::Allocated(first), EdwardsGroupType::Allocated(second)) => {
            //     compare_allocated_edwards_bls_gadgets(cs, first, second)
            // }
            // (EdwardsGroupType::Constant(constant_value), EdwardsGroupType::Allocated(allocated_value))
            // | (EdwardsGroupType::Allocated(allocated_value), EdwardsGroupType::Constant(constant_value)) => {
            //     let allocated_constant_value =
            //         <EdwardsBls12Gadget as AllocGadget<Affine<EdwardsParameters>, Fq>>::alloc(
            //             &mut cs.ns(|| format!("alloc constant for eq")),
            //             || Ok(constant_value),
            //         )?;
            //     compare_allocated_edwards_bls_gadgets(cs, allocated_value, &allocated_constant_value)
            // }
        }
    }
}

impl EqGadget<Fq> for EdwardsGroupType {}

impl ConditionalEqGadget<Fq> for EdwardsGroupType {
    #[inline]
    fn conditional_enforce_equal<CS: ConstraintSystem<Fq>>(
        &self,
        mut cs: CS,
        other: &Self,
        condition: &Boolean,
    ) -> Result<(), SynthesisError> {
        match (self, other) {
            // c - c
            (EdwardsGroupType::Constant(self_value), EdwardsGroupType::Constant(other_value)) => {
                if self_value == other_value {
                    return Ok(());
                }
                Err(SynthesisError::AssignmentMissing)
            }
            // a - a
            (EdwardsGroupType::Allocated(self_value), EdwardsGroupType::Allocated(other_value)) => {
                <EdwardsBls12Gadget>::conditional_enforce_equal(self_value, cs, other_value, condition)
            }
            // c - a = a - c
            (EdwardsGroupType::Constant(constant_value), EdwardsGroupType::Allocated(allocated_value))
            | (EdwardsGroupType::Allocated(allocated_value), EdwardsGroupType::Constant(constant_value)) => {
                let x = FpGadget::from(AllocatedFp::from(&mut cs, &constant_value.x));
                let y = FpGadget::from(AllocatedFp::from(&mut cs, &constant_value.y));
                let constant_gadget = EdwardsBls12Gadget::new(x, y);

                constant_gadget.conditional_enforce_equal(cs, allocated_value, condition)
            }
        }
    }

    fn cost() -> usize {
        2 * <EdwardsBls12Gadget as ConditionalEqGadget<Fq>>::cost() //upper bound
    }
}

impl CondSelectGadget<Fq> for EdwardsGroupType {
    fn conditionally_select<CS: ConstraintSystem<Fq>>(
        mut cs: CS,
        cond: &Boolean,
        first: &Self,
        second: &Self,
    ) -> Result<Self, SynthesisError> {
        if let Boolean::Constant(cond) = *cond {
            if cond {
                Ok(first.clone())
            } else {
                Ok(second.clone())
            }
        } else {
            let first_gadget = first.allocated(cs.ns(|| "first"))?;
            let second_gadget = second.allocated(cs.ns(|| "second"))?;
            let result = EdwardsBls12Gadget::conditionally_select(cs, cond, &first_gadget, &second_gadget)?;

            Ok(EdwardsGroupType::Allocated(Box::new(result)))
        }
    }

    fn cost() -> usize {
        2 * <EdwardsBls12Gadget as CondSelectGadget<Fq>>::cost()
    }
}

impl ToBitsBEGadget<Fq> for EdwardsGroupType {
    fn to_bits_be<CS: ConstraintSystem<Fq>>(&self, mut cs: CS) -> Result<Vec<Boolean>, SynthesisError> {
        let self_gadget = self.allocated(&mut cs)?;
        self_gadget.to_bits_be(cs)
    }

    fn to_bits_be_strict<CS: ConstraintSystem<Fq>>(&self, mut cs: CS) -> Result<Vec<Boolean>, SynthesisError> {
        let self_gadget = self.allocated(&mut cs)?;
        self_gadget.to_bits_be_strict(cs)
    }
}

impl ToBytesGadget<Fq> for EdwardsGroupType {
    fn to_bytes<CS: ConstraintSystem<Fq>>(&self, mut cs: CS) -> Result<Vec<UInt8>, SynthesisError> {
        let self_gadget = self.allocated(&mut cs)?;
        self_gadget.to_bytes(cs)
    }

    fn to_bytes_strict<CS: ConstraintSystem<Fq>>(&self, mut cs: CS) -> Result<Vec<UInt8>, SynthesisError> {
        let self_gadget = self.allocated(&mut cs)?;
        self_gadget.to_bytes_strict(cs)
    }
}

fn edwards_affine_one() -> Affine<EdwardsParameters> {
    let (x, y) = EdwardsParameters::AFFINE_GENERATOR_COEFFS;

    EdwardsAffine::new(x, y)
}

impl One for EdwardsGroupType {
    fn one() -> Self {
        let one = edwards_affine_one();

        Self::Constant(one)
    }

    fn is_one(&self) -> bool {
        self.eq(&Self::one())
    }
}

impl std::fmt::Display for EdwardsGroupType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            EdwardsGroupType::Constant(constant) => write!(f, "({}, {})group", constant.x, constant.y),
            EdwardsGroupType::Allocated(allocated) => match (allocated.x.get_value(), allocated.y.get_value()) {
                (Some(x), Some(y)) => write!(f, "({}, {})group", x, y),
                allocated => write!(f, "{:?}", allocated),
            },
        }
    }
}
