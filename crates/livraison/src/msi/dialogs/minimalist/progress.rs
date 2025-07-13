use msi_installer::ui::{self, dialog::DialogSize, event::EndDialogAction};

pub fn create(builder: ui::dialog::DialogBuilder) -> ui::dialog::DialogBuilder {
    builder
        .size(DialogSize::minimal())
        .modeless()
        .add(
            ui::control::text("Title", "{\\TitleFont}Installing [ProductName]")
                .pos((20, 10))
                .size((220, 20)),
        )
        .add(
            ui::control::text(
                "Text",
                "Please wait while [ProductName] is [Text_done]. This may take several minutes.",
            )
            .pos((20, 30))
            .size((220, 40)),
        )
        .add(
            ui::control::progress_bar("ProgressBar")
                .pos((20, 60))
                .width(220),
        )
        .add(
            ui::control::button("Cancel", "Cancel")
                .pos((134, 75))
                .trigger(ui::event::end_dialog(EndDialogAction::Exit)),
        )
        .add(ui::control::button("Next", "Next").pos((66, 75)).disable())
}
