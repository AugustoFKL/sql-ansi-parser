use crate::model::identifiers::identifier::Identifier;

/// # Syntax
/// ```php
/// <unqualified schema name> ::=
///     <identifier>
/// ```
/// `<identifier>`: [Identifier]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct UnqualifiedSchemaName {
    identifier: Identifier,
}

impl UnqualifiedSchemaName {
    #[must_use]
    pub fn new(identifier: Identifier) -> Self {
        Self { identifier }
    }

    #[must_use]
    pub fn identifier(&self) -> &Identifier {
        &self.identifier
    }
}
