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
    AsgConvertError,
    CircuitMember,
    ConstValue,
    Expression,
    ExpressionNode,
    FromAst,
    Function,
    FunctionQualifier,
    Node,
    PartialType,
    Scope,
    Span,
    Type,
};
pub use leo_ast::{BinaryOperation, Node as AstNode};

use std::cell::Cell;

#[derive(Clone)]
pub struct CallExpression<'a> {
    pub parent: Cell<Option<&'a Expression<'a>>>,
    pub span: Option<Span>,
    pub function: Cell<&'a Function<'a>>,
    pub target: Cell<Option<&'a Expression<'a>>>,
    pub arguments: Vec<Cell<&'a Expression<'a>>>,
}

impl<'a> Node for CallExpression<'a> {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl<'a> ExpressionNode<'a> for CallExpression<'a> {
    fn set_parent(&self, parent: &'a Expression<'a>) {
        self.parent.replace(Some(parent));
    }

    fn get_parent(&self) -> Option<&'a Expression<'a>> {
        self.parent.get()
    }

    fn enforce_parents(&self, expr: &'a Expression<'a>) {
        if let Some(target) = self.target.get() {
            target.set_parent(expr);
        }
        self.arguments.iter().for_each(|element| {
            element.get().set_parent(expr);
        })
    }

    fn get_type(&self) -> Option<Type<'a>> {
        Some(self.function.get().output.clone())
    }

    fn is_mut_ref(&self) -> bool {
        true
    }

    fn const_value(&self) -> Option<ConstValue> {
        // static function const evaluation
        None
    }

    fn is_consty(&self) -> bool {
        self.target.get().map(|x| x.is_consty()).unwrap_or(true) && self.arguments.iter().all(|x| x.get().is_consty())
    }
}

impl<'a> FromAst<'a, leo_ast::CallExpression> for CallExpression<'a> {
    fn from_ast(
        scope: &'a Scope<'a>,
        value: &leo_ast::CallExpression,
        expected_type: Option<PartialType<'a>>,
    ) -> Result<CallExpression<'a>, AsgConvertError> {
        let (target, function) = match &*value.function {
            leo_ast::Expression::Identifier(name) => (
                None,
                scope
                    .resolve_function(&name.name)
                    .ok_or_else(|| AsgConvertError::unresolved_function(&name.name, &name.span))?,
            ),
            leo_ast::Expression::CircuitMemberAccess(leo_ast::CircuitMemberAccessExpression {
                circuit: ast_circuit,
                name,
                span,
            }) => {
                let target = <&Expression<'a>>::from_ast(scope, &**ast_circuit, None)?;
                let circuit = match target.get_type() {
                    Some(Type::Circuit(circuit)) => circuit,
                    type_ => {
                        return Err(AsgConvertError::unexpected_type(
                            "circuit",
                            type_.map(|x| x.to_string()).as_deref(),
                            span,
                        ));
                    }
                };
                let circuit_name = circuit.name.borrow().name.clone();
                let member = circuit.members.borrow();
                let member = member
                    .get(name.name.as_ref())
                    .ok_or_else(|| AsgConvertError::unresolved_circuit_member(&circuit_name, &name.name, span))?;
                match member {
                    CircuitMember::Function(body) => {
                        if body.qualifier == FunctionQualifier::Static {
                            return Err(AsgConvertError::circuit_static_call_invalid(
                                &circuit_name,
                                &name.name,
                                span,
                            ));
                        } else if body.qualifier == FunctionQualifier::MutSelfRef && !target.is_mut_ref() {
                            return Err(AsgConvertError::circuit_member_mut_call_invalid(
                                &circuit_name,
                                &name.name,
                                span,
                            ));
                        }
                        (Some(target), *body)
                    }
                    CircuitMember::Variable(_) => {
                        return Err(AsgConvertError::circuit_variable_call(&circuit_name, &name.name, span));
                    }
                }
            }
            leo_ast::Expression::CircuitStaticFunctionAccess(leo_ast::CircuitStaticFunctionAccessExpression {
                circuit: ast_circuit,
                name,
                span,
            }) => {
                let circuit = if let leo_ast::Expression::Identifier(circuit_name) = &**ast_circuit {
                    scope
                        .resolve_circuit(&circuit_name.name)
                        .ok_or_else(|| AsgConvertError::unresolved_circuit(&circuit_name.name, &circuit_name.span))?
                } else {
                    return Err(AsgConvertError::unexpected_type("circuit", None, span));
                };
                let circuit_name = circuit.name.borrow().name.clone();

                let member = circuit.members.borrow();
                let member = member
                    .get(name.name.as_ref())
                    .ok_or_else(|| AsgConvertError::unresolved_circuit_member(&circuit_name, &name.name, span))?;
                match member {
                    CircuitMember::Function(body) => {
                        if body.qualifier != FunctionQualifier::Static {
                            return Err(AsgConvertError::circuit_member_call_invalid(
                                &circuit_name,
                                &name.name,
                                span,
                            ));
                        }
                        (None, *body)
                    }
                    CircuitMember::Variable(_) => {
                        return Err(AsgConvertError::circuit_variable_call(&circuit_name, &name.name, span));
                    }
                }
            }
            _ => {
                return Err(AsgConvertError::illegal_ast_structure(
                    "non Identifier/CircuitMemberAccess/CircuitStaticFunctionAccess as call target",
                ));
            }
        };
        if let Some(expected) = expected_type {
            let output: Type = function.output.clone();
            if !expected.matches(&output) {
                return Err(AsgConvertError::unexpected_type(
                    &expected.to_string(),
                    Some(&*output.to_string()),
                    &value.span,
                ));
            }
        }
        if value.arguments.len() != function.arguments.len() {
            return Err(AsgConvertError::unexpected_call_argument_count(
                function.arguments.len(),
                value.arguments.len(),
                &value.span,
            ));
        }

        let arguments = value
            .arguments
            .iter()
            .zip(function.arguments.iter())
            .map(|(expr, (_, argument))| {
                let argument = argument.get().borrow();
                let converted = <&Expression<'a>>::from_ast(scope, expr, Some(argument.type_.clone().partial()))?;
                if argument.const_ && !converted.is_consty() {
                    return Err(AsgConvertError::unexpected_nonconst(expr.span()));
                }
                Ok(Cell::new(converted))
            })
            .collect::<Result<Vec<_>, AsgConvertError>>()?;

        if function.is_test() {
            return Err(AsgConvertError::call_test_function(&value.span));
        }
        Ok(CallExpression {
            parent: Cell::new(None),
            span: Some(value.span.clone()),
            arguments,
            function: Cell::new(function),
            target: Cell::new(target),
        })
    }
}

impl<'a> Into<leo_ast::CallExpression> for &CallExpression<'a> {
    fn into(self) -> leo_ast::CallExpression {
        let target_function = if let Some(target) = self.target.get() {
            target.into()
        } else {
            let circuit = self.function.get().circuit.get();
            if let Some(circuit) = circuit {
                leo_ast::Expression::CircuitStaticFunctionAccess(leo_ast::CircuitStaticFunctionAccessExpression {
                    circuit: Box::new(leo_ast::Expression::Identifier(circuit.name.borrow().clone())),
                    name: self.function.get().name.borrow().clone(),
                    span: self.span.clone().unwrap_or_default(),
                })
            } else {
                leo_ast::Expression::Identifier(self.function.get().name.borrow().clone())
            }
        };
        leo_ast::CallExpression {
            function: Box::new(target_function),
            arguments: self.arguments.iter().map(|arg| arg.get().into()).collect(),
            span: self.span.clone().unwrap_or_default(),
        }
    }
}
