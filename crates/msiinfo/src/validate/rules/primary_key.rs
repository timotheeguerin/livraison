use crate::validate::rule::{PackageReader, Rule, RuleContext, macros::hl};

use msi::Table;

pub struct PrimaryKeysRule {}
impl Rule for PrimaryKeysRule {
    fn code(&self) -> &'static str {
        "invalid-primary-keys"
    }

    fn validate_pks(
        &self,
        ctx: &mut RuleContext,
        package: &mut dyn PackageReader,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let tables = package.tables();

        for table in tables {
            validate_table_pk(ctx, table)
        }

        Ok(())
    }
}

/// Validates that the primary key is a valid column in the table.
fn validate_table_pk(ctx: &mut RuleContext, table: &Table) {
    let columns = table.columns();
    let pk_columns: Vec<(usize, &msi::Column)> = columns
        .iter()
        .enumerate()
        .filter(|(_, c)| c.is_primary_key())
        .collect();
    if pk_columns.is_empty() {
        ctx.error(hl!("Table {} doesn't have any primary key.", table.name()));
        return;
    }

    let mut next_expected = 0;

    for (i, c) in pk_columns {
        if i != next_expected {
            ctx.error(hl!("Table {} column {} (#{}) is a primary key but one or more columns defined before is not. Primary keys must be the leading columns of a table.", table.name(), c.name(), i));
            return;
        }
        next_expected = i + 1;
    }
}
