pub mod pngs {
    pub static IMG_LOGO: &[u8] = include_bytes!("../../resources/logo/icons.png");
    pub static C_LOGO: &[u8] = include_bytes!("../../resources/logo/logo.png");
    pub static RIGHT: &[u8] = include_bytes!("../../resources/assets/right.png");
    pub static WRONGS: &[u8] = include_bytes!("../../resources/assets/wrongs.png");
    pub static WAIT: &[u8] = include_bytes!("../../resources/assets/wait.png");
}
pub mod fonts {
    pub static FONT: &[u8] =
        include_bytes!("../../resources/assets/SourceHanSansHWSC-Regular.otf").as_slice();
}
