use crate::validate::rule::{Rule, RuleContext, RuleData, macros::hl};

pub struct ControlEventRefRule {}
impl Rule for ControlEventRefRule {
    fn code(&self) -> &'static str {
        "invalid-control-event-ref"
    }

    fn run(
        &self,
        ctx: &mut RuleContext,
        data: &RuleData,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let RuleData {
            control_events,
            dialog_map,
            ..
        } = data;

        for row in control_events {
            let dialog = match dialog_map.get(&row.dialog) {
                Some(dialog) => dialog,
                None => {
                    ctx.error(hl!(
                        "{} table is referencing a missing dialog: {}",
                        "ControlEvent",
                        row.dialog
                    ));
                    continue;
                }
            };

            let control = dialog.controls.get(&row.control);
            if control.is_none() {
                ctx.error(hl!(
                    "{} table is referencing a missing control: {} on dialog: {}",
                    "ControlEvent",
                    row.control,
                    row.dialog
                ));
                continue;
            };
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use msi::{Package, PackageType};
    use msi_installer::tables::{ControlEvent, Dialog, Entity};

    use super::ControlEventRefRule;
    use crate::validate::rule::test_rule;

    #[test]
    fn report_if_dialog_not_defined() {
        let mut package = mock_package();
        ControlEvent::create_table(&mut package).unwrap();
        ControlEvent::insert(
            &mut package,
            &[ControlEvent {
                dialog: "Dialog".to_string(),
                control: "Control".to_string(),
                event: "Event".to_string(),
                argument: "Argument".to_string(),
                ..Default::default()
            }],
        )
        .unwrap();
        let diagnostics = test_rule(ControlEventRefRule {}, &mut package);

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].code, "invalid-control-event-ref");
    }

    #[test]
    fn report_if_control_not_defined() {
        let mut package = mock_package();
        ControlEvent::create_table(&mut package).unwrap();
        ControlEvent::insert(
            &mut package,
            &[ControlEvent {
                dialog: "Dialog".to_string(),
                control: "Control".to_string(),
                event: "Event".to_string(),
                argument: "Argument".to_string(),
                ..Default::default()
            }],
        )
        .unwrap();
        Dialog::create_table(&mut package).unwrap();
        Dialog::insert(
            &mut package,
            &[Dialog {
                dialog: "Dialog".to_string(),
                control_first: "ControlFirst".to_string(),
                ..Default::default()
            }],
        )
        .unwrap();
        let diagnostics = test_rule(ControlEventRefRule {}, &mut package);

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].code, "invalid-control-event-ref");
    }

    fn mock_package() -> msi::Package<Cursor<Vec<u8>>> {
        let cursor = Cursor::new(Vec::new());
        Package::create(PackageType::Installer, cursor).expect("create")
    }
}
