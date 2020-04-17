//! A zokrates_program consists of nodes that keep track of position and wrap zokrates_program types.
//!
//! @file types.rs
//! @author Collin Chin <collin@aleo.org>
//! @date 2020

use crate::aleo_program::Import;
use std::collections::HashMap;

/// A variable in a constraint system.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Variable(pub String);

/// Spread operator
#[derive(Debug, Clone)]
pub struct FieldSpread(pub FieldExpression);

/// Spread or field expression enum
#[derive(Debug, Clone)]
pub enum FieldSpreadOrExpression {
    Spread(FieldSpread),
    FieldExpression(FieldExpression),
}

/// Range or field expression enum
#[derive(Debug, Clone)]
pub enum FieldRangeOrExpression {
    Range(Option<FieldExpression>, Option<FieldExpression>),
    FieldExpression(FieldExpression),
}

/// Expression that evaluates to a field value
#[derive(Debug, Clone)]
pub enum FieldExpression {
    Variable(Variable),
    Number(u32),
    // Operators
    Add(Box<FieldExpression>, Box<FieldExpression>),
    Sub(Box<FieldExpression>, Box<FieldExpression>),
    Mul(Box<FieldExpression>, Box<FieldExpression>),
    Div(Box<FieldExpression>, Box<FieldExpression>),
    Pow(Box<FieldExpression>, Box<FieldExpression>),
    // Conditionals
    IfElse(
        Box<BooleanExpression>,
        Box<FieldExpression>,
        Box<FieldExpression>,
    ),
    // Arrays
    Array(Vec<Box<FieldSpreadOrExpression>>),
}

/// Spread operator
#[derive(Debug, Clone)]
pub struct BooleanSpread(pub BooleanExpression);

/// Spread or field expression enum
#[derive(Debug, Clone)]
pub enum BooleanSpreadOrExpression {
    Spread(BooleanSpread),
    BooleanExpression(BooleanExpression),
}

/// Expression that evaluates to a boolean value
#[derive(Debug, Clone)]
pub enum BooleanExpression {
    Variable(Variable),
    Value(bool),
    // Boolean operators
    Not(Box<BooleanExpression>),
    Or(Box<BooleanExpression>, Box<BooleanExpression>),
    And(Box<BooleanExpression>, Box<BooleanExpression>),
    BoolEq(Box<BooleanExpression>, Box<BooleanExpression>),
    // Field operators
    FieldEq(Box<FieldExpression>, Box<FieldExpression>),
    Geq(Box<FieldExpression>, Box<FieldExpression>),
    Gt(Box<FieldExpression>, Box<FieldExpression>),
    Leq(Box<FieldExpression>, Box<FieldExpression>),
    Lt(Box<FieldExpression>, Box<FieldExpression>),
    // Conditionals
    IfElse(
        Box<BooleanExpression>,
        Box<BooleanExpression>,
        Box<BooleanExpression>,
    ),
    // Arrays
    Array(Vec<Box<BooleanSpreadOrExpression>>),
}

/// Expression that evaluates to a value
#[derive(Debug, Clone)]
pub enum Expression {
    Boolean(BooleanExpression),
    FieldElement(FieldExpression),
    Variable(Variable),
    Struct(Variable, Vec<StructMember>),
    ArrayAccess(Box<Expression>, FieldRangeOrExpression),
    StructMemberAccess(Box<Expression>, Variable), // (struct name, struct member name)
    FunctionCall(Box<Expression>, Vec<Expression>),
}

#[derive(Debug, Clone)]
pub enum Assignee {
    Variable(Variable),
    Array(Box<Assignee>, FieldRangeOrExpression),
    StructMember(Box<Assignee>, Variable),
}

/// Program statement that defines some action (or expression) to be carried out.
#[derive(Clone)]
pub enum Statement {
    // Declaration(Variable),
    Definition(Assignee, Expression),
    For(Variable, FieldExpression, FieldExpression, Vec<Statement>),
    Return(Vec<Expression>),
}

#[derive(Clone, Debug)]
pub enum Type {
    Boolean,
    FieldElement,
    Array(Box<Type>, usize),
    Struct(Variable),
}

#[derive(Clone, Debug)]
pub struct StructMember {
    pub variable: Variable,
    pub expression: Expression,
}

#[derive(Clone)]
pub struct StructField {
    pub variable: Variable,
    pub ty: Type,
}

#[derive(Clone)]
pub struct Struct {
    pub variable: Variable,
    pub fields: Vec<StructField>,
}

#[derive(Clone, Debug)]
pub enum Visibility {
    Public,
    Private,
}

#[derive(Clone)]
pub struct Parameter {
    pub visibility: Option<Visibility>,
    pub ty: Type,
    pub variable: Variable,
}

/// The given name for a defined function in the program.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct FunctionName(pub String);

#[derive(Clone)]
pub struct Function {
    pub function_name: FunctionName,
    pub parameters: Vec<Parameter>,
    pub returns: Vec<Type>,
    pub statements: Vec<Statement>,
}

impl Function {
    pub fn get_name(&self) -> String {
        self.function_name.0.clone()
    }
}

/// A simple program with statement expressions, program arguments and program returns.
#[derive(Debug, Clone)]
pub struct Program<'ast> {
    pub name: Variable,
    pub imports: Vec<Import<'ast>>,
    pub structs: HashMap<Variable, Struct>,
    pub functions: HashMap<FunctionName, Function>,
}

impl<'ast> Program<'ast> {
    pub fn name(mut self, name: String) -> Self {
        self.name = Variable(name);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable() {
        let variable = Variable("1".into());

        println!("{:#?}", variable);
    }
}
