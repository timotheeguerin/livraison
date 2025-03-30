use msi_installer::ui::{self, dialog::DialogSize, event::EndDialogAction};

use super::common;

pub fn create() -> ui::dialog::DialogBuilder {
    ui::dialog::new("WelcomeDialog", "[ProductName] Setup")
        .size(DialogSize::minimal())
        .add(common::background_image())
        .add(
            ui::control::text(
                "Title",
                "{\\TitleFont}Welcome to the [ProductName] installer",
            )
            .pos((135, 20))
            .size((220, 60)),
        )
        .add(
            ui::control::text(
                "Description",
                "{\\DefaultFont}This will install [ProductName] on your computer. Click Install to continue or Cancel to exit the installer.",
            )
            .pos((135, 80))
            .size((220, 60)),
        )
        .add(
            ui::control::button(
                "Next",
                "Install",
            )
            .pos((236, 243))
            .trigger(ui::event::end_dialog(EndDialogAction::Return)),
        )
        .add(
            ui::control::button(
                "Cancel",
                "Cancel",
            )
            .pos((304, 243))
            .trigger(ui::event::end_dialog(EndDialogAction::Exit)),
        )
}
