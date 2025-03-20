use msi_installer::tables::Dialog;

pub fn create_welcome_dialog() -> Dialog {
    Dialog {
        dialog: "WelcomeDialog".to_string(),
        h_centering: 50,
        v_centering: 50,
        width: 370,
        height: 270,
        attributes: 3,
        title: Some("[ProductName] Setup".to_string()),
        control_first: "WelcomeInstall".to_string(),
        control_default: Some("WelcomeInstall".to_string()),
        control_cancel: Some("WelcomeInstall".to_string()),
    }
}
