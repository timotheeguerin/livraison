pub enum Doc {
    /// Create a hard line
    Hardline,
    /// Basic string
    Text(String),
    /// Array of documents
    Items(Vec<Doc>),
    /// Indented document
    Indent(Box<Doc>),
    /// Joined document
    Join(Vec<Doc>, Box<Doc>),
}

impl Doc {
    pub fn serialize(&self) -> String {
        match self {
            Doc::Hardline => "\n".to_string(),
            Doc::Text(s) => s.clone(),
            Doc::Items(docs) => docs
                .iter()
                .map(|d| d.serialize())
                .collect::<Vec<_>>()
                .join(""),
            Doc::Indent(s) => indent(&s.serialize(), "  "),
            Doc::Join(docs, sep) => docs
                .iter()
                .map(|d| d.serialize())
                .collect::<Vec<_>>()
                .join(&sep.serialize()),
        }
    }
}

fn indent(s: &str, indent: &str) -> String {
    s.lines()
        .map(|line| format!("{}{}", indent, line))
        .collect::<Vec<_>>()
        .join("\n")
}
