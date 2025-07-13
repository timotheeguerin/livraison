use super::{Entity, RowView, error::MsiDataBaseError};
use bitflags::bitflags;

/// Text Style Table
/// https://learn.microsoft.com/en-us/windows/win32/msi/textstyle-table
#[derive(Debug, Clone, Default)]
pub struct TextStyle {
    pub text_style: String,
    pub face_name: String,
    pub size: i32,
    pub color: i32,
    pub attributes: Option<StyleAttributes>,
}

impl Entity for TextStyle {
    fn table_name() -> &'static str {
        "TextStyle"
    }

    fn definition() -> Vec<msi::Column> {
        vec![
            msi::Column::build("TextStyle").primary_key().id_string(72),
            msi::Column::build("FaceName")
                .category(msi::Category::Text)
                .string(32),
            msi::Column::build("Size").range(0, 0x7fff).int16(),
            msi::Column::build("Color")
                .nullable()
                .range(0, 0xffffff)
                .int32(),
            msi::Column::build("StyleBits")
                .nullable()
                .range(0, 15)
                .int16(),
        ]
    }

    fn from_row(row: &RowView) -> Result<TextStyle, MsiDataBaseError> {
        Ok(TextStyle {
            text_style: row.string(0)?,
            face_name: row.string(1)?,
            size: row.i32(2)?,
            color: row.i32(3)?,
            attributes: row.opt_i32(4)?.map(StyleAttributes::from_bits_retain),
        })
    }

    fn to_row(&self) -> Vec<msi::Value> {
        vec![
            msi::Value::Str(self.text_style.clone()),
            msi::Value::Str(self.face_name.clone()),
            msi::Value::Int(self.size),
            msi::Value::Int(self.color),
            match &self.attributes {
                Some(attr) => msi::Value::Int(attr.bits()),
                None => msi::Value::Null,
            },
        ]
    }
}

bitflags! {
    /// Style Attributes
    /// https://learn.microsoft.com/en-us/windows/win32/msi/textstyle-table#StyleBits
    #[derive(Debug, Clone, Default)]
    pub struct StyleAttributes: i32 {
        const Bold = 1;
        const Italic = 2;
        const Underline = 4;
        const Strike = 8;
    }
}
