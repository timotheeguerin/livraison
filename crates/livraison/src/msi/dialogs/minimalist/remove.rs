use msi_installer::ui::{self, dialog::DialogSize, event::EndDialogAction};

pub fn create() -> ui::dialog::DialogBuilder {
    ui::dialog::new("RemoveDialog", "[ProductName] Setup")
        .size(DialogSize::minimal())
        .add(
            ui::control::text(
                "Title",
                "{\\TitleFont}Uninstall [ProductName]",
            )
            .pos((20, 10))
            .size((220, 20)),
        )
        .add(
            ui::control::text(
                "Description",
                "This will remove [ProductName] from your computer. Click Remove to continue or Cancel to exit the uninstaller.",
            )
            .pos((20, 30))
            .size((220, 40)),
        )
        .add(
            ui::control::button(
                "Remove",
                "Remove",
            )
            .pos((66, 75))
            .trigger(ui::event::end_dialog(EndDialogAction::Return)),
        )
        .add(
            ui::control::button(
                "Cancel",
                "Cancel",
            )
            .pos((134, 75))
            .trigger(ui::event::end_dialog(EndDialogAction::Exit)),
        )
}
