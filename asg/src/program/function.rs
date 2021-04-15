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
    BlockStatement,
    Circuit,
    FromAst,
    Identifier,
    MonoidalDirector,
    ReturnPathReducer,
    Scope,
    Span,
    Statement,
    Type,
    Variable,
};
use indexmap::IndexMap;
pub use leo_ast::Annotation;
use leo_ast::FunctionInput;

use std::cell::{Cell, RefCell};

#[derive(Clone, Copy, PartialEq)]
pub enum FunctionQualifier {
    SelfRef,
    ConstSelfRef,
    MutSelfRef,
    Static,
}

#[derive(Clone)]
pub struct Function<'a> {
    pub id: u32,
    pub name: RefCell<Identifier>,
    pub output: Type<'a>,
    pub arguments: IndexMap<String, Cell<&'a Variable<'a>>>,
    pub circuit: Cell<Option<&'a Circuit<'a>>>,
    pub span: Option<Span>,
    pub body: Cell<Option<&'a Statement<'a>>>,
    pub scope: &'a Scope<'a>,
    pub qualifier: FunctionQualifier,
    pub annotations: Vec<Annotation>,
}

impl<'a> PartialEq for Function<'a> {
    fn eq(&self, other: &Function<'a>) -> bool {
        if self.name.borrow().name != other.name.borrow().name {
            return false;
        }
        self.id == other.id
    }
}

impl<'a> Eq for Function<'a> {}

impl<'a> Function<'a> {
    pub(crate) fn init(scope: &'a Scope<'a>, value: &leo_ast::Function) -> Result<&'a Function<'a>, AsgConvertError> {
        let output: Type<'a> = value
            .output
            .as_ref()
            .map(|t| scope.resolve_ast_type(t))
            .transpose()?
            .unwrap_or_else(|| Type::Tuple(vec![]));
        let mut qualifier = FunctionQualifier::Static;
        let new_scope = scope.make_subscope();

        let mut arguments = IndexMap::new();
        {
            for input in value.input.iter() {
                match input {
                    FunctionInput::SelfKeyword(_) => {
                        qualifier = FunctionQualifier::SelfRef;
                    }
                    FunctionInput::ConstSelfKeyword(_) => {
                        qualifier = FunctionQualifier::ConstSelfRef;
                    }
                    FunctionInput::MutSelfKeyword(_) => {
                        qualifier = FunctionQualifier::MutSelfRef;
                    }
                    FunctionInput::Variable(leo_ast::FunctionInputVariable {
                        type_,
                        identifier,
                        const_,
                        mutable,
                        ..
                    }) => {
                        let variable = scope.context.alloc_variable(RefCell::new(crate::InnerVariable {
                            id: scope.context.get_id(),
                            name: identifier.clone(),
                            type_: scope.resolve_ast_type(&type_)?,
                            mutable: *mutable,
                            const_: *const_,
                            declaration: crate::VariableDeclaration::Parameter,
                            references: vec![],
                            assignments: vec![],
                        }));
                        arguments.insert(identifier.name.to_string(), Cell::new(&*variable));
                    }
                }
            }
        }
        if qualifier != FunctionQualifier::Static && scope.circuit_self.get().is_none() {
            return Err(AsgConvertError::invalid_self_in_global(&value.span));
        }
        let function = scope.context.alloc_function(Function {
            id: scope.context.get_id(),
            name: RefCell::new(value.identifier.clone()),
            output,
            arguments,
            circuit: Cell::new(None),
            body: Cell::new(None),
            qualifier,
            scope: new_scope,
            span: Some(value.span.clone()),
            annotations: value.annotations.clone(),
        });
        function.scope.function.replace(Some(function));

        Ok(function)
    }

    pub(super) fn fill_from_ast(self: &'a Function<'a>, value: &leo_ast::Function) -> Result<(), AsgConvertError> {
        if self.qualifier != FunctionQualifier::Static {
            let circuit = self.circuit.get();
            let self_variable = self.scope.context.alloc_variable(RefCell::new(crate::InnerVariable {
                id: self.scope.context.get_id(),
                name: Identifier::new("self".into()),
                type_: Type::Circuit(circuit.as_ref().unwrap()),
                mutable: self.qualifier == FunctionQualifier::MutSelfRef,
                const_: false,
                declaration: crate::VariableDeclaration::Parameter,
                references: vec![],
                assignments: vec![],
            }));
            self.scope
                .variables
                .borrow_mut()
                .insert("self".to_string(), self_variable);
        }
        for (name, argument) in self.arguments.iter() {
            self.scope.variables.borrow_mut().insert(name.clone(), argument.get());
        }

        let main_block = BlockStatement::from_ast(self.scope, &value.block, None)?;
        let mut director = MonoidalDirector::new(ReturnPathReducer::new());
        if !director.reduce_block(&main_block).0 && !self.output.is_unit() {
            return Err(AsgConvertError::function_missing_return(
                &self.name.borrow().name,
                &value.span,
            ));
        }

        #[allow(clippy::never_loop)] // TODO @Protryon: How should we return multiple errors?
        for (span, error) in director.reducer().errors {
            return Err(AsgConvertError::function_return_validation(
                &self.name.borrow().name,
                &error,
                &span,
            ));
        }

        self.body
            .replace(Some(self.scope.context.alloc_statement(Statement::Block(main_block))));

        Ok(())
    }

    pub fn is_test(&self) -> bool {
        self.annotations.iter().any(|x| x.name.name.as_ref() == "test")
    }
}

impl<'a> Into<leo_ast::Function> for &Function<'a> {
    fn into(self) -> leo_ast::Function {
        let input = self
            .arguments
            .iter()
            .map(|(_, variable)| {
                let variable = variable.get().borrow();
                leo_ast::FunctionInput::Variable(leo_ast::FunctionInputVariable {
                    identifier: variable.name.clone(),
                    mutable: variable.mutable,
                    const_: variable.const_,
                    type_: (&variable.type_).into(),
                    span: Span::default(),
                })
            })
            .collect();
        let (body, span) = match self.body.get() {
            Some(Statement::Block(block)) => (block.into(), block.span.clone().unwrap_or_default()),
            Some(_) => unimplemented!(),
            None => (
                leo_ast::Block {
                    statements: vec![],
                    span: Default::default(),
                },
                Default::default(),
            ),
        };
        let output: Type = self.output.clone();
        leo_ast::Function {
            identifier: self.name.borrow().clone(),
            input,
            block: body,
            output: Some((&output).into()),
            span,
            annotations: self.annotations.clone(),
        }
    }
}
