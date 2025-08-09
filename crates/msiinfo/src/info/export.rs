use std::{
    cmp,
    io::{Read, Seek},
};

use crate::info::style::TableStyles;
use color::{bold, cyan, green, magenta, yellow};

pub fn print_table_contents<F: Read + Seek>(package: &mut msi::Package<F>, table_name: &str) {
    let mut col_widths: Vec<usize> = package
        .get_table(table_name)
        .unwrap()
        .columns()
        .iter()
        .map(|column| column.name().len())
        .collect();
    let mut rows: Vec<Vec<msi::Value>> = package
        .select_rows(msi::Select::table(table_name))
        .expect("select")
        .map(|row| {
            let mut strings = Vec::with_capacity(row.len());
            for index in 0..row.len() {
                let string = row[index].to_string();
                col_widths[index] = cmp::max(col_widths[index], string.len());
                strings.push(row[index].clone());
            }
            strings
        })
        .collect();
    print_separator(&col_widths);
    {
        let mut line = String::new();
        for (index, column) in package
            .get_table(table_name)
            .unwrap()
            .columns()
            .iter()
            .enumerate()
        {
            let name = column.name().to_string();
            let string = bold(cyan(&name)) + &whitespaces(col_widths[index] - name.len());
            line.push_str(&string);
            line.push_str("  ");
        }
        println!("{line}");
    }
    print_separator(&col_widths);

    rows.sort();
    for row in rows.into_iter() {
        let mut line = String::new();
        for (index, value) in row.into_iter().enumerate() {
            let string = print_value_and_pad(&value, col_widths[index]);
            line.push_str(&string);
            line.push_str("  ");
        }
        println!("{line}");
    }
}

fn print_separator(col_widths: &[usize]) {
    let mut line = String::new();
    for &width in col_widths.iter() {
        let string = cyan(TableStyles::H_BORDER.repeat(width));
        line.push_str(&string);
        line.push_str("  ");
    }
    println!("{line}");
}
fn print_value(value: &msi::Value) -> String {
    match value {
        msi::Value::Null => magenta("null"),
        msi::Value::Int(x) => yellow(x.to_string()),
        msi::Value::Str(x) => green(format!("\"{x}\"")),
    }
}

fn print_value_and_pad(value: &msi::Value, width: usize) -> String {
    let len = value.to_string().len();
    format!("{}{}", print_value(value), whitespaces(width - len))
}

fn whitespaces(n: usize) -> String {
    " ".repeat(n)
}
