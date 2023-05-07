use crate::model::identifiers::authorization_identifier::AuthorizationIdentifier;

/// # Syntax
/// ```php
/// <schema authorization identifier> ::=
///     <authorization identifier>
/// ```
///
/// `<authorization identifier>`: [AuthorizationIdentifier]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct SchemaAuthorizationIdentifier {
    authorization_identifier: AuthorizationIdentifier,
}

impl SchemaAuthorizationIdentifier {
    #[must_use]
    pub fn new(authorization_identifier: AuthorizationIdentifier) -> Self {
        Self {
            authorization_identifier,
        }
    }

    #[must_use]
    pub fn authorization_identifier(&self) -> &AuthorizationIdentifier {
        &self.authorization_identifier
    }
}
