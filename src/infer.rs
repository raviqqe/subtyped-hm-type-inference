use crate::ast::*;
use crate::types::Type;
use std::collections::HashMap;

#[derive(Debug)]
pub struct InferenceError;

pub fn infer_type(expression: &Expression) -> Result<Type, InferenceError> {
    let (substitutions, type_) = infer(&Default::default(), expression)?;

    dbg!(&substitutions);

    Ok(type_)
}

fn infer(
    environment: &HashMap<String, Type>,
    expression: &Expression,
) -> Result<((Vec<(usize, Type)>, Vec<(usize, Type)>), Type), InferenceError> {
    Ok(match expression {
        Expression::Application(function, argument) => {
            let (substitutions, function_type) = infer(&environment, &function)?;
            let (other_substitutions, argument_type) = infer(&environment, &argument)?;

            let result_type = Type::variable();

            (
                merge_substitutions(&[
                    substitutions,
                    other_substitutions,
                    unify(
                        &function_type,
                        &Type::Function(argument_type.into(), result_type.clone().into()),
                    )?,
                ]),
                result_type,
            )
        }
        Expression::Lambda(variable, expression) => {
            let argument_type = Type::variable();

            let (substitutions, result_type) = infer(
                &environment
                    .clone()
                    .into_iter()
                    .chain(vec![(variable.clone(), argument_type.clone())])
                    .collect(),
                &expression,
            )?;

            (
                substitutions,
                Type::Function(argument_type.into(), result_type.into()),
            )
        }
        Expression::Let(variable, bound_expression, expression) => {
            let (substitutions, bound_type) = infer(&environment, &bound_expression)?;

            let environment = environment
                .clone()
                .into_iter()
                .chain(vec![(variable.clone(), bound_type)])
                .collect();

            let (other_substitutions, type_) = infer(&environment, &expression)?;

            (
                merge_substitutions(&[substitutions, other_substitutions]),
                type_,
            )
        }
        Expression::Number(_) => ((Default::default(), Default::default()), Type::Number),
        Expression::Variable(id) => (
            (Default::default(), Default::default()),
            environment.get(id).ok_or(InferenceError)?.clone(),
        ),
    })
}

fn unify(
    lower: &Type,
    upper: &Type,
) -> Result<(Vec<(usize, Type)>, Vec<(usize, Type)>), InferenceError> {
    Ok(match (lower, upper) {
        (Type::Variable(id), upper) => (vec![], vec![(*id, upper.clone())]),
        (lower, Type::Variable(id)) => (vec![(*id, lower.clone())], vec![]),
        (Type::Union(one, other), upper) => {
            merge_substitutions(&[unify(one, upper)?, unify(other, upper)?])
        }
        (_, Type::Union(_, _)) => todo!(),
        (
            Type::Function(one_argument, one_result),
            Type::Function(other_argument, other_result),
        ) => merge_substitutions(&[
            unify(other_argument, one_argument)?,
            unify(one_result, other_result)?,
        ]),
        (Type::Number, Type::Number) => (vec![], vec![]),
        _ => return Err(InferenceError),
    })
}

fn merge_substitutions(
    substitutions: &[(Vec<(usize, Type)>, Vec<(usize, Type)>)],
) -> (Vec<(usize, Type)>, Vec<(usize, Type)>) {
    (
        substitutions
            .iter()
            .flat_map(|(lower, _)| lower.clone())
            .collect(),
        substitutions
            .iter()
            .flat_map(|(_, upper)| upper.clone())
            .collect(),
    )
}
