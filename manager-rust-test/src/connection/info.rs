use winapi::{
    shared::windef::HWND,
    um::winuser::IsWindowVisible,
};

use super::title::get_title;

pub fn is_window_visible(window: &HWND) -> bool {
    let visible;
    unsafe {
        let title: String = get_title(*window);
        visible = title.len() != 0 && IsWindowVisible(*window) != 0 && title != "Program Manager";
    }

    visible
}