use std::fmt::{Display, Formatter};

use crate::model::identifiers::identifier::Identifier;

/// # Syntax
/// ```php
/// <catalog name> ::=
///     <identifier>
/// ```
///
/// `<identifier>`: [Identifier]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct CatalogName {
    identifier: Identifier,
}

impl Display for CatalogName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.identifier())
    }
}

impl CatalogName {
    #[must_use]
    pub fn new(identifier: Identifier) -> Self {
        Self { identifier }
    }

    #[must_use]
    pub fn identifier(&self) -> &Identifier {
        &self.identifier
    }
}
