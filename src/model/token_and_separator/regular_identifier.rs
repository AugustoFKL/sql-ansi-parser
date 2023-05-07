use crate::model::token_and_separator::identifier_body::IdentifierBody;

/// # Syntax
/// ```php
/// <regular identifier> ::=
///     <identifier body>
/// ```
///
/// `<identifier body>`: [IdentifierBody]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct RegularIdentifier {
    identifier_body: IdentifierBody,
}

impl RegularIdentifier {
    #[must_use]
    pub fn new(identifier_body: IdentifierBody) -> Self {
        Self { identifier_body }
    }

    #[must_use]
    pub fn identifier_body(&self) -> &IdentifierBody {
        &self.identifier_body
    }
}
