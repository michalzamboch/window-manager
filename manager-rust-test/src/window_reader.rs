use winapi::{
    shared::{
        minwindef::{BOOL, LPARAM},
        windef::HWND,
    },
    um::{
        winnt::LPWSTR,
        winuser::{EnumWindows, GetWindowTextLengthW, GetWindowTextW, IsWindowVisible},
    },
};

use crate::{types::ConnectionTrait, types::Result};

pub struct Connection;
impl ConnectionTrait for Connection {
    fn new() -> Result<Self> {
        Ok(Self)
    }

    fn window_titles(&self) -> Result<Vec<String>> {
        let state: Box<Vec<String>> = Box::new(Vec::new());
        let ptr = Box::into_raw(state);
        let state;
        unsafe {
            EnumWindows(Some(enumerate_filtered_windows), ptr as LPARAM);
            state = Box::from_raw(ptr);
        }
        Ok(*state)
    }
}

fn get_filtered_window_titles() -> Result<Vec<String>> {
    let state: Box<Vec<String>> = Box::new(Vec::new());
    let ptr = Box::into_raw(state);
    let state;
    unsafe {
        EnumWindows(Some(enumerate_filtered_windows), ptr as LPARAM);
        state = Box::from_raw(ptr);
    }
    Ok(*state)
}

unsafe extern "system" fn enumerate_filtered_windows(window: HWND, state: LPARAM) -> BOOL {
    if IsWindowVisible(window) == 0 {
        return true.into();
    }

    let state = state as *mut Vec<String>;
    let mut length = GetWindowTextLengthW(window);

    if length == 0 {
        return true.into();
    }

    length = length + 1;
    let mut title: Vec<u16> = vec![0; length as usize];
    let textw = GetWindowTextW(window, title.as_mut_ptr() as LPWSTR, length);

    if textw != 0 {
        if let Ok(title) = String::from_utf16(title[0..(textw as usize)].as_ref()) {
            (*state).push(title);
        }
    }

    true.into()
}

// ------------------------------------------------------

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

pub fn get_window_all_titles() -> Vec<String> {
    let state: Box<Vec<String>> = Box::new(Vec::new());
    let ptr = Box::into_raw(state);
    let state;
    unsafe {
        EnumWindows(Some(enumerate_all_windows), ptr as LPARAM);
        state = Box::from_raw(ptr);
    }
    *state
}

unsafe extern "system" fn enumerate_all_windows(window: HWND, state: LPARAM) -> BOOL {
    let state = state as *mut Vec<String>;
    let title = get_title(window);

    (*state).push(title);
    true.into()
}

unsafe fn get_title(window: HWND) -> String {
    let mut length = GetWindowTextLengthW(window);

    if length == 0 {
        return "".into();
    }

    length = length + 1;
    let mut title: Vec<u16> = vec![0; length as usize];
    let textw = GetWindowTextW(window, title.as_mut_ptr() as LPWSTR, length);

    if textw == 0 {
        return "".into();
    }

    match String::from_utf16(title[0..(textw as usize)].as_ref()) {
        Ok(title) => return title,
        Err(_) => return "".into(),
    }
}

pub fn visible_window(window: HWND) -> bool {
    let visible;    
    unsafe {
        visible = get_title(window).len() != 0 && IsWindowVisible(window) != 0;
    }

    visible
}
