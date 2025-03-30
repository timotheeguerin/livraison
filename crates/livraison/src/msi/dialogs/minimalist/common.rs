use msi_installer::ui;

pub fn background_image() -> ui::control::Bitmap {
    ui::control::bitmap("Background", "MinimalistBackground")
        .pos((0, 0))
        .size((100, 234))
}
