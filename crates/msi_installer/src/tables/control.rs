use super::{Entity, RowView, error::MsiDataBaseError};

use bitflags::bitflags;

/// Control Table
/// https://learn.microsoft.com/en-us/windows/win32/msi/control-table
#[derive(Debug, Clone)]
pub struct Control {
    pub dialog: String,
    pub control: String,
    pub type_: ControlType,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub attributes: ControlAttributes,
    pub property: Option<String>,
    pub text: Option<String>,
    pub control_next: Option<String>,
    pub help: Option<String>,
}

impl Entity for Control {
    fn table_name() -> &'static str {
        "Control"
    }

    fn definition() -> Vec<msi::Column> {
        vec![
            msi::Column::build("Dialog_")
                .primary_key()
                .category(msi::Category::Identifier)
                .id_string(72),
            msi::Column::build("Control")
                .primary_key()
                .category(msi::Category::Identifier)
                .string(50),
            msi::Column::build("Type")
                .category(msi::Category::Identifier)
                .string(20),
            msi::Column::build("X").range(0, 0x7fff).int16(),
            msi::Column::build("Y").range(0, 0x7fff).int16(),
            msi::Column::build("Width").range(0, 0x7fff).int16(),
            msi::Column::build("Height").range(0, 0x7fff).int16(),
            msi::Column::build("Attributes")
                .nullable()
                .range(-4, 0x7fffffff)
                .int32(),
            msi::Column::build("Property")
                .nullable()
                .category(msi::Category::Identifier)
                .string(50),
            msi::Column::build("Text")
                .nullable()
                .category(msi::Category::Formatted)
                .string(0),
            msi::Column::build("Control_Next")
                .nullable()
                .category(msi::Category::Identifier)
                .string(50),
            msi::Column::build("Help")
                .nullable()
                .category(msi::Category::Text)
                .string(50),
        ]
    }

    fn from_row(row: &RowView) -> Result<Control, MsiDataBaseError> {
        Ok(Control {
            dialog: row.string(0)?,
            control: row.string(1)?,
            type_: ControlType::from_str(&row.string(2)?),
            x: row.i32(3)?,
            y: row.i32(4)?,
            width: row.i32(5)?,
            height: row.i32(6)?,
            attributes: ControlAttributes::from_bits_retain(row.i32(7)? as u32),
            property: row.opt_string(8)?,
            text: row.opt_string(9)?,
            control_next: row.opt_string(10)?,
            help: row.opt_string(11)?,
        })
    }

    fn to_row(&self) -> Vec<msi::Value> {
        vec![
            msi::Value::Str(self.dialog.clone()),
            msi::Value::Str(self.control.clone()),
            msi::Value::Str(self.type_.as_str().to_string()),
            msi::Value::Int(self.x),
            msi::Value::Int(self.y),
            msi::Value::Int(self.width),
            msi::Value::Int(self.height),
            msi::Value::Int(self.attributes.bits() as i32),
            msi::Value::from_opt_string(&self.property),
            msi::Value::from_opt_string(&self.text),
            msi::Value::from_opt_string(&self.control_next),
            msi::Value::from_opt_string(&self.help),
        ]
    }
}

bitflags! {
    /// Dialog control Attributes
    /// https://learn.microsoft.com/en-us/windows/win32/msi/control-attributes
    #[derive(Debug, Clone)]
    pub struct ControlAttributes: u32 {
        const Visible = 1;
        const Enabled = 2;
        const Sunken = 4;
        const Indirect = 8;
        const IntegerControl = 16;
        const RTLRO = 32;
        const RightAligned = 64;
        const LeftScroll = 128;
        const BiDi = 224;

        // Text attributes
        const Transparent = 65536;
        const NoPrefix = 131072;
        const NoWrap = 262144;
        const FormatSize = 524288;
        const UsersLanguage = 1048576;
        const Password = 2097152;

        // Progress control attributes
        const Progress95 = 65536;

        // Edit control attributes
        const MultiLine	= 65536;
    }
}

/// Enum representing the different types of controls in an MSI dialog.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ControlType {
    Billboard,
    Bitmap,
    CheckBox,
    ComboBox,
    DirectoryCombo,
    DirectoryList,
    Edit,
    GroupBox,
    Hyperlink,
    Icon,
    Line,
    ListBox,
    ListView,
    MaskedEdit,
    PathEdit,
    ProgressBar,
    PushButton,
    RadioButtonGroup,
    ScrollableText,
    SelectionTree,
    Text,
    VolumeCostList,
    VolumeSelectCombo,
    Unknown(String),
}

impl ControlType {
    pub fn as_str(&self) -> &str {
        match self {
            ControlType::Billboard => "Billboard",
            ControlType::Bitmap => "Bitmap",
            ControlType::CheckBox => "CheckBox",
            ControlType::ComboBox => "ComboBox",
            ControlType::DirectoryCombo => "DirectoryCombo",
            ControlType::DirectoryList => "DirectoryList",
            ControlType::Edit => "Edit",
            ControlType::GroupBox => "GroupBox",
            ControlType::Hyperlink => "Hyperlink",
            ControlType::Icon => "Icon",
            ControlType::Line => "Line",
            ControlType::ListBox => "ListBox",
            ControlType::ListView => "ListView",
            ControlType::MaskedEdit => "MaskedEdit",
            ControlType::PathEdit => "PathEdit",
            ControlType::ProgressBar => "ProgressBar",
            ControlType::PushButton => "PushButton",
            ControlType::RadioButtonGroup => "RadioButtonGroup",
            ControlType::ScrollableText => "ScrollableText",
            ControlType::SelectionTree => "SelectionTree",
            ControlType::Text => "Text",
            ControlType::VolumeCostList => "VolumeCostList",
            ControlType::VolumeSelectCombo => "VolumeSelectCombo",
            ControlType::Unknown(val) => val,
        }
    }

    fn from_str(s: &str) -> Self {
        match s {
            "Billboard" => ControlType::Billboard,
            "Bitmap" => ControlType::Bitmap,
            "CheckBox" => ControlType::CheckBox,
            "ComboBox" => ControlType::ComboBox,
            "DirectoryCombo" => ControlType::DirectoryCombo,
            "DirectoryList" => ControlType::DirectoryList,
            "Edit" => ControlType::Edit,
            "GroupBox" => ControlType::GroupBox,
            "Hyperlink" => ControlType::Hyperlink,
            "Icon" => ControlType::Icon,
            "Line" => ControlType::Line,
            "ListBox" => ControlType::ListBox,
            "ListView" => ControlType::ListView,
            "MaskedEdit" => ControlType::MaskedEdit,
            "PathEdit" => ControlType::PathEdit,
            "ProgressBar" => ControlType::ProgressBar,
            "PushButton" => ControlType::PushButton,
            "RadioButtonGroup" => ControlType::RadioButtonGroup,
            "ScrollableText" => ControlType::ScrollableText,
            "SelectionTree" => ControlType::SelectionTree,
            "Text" => ControlType::Text,
            "VolumeCostList" => ControlType::VolumeCostList,
            "VolumeSelectCombo" => ControlType::VolumeSelectCombo,
            _ => ControlType::Unknown(s.to_string()),
        }
    }
}
