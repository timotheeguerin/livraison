use msi_installer::ui::{self, event::EndDialogAction};

#[allow(dead_code)]
pub fn create(builder: ui::dialog::DialogBuilder) -> ui::dialog::DialogBuilder {
    builder
        .size((260, 85))
        .add(
            ui::control::text("Text", "Do you want to abort?")
                .pos((72, 25))
                .size((194, 30)),
        )
        .add(
            ui::control::button("Cancel", "No")
                .pos((132, 57))
                .trigger(ui::event::end_dialog(EndDialogAction::Return)),
        )
        .add(
            ui::control::button("Yes", "Yes")
                .pos((72, 57))
                .trigger(ui::event::end_dialog(EndDialogAction::Exit)),
        )
}
