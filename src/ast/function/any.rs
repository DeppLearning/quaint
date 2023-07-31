use super::Function;
use crate::ast::Expression;

#[derive(Debug, Clone, PartialEq)]
pub struct Any<'a> {
    pub(crate) value: Box<Expression<'a>>,
}

pub fn any<'a, T>(expr: T) -> Function<'a>
where
    T: Into<Expression<'a>>,
{
    let fun = Any {
        value: Box::new(expr.into()),
    };

    fun.into()
}
