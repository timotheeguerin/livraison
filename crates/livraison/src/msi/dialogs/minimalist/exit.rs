use msi_installer::ui::{self, dialog::DialogSize, event::EndDialogAction};

use super::common;

pub fn create() -> ui::dialog::DialogBuilder {
    ui::dialog::new("ExitDialog", "[ProductName] Setup")
        .size(DialogSize::minimal())
        .add(common::background_image())
        .add(
            ui::control::text("Title", "{\\TitleFont}[ProductName] [Text_action] complete")
                .pos((135, 20))
                .size((220, 60)),
        )
        .add(ui::control::line("BottomLine").pos((0, 234)).width(374))
        .add(
            ui::control::button("Finish", "Finish")
                .pos((236, 243))
                .trigger(ui::event::end_dialog(EndDialogAction::Return)),
        )
        .add(
            ui::control::button("Back", "Back")
                .pos((180, 243))
                .disable(),
        )
        .add(
            ui::control::button("Cancel", "Cancel")
                .pos((304, 243))
                .disable(),
        )
}
