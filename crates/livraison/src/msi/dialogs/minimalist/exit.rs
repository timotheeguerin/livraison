use msi_installer::ui::{self, dialog::DialogSize, event::EndDialogAction};

pub fn create(builder: ui::dialog::DialogBuilder) -> ui::dialog::DialogBuilder {
    builder
        .size(DialogSize::minimal())
        .add(
            ui::control::text("Title", "{\\TitleFont}[ProductName] [Text_action] complete")
                .pos((20, 20))
                .size((220, 60)),
        )
        .add(
            ui::control::button("Finish", "Finish")
                .pos((66, 75))
                .trigger(ui::event::end_dialog(EndDialogAction::Return)),
        )
        .add(
            ui::control::button("Cancel", "Cancel")
                .pos((134, 75))
                .trigger(ui::event::end_dialog(EndDialogAction::Exit))
                .disable(),
        )
}
