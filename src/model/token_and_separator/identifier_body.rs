/// # Syntax
/// ```php
/// <identifier body> ::=
///     <identifier start>  [ <identifier part> ... ]
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct IdentifierBody {
    identifier: String,
}

impl IdentifierBody {
    #[must_use]
    pub fn new<T>(identifier: &str) -> Self {
        Self {
            identifier: identifier.to_owned(),
        }
    }

    #[must_use]
    pub fn identifier(&self) -> &str {
        &self.identifier
    }
}
