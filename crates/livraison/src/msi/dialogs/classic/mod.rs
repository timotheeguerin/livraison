use msi_installer::ui;

mod cancel;
mod common;
mod exit;
mod fatal_error;
mod progress;
mod remove;
mod welcome;

pub fn create_classic_dialogs() -> Vec<ui::dialog::DialogBuilder> {
    vec![
        welcome::create(),
        remove::create(),
        fatal_error::create(),
        progress::create(),
        exit::create(),
        cancel::create(),
    ]
}
