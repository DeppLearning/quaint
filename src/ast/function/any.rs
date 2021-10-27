use super::Function;
use crate::ast::Expression;

#[derive(Debug, Clone, PartialEq)]
/// A date function that coerces expression results into dates.
pub struct Any<'a> {
    pub(crate) value: Box<Expression<'a>>,
}

/// Coerces the given expression result into Date.
///
/// ```rust
/// # use quaint::{ast::*, visitor::{Visitor, Sqlite}};
/// # fn main() -> Result<(), quaint::error::Error> {
/// let query = Select::from_table("users").value(date(Column::new("createdAt")));
///
/// let (sql, _) = Sqlite::build(query)?;
/// assert_eq!("SELECT DATE(`createdAt`) FROM `users`", sql);
/// # Ok(())
/// # }
/// ```
pub fn any<'a, T>(expr: T) -> Function<'a>
where
    T: Into<Expression<'a>>,
{
    let fun = Any {
        value: Box::new(expr.into()),
    };

    fun.into()
}
