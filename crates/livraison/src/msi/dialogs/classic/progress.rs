use msi_installer::ui::{self, dialog::DialogSize};

pub fn create() -> ui::dialog::DialogBuilder {
    ui::dialog::new("ProgressDialog", "[ProductName] Setup")
        .size(DialogSize::classic())
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
            ui::control::dyn_text("ActionText")
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
                .pos((304, 243))
                .trigger(ui::event::spawn_dialog("CancelDialog")),
        )
        .add(
            ui::control::button("Next", "Next")
                .pos((236, 243))
                .disable(),
        )
        .add(
            ui::control::button("Back", "Back")
                .pos((180, 243))
                .disable(),
        )
}
