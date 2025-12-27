use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Eq, PartialEq, Hash, Clone, Debug, Serialize, Deserialize)]
pub struct Variable(String);
impl Variable {
    pub fn new(name: &str) -> Variable {
        /* todo: check if valid variable name */
        Variable(name.to_string())
    }
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum Expression {
    Variable(Variable),
    Value(ExpressionValue),
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum ExpressionValue {
    RealNumber(f64),
    IntegerNumber(i64),
    NaturalNumber(u64),
    PlusOperator(Vec<Expression>),
    MinusOperator(Vec<Expression>),
    DivideOperator(Vec<Expression>),
    MultiplyOperator(Vec<Expression>),
    Sin(Box<Expression>),
    Cos(Box<Expression>),
    Tan(Box<Expression>),
    CoTan(Box<Expression>),
    Arcsin(Box<Expression>),
    Arccos(Box<Expression>),
    Arctan(Box<Expression>)
}

#[derive(Clone, Debug)]
pub struct ParamSet {
    pub parameters: HashMap<Variable, ExpressionValue>,
}

impl ParamSet {
    pub fn insert_parameter(&mut self, variable: Variable, parameter: ExpressionValue) {
        self.parameters.insert(variable, parameter);
    }
}
