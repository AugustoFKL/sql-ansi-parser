use crate::model::identifiers::schema_name::SchemaName;

/// # Syntax
/// ```php
/// <schema name clause> ::=
///     <schema name>
///     | AUTHORIZATION <schema authorization identifier>
///     | <schema name>  AUTHORIZATION <schema authorization identifier>
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum SchemaNameClause {
    /// # Syntax
    /// ```php
    /// <schema name>
    /// ```
    Schema(SchemaName),
    /// # Syntax
    /// ```php
    /// AUTHORIZATION <schema authorization identifier>
    /// ```
    Authorization,
    /// # Syntax
    /// ```php
    /// <schema name>  AUTHORIZATION <schema authorization identifier>
    /// ```
    SchemaAndAuthorization,
}
