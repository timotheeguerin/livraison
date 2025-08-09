use super::Doc;

pub fn text(text: impl Into<String>) -> Doc {
    Doc::Text(text.into())
}

pub fn indent(text: impl Into<Doc>) -> Doc {
    Doc::Indent(Box::new(text.into()))
}

pub fn join(docs: Vec<Doc>, joiner: impl Into<Doc>) -> Doc {
    Doc::Join(docs, Box::new(joiner.into()))
}

pub fn group(docs: Vec<Doc>) -> Doc {
    Doc::Items(docs)
}

#[allow(non_upper_case_globals)]
pub const hardline: Doc = Doc::Hardline;

impl From<Vec<Doc>> for Doc {
    fn from(val: Vec<Doc>) -> Self {
        Doc::Items(val)
    }
}

impl From<String> for Doc {
    fn from(val: String) -> Self {
        Doc::Text(val)
    }
}

impl From<&str> for Doc {
    fn from(val: &str) -> Self {
        Doc::Text(val.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::{hardline, indent, join, text};

    #[test]
    fn text_with_str() {
        let doc = text("Hello, world!");
        assert_eq!(doc.serialize(), "Hello, world!");
    }

    #[test]
    fn text_with_string() {
        let s = "Hello, world!".to_string();
        let doc = text(s);
        assert_eq!(doc.serialize(), "Hello, world!");
    }

    #[test]
    fn indent_single_line() {
        let doc = indent("Hello, world!");
        assert_eq!(doc.serialize(), "  Hello, world!");
    }

    #[test]
    fn indent_multi_lines() {
        let doc = indent(vec![text("one"), hardline, text("two")]);
        assert_eq!(doc.serialize(), "  one\n  two");
    }

    #[test]
    fn join_docs() {
        let doc = join(vec![text("one"), text("two")], " | ");
        assert_eq!(doc.serialize(), "one | two");
    }

    #[test]
    fn vec_can_be_into_doc() {
        let doc = indent(vec![text("Hello, "), text("world!")]);
        assert_eq!(doc.serialize(), "  Hello, world!");
    }
}
