use uuid::Uuid;

use crate::validate::rule::{Rule, RuleContext, RuleData, macros::hl};

pub struct InvalidPropertyRule {}
impl Rule for InvalidPropertyRule {
    fn code(&self) -> &'static str {
        "invalid-property"
    }

    fn run(
        &self,
        ctx: &mut RuleContext,
        data: &RuleData,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let RuleData { properties, .. } = data;

        let map = properties
            .iter()
            .map(|p| (p.property.clone(), p.value.clone()))
            .collect::<std::collections::HashMap<_, _>>();

        check_property_defined(ctx, &map, "ProductVersion");
        check_property_defined(ctx, &map, "ProductCode");
        check_property_defined(ctx, &map, "Manufacturer");
        check_property_defined(ctx, &map, "ProductName");
        if check_property_defined(ctx, &map, "ProductCode") {
            let code = map.get("ProductCode").unwrap();
            if !(code.starts_with('{')
                && code.ends_with('}')
                && Uuid::parse_str(&code[1..code.len() - 1]).is_ok())
            {
                ctx.error(hl!(
                    "ProductCode {} property must be a uuid surrounded by braces {}",
                    code
                ));
            }
        }

        Ok(())
    }
}

fn check_property_defined(
    ctx: &mut RuleContext,
    map: &std::collections::HashMap<String, String>,
    property: &str,
) -> bool {
    if !map.contains_key(property) {
        ctx.error(hl!("{} property is missing", property));
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use msi::{Package, PackageType};
    use msi_installer::tables::{Entity, Property};

    use super::InvalidPropertyRule;
    use crate::validate::rule::test_rule;

    #[test]
    fn report_if_upgrade_code_not_valid_guid() {
        let mut package = mock_package();
        Property::create_table(&mut package).unwrap();
        Property::insert(
            &mut package,
            &[
                Property {
                    property: "ProductVersion".to_string(),
                    value: "1.0.0".to_string(),
                },
                Property {
                    property: "UpgradeCode".to_string(),
                    value: "51598408-5336-58ae-9d26-1a7472fe961e".to_string(),
                },
                Property {
                    property: "Manufacturer".to_string(),
                    value: "Test Inc".to_string(),
                },
                Property {
                    property: "ProductName".to_string(),
                    value: "Test".to_string(),
                },
                Property {
                    property: "ProductLanguage".to_string(),
                    value: "1033".to_string(),
                },
            ],
        )
        .unwrap();
        let diagnostics = test_rule(InvalidPropertyRule {}, &mut package);

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].code, "invalid-property");
    }

    #[test]
    fn report_missing_required_property() {
        let mut package = mock_package();
        Property::create_table(&mut package).unwrap();
        Property::insert(&mut package, &[]).unwrap();
        let diagnostics = test_rule(InvalidPropertyRule {}, &mut package);

        assert_eq!(diagnostics.len(), 5);
        assert_eq!(diagnostics[0].code, "invalid-property");
        assert_eq!(diagnostics[1].code, "invalid-property");
        assert_eq!(diagnostics[2].code, "invalid-property");
        assert_eq!(diagnostics[3].code, "invalid-property");
        assert_eq!(diagnostics[4].code, "invalid-property");
    }

    fn mock_package() -> msi::Package<Cursor<Vec<u8>>> {
        let cursor = Cursor::new(Vec::new());
        Package::create(PackageType::Installer, cursor).expect("create")
    }
}
