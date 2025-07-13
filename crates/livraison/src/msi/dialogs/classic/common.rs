use msi_installer::ui;

pub fn side_image() -> ui::control::Bitmap {
    ui::control::bitmap("LeftBg", "ClassicImage")
        .pos((0, 0))
        .size((100, 234))
}
