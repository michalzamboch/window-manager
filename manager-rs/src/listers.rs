#![allow(dead_code)]

use winapi::shared::windef::HWND;

use crate::connection::{hwnd::*, size::*, title::*};

pub fn list_windows_info_trimmed_titles() {
    let hwnds = get_visible_windows_hwnds();
    
    for hwnd in hwnds {
        let rect = get_window_rect(&hwnd);
        let trimmed_title = get_trimmed_title(&get_title(hwnd));
        println!("{:?}: {} {}", hwnd, trimmed_title, rect_to_string(&rect));
    }
}

pub fn list_windows_info() {
    let hwnds = get_visible_windows_hwnds();
    
    for hwnd in hwnds {
        let rect = get_window_rect(&hwnd);
        println!("{:?}: {} {}", hwnd, get_title(hwnd), rect_to_string(&rect));
    }
}

pub fn list_all_hwnds() {
    let hwnds = get_window_all_hwnds();
    list_hwnds(hwnds);
}

fn list_hwnds(hwnds: Vec<HWND>) {
    for hwnd in hwnds {
        println!("{:?}", hwnd);
    }
}

pub fn list_all_window() {
    let titles = get_window_all_titles();
    list_windows(titles);
}

fn list_windows(window_titles: Vec<String>) {
    for title in window_titles {
        println!("{}", title);
    }
}

pub fn list_wisible_windows() {
    let titles = get_windows_visible_titles();

    titles.into_iter().for_each(|title| println!("{}", title));
}

pub fn list_window_placements() {
    let hwnds = get_visible_windows_hwnds();
    
    for hwnd in hwnds {
        let placement = get_window_placement(&hwnd);
        println!("{} {}", get_title(hwnd), window_placement_string(&placement));
    }
}

pub fn list_window_rects() {
    let hwnds = get_visible_windows_hwnds();
    
    for hwnd in hwnds {
        let rect = get_window_rect(&hwnd);
        println!("{} {}", get_title(hwnd), rect_to_string(&rect));
    }
}