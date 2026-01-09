use iced::widget::svg::{Handle, Svg};
use std::sync::LazyLock;

pub static SAVE_ICON: LazyLock<Handle> = LazyLock::new(|| Handle::from_memory(include_bytes!("../../icons/floppy-disk.svg")));
pub static LOAD_ICON: LazyLock<Handle> = LazyLock::new(|| Handle::from_memory(include_bytes!("../../icons/open-new-window.svg")));
pub static EDIT_ICON: LazyLock<Handle> = LazyLock::new(|| Handle::from_memory(include_bytes!("../../icons/page-edit.svg")));    


pub fn save_icon() -> Svg<'static> {
    Svg::new(SAVE_ICON.clone()).width(iced::Shrink)
}

pub fn load_icon() -> Svg<'static> {
    Svg::new(LOAD_ICON.clone()).width(iced::Shrink)
}

pub fn edit_icon() -> Svg<'static> {
        Svg::new(EDIT_ICON.clone()).width(iced::Shrink)
}