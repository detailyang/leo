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

use crate::{ConditionalStatement, Node};
use leo_errors::Span;

use super::*;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Program statement that defines some action (or expression) to be carried out.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Statement {
    Return(ReturnStatement),
    Definition(DefinitionStatement),
    Assign(Box<AssignStatement>),
    Conditional(ConditionalStatement),
    Iteration(Box<IterationStatement>),
    Console(ConsoleStatement),
    Expression(ExpressionStatement),
    Block(Block),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Statement::Return(x) => x.fmt(f),
            Statement::Definition(x) => x.fmt(f),
            Statement::Assign(x) => x.fmt(f),
            Statement::Conditional(x) => x.fmt(f),
            Statement::Iteration(x) => x.fmt(f),
            Statement::Console(x) => x.fmt(f),
            Statement::Expression(x) => x.fmt(f),
            Statement::Block(x) => x.fmt(f),
        }
    }
}

impl Node for Statement {
    fn span(&self) -> &Span {
        use Statement::*;
        match &self {
            Return(n) => n.span(),
            Definition(n) => n.span(),
            Assign(n) => n.span(),
            Conditional(n) => n.span(),
            Iteration(n) => n.span(),
            Console(n) => n.span(),
            Expression(n) => n.span(),
            Block(n) => n.span(),
        }
    }

    fn set_span(&mut self, span: Span) {
        use Statement::*;
        match self {
            Return(n) => n.set_span(span),
            Definition(n) => n.set_span(span),
            Assign(n) => n.set_span(span),
            Conditional(n) => n.set_span(span),
            Iteration(n) => n.set_span(span),
            Console(n) => n.set_span(span),
            Expression(n) => n.set_span(span),
            Block(n) => n.set_span(span),
        }
    }
}
