use super::{
    rule::{Diagnostic, Linter},
    rules::{ControlEventRefRule, DialogRule, PrimaryKeysRule},
};
use crate::color::{cyan, green, red};
use msi::Package;
use std::io::{Read, Seek};

// Validators to add
// - table primary keys must be all the first columns(can't skip)
// - table order respecting key string id map not string itself
pub fn validate_msi_installer<F: Read + Seek>(package: &mut Package<F>) {
    let mut linter = Linter::new();
    linter.register(PrimaryKeysRule {});
    linter.register(DialogRule {});
    linter.register(ControlEventRefRule {});

    let diagnostics = linter.lint(package);
    // let mut errors: Vec<ValidationError> = Vec::new();
    // errors.extend(safe_result(validate_pks(package)));
    // errors.extend(safe_result(validate_dialogs(package)));

    if diagnostics.is_empty() {
        println!("{} No errors found", green("✓"));
    } else {
        print_errors(&diagnostics);
        println!();
        println!(
            "{} Found {} errors",
            red("✗"),
            red(diagnostics.len().to_string())
        );
    }
}

fn print_errors(errors: &Vec<Diagnostic>) {
    for error in errors {
        println!(
            "{} {} {}",
            red("error"),
            cyan(error.code.clone()),
            error.message
        );
    }
}
