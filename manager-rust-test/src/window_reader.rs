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

pub fn is_window_visible(window: &HWND) -> bool {
    let visible;
    unsafe {
        let title: String = get_title(*window);
        visible = title.len() != 0 && IsWindowVisible(*window) != 0 && title != "Program Manager";
    }

    visible
}

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

pub fn get_window_all_titles() -> Vec<String> {
    let state: Box<Vec<String>> = Box::new(Vec::new());
    let ptr = Box::into_raw(state);
    let state;
    unsafe {
        EnumWindows(Some(enumerate_all_windows_title), ptr as LPARAM);
        state = Box::from_raw(ptr);
    }
    *state
}

pub fn get_windows_visible_titles() -> Vec<String> {
    let result: Vec<String> = get_visible_windows_hwnds()
        .iter()
        .map(|window| get_title(window.clone()))
        .collect();

    result
}

unsafe extern "system" fn enumerate_all_windows_title(window: HWND, state: LPARAM) -> BOOL {
    let state = state as *mut Vec<String>;
    let title = get_title(window);

    (*state).push(title);
    true.into()
}

fn get_title(window: HWND) -> String {
    let mut title: Vec<u16>;
    let textw ;

    unsafe {
        let mut length = GetWindowTextLengthW(window);

        if length == 0 {
            return "".into();
        }

        length = length + 1;
        title = vec![0; length as usize];
        textw = GetWindowTextW(window, title.as_mut_ptr() as LPWSTR, length);

        if textw == 0 {
            return "".into();
        }
    }

    match String::from_utf16(title[0..(textw as usize)].as_ref()) {
        Ok(title) => return title,
        Err(_) => return "".into(),
    }
}
