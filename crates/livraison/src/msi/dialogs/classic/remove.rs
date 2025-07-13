use msi_installer::ui::{self, dialog::DialogSize, event::EndDialogAction};

use super::common;

#[allow(dead_code)]
pub fn create(builder: ui::dialog::DialogBuilder) -> ui::dialog::DialogBuilder {
    builder
        .size(DialogSize::classic())
        .add(
            ui::control::text(
                "Title",
                "{\\TitleFont}Uninstall [ProductName]",
            )
            .pos((135, 20))
            .size((220, 60)),
        )
        .add(
            ui::control::text(
                "Description",
                "This will remove [ProductName] from your computer. Click Remove to continue or Cancel to exit the uninstaller.",
            )
            .pos((135, 80))
            .size((220, 60)),
        )
        .add(common::side_image())
        .add(
            ui::control::line("BottomLine")
                .pos((0, 234))
                .width(374),
        )
        .add(
            ui::control::button(
                "Remove",
                "Remove",
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
            .trigger(ui::event::spawn_dialog("CancelDlg")),
        )
        .add(
            ui::control::button(
                "Back",
                "Back",
            )
            .pos((180, 243))
            .disable()
        )
}
