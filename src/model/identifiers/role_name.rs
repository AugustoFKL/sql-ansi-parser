use crate::model::identifiers::identifier::Identifier;

/// # Syntax
/// ```php
/// <role name> ::=
///     <identifier>
/// ```
///
/// `<identifier>`: [Identifier]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct RoleName {
    identifier: Identifier,
}

impl RoleName {
    #[must_use]
    pub fn new(identifier: Identifier) -> Self {
        Self { identifier }
    }
}
