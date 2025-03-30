use msi_installer::ui;

mod cancel;
mod common;
mod exit;
mod fatal_error;
mod progress;
mod remove;
mod welcome;

pub fn create() -> ui::UiBuilder {
    ui::new()
        .new_dialog("WelcomeDlg", welcome::create)
        .new_dialog("RemoveDlg", remove::create)
        .new_dialog("FatalErrorDlg", fatal_error::create)
        .new_dialog("ProgressDlg", progress::create)
        .new_dialog("ExitDlg", exit::create)
        .new_dialog("CancelDlg", exit::create)
}
