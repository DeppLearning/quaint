use super::Function;
use crate::ast::Expression;

/// Represents a function defined and stored in the database.
#[derive(Debug, Clone, PartialEq)]
pub struct StoredFunction<'a> {
    pub(crate) name: &'static str,
    pub(crate) arguments: Vec<Box<Expression<'a>>>,
}

/// Builds calls to arbitrary functions defined in the database.
///
/// ```rust
/// # use quaint::{ast::*, visitor::{Visitor, Postgres}, col};
/// # fn main() -> Result<(), quaint::error::Error> {
/// let query = Select::from_table("users").value(stored_function("upper", col!("name")));
/// let (sql, _) = Postgres::build(query)?;
/// assert_eq!("SELECT upper(\"name\") FROM \"users\"", sql);
/// # Ok(())
/// # }
/// ```
pub fn stored_function<'a>(name: &'static str, arguments: impl StoredFunctionArguments<'a>) -> Function<'a> {
    let fun = StoredFunction {
        name,
        arguments: arguments.into_args(),
    };
    fun.into()
}

pub trait StoredFunctionArguments<'a> {
    fn into_args(self) -> Vec<Box<Expression<'a>>>;
}

impl<'a> StoredFunctionArguments<'a> for () {
    fn into_args(self) -> Vec<Box<Expression<'a>>> {
        Vec::with_capacity(0)
    }
}

impl<'a, T: Into<Box<Expression<'a>>>> StoredFunctionArguments<'a> for T {
    fn into_args(self) -> Vec<Box<Expression<'a>>> {
        vec![self.into()]
    }
}

impl<'a, T1: Into<Box<Expression<'a>>>, T2: Into<Box<Expression<'a>>>> StoredFunctionArguments<'a> for (T1, T2) {
    fn into_args(self) -> Vec<Box<Expression<'a>>> {
        vec![self.0.into(), self.1.into()]
    }
}

impl<'a, T1: Into<Box<Expression<'a>>>, T2: Into<Box<Expression<'a>>>, T3: Into<Box<Expression<'a>>>>
    StoredFunctionArguments<'a> for (T1, T2, T3)
{
    fn into_args(self) -> Vec<Box<Expression<'a>>> {
        vec![self.0.into(), self.1.into(), self.2.into()]
    }
}

impl<
        'a,
        T1: Into<Box<Expression<'a>>>,
        T2: Into<Box<Expression<'a>>>,
        T3: Into<Box<Expression<'a>>>,
        T4: Into<Box<Expression<'a>>>,
    > StoredFunctionArguments<'a> for (T1, T2, T3, T4)
{
    fn into_args(self) -> Vec<Box<Expression<'a>>> {
        vec![self.0.into(), self.1.into(), self.2.into(), self.3.into()]
    }
}

impl<
        'a,
        T1: Into<Box<Expression<'a>>>,
        T2: Into<Box<Expression<'a>>>,
        T3: Into<Box<Expression<'a>>>,
        T4: Into<Box<Expression<'a>>>,
        T5: Into<Box<Expression<'a>>>,
    > StoredFunctionArguments<'a> for (T1, T2, T3, T4, T5)
{
    fn into_args(self) -> Vec<Box<Expression<'a>>> {
        vec![self.0.into(), self.1.into(), self.2.into(), self.3.into()]
    }
}

impl<
        'a,
        T1: Into<Box<Expression<'a>>>,
        T2: Into<Box<Expression<'a>>>,
        T3: Into<Box<Expression<'a>>>,
        T4: Into<Box<Expression<'a>>>,
        T5: Into<Box<Expression<'a>>>,
        T6: Into<Box<Expression<'a>>>,
    > StoredFunctionArguments<'a> for (T1, T2, T3, T4, T5, T6)
{
    fn into_args(self) -> Vec<Box<Expression<'a>>> {
        vec![self.0.into(), self.1.into(), self.2.into(), self.3.into()]
    }
}
