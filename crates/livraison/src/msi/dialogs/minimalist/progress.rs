use msi_installer::ui::{self, dialog::DialogSize, event::EndDialogAction};

pub fn create() -> ui::dialog::DialogBuilder {
    ui::dialog::new("ProgressDialog", "[ProductName] Setup")
        .size(DialogSize::minimal())
        .modeless()
        .add(
            ui::control::text("Title", "{\\TitleFont}Installing [ProductName]")
                .pos((20, 15))
                .size((330, 15)),
        )
        .add(
            ui::control::text(
                "Text",
                "Please wait while [ProductName] is [Text_done]. This may take several minutes.",
            )
            .pos((20, 65))
            .size((330, 35)),
        )
        .add(ui::control::line("BannerLine").pos((0, 44)).width(374))
        .add(
            ui::control::dyn_text("ActionText", "ActionText")
                .pos((70, 100))
                .size((285, 10)),
        )
        .add(
            ui::control::text("StatusLabel", "Status:")
                .pos((20, 100))
                .size((50, 10)),
        )
        .add(
            ui::control::progress_bar("ProgressBar")
                .pos((20, 115))
                .width(330),
        )
        .add(ui::control::line("BottomLine").pos((0, 234)).width(374))
        .add(
            ui::control::button("Cancel", "Cancel")
                .pos((104, 80))
                .trigger(ui::event::end_dialog(EndDialogAction::Exit)),
        )
        .add(ui::control::button("Next", "Next").pos((36, 80)).disable())
}
