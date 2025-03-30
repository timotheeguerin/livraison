use msi_installer::ui::{self, dialog::DialogSize};

pub fn create() -> ui::dialog::DialogBuilder {
    ui::dialog::new("FatalErrorDialog", "[ProductName] Setup")
    .size(DialogSize::classic())
        .add(
            ui::control::text("Title", "{\\TitleFont}[ProductName] [Text_agent] ended prematurely")
                .pos((20, 20))
                .size((220, 60)),
        )
        .add(
            ui::control::text(
                "Description1",
                "[ProductName] [Text_action] ended because of an error. The program has not been installed. This installer can be run again at a later time.",
            )
            .pos((20, 30))
            .size((220, 40)),
        )
        .add(
            ui::control::button("Finish", "Finish")
                .pos((66, 75))
                .trigger(ui::event::end_dialog(msi_installer::ui::event::EndDialogAction::Exit)),
        )
}
