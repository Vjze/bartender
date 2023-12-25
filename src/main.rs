#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use iced::{
    window::{self, Position},
    Application, Font, Settings, Size,
};
use print_tool::{resource::assets::pngs::IMG_LOGO, widgets::ui::Bartender};

fn main() -> iced::Result {
    let icon = window::icon::from_file_data(IMG_LOGO, Some(image::ImageFormat::Png)).ok();
    Bartender::run(Settings {
        window: window::Settings {
            icon,
            size: Size::new(1280.0, 720.0),
            // max_size:true,
            position: Position::Centered,
            resizable: false,
            ..window::Settings::default()
        },
        default_font: Font::with_name("Source Han Sans HW SC"),
        ..Default::default()
    })?;

    Ok(())
}
