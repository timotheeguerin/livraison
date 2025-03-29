use msi_installer::ui::{self, event::EndDialogAction};

pub fn create() -> ui::dialog::DialogBuilder {
    ui::dialog::new("CancelDialog", "[ProductName] Setup")
        .size((260, 85))
        .add(
            ui::control::text("Text", "Do you want to abort?")
                .pos((48, 15))
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
