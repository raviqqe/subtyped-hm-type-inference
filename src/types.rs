use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::sync::Arc;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Type {
    Function(Arc<Type>, Arc<Type>),
    Number,
    Union(Arc<Type>, Arc<Type>),
    Variable(usize),
}

impl Type {
    pub fn variable() -> Type {
        Type::Variable(rand::random())
    }

    pub fn substitute(&self, substitutions: &HashMap<usize, Type>) -> Type {
        match self {
            Self::Function(argument, result) => Self::Function(
                argument.substitute(substitutions).into(),
                result.substitute(substitutions).into(),
            ),
            Self::Number => Self::Number,
            Self::Union(one, other) => Self::Union(
                one.substitute(substitutions).into(),
                other.substitute(substitutions).into(),
            ),
            Self::Variable(id) => substitutions.get(id).unwrap_or_else(|| self).clone(),
        }
    }

    pub fn variables(&self) -> HashSet<usize> {
        match self {
            Self::Function(argument, result) => argument
                .variables()
                .into_iter()
                .chain(result.variables())
                .collect(),
            Self::Number => Default::default(),
            Self::Union(one, other) => one
                .variables()
                .into_iter()
                .chain(other.variables())
                .collect(),
            Self::Variable(id) => vec![*id].into_iter().collect(),
        }
    }
}

impl Display for Type {
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::Function(argument, result) => write!(formatter, "{} -> {}", argument, result),
            Self::Number => write!(formatter, "Number"),
            Self::Union(one, other) => write!(formatter, "{} | {}", one, other),
            Self::Variable(id) => write!(formatter, "<{}>", &format!("{:04x}", id)[..4]),
        }
    }
}
