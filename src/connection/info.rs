use winapi::{shared::windef::HWND, um::winuser::IsWindowVisible};

use super::{size::get_window_placement, title::get_title};

pub fn is_window_visible(window: &HWND) -> bool {
    let visible;
    let placement = get_window_placement(window);
    unsafe {
        let title: String = get_title(*window);
        visible = title.len() != 0 && IsWindowVisible(*window) != 0 && placement.showCmd != 1;
    }

    visible
}
