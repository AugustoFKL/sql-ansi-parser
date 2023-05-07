use crate::model::schema_definition::schema_name_clause::SchemaNameClause;

/// # Syntax
/// ```php
/// <schema definition> ::=
///     CREATE SCHEMA <schema name clause>
///         [ <schema character set or path>  ]
///         [ <schema element> ... ]
/// ```
///
/// `<schema name clause>`: [SchemaNameClause]
pub struct CreateSchema {
    schema_name_clause: SchemaNameClause,
}

impl CreateSchema {
    #[must_use]
    pub fn new(schema_name_clause: SchemaNameClause) -> Self {
        Self { schema_name_clause }
    }

    #[must_use]
    pub fn schema_name_clause(&self) -> &SchemaNameClause {
        &self.schema_name_clause
    }
}
