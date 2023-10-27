#![allow(dead_code)]

use winapi::{
    shared::windef::{HWND, POINT, RECT},
    um::winuser::{
        self, GetWindowPlacement, GetWindowRect, SetRect, SetWindowPos, SW_SHOWNORMAL,
        WINDOWPLACEMENT,
    },
};

pub fn get_window_placement(hwnd: &HWND) -> WINDOWPLACEMENT {
    let mut wp = get_empty_window_placement();

    unsafe {
        GetWindowPlacement(*hwnd, &mut wp);
    }

    wp
}

fn get_empty_window_placement() -> WINDOWPLACEMENT {
    WINDOWPLACEMENT {
        length: 0,
        flags: 0,
        showCmd: SW_SHOWNORMAL as u32,
        ptMinPosition: POINT { x: 0, y: 0 },
        ptMaxPosition: POINT { x: 0, y: 0 },
        rcNormalPosition: RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        },
    }
}

pub fn set_window_pos(hwnd: &HWND, new_rect: RECT) {
    unsafe {
        SetWindowPos(
            *hwnd,
            winuser::HWND_TOPMOST,
            new_rect.right,
            new_rect.top,
            new_rect.left,
            new_rect.bottom,
            winuser::SWP_SHOWWINDOW,
        );
    }
}

fn set_rect(rect: &mut RECT, x: i32, y: i32, x2: i32, y2: i32) {
    let boxed_rect = Box::new(*rect);
    let ptr = Box::into_raw(boxed_rect);

    unsafe {
        SetRect(ptr, x, y, x2, y2);
    }
}

fn set_rect_values(rect: &mut RECT, x: i32, y: i32, x2: i32, y2: i32) {
    rect.right = x;
    rect.left = x2;
    rect.top = y;
    rect.bottom = y2;
}

pub fn set_coords(wp: &mut WINDOWPLACEMENT, x: i32, y: i32, w: i32, h: i32) {
    wp.showCmd = SW_SHOWNORMAL as u32;
    wp.ptMinPosition.x = 100;
    wp.ptMinPosition.y = 100;
    wp.ptMaxPosition.x = 800;
    wp.ptMaxPosition.y = 600;
    wp.rcNormalPosition.left = x;
    wp.rcNormalPosition.top = y;
    wp.rcNormalPosition.right = x + w;
    wp.rcNormalPosition.bottom = y + h;
}

pub fn window_placement_string(wp: &WINDOWPLACEMENT) -> String {
    format!(
        "cmd {} length {} flags {} min.x {} min.y {} max.x {} max.y {} left {} top {} right {} bottom {}",
        wp.showCmd,
        wp.length,
        wp.flags,
        wp.ptMinPosition.x,
        wp.ptMinPosition.y,
        wp.ptMaxPosition.x,
        wp.ptMaxPosition.y,
        wp.rcNormalPosition.left,
        wp.rcNormalPosition.top,
        wp.rcNormalPosition.right,
        wp.rcNormalPosition.bottom
    )
}

pub fn print_window_placement(wp: &WINDOWPLACEMENT) {
    println!("{}", window_placement_string(wp));
}

pub fn print_window_placement_hwnd(hwnd: &HWND) {
    let wp = &get_window_placement(hwnd);
    println!("{}", window_placement_string(wp));
}

pub fn rect_to_string(rect: &RECT) -> String {
    format!(
        "left {} top {} right {} bottom {}",
        rect.left, rect.top, rect.right, rect.bottom
    )
}

pub fn get_window_rect(hwnd: &HWND) -> RECT {
    let rect = get_empty_rect();
    let boxed_rect = Box::new(rect);
    let ptr = Box::into_raw(boxed_rect);

    unsafe {
        GetWindowRect(*hwnd, ptr);

        *ptr
    }
}

pub fn get_empty_rect() -> RECT {
    RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    }
}
