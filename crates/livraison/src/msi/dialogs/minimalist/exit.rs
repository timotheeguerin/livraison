use msi_installer::ui::{self, dialog::DialogSize, event::EndDialogAction};

pub fn create() -> ui::dialog::DialogBuilder {
    ui::dialog::new("ExitDialog", "[ProductName] Setup")
        .size(DialogSize::minimal())
        .add(
            ui::control::text("Title", "{\\TitleFont}[ProductName] [Text_action] complete")
                .pos((20, 20))
                .size((220, 60)),
        )
        .add(
            ui::control::button("Finish", "Finish")
                .pos((134, 75))
                .trigger(ui::event::end_dialog(EndDialogAction::Return)),
        )
}
