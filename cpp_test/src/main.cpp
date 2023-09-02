#include <windows.h>
#include <iostream>
#include <vector>
#include <sstream>

#include "WinRect.h"

BOOL CALLBACK speichereFenster(HWND hwnd, LPARAM lParam){
    const DWORD TITLE_SIZE = 1024;
    WCHAR windowTitle[TITLE_SIZE];

    GetWindowTextW(hwnd, windowTitle, TITLE_SIZE);

    int length = ::GetWindowTextLength(hwnd);
    std::wstring title(&windowTitle[0]);
    if (!IsWindowVisible(hwnd) || length == 0 || title == L"Program Manager") {
        return TRUE;
    }
    
    // Retrieve the pointer passed into this callback, and re-'type' it.
    // The only way for a C API to pass arbitrary data is by means of a void*.
    std::vector<std::wstring>& titles =
                              *reinterpret_cast<std::vector<std::wstring>*>(lParam);
    titles.push_back(title);

    return TRUE;
}

BOOL CALLBACK GetWindowsRects(HWND hwnd, LPARAM lParam){
    RECT rect;

    GetWindowRect(hwnd, &rect);

    if (!IsWindowVisible(hwnd)) {
        return TRUE;
    }
    
    WinRect winRect(rect);
    std::vector<WinRect>& rectangles = *reinterpret_cast<std::vector<WinRect>*>(lParam);
    rectangles.push_back(winRect);

    return TRUE;
}

int main() {
    std::vector<std::wstring> titles;
    EnumWindows(speichereFenster, reinterpret_cast<LPARAM>(&titles));
    
    for ( auto& title : titles )
        std::wcout << L"Title: " << title << std::endl;

    std::vector<WinRect> rectangles;
    EnumWindows(GetWindowsRects, reinterpret_cast<LPARAM>(&rectangles));
    for (auto& rect : rectangles)
    {
        rect.print();
    }
    
    return 0;
}