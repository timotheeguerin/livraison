use msiinfo::color::{blue, cyan, yellow};
use std::fs::File;
use std::io::{Read, Seek};
use std::{cmp, io};
use time::OffsetDateTime;

use msiinfo::validate::validator::validate_msi_installer;

fn pad(mut string: String, fill: char, width: usize) -> String {
    while string.len() < width {
        string.push(fill);
    }
    string
}

use clap::{Parser, Subcommand, command};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct AppArgs {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    #[command(visible_alias = "suminfo")]
    Summary {
        path: String,
    },
    Tables {
        path: String,
    },
    Describe {
        path: String,
        table: String,
    },
    Export {
        path: String,
        table: String,
    },
    Extract {
        path: String,
        stream: String,
        #[arg(short = 'o', long)]
        out: String,
    },
    Streams {
        path: String,
    },
    Validate {
        path: String,
    },
    Content {
        path: String,
    },
}

fn print_summary_info<F>(package: &msi::Package<F>) {
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

fn print_table_description<F: Read + Seek>(package: &mut msi::Package<F>, table_name: &str) {
    if let Some(table) = package.get_table(table_name) {
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
    } else {
        println!("No table {table_name:?} exists in the database.");
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

fn main() {
    let args = AppArgs::parse();

    match args.command {
        Command::Summary { path } => {
            let package = msi::open(path).expect("open package");
            print_summary_info(&package);
        }
        Command::Tables { path } => {
            let package = msi::open(path).expect("open package");
            for table in package.tables() {
                println!("{}", table.name());
            }
        }
        Command::Describe { path, table } => {
            let mut package = msi::open(path).expect("open package");
            print_table_description(&mut package, &table);
        }
        Command::Export { path, table } => {
            let mut package = msi::open(path).expect("open package");
            print_table_contents(&mut package, &table)
        }
        Command::Streams { path } => {
            let package = msi::open(path).expect("open package");
            for stream_name in package.streams() {
                println!("{stream_name}");
            }
        }
        Command::Extract { path, stream, out } => {
            let mut package = msi::open(path).expect("open package");
            let mut stream = package.read_stream(&stream).expect("open stream");
            let mut file = File::create(out).expect("create file");
            io::copy(&mut stream, &mut file).expect("wrote stream to file");
        }
        Command::Validate { path } => {
            let mut package = msi::open(path).expect("open package");
            validate_msi_installer(&mut package);
        }
        Command::Content { path } => {
            let mut package = msi::open(path).expect("open package");
            print_content(&mut package);
        }
    }
}

fn print_content<F: Read + Seek>(package: &mut msi::Package<F>) {
    let stream_names: Vec<String> = package.streams().collect();
    for stream_name in stream_names {
        if !stream_name.ends_with(".cab") {
            continue;
        }

        let cab_stream = package.read_stream(&stream_name).expect("open cab stream");
        println!("{}:", blue(stream_name));

        for folder in cab::Cabinet::new(cab_stream)
            .expect("open cab")
            .folder_entries()
        {
            // println!("  Folder: {}", folder.);
            for file in folder.file_entries() {
                println!(
                    "    {} {}",
                    cyan(file.name()),
                    yellow(print_byte_size(file.uncompressed_size())),
                );
            }
        }
    }
}

fn print_byte_size(size: u32) -> String {
    if size < 1024 {
        format!("{size}B")
    } else if size < 1024 * 1024 {
        format!("{:.2}KB", size / 1024)
    } else if size < 1024 * 1024 * 1024 {
        format!("{:.2}MB", size / 1024 / 1024)
    } else {
        format!("{:.2}GB", size / 1024 / 1024 / 1024)
    }
}
