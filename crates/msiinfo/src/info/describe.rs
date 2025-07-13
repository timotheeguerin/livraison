use std::io::{Read, Seek};

use crate::{
    color::{blue, bold, cyan, magenta, yellow},
    info::style::TableStyles,
};

pub fn print_table_description<F: Read + Seek>(package: &mut msi::Package<F>, table_name: &str) {
    if let Some(table) = package.get_table(table_name) {
        println!(
            "{} {}",
            bold(cyan(format!("{:<16}", "Column"))),
            bold(cyan("Type"))
        );
        println!("{}", bold(cyan(TableStyles::H_BORDER.repeat(50))),);
        for column in table.columns() {
            println!(
                "{:<16} {}{}{}{}",
                column.name(),
                if column.is_primary_key() {
                    bold(magenta("*"))
                } else {
                    ' '.to_string()
                },
                color_col_type(column.coltype()),
                if column.is_nullable() {
                    bold(magenta("?"))
                } else {
                    "".to_string()
                },
                match column.get_foreign_key() {
                    Some((table, index)) => {
                        format!(
                            " -> {}.{}",
                            bold(cyan(table)),
                            bold(cyan(index.to_string()))
                        )
                    }
                    None => "".to_string(),
                }
            );
        }
    } else {
        println!("No table {table_name:?} exists in the database.");
    }
}

fn color_col_type(coltype: msi::ColumnType) -> String {
    match coltype {
        msi::ColumnType::Binary => blue("binary"),
        msi::ColumnType::Int16 => blue("int16"),
        msi::ColumnType::Int32 => blue("int32"),
        msi::ColumnType::Str(s) => format!("{}({})", blue("string"), yellow(s.to_string())),
    }
}
