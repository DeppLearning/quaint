use super::Function;
use crate::ast::Expression;

/// A represention of the `MAX` function in the database.
#[derive(Debug, Clone, PartialEq)]
pub struct ToTsvector<'a> {
    pub(crate) expression: Box<Expression<'a>>,
}

/// Converts text into a text search vector.
///
/// ```rust
/// # use quaint::{ast::*, visitor::{Visitor, Postgres}, col};
/// # fn main() -> Result<(), quaint::error::Error> {
/// let query = Select::from_table("users").value(to_tsvector(col!("name")));
/// let (sql, _) = Postgres::build(query)?;
/// assert_eq!("SELECT to_tsquery(\"name\") FROM \"users\"", sql);
/// # Ok(())
/// # }
/// ```
pub fn to_tsvector<'a, E>(expr: E) -> Function<'a>
where
    E: Into<Expression<'a>>,
{
    let fun = ToTsvector {
        expression: Box::new(expr.into()),
    };
    fun.into()
}
