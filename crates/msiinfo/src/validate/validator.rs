use std::io::{Read, Seek};

use clap::error;
use msi::{Package, Row};

use crate::color::{green, red};

struct Error {
    message: String,
}

pub fn validateMsiInstaller<F: Read + Seek>(package: &mut Package<F>) {
    let errors = validateDiaglogs(package);
    if errors.is_empty() {
        println!("{} No errors found", green("✓"));
    } else {
        printErrors(&errors);
    }
}

fn printErrors(errors: &Vec<Error>) {
    for error in errors {
        println!("{} {}", red("error"), error.message);
    }
}

fn validateDiaglogs<F: Read + Seek>(package: &mut Package<F>) -> Vec<Error> {
    let ui_sequence_table = package.get_table("InstallUISequence");
    if ui_sequence_table.is_none() {
        return vec![Error {
            message: "InstallUISequence table is missing".to_string(),
        }];
    }

    let rows: Vec<Row> = package
        .select_rows(msi::Select::table("InstallUISequence"))
        .expect("select")
        .collect();

    let mut errors = Vec::new();

    for row in rows.into_iter() {
        // Add your validation logic here and push errors to the errors vector
        dbg!(row[0].to_string());
    }

    errors
}
