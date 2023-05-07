use crate::model::token_and_separator::regular_identifier::RegularIdentifier;
use std::fmt::{Display, Formatter};

/// # Syntax
/// ```php
/// <actual identifier> ::=
///     <regular identifier>
///     | <delimited identifier>
///     | <Unicode delimited identifier>
/// ```
///
/// `<regular identifier>`: [RegularIdentifier]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Identifier {
    RegularIdentifier(RegularIdentifier),
    DelimitedIdentifier,
    UnicodeDelimitedIdentifier,
}

impl Display for Identifier {
    fn fmt(&self, _: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
