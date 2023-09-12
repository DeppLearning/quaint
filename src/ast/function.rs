mod aggregate_to_string;
mod average;
mod coalesce;
mod concat;
mod count;
#[cfg(all(feature = "json", any(feature = "postgresql", feature = "mysql")))]
mod json_extract;
mod lower;
mod maximum;
mod minimum;
mod row_number;
#[cfg(all(feature = "json", feature = "postgresql"))]
mod row_to_json;
mod sum;
mod upper;
mod date;
mod text;
mod any;

pub use aggregate_to_string::*;
pub use average::*;
pub use coalesce::*;
pub use concat::*;
pub use count::*;
#[cfg(all(feature = "json", any(feature = "postgresql", feature = "mysql")))]
pub use json_extract::*;
pub use lower::*;
pub use maximum::*;
pub use minimum::*;
pub use row_number::*;
#[cfg(all(feature = "json", feature = "postgresql"))]
pub use row_to_json::*;
pub use sum::*;
pub use upper::*;
pub use date::*;
pub use text::*;
pub use any::*;

use super::{Aliasable, Expression};
use std::borrow::Cow;

/// A database function definition
#[derive(Debug, Clone, PartialEq)]
pub struct Function<'a> {
    pub(crate) typ_: FunctionType<'a>,
    pub(crate) alias: Option<Cow<'a, str>>,
}

/// A database function type
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum FunctionType<'a> {
    #[cfg(all(feature = "json", feature = "postgresql"))]
    RowToJson(RowToJson<'a>),
    RowNumber(RowNumber<'a>),
    Count(Count<'a>),
    AggregateToString(AggregateToString<'a>),
    Average(Average<'a>),
    Sum(Sum<'a>),
    Lower(Lower<'a>),
    Upper(Upper<'a>),
    Minimum(Minimum<'a>),
    Maximum(Maximum<'a>),
    Coalesce(Coalesce<'a>),
    Concat(Concat<'a>),
    #[cfg(all(feature = "json", any(feature = "postgresql", feature = "mysql")))]
    JsonExtract(JsonExtract<'a>),
    Date(Date<'a>),
    Text(Text<'a>),
    Any(Any<'a>),
}

impl<'a> Aliasable<'a> for Function<'a> {
    type Target = Function<'a>;

    fn alias<T>(mut self, alias: T) -> Self::Target
    where
        T: Into<Cow<'a, str>>,
    {
        self.alias = Some(alias.into());
        self
    }
}

#[cfg(all(feature = "json", feature = "postgresql"))]
function!(RowToJson);

#[cfg(all(feature = "json", any(feature = "postgresql", feature = "mysql")))]
function!(JsonExtract);

function!(
    RowNumber,
    Count,
    AggregateToString,
    Average,
    Sum,
    Lower,
    Upper,
    Minimum,
    Maximum,
    Coalesce,
    Concat,
    Date,
    Text,
    Any
);
