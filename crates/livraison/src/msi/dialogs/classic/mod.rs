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
        .new_dialog("WelcomeDialog", welcome::create)
        .new_dialog("RemoveDialog", remove::create)
        .new_dialog("FatalErrorDialog", fatal_error::create)
        .new_dialog("ProgressDialog", progress::create)
        .new_dialog("ExitDialog", exit::create)
        .new_dialog("CancelDialog", exit::create)
}
