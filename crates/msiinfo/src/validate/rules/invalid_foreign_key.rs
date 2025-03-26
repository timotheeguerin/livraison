use crate::validate::rule::{PackageReader, Rule, RuleContext, macros::hl};

pub struct InvalidForeignKeyRule {}
impl Rule for InvalidForeignKeyRule {
    fn code(&self) -> &'static str {
        "invalid-foreign-key"
    }

    fn validate_pks(
        &self,
        ctx: &mut RuleContext,
        package: &mut dyn PackageReader,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let foreign_key_cols = find_foreign_key_cols(package);
        for item in foreign_key_cols {
            match package.get_table(&item.ref_table) {
                Some(k) => k,
                None => {
                    ctx.error(hl!(
                        "Foreign key column {} in table {} is referencing a missing table: {}",
                        item.col_name,
                        item.table,
                        item.ref_table
                    ));
                    continue;
                }
            };

            let values = package
                .select_rows(msi::Select::table(&item.table))?
                .map(|row| row[item.col_index].clone())
                .collect::<Vec<_>>();

            for value in values {
                if value.is_null() {
                    continue;
                }
                let ref_row = package
                    .select_rows(msi::Select::table(&item.ref_table))?
                    .find(|row| row[item.ref_col_index - 1].to_string() == value.to_string());
                if ref_row.is_none() {
                    ctx.error(hl!(
                        "Foreign key column {} in table {} is referencing a missing entry in table {} with key {}",
                        item.col_name,
                        item.table,
                        item.ref_table,
                        value
                    ));
                }
            }
        }

        Ok(())
    }
}

struct ForeignKeyCol {
    table: String,
    col_name: String,
    col_index: usize,
    ref_table: String,
    ref_col_index: usize,
}

fn find_foreign_key_cols(package: &mut dyn PackageReader) -> Vec<ForeignKeyCol> {
    let mut result = Vec::new();
    for table in package.tables() {
        for (col_index, col) in table.columns().iter().enumerate() {
            if let Some((ref_table_name, ref_col_index)) = col.get_foreign_key() {
                result.push(ForeignKeyCol {
                    table: table.name().to_string(),
                    col_name: col.name().to_string(),
                    col_index,
                    ref_table: ref_table_name,
                    ref_col_index: ref_col_index as usize,
                });
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use msi::{Package, PackageType};
    use msi_installer::tables::{Component, Directory, Entity, File};

    use super::InvalidForeignKeyRule;
    use crate::validate::rule::test_rule;

    #[test]
    fn report_if_component_not_defined() {
        let mut package = mock_package();
        File::create_table(&mut package).unwrap();
        File::insert(
            &mut package,
            &[File {
                file: "File1".to_string(),
                component: "Component1".to_string(),
                sequence: 1,
                ..Default::default()
            }],
        )
        .unwrap();
        let diagnostics = test_rule(InvalidForeignKeyRule {}, &mut package);

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].code, "invalid-foreign-key");
    }
    #[test]
    fn success_if_component_is_defined() {
        let mut package = mock_package();
        File::create_table(&mut package).unwrap();
        File::insert(
            &mut package,
            &[File {
                file: "File1".to_string(),
                component: "Component1".to_string(),
                sequence: 1,
                ..Default::default()
            }],
        )
        .unwrap();
        Component::create_table(&mut package).unwrap();
        Component::insert(
            &mut package,
            &[Component {
                component: "Component1".to_string(),
                directory: "Directory1".to_string(),
                ..Default::default()
            }],
        )
        .unwrap();
        Directory::create_table(&mut package).unwrap();
        Directory::insert(
            &mut package,
            &[Directory {
                directory: "Directory1".to_string(),
                default_dir: "foo/bar".to_string(),
                parent: None,
            }],
        )
        .unwrap();
        let diagnostics = test_rule(InvalidForeignKeyRule {}, &mut package);

        assert_eq!(diagnostics.len(), 0);
    }

    fn mock_package() -> msi::Package<Cursor<Vec<u8>>> {
        let cursor = Cursor::new(Vec::new());
        Package::create(PackageType::Installer, cursor).expect("create")
    }
}
