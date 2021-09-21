use super::Function;
use crate::ast::Expression;

#[derive(Debug, Clone, PartialEq)]
/// A Text function that coerces expression results into Texts.
pub struct Text<'a> {
    pub(crate) value: Box<Expression<'a>>,
}

/// Coerces the given expression result into Text.
///
/// ```rust
/// # use quaint::{ast::*, visitor::{Visitor, Sqlite}};
/// # fn main() -> Result<(), quaint::error::Error> {
/// let query = Select::from_table("users").value(text(Column::new("createdAt")));
///
/// let (sql, _) = Sqlite::build(query)?;
/// assert_eq!("SELECT text(`createdAt`) FROM `users`", sql);
/// # Ok(())
/// # }
/// ```
pub fn text<'a, T>(expr: T) -> Function<'a>
where
    T: Into<Expression<'a>>,
{
    let fun = Text {
        value: Box::new(expr.into()),
    };

    fun.into()
}
