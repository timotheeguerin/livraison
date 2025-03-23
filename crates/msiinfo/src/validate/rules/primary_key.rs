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

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use msi::{Package, PackageType};

    use super::PrimaryKeysRule;
    use crate::validate::rule::test_rule;

    #[test]
    fn report_if_pk_out_of_order() {
        let mut package = mock_package();
        package
            .create_table(
                "Dummy",
                vec![
                    msi::Column::build("A").id_string(72),
                    msi::Column::build("B").primary_key().id_string(72),
                    msi::Column::build("C").int16(),
                    msi::Column::build("D").int16(),
                    msi::Column::build("E").int16(),
                ],
            )
            .unwrap();
        let diagnostics = test_rule(PrimaryKeysRule {}, &mut package);

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].code, "invalid-primary-keys");
    }

    fn mock_package() -> msi::Package<Cursor<Vec<u8>>> {
        let cursor = Cursor::new(Vec::new());
        Package::create(PackageType::Installer, cursor).expect("create")
    }
}
