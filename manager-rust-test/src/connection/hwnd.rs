use winapi::{
    shared::{
        minwindef::{BOOL, LPARAM},
        windef::HWND,
    },
    um::winuser::EnumWindows,
};

use super::info::is_window_visible;

pub fn get_visible_windows_hwnds() -> Vec<HWND> {
    let result: Vec<HWND> = get_window_all_hwnds()
        .iter()
        .filter(|hwnd| is_window_visible(&hwnd))
        .cloned()
        .collect();

    result
}

pub fn get_window_all_hwnds() -> Vec<HWND> {
    let state: Box<Vec<HWND>> = Box::new(Vec::new());
    let ptr = Box::into_raw(state);
    let state;
    unsafe {
        EnumWindows(Some(enumerate_all_windows_hwnds), ptr as LPARAM);
        state = Box::from_raw(ptr);
    }
    *state
}

unsafe extern "system" fn enumerate_all_windows_hwnds(window: HWND, state: LPARAM) -> BOOL {
    let state = state as *mut Vec<HWND>;
    (*state).push(window);

    true.into()
}