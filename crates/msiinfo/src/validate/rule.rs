use msi::{Package, Rows, Select};
use msi_installer::tables::{
    Component, Control, ControlEvent, Dialog, Entity, File, InstallUISequence, Property,
};
use std::{
    error::Error,
    io::{self, Read, Seek},
};

use super::dialog_map::DialogMap;

#[allow(dead_code)]
pub trait PackageReader {
    fn package_type(&self) -> msi::PackageType;
    fn summary_info(&self) -> &msi::SummaryInfo;
    fn database_codepage(&self) -> msi::CodePage;
    fn has_table(&self, table_name: &str) -> bool;
    fn get_table(&self, table_name: &str) -> Option<&msi::Table>;
    fn tables(&self) -> msi::Tables;
    fn has_stream(&self, stream_name: &str) -> bool;
    fn has_digital_signature(&self) -> bool;
    // fn read_stream(&mut self, stream_name: &str) -> io::Result<StreamReader<F>>;

    fn select_rows(&mut self, query: Select) -> io::Result<Rows>;
}

impl<F: Read + Seek> PackageReader for Package<F> {
    fn package_type(&self) -> msi::PackageType {
        self.package_type()
    }

    fn summary_info(&self) -> &msi::SummaryInfo {
        self.summary_info()
    }

    fn database_codepage(&self) -> msi::CodePage {
        self.database_codepage()
    }

    fn has_table(&self, table_name: &str) -> bool {
        self.has_table(table_name)
    }

    fn get_table(&self, table_name: &str) -> Option<&msi::Table> {
        self.get_table(table_name)
    }

    fn tables(&self) -> msi::Tables {
        self.tables()
    }

    fn has_stream(&self, stream_name: &str) -> bool {
        self.has_stream(stream_name)
    }

    fn has_digital_signature(&self) -> bool {
        self.has_digital_signature()
    }
    fn select_rows(&mut self, query: Select) -> io::Result<Rows> {
        self.select_rows(query)
    }
}

#[allow(dead_code)]
pub struct RuleData {
    pub dialogs: Vec<Dialog>,
    pub controls: Vec<Control>,
    pub control_events: Vec<ControlEvent>,
    pub properties: Vec<Property>,
    pub install_ui_sequences: Vec<InstallUISequence>,
    pub components: Vec<Component>,
    pub files: Vec<File>,
    pub dialog_map: DialogMap,
}

pub trait Rule {
    fn code(&self) -> &'static str;

    fn validate_pks(
        &self,
        _ctx: &mut RuleContext,
        _package: &mut dyn PackageReader,
    ) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn run(&self, _ctx: &mut RuleContext, _data: &RuleData) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct Diagnostic {
    pub code: String,
    pub message: String,
}

pub struct RuleContext {
    code: String,
    diagnostics: Vec<Diagnostic>,
}

impl RuleContext {
    pub fn error(&mut self, message: String) {
        self.diagnostics.push(Diagnostic {
            code: self.code.clone(),
            message: message.to_string(),
        });
    }
}

pub struct Linter {
    rules: Vec<Box<dyn Rule>>,
}

impl Linter {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn register<T: Rule + 'static>(&mut self, rule: T) {
        self.rules.push(Box::new(rule));
    }

    pub fn lint<F: Read + Seek>(&self, package: &mut Package<F>) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();

        let data = get_rule_data(&mut diagnostics, package);
        for rule in &self.rules {
            diagnostics.extend(run_rule(rule.as_ref(), package, &data));
        }

        diagnostics
    }
}

fn run_rule<F: Read + Seek>(
    rule: &dyn Rule,
    package: &mut Package<F>,
    data: &RuleData,
) -> Vec<Diagnostic> {
    let mut ctx = RuleContext {
        code: rule.code().to_string(),
        diagnostics: Vec::new(),
    };
    if let Err(e) = rule.validate_pks(&mut ctx, package) {
        ctx.error(e.to_string());
    }

    if let Err(e) = rule.run(&mut ctx, data) {
        ctx.error(e.to_string());
    }
    ctx.diagnostics
}

#[allow(dead_code)]
pub fn test_rule<F: Read + Seek>(rule: impl Rule, package: &mut Package<F>) -> Vec<Diagnostic> {
    let data = get_rule_data(&mut vec![], package);
    run_rule(&rule, package, &data)
}

fn get_rule_data<F: Read + Seek>(
    diagnostics: &mut Vec<Diagnostic>,
    package: &mut Package<F>,
) -> RuleData {
    let dialogs = safe_list::<Dialog, F>(diagnostics, package);
    let controls = safe_list::<Control, F>(diagnostics, package);
    let control_events = safe_list::<ControlEvent, F>(diagnostics, package);
    let install_ui_sequences = safe_list::<InstallUISequence, F>(diagnostics, package);
    let dialog_map = DialogMap::new(dialogs.clone(), controls.clone());

    RuleData {
        dialogs,
        controls,
        control_events,
        properties: safe_list::<Property, F>(diagnostics, package),
        install_ui_sequences,
        dialog_map,
        components: safe_list::<Component, F>(diagnostics, package),
        files: safe_list::<File, F>(diagnostics, package),
    }
}

fn safe_list<T: Entity, F: Read + Seek>(
    diagnostics: &mut Vec<Diagnostic>,
    package: &mut Package<F>,
) -> Vec<T> {
    match T::list(package) {
        Ok(val) => val,
        Err(e) => {
            diagnostics.push(Diagnostic {
                code: "table-read".to_string(),
                message: e.to_string(),
            });
            vec![]
        }
    }
}

pub mod macros {
    macro_rules! hl {
        ($fmt:expr, $($arg:expr),*) => {{
            let mut fmt_str = $fmt.to_string();
            let args = vec![$(crate::color::magenta($arg.to_string())),*];
            let mut arg_index = 0;
            while let Some(start) = fmt_str.find("{}") {
                if arg_index >= args.len() {
                    break;
                }
                let end = start + 2;
                fmt_str.replace_range(start..end, &args[arg_index]);
                arg_index += 1;
            }
            fmt_str
        }};
    }
    pub(crate) use hl;
}
