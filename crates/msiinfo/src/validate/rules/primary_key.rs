use std::io::{Read, Seek};

use msi::{Package, Table};

use crate::validate::error::{ValidationError, ValidationResult};

/// Validates that the primary key is a valid column in the table.
pub fn validate_pks<F: Read + Seek>(package: &mut Package<F>) -> ValidationResult {
    let tables = package.tables();
    let mut errors = Vec::new();

    for table in tables {
        errors.extend(validate_table_pk(table));
    }

    Ok(errors)
}

fn validate_table_pk(table: &Table) -> Vec<ValidationError> {
    let columns = table.columns();
    let pk_columns: Vec<(usize, &msi::Column)> = columns
        .iter()
        .enumerate()
        .filter(|(i, c)| c.is_primary_key())
        .collect();
    if pk_columns.is_empty() {
        return vec![ValidationError::MissingPrimaryKey {
            table: table.name().to_string(),
        }];
    }

    let mut next_expected = 0;

    for (i, c) in pk_columns {
        if i != next_expected {
            return vec![ValidationError::PrimaryKeyLeading {
                table: table.name().to_string(),
                column: c.name().to_string(),
                index: i,
            }];
        }
        next_expected = i + 1;
    }

    vec![]
}
