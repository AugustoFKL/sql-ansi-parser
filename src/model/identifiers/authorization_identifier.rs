use crate::model::identifiers::role_name::RoleName;
use crate::model::identifiers::user_identifier::UserIdentifier;

/// # Syntax
/// ```php
/// <authorization identifier> ::=
///     <role name>
///     | <user identifier>
/// ```
///
/// `<role name>`: [RoleName]
///
/// `<user identifier>`: [UserIdentifier]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum AuthorizationIdentifier {
    RoleName(RoleName),
    UserIdentifier(UserIdentifier),
}
