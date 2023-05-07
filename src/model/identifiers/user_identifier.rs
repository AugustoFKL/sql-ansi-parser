use crate::model::identifiers::identifier::Identifier;

/// # Syntax
/// ```php
/// <user identifier> ::=
///     <identifier>
/// ```
///
/// `<identifier>`: [Identifier]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct UserIdentifier {
    identifier: Identifier,
}

impl UserIdentifier {
    #[must_use]
    pub fn new(identifier: Identifier) -> Self {
        Self { identifier }
    }

    #[must_use]
    pub fn identifier(&self) -> &Identifier {
        &self.identifier
    }
}
