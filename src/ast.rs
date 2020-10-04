use std::fmt::{Display, Formatter};
use std::sync::Arc;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expression {
    Application(Arc<Expression>, Arc<Expression>),
    Lambda(String, Arc<Expression>),
    Let(String, Arc<Expression>, Arc<Expression>),
    Number,
    Variable(String),
}

impl Display for Expression {
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::Application(function, argument) => write!(formatter, "{} {}", function, argument),
            Self::Lambda(variable, expression) => {
                write!(formatter, "\\{}. {}", variable, expression)
            }
            Self::Let(variable, bound_expression, expression) => write!(
                formatter,
                "let {} = {} in {}",
                variable, bound_expression, expression
            ),
            Self::Number => write!(formatter, "42"),
            Self::Variable(name) => write!(formatter, "{}", name),
        }
    }
}
