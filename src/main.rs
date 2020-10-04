mod ast;
mod infer;
mod types;

use ast::*;
use infer::infer;
use types::*;

fn main() {
    for (expression, expected_type) in &[
        (Expression::Number, Type::Number),
        (
            Expression::Let(
                "f".into(),
                Expression::Lambda("x".into(), Expression::Number.into()).into(),
                Expression::Application(
                    Expression::Variable("f".into()).into(),
                    Expression::Number.into(),
                )
                .into(),
            ),
            Type::Number,
        ),
        (
            Expression::Let(
                "f".into(),
                Expression::Lambda("x".into(), Expression::Number.into()).into(),
                Expression::Let(
                    "y".into(),
                    Expression::Application(
                        Expression::Variable("f".into()).into(),
                        Expression::Number.into(),
                    )
                    .into(),
                    Expression::Variable("f".into()).into(),
                )
                .into(),
            ),
            Type::Function(Type::Number.into(), Type::Number.into()),
        ),
    ] {
        let (_, type_) = infer(&Default::default(), expression).unwrap();

        println!("{} : {}", expression, type_);
        assert_eq!(&type_, expected_type);
    }
}
