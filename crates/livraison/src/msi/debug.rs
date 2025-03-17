use std::{
    cmp,
    io::{Read, Seek},
};

use time::OffsetDateTime;

pub fn print_all<F: Read + Seek>(package: &mut msi::Package<F>) {
    println!("--------------------------------------------------------------------------");
    println!("           Print summary info ");
    println!("--------------------------------------------------------------------------");
    print_summary_info(package);

    println!("--------------------------------------------------------------------------");
    println!("           FeatureComponents ");
    println!("--------------------------------------------------------------------------");
    print_table_description(package.get_table("FeatureComponents").unwrap());
    print_table_contents(package, "FeatureComponents");

    println!("--------------------------------------------------------------------------");
    println!("           Property ");
    println!("--------------------------------------------------------------------------");
    print_table_description(package.get_table("Property").unwrap());
    print_table_contents(package, "Property");
}

pub fn print_summary_info<F>(package: &msi::Package<F>) {
    println!("Package type: {:?}", package.package_type());
    let is_signed = package.has_digital_signature();
    let summary_info = package.summary_info();
    let codepage = summary_info.codepage();
    println!("   Code page: {} ({})", codepage.id(), codepage.name());
    if let Some(title) = summary_info.title() {
        println!("       Title: {title}");
    }
    if let Some(subject) = summary_info.subject() {
        println!("     Subject: {subject}");
    }
    if let Some(author) = summary_info.author() {
        println!("      Author: {author}");
    }
    if let Some(uuid) = summary_info.uuid() {
        println!("        UUID: {}", uuid.hyphenated());
    }
    if let Some(arch) = summary_info.arch() {
        println!("        Arch: {arch}");
    }
    let languages = summary_info.languages();
    if !languages.is_empty() {
        let tags: Vec<&str> = languages.iter().map(msi::Language::tag).collect();
        println!("    Language: {}", tags.join(", "));
    }
    if let Some(timestamp) = summary_info.creation_time() {
        println!("  Created at: {}", OffsetDateTime::from(timestamp));
    }
    if let Some(app_name) = summary_info.creating_application() {
        println!("Created with: {app_name}");
    }
    println!("      Signed: {}", if is_signed { "yes" } else { "no" });
    if let Some(comments) = summary_info.comments() {
        println!("Comments:");
        for line in comments.lines() {
            println!("  {line}");
        }
    }
}

fn print_table_description(table: &msi::Table) {
    println!("{}", table.name());
    for column in table.columns() {
        println!(
            "  {:<16} {}{}{}",
            column.name(),
            if column.is_primary_key() { '*' } else { ' ' },
            column.coltype(),
            if column.is_nullable() { "?" } else { "" }
        );
    }
}

fn print_table_contents<F: Read + Seek>(package: &mut msi::Package<F>, table_name: &str) {
    let mut col_widths: Vec<usize> = package
        .get_table(table_name)
        .unwrap()
        .columns()
        .iter()
        .map(|column| column.name().len())
        .collect();
    let rows: Vec<Vec<String>> = package
        .select_rows(msi::Select::table(table_name))
        .expect("select")
        .map(|row| {
            let mut strings = Vec::with_capacity(row.len());
            for index in 0..row.len() {
                let string = row[index].to_string();
                col_widths[index] = cmp::max(col_widths[index], string.len());
                strings.push(string);
            }
            strings
        })
        .collect();
    {
        let mut line = String::new();
        for (index, column) in package
            .get_table(table_name)
            .unwrap()
            .columns()
            .iter()
            .enumerate()
        {
            let string = pad(column.name().to_string(), ' ', col_widths[index]);
            line.push_str(&string);
            line.push_str("  ");
        }
        println!("{line}");
    }
    {
        let mut line = String::new();
        for &width in col_widths.iter() {
            let string = pad(String::new(), '-', width);
            line.push_str(&string);
            line.push_str("  ");
        }
        println!("{line}");
    }
    for row in rows.into_iter() {
        let mut line = String::new();
        for (index, value) in row.into_iter().enumerate() {
            let string = pad(value, ' ', col_widths[index]);
            line.push_str(&string);
            line.push_str("  ");
        }
        println!("{line}");
    }
}

fn pad(mut string: String, fill: char, width: usize) -> String {
    while string.len() < width {
        string.push(fill);
    }
    string
}
