use crate::model::identifiers::catalog_name::CatalogName;
use crate::model::identifiers::unqualified_schema_name::UnqualifiedSchemaName;

/// # Syntax
/// ```php
/// <schema name> ::=
///     [ <catalog name>  <period>  ] <unqualified schema name>
/// ```
///
/// `<catalog name>`: [CatalogName]
///
/// `<unqualified schema name>`: [UnqualifiedSchemaName]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct SchemaName {
    catalog_name: Option<CatalogName>,
    unqualified_schema_name: UnqualifiedSchemaName,
}

impl SchemaName {
    #[must_use]
    pub fn new(unqualified_schema_name: UnqualifiedSchemaName) -> Self {
        Self {
            catalog_name: None,
            unqualified_schema_name,
        }
    }

    #[must_use]
    pub fn with_catalog_name(mut self, catalog_name: CatalogName) -> Self {
        self.catalog_name = Some(catalog_name);
        self
    }
}
